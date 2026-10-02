//! Manual `і → й` replacement (no regex engine).
//!
//! Mirrors the JS `replaceIbyJ` step (`--jalways`): `/(?<=[аеёіоуыэюя] )і (ў?)/`
//! with a `й `/`й у` replacement (or a coin flip keeping `$0` when `always`
//! is false). The lookbehind is a fixed `V␣` (2-byte vowel + space) checked
//! against the original text, like the regex engine does: leftmost scan,
//! no rescan past matches.

use super::{byte_pair, utf8_char_len};

/// Byte patterns (`(lead, second)` pairs and fixed slices) spelled with the
/// character literals, so the hot loop below reads as characters but runs
/// as raw byte compares — no per-position decoding or `Pattern` overhead.
const A_PAIR: (u8, u8) = byte_pair("а");
const YE_PAIR: (u8, u8) = byte_pair("е");
const YO_PAIR: (u8, u8) = byte_pair("ё");
const II_PAIR: (u8, u8) = byte_pair("і");
const O_PAIR: (u8, u8) = byte_pair("о");
const U_PAIR: (u8, u8) = byte_pair("у");
const Y_PAIR: (u8, u8) = byte_pair("ы");
const E_PAIR: (u8, u8) = byte_pair("э");
const YU_PAIR: (u8, u8) = byte_pair("ю");
const YA_PAIR: (u8, u8) = byte_pair("я");
/// `і ` (pattern head) and ` ў` (trailer) as byte slices.
const II_SP: &[u8] = "і ".as_bytes();
const U_SHORT: &[u8] = "ў".as_bytes();

/// Replace `V␣і␣(ў?)` with `й␣`/`й␣у`; `always = false` keeps the match
/// on a coin flip, like `Math.random() >= 0.5` in JS.
pub(crate) fn replace_i_by_j(text: &str, always: bool) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush = 0usize;
    let mut i = 0usize;
    // ` і ` is space(1) + `і`(2) + space(1); ` ў` trailer adds `ў`(2).
    while i < len {
        let b = bytes[i];
        // The pattern starts with non-ASCII `і`: ASCII bytes never match.
        if b.is_ascii() {
            i += 1;
            continue;
        }
        // `V` of the `(?<=[аеёіоуыэюя] )` lookbehind is always 2 bytes,
        // ending right before the space at `i - 1`.
        if bytes[i..].starts_with(II_SP)
            && i >= 3
            && bytes[i - 1] == b' '
            && matches!(
                (bytes[i - 3], bytes[i - 2]),
                A_PAIR | YE_PAIR | YO_PAIR | II_PAIR | O_PAIR | U_PAIR | Y_PAIR | E_PAIR
                    | YU_PAIR | YA_PAIR
            )
        {
            // `(ў?)`: optional `ў` right after the space.
            let has_u = bytes[i + II_SP.len()..].starts_with(U_SHORT);
            let end = if has_u {
                i + II_SP.len() + U_SHORT.len()
            } else {
                i + II_SP.len()
            };
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
        i += utf8_char_len(b);
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
