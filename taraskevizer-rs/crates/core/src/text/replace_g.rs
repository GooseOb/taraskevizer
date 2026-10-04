//! `Ґ → Г` / `ґ → г` mapping helpers (the `g` step and highlight comparison).

use std::borrow::Cow;

/// Shared scan behind [`replace_g_str`] and [`replace_g_with_map`].
///
/// Finds each `Ґ`/`ґ` (both share the `0xD2` lead byte: `Ґ = D2 90`,
/// `ґ = D2 91`, so a single SIMD `memchr` scan finds every candidate),
/// bulk-copies the gaps, and calls `push_mapped` with the original char
/// (`'Ґ'` or `'ґ'`) for each hit. Borrows the input when no target is
/// present (the common case — zero allocation).
///
/// Slicing is safe: `&str` is valid UTF-8, hence each `0xD2` byte starts a
/// 2-byte sequence (so `pos + 1` is in bounds) and every split below lands
/// on a char boundary.
fn map_g_targets(text: &str, mut push_mapped: impl FnMut(char, &mut String)) -> Cow<'_, str> {
    let bytes = text.as_bytes();
    let mut from = 0usize;
    let mut flush = 0usize;
    let mut out: Option<String> = None;
    while let Some(rel) = memchr::memchr(0xD2, &bytes[from..]) {
        let pos = from + rel;
        let target = match bytes[pos + 1] {
            0x90 => 'Ґ',
            0x91 => 'ґ',
            // Same lead byte, different char (e.g. `Ғ` = D2 92).
            _ => {
                from = pos + 1;
                continue;
            }
        };
        let buf = out.get_or_insert_with(|| String::with_capacity(text.len()));
        buf.push_str(&text[flush..pos]);
        push_mapped(target, buf);
        flush = pos + 2;
        from = flush;
    }
    match out {
        Some(mut buf) => {
            buf.push_str(&text[flush..]);
            Cow::Owned(buf)
        }
        None => Cow::Borrowed(text),
    }
}

/// Map `Ґ → Г`, `ґ → г`, borrowing the input when neither is present.
pub(crate) fn replace_g_str(text: &str) -> Cow<'_, str> {
    // `ch` is always `Ґ` or `ґ` here (driver-filtered).
    map_g_targets(text, |ch, buf| {
        buf.push(if ch == 'Ґ' { 'Г' } else { 'г' })
    })
}

/// Replace each `Ґ`/`ґ` with `f(ch)`, borrowing the input when neither is
/// present.
pub(crate) fn replace_g_with_map(text: &str, f: impl Fn(char) -> String) -> Cow<'_, str> {
    map_g_targets(text, |ch, buf| buf.push_str(&f(ch)))
}

#[cfg(test)]
mod tests {
    use super::{replace_g_str, replace_g_with_map};
    use std::borrow::Cow;

    #[test]
    fn maps_both_cases() {
        assert_eq!(replace_g_str("ґазета Ґедзь").into_owned(), "газета Гедзь");
        assert_eq!(replace_g_str("аҐ").into_owned(), "аГ");
        assert_eq!(replace_g_str("ґ").into_owned(), "г");
    }

    #[test]
    fn borrows_when_no_g() {
        assert!(matches!(replace_g_str("планета"), Cow::Borrowed(_)));
        assert!(matches!(replace_g_str(""), Cow::Borrowed(_)));
    }

    #[test]
    fn ignores_same_lead_byte_chars() {
        // `Ғ` = D2 92 shares the lead byte with `Ґ`/`ґ` but passes through.
        assert!(matches!(replace_g_str("Ғ"), Cow::Borrowed(_)));
        assert!(matches!(replace_g_str("аҒб"), Cow::Borrowed(_)));
        assert_eq!(replace_g_str("Ғґ").into_owned(), "Ғг");
    }

    #[test]
    fn with_map_wraps_only_g() {
        assert_eq!(
            replace_g_with_map("аґбҐв", |ch| format!("<{ch}>")).into_owned(),
            "а<ґ>б<Ґ>в"
        );
    }

    #[test]
    fn with_map_borrows_when_no_g() {
        assert!(matches!(
            replace_g_with_map("планета", |ch| format!("<{ch}>")),
            Cow::Borrowed(_)
        ));
    }
}
