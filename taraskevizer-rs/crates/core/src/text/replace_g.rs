//! `Ґ → Г` / `ґ → г` mapping helpers (the `g` step and highlight comparison).

pub(crate) fn replace_g_str(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        out.push(match ch {
            'Ґ' => 'Г',
            'ґ' => 'г',
            _ => ch,
        });
    }
    out
}

pub(crate) fn replace_g_with_map(text: &str, f: impl Fn(char) -> String) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch == 'Ґ' || ch == 'ґ' {
            out.push_str(&f(ch));
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{replace_g_str, replace_g_with_map};

    #[test]
    fn maps_both_cases() {
        assert_eq!(replace_g_str("ґазета Ґедзь"), "газета Гедзь");
    }

    #[test]
    fn leaves_other_chars() {
        assert_eq!(replace_g_str("планета"), "планета");
        assert_eq!(replace_g_str(""), "");
    }

    #[test]
    fn with_map_wraps_only_g() {
        assert_eq!(
            replace_g_with_map("аґбҐв", |ch| format!("<{ch}>")),
            "а<ґ>б<Ґ>в"
        );
    }
}
