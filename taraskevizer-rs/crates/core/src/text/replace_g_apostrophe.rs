/// Replace `г'` with `ґ`, unless followed by one of `еёіюя`.
///
/// Equivalent to `/г'(?![еёіюя])/g`, but without the regex engine:
/// single pass over the input, one allocation (output is never longer
/// than the input: 3 bytes `D0 B3 27` shrink to 2 bytes `D2 91`),
/// SIMD-accelerated scanning via `str::find('\'')`, and a byte-level
/// lookahead (`е`=D0 B5, `ё`=D1 91, `і`=D1 96, `ю`=D1 8E, `я`=D1 8F).
pub(crate) fn replace_g_apostrophe(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut flush_from = 0usize;
    let mut search_from = 0usize;
    // `'` is rare, so jumping between apostrophes with the vectorized
    // `find` keeps the hot loop in SIMD `memchr` instead of a per-byte
    // branch. Slicing is safe: `search_from`/`flush_from` always follow
    // an ASCII `'` (or 0), and `pos - 2` points at a `0xD0` lead byte,
    // which in valid UTF-8 can only start a char.
    while let Some(rel) = text[search_from..].find('\'') {
        let pos = search_from + rel; // byte index of `'`
        search_from = pos + 1;
        // Must be preceded by `г` (D0 B3).
        if pos < 2 || bytes[pos - 2] != 0xD0 || bytes[pos - 1] != 0xB3 {
            continue;
        }
        // Negative lookahead for [еёіюя].
        let after = pos + 1;
        let keep = if after >= bytes.len() {
            false
        } else {
            let b1 = bytes[after];
            if b1 == 0xD0 {
                after + 1 < bytes.len() && bytes[after + 1] == 0xB5
            } else if b1 == 0xD1 {
                after + 1 < bytes.len() && matches!(bytes[after + 1], 0x91 | 0x96 | 0x8E | 0x8F)
            } else {
                false
            }
        };
        if keep {
            continue;
        }
        out.push_str(&text[flush_from..pos - 2]);
        out.push('ґ');
        flush_from = pos + 1;
    }
    out.push_str(&text[flush_from..]);
    out
}

#[cfg(test)]
mod tests {
    use super::replace_g_apostrophe;

    fn check(input: &str, expected: &str) {
        assert_eq!(replace_g_apostrophe(input), expected, "input: {input:?}");
    }

    #[test]
    fn replaces_plain() {
        check("г'", "ґ");
        check("аг'б", "аґб");
        check("г' г'", "ґ ґ");
        check("г'а г'о", "ґа ґо");
    }

    #[test]
    fn lookahead_blocks() {
        for ch in ['е', 'ё', 'і', 'ю', 'я'] {
            let input = format!("г'{ch}");
            check(&input, &input);
        }
        // End of string still replaces (negative lookahead succeeds).
        check("г'", "ґ");
    }

    #[test]
    fn no_false_positives() {
        check("", "");
        check("hello", "hello");
        check("'", "'");
        check("г", "г");
        check("Г'", "Г'"); // uppercase untouched, like the original regex
        check("а'е", "а'е");
        check("гʼе", "гʼе"); // U+02BC modifier, not ASCII apostrophe
                             // Uppercase lookahead letters don't block (regex class is lowercase-only).
        check("г'Е", "ґЕ");
        check("г'Ё", "ґЁ");
    }

    #[test]
    fn mixed_runs() {
        check("г'е г'а г'ю г'б", "г'е ґа г'ю ґб");
        check("г''", "ґ'");
        check("'г", "'г");
        check("г' г''", "ґ ґ'");
    }

    #[test]
    fn matches_fancy_regex() {
        let re = fancy_regex::Regex::new(r"г'(?![еёіюя])").unwrap();
        let inputs = [
            "",
            "'",
            "г",
            "г'",
            "г'е",
            "г'ё",
            "г'і",
            "г'ю",
            "г'я",
            "г'а",
            "г'Е",
            "г' ",
            "г'.",
            "prefix г' suffix",
            "г'г'",
            "г''г'",
            "аб'г'в",
            "г'ег'а",
            "Г' г'",
            "г'\u{301}е",
            "emoji 😀 г' end",
        ];
        for input in inputs {
            let expected = re.replace_all(input, "ґ").into_owned();
            assert_eq!(replace_g_apostrophe(input), expected, "input: {input:?}");
        }
    }
}
