use super::utf8_char_len;

/// Replace `'`, `` ` ``, `’` with `ʼ` when followed by a non-whitespace char.
///
/// Equivalent to `/['`’](?=\S)/g`, but without the regex engine: single
/// pass, one allocation, byte-level scanning. The lookahead uses
/// `char::is_whitespace`, which was probed to match `fancy_regex`'s `\S`
/// exactly (blocks on the full Unicode White_Space set, replaces before
/// U+FEFF/U+200B, fails at end-of-string).
pub(crate) fn normalize_apostrophes(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    // Quotes are rare, so skip whole chars to step over Cyrillic text
    // 2 bytes at a time instead of byte-by-byte. `i` always stays on a
    // char boundary (`&str` is valid UTF-8), so all slicing is safe.
    while i < len {
        let b = bytes[i];
        // (quote, byte length): `'` = 27, `` ` `` = 60, `’` = E2 80 99.
        let qlen = if b == 0x27 || b == 0x60 {
            1
        } else if b == 0xE2 && i + 3 <= len && bytes[i + 1] == 0x80 && bytes[i + 2] == 0x99
        {
            3
        } else {
            i += utf8_char_len(b);
            continue;
        };
        // Lookahead `(?=\S)`: end-of-string or whitespace blocks the change.
        let replace = match text[i + qlen..].chars().next() {
            None => false,
            Some(c) => !c.is_whitespace(),
        };
        if replace {
            out.push_str(&text[flush_from..i]);
            out.push('ʼ');
            flush_from = i + qlen;
        }
        i += qlen;
    }
    out.push_str(&text[flush_from..]);
    out
}

#[cfg(test)]
mod tests {
    use super::normalize_apostrophes;

    fn check_apos(input: &str, expected: &str) {
        assert_eq!(normalize_apostrophes(input), expected, "input: {input:?}");
    }

    #[test]
    fn apos_replaces_before_non_space() {
        check_apos("a'b", "aʼb");
        check_apos("a`b", "aʼb");
        check_apos("a’b", "aʼb");
        check_apos("'a", "ʼa");
        check_apos("''a", "ʼʼa");
        check_apos("’’a", "ʼʼa");
        check_apos("a''b", "aʼʼb");
        check_apos("don't", "donʼt");
        check_apos("a'.", "aʼ.");
        check_apos("a'😀", "aʼ😀");
    }

    #[test]
    fn apos_keeps_before_whitespace_or_eos() {
        check_apos("", "");
        check_apos("'", "'");
        check_apos("`", "`");
        check_apos("’", "’");
        check_apos("a'", "a'");
        check_apos("a' ", "a' ");
        check_apos("a'  b", "a'  b");
        check_apos("a'\tb", "a'\tb");
        check_apos("a'\nb", "a'\nb");
        check_apos("a'\u{00A0}b", "a'\u{00A0}b");
        check_apos("a'\u{2028}b", "a'\u{2028}b");
        // Already-normalized modifier stays untouched.
        check_apos("aʼb", "aʼb");
        // U+FEFF is not whitespace for `\S`: still replaces.
        check_apos("a'\u{FEFF}b", "aʼ\u{FEFF}b");
    }

    #[test]
    fn apos_matches_fancy_regex() {
        let re = fancy_regex::Regex::new(r"['`’](?=\S)").unwrap();
        let spaces = [
            " ", "\t", "\n", "\r", "\u{00A0}", "\u{0085}", "\u{1680}", "\u{2000}",
            "\u{2009}", "\u{2028}", "\u{2029}", "\u{202F}", "\u{205F}", "\u{3000}",
            "\u{FEFF}", "\u{200B}",
        ];
        let mut inputs = vec![
            String::new(),
            "'".into(),
            "`".into(),
            "’".into(),
            "ʼ".into(),
            "a'b".into(),
            "''a".into(),
            "a''".into(),
            "’'’".into(),
            "пад'ём аб'яднанне".into(),
            "— ’quote’ —".into(),
            "emoji 😀'x".into(),
            "г'е".into(),
        ];
        for w in spaces {
            for q in ["'", "`", "’"] {
                inputs.push(format!("a{q}{w}b"));
            }
        }
        for input in &inputs {
            let expected = re.replace_all(input, "ʼ").into_owned();
            assert_eq!(normalize_apostrophes(input), expected, "input: {input:?}");
        }
    }
}
