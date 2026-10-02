/// Replace `г'` with `ґ`, unless followed by one of `еёіюя`.
///
/// Equivalent to `/г'(?![еёіюя])/g`, but without the regex engine:
/// single pass over the input, one allocation (output is never longer
/// than the input: `г'` shrinks to `ґ`), SIMD-accelerated scanning via
/// `str::find('\'')`, and a `starts_with` lookahead for `[еёіюя]`.
pub(crate) fn replace_g_apostrophe(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut flush_from = 0usize;
    let mut search_from = 0usize;
    // `'` is rare, so jumping between apostrophes with the vectorized
    // `find` keeps the hot loop in SIMD `memchr` instead of a per-byte
    // branch. Slicing is safe: `search_from`/`flush_from` always follow
    // an ASCII `'` (or 0), and `pos - 'г'.len_utf8()` points at a `г`
    // char boundary in valid UTF-8.
    while let Some(rel) = text[search_from..].find('\'') {
        let pos = search_from + rel; // byte index of `'`
        search_from = pos + 1;
        // Must be preceded by `г` (byte `memcmp`, no decoding).
        if !text[..pos].ends_with("г") {
            continue;
        }
        // Negative lookahead for [еёіюя].
        let after = &text[pos + 1..];
        let look = after.chars().next();
        if matches!(look, Some('е' | 'ё' | 'і' | 'ю' | 'я')) {
            continue;
        }
        out.push_str(&text[flush_from..pos - 'г'.len_utf8()]);
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
