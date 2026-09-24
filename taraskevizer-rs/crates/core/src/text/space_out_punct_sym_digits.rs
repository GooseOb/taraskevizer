use super::{is_ascii_punct_sym, is_spaced_cluster_char};

/// Wrap runs of Unicode punctuation/symbols, decimal digits, and U+FEFF in spaces.
///
/// Like `/\p{P}|\p{S}|\d+/gu` with `" $0 "`, except consecutive matches form
/// a single cluster wrapped only at its edges (`"!?"` → `" !? "` instead of
/// `" !  ? "`). Single pass, one allocation, byte-level scanning with
/// table-driven classification (`\d` is `\p{Nd}`; U+FEFF is Cf and handled
/// alongside). `i`/`j` always stay on char boundaries, so slicing is safe.
pub(crate) fn space_out_punct_sym_digits(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        let first_len = if b < 0x80 && !b.is_ascii_digit() {
            if !is_ascii_punct_sym(b) {
                i += 1;
                continue;
            }
            1
        } else {
            // `i` is a boundary of valid UTF-8, so this always yields a char.
            let c = text[i..].chars().next().unwrap_or('\0');
            if is_spaced_cluster_char(c) {
                c.len_utf8()
            } else {
                i += c.len_utf8();
                continue;
            }
        };
        // Extend the cluster over following P/S/digit/FEFF chars.
        let mut j = i + first_len;
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
        out.push_str(&text[flush_from..i]);
        out.push(' ');
        out.push_str(&text[i..j]);
        out.push(' ');
        flush_from = j;
        i = j;
    }
    out.push_str(&text[flush_from..]);
    out
}

#[cfg(test)]
mod tests {
    use super::space_out_punct_sym_digits;

    fn check_space(input: &str, expected: &str) {
        assert_eq!(
            space_out_punct_sym_digits(input),
            expected,
            "input: {input:?}"
        );
    }

    #[test]
    fn space_basic() {
        check_space("", "");
        check_space("hello", "hello");
        check_space("плянэта", "плянэта");
        check_space("a,b", "a , b");
        check_space("a.b!c", "a . b ! c");
        check_space("(a)", " ( a ) ");
        // Consecutive matches merge into one cluster wrapped at the edges.
        check_space("!?", " !? ");
        check_space("«плянэта»", " « плянэта » ");
        check_space("a—b", "a — b");
        // ASCII `'` is Po: wrapped, not normalized, by this step.
        check_space("don't", "don ' t");
        // Already-normalized modifier (Lm) stays untouched.
        check_space("aʼb", "aʼb");
    }

    #[test]
    fn space_digits_and_feff() {
        // Digit runs (possibly mixed-script) merge with touching punct.
        check_space("abc123def", "abc 123 def");
        check_space("12", " 12 ");
        check_space("1,2", " 1,2 ");
        check_space("100%", " 100% ");
        check_space("3.14", " 3.14 ");
        check_space("+375 29 123-45-67", " +375   29   123-45-67 ");
        // `\d` is `\p{Nd}`: non-ASCII decimal digits wrap too.
        check_space("٣٤٥", " ٣٤٥ ");
        check_space("1٣2", " 1٣2 ");
        // U+FEFF (Cf) is wrapped by the merged-in second operation.
        check_space("\u{FEFF}", " \u{FEFF} ");
        check_space("a\u{FEFF}b", "a \u{FEFF} b");
        check_space("\u{FEFF}\u{FEFF}", " \u{FEFF}\u{FEFF} ");
    }

    #[test]
    fn space_clusters() {
        // Adjacent matches merge into one cluster wrapped at the edges.
        check_space("!?", " !? ");
        check_space("1,2", " 1,2 ");
        check_space("12%", " 12% ");
        check_space("100%!", " 100%! ");
        check_space("3.14", " 3.14 ");
        check_space("(...)", " (...) ");
        check_space("\u{FEFF}\u{FEFF}", " \u{FEFF}\u{FEFF} ");
        check_space("1٣2", " 1٣2 ");
        check_space("a,b.c!d?e:f;g", "a , b . c ! d ? e : f ; g");
        check_space("abc123def", "abc 123 def");
        check_space("+375 29 123-45-67", " +375   29   123-45-67 ");
        check_space("Я 77-ы", "Я  77- ы");
        check_space("emoji 😀🎉!", "emoji  😀🎉! ");
        check_space("0x123", " 0 x 123 ");
        check_space("12 34", " 12   34 ");
        check_space("100\u{00A0}000", " 100 \u{00A0} 000 ");
        check_space(" \u{FEFF} ", "  \u{FEFF}  ");
    }

    #[test]
    fn space_exhaustive_matches_fancy_regex() {
        let re = fancy_regex::Regex::new(r"\p{P}|\p{S}|\d+").unwrap();
        // Every BMP char isolated by '\n' (Cc: never matches, breaks digit
        // runs), so one batched call checks each char independently.
        // U+FEFF is skipped: Cf is outside P/S and covered by its own tests.
        let mut batch = String::new();
        for cp in 0u32..0x10000 {
            if (0xD800..0xE000).contains(&cp) || cp == 0xFEFF {
                continue;
            }
            batch.push(char::from_u32(cp).unwrap());
            batch.push('\n');
        }
        // Astral range edges (where table errors would live) plus strided
        // samples across the supplementary planes.
        let mut extra = Vec::new();
        for &(lo, hi) in super::super::is_punct_or_symbol::PUNCT_SYM_RANGES.iter().filter(|&&(_, hi)| hi >= 0x10000) {
            for cp in [lo.saturating_sub(1), lo, hi, (hi + 1).min(0x10FFFF)] {
                if char::from_u32(cp).is_some() {
                    extra.push(cp);
                }
            }
        }
        let mut cp = 0x10000u32;
        while cp <= 0x10FFFF {
            extra.push(cp);
            cp += 997;
        }
        for cp in extra {
            batch.push(char::from_u32(cp).unwrap());
            batch.push('\n');
        }
        let expected = re.replace_all(&batch, " $0 ").into_owned();
        assert_eq!(space_out_punct_sym_digits(&batch), expected);
    }
}
