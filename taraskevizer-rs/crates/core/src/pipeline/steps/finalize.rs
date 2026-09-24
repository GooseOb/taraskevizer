use super::prepare::{is_ascii_punct_sym, is_spaced_cluster_char, utf8_char_len};
use crate::pipeline::PipelineContext;

/// Strip single spaces flanking punctuation/symbol/digit/BOM clusters.
///
/// Inverse of the clustered spacing in `step_prepare`: `␣<cluster>␣` →
/// `<cluster>`, where a cluster is 1+ consecutive P/S/Nd/FEFF chars (same
/// predicate as the spacing step). A strip happens only with a literal
/// space on BOTH sides; scanning is non-overlapping left-to-right, so
/// `" +375   29 "` collapses to `"+375 29"`. Single pass, one allocation
/// (output never grows).
pub(crate) fn unspace_punct_sym_digits(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    // `i` always stays on a char boundary, so all slicing is safe.
    while i < len {
        // A strip needs a leading literal space.
        if bytes[i] != b' ' {
            i += if bytes[i] < 0x80 {
                1
            } else {
                utf8_char_len(bytes[i])
            };
            continue;
        }
        // The cluster must start right after it…
        let mut j = i + 1;
        let first_ok = if j < len && bytes[j] < 0x80 {
            bytes[j].is_ascii_digit() || is_ascii_punct_sym(bytes[j])
        } else if j < len {
            // `j` is a boundary of valid UTF-8, so this always yields a char.
            is_spaced_cluster_char(text[j..].chars().next().unwrap_or('\0'))
        } else {
            false
        };
        if !first_ok {
            i += 1;
            continue;
        }
        // …extend over following cluster chars…
        j += if bytes[j] < 0x80 {
            1
        } else {
            text[j..].chars().next().unwrap_or('\0').len_utf8()
        };
        while j < len {
            let nb = bytes[j];
            if nb < 0x80 {
                if nb.is_ascii_digit() || is_ascii_punct_sym(nb) {
                    j += 1;
                } else {
                    break;
                }
            } else {
                let nc = text[j..].chars().next().unwrap_or('\0');
                if is_spaced_cluster_char(nc) {
                    j += nc.len_utf8();
                } else {
                    break;
                }
            }
        }
        // …and end right before a trailing literal space.
        if j >= len || bytes[j] != b' ' {
            i += 1;
            continue;
        }
        out.push_str(&text[flush_from..i]);
        out.push_str(&text[i + 1..j]);
        flush_from = j + 1;
        i = j + 1;
    }
    out.push_str(&text[flush_from..]);
    out
}

pub fn step_finalize(ctx: &mut PipelineContext) {
    let text = std::mem::take(&mut ctx.text);
    let mut t = text.replace("&#40", "(").replace("&nbsp;", " ");
    t = unspace_punct_sym_digits(&t);
    t = t.replace(
        &format!(" {} ", ctx.cfg.left_angle_bracket),
        ctx.cfg.left_angle_bracket.as_str(),
    );
    if ctx.cfg.new_line != "\n" {
        t = t.replace('\n', &ctx.cfg.new_line);
    }
    ctx.text = t.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::unspace_punct_sym_digits;

    fn check(input: &str, expected: &str) {
        assert_eq!(unspace_punct_sym_digits(input), expected, "input: {input:?}");
    }

    #[test]
    fn unspaces_clusters() {
        check("", "");
        check("hello", "hello");
        check("  ", "  ");
        check(" , ", ",");
        check(" !? ", "!?");
        check(" 12% ", "12%");
        check(" (...) ", "(...)");
        check(" \u{FEFF} ", "\u{FEFF}");
        check(" \u{FEFF}\u{FEFF} ", "\u{FEFF}\u{FEFF}");
        // Single spaces around an isolated mark collapse (long-standing).
        check("a , b", "a,b");
        // One level of added spacing comes off, originals stay.
        check("a  ,  b", "a , b");
        check(" 12   34 ", "12 34");
        check(" +375   29 ", "+375 29");
    }

    #[test]
    fn keeps_without_flanking_spaces() {
        // Stripping needs a literal space on BOTH sides.
        check(", b", ", b");
        check("a ,", "a ,");
        check(",", ",");
        check("a,b", "a,b");
        check("12", "12");
        check("a\n,\nb", "a\n,\nb");
    }

    #[test]
    fn round_trips_prepare_spacing() {
        use super::super::prepare::space_out_punct_sym_digits;
        let inputs = [
            "",
            "hello",
            "плянэта",
            "a,b",
            "a , b",
            "(a)",
            "a,,b",
            "a  ,  b",
            "  ",
            " , ",
            ",",
            ",b",
            "a,",
            "!?",
            "abc123def",
            "1,2",
            "12%",
            "100%!",
            "+375 29 123-45-67",
            "0x123",
            "3.14",
            "12 34",
            "٣٤٥",
            "1٣2",
            "² ½",
            "ⅠⅡⅢ",
            "a\u{FEFF}b",
            "\u{FEFF}",
            "\u{FEFF}\u{FEFF}",
            " \u{FEFF} ",
            "aʼb",
            "don't",
            "«плянэта»",
            "Я 77-ы",
            "emoji 😀🎉!",
            "100\u{00A0}000",
            "e\u{301}cole",
        ];
        for input in inputs {
            assert_eq!(
                unspace_punct_sym_digits(&space_out_punct_sym_digits(input)),
                input,
                "input: {input:?}"
            );
        }
    }
}
