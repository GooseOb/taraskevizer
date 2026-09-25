//! Manual `і → й` replacement (no regex engine).
//!
//! Mirrors the JS `replaceIbyJ` step (`--jalways`): `/(?<=[аеёіоуыэюя] )і (ў?)/`
//! with a `й `/`й у` replacement (or a coin flip keeping `$0` when `always`
//! is false). The lookbehind is a fixed `V␣` (2-byte vowel + space) checked
//! against the original text, like the regex engine does: leftmost scan,
//! no rescan past matches.

use super::utf8_char_len;

/// `V` of the `(?<=[аеёіоуыэюя] )` lookbehind: `аеёіоуыэюя`, all 2-byte.
fn is_i_to_j_vow(b0: u8, b1: u8) -> bool {
    matches!(
        (b0, b1),
        (0xD0, 0xB0) // а
            | (0xD0, 0xB5) // е
            | (0xD1, 0x91) // ё
            | (0xD1, 0x96) // і
            | (0xD0, 0xBE) // о
            | (0xD1, 0x83) // у
            | (0xD1, 0x8B) // ы
            | (0xD1, 0x8D) // э
            | (0xD1, 0x8E) // ю
            | (0xD1, 0x8F) // я
    )
}

/// Replace `V␣і␣(ў?)` with `й␣`/`й␣у`; `always = false` keeps the match
/// on a coin flip, like `Math.random() >= 0.5` in JS.
pub(crate) fn replace_i_by_j(text: &str, always: bool) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + 3 <= len
            && bytes[i] == 0xD1
            && bytes[i + 1] == 0x96
            && bytes[i + 2] == b' '
            && i >= 3
            && bytes[i - 1] == b' '
            && is_i_to_j_vow(bytes[i - 3], bytes[i - 2])
        {
            // `(ў?)`: optional `ў` right after the space.
            let has_u =
                i + 5 <= len && bytes[i + 3] == 0xD1 && bytes[i + 4] == 0x9E;
            let end = if has_u { i + 5 } else { i + 3 };
            let replace = always || rand::random::<f64>() >= 0.5;
            out.push_str(&text[flush..i]);
            if replace {
                out.push_str(if has_u { "й у" } else { "й " });
            } else {
                out.push_str(&text[i..end]);
            }
            flush = end;
            i = end;
            continue;
        }
        i += if bytes[i] < 0x80 {
            1
        } else {
            utf8_char_len(bytes[i])
        };
    }
    out.push_str(&text[flush..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_always(input: &str, expected: &str) {
        assert_eq!(replace_i_by_j(input, true), expected, "{input:?}");
    }

    #[test]
    fn replaces_after_vowel() {
        check_always("а і б", "а й б");
        check_always("а і ў", "а й у");
        check_always("а і ўб", "а й уб");
        check_always("у і я", "у й я");
        check_always("а і  б", "а й  б");
        check_always("а і м", "а й м");
    }

    #[test]
    fn leaves_other_contexts() {
        // `й` is not in the lookbehind class; `і` needs a trailing space
        // and a `V␣` lookbehind with room before it.
        for s in ["й і б", "а і", " і б", "і ", "б і б"] {
            check_always(s, s);
        }
    }
}
