//! Manual Belarusian → Arabic conversion (no regex engine).
//!
//! Applies the 48 `alphabets.json` lower entries in order, grouped into a
//! few byte scans. Entries fuse into one scan only when their outputs cannot
//! re-match any pattern of the same scan (outputs are Arabic/diacritics,
//! patterns need Cyrillic — or the matches are provably disjoint); the
//! interacting entries (lam → shadda → sukun, `А→اа`, space-alif, soft,
//! tzIi, presoft) stay sequential because later entries consume earlier
//! output (`с\u{652}і` → soft → `ثі` → tzIi → `ы`; presoft eats `ْ?`).
//! Case handling is exactly like the JSON (case-sensitive: the JS `gi`
//! flags were dropped by the JSON export, a pre-existing deviation kept).

use super::utf8_char_len;

/// Advance-bytes helper: copy `text[flush..i]` then skip `n` bytes.
macro_rules! emit {
    ($out:expr, $text:expr, $flush:expr, $i:expr, $s:expr, $n:expr) => {{
        $out.push_str(&$text[$flush..$i]);
        $out.push_str($s);
        $flush = $i + $n;
        $i += $n;
    }};
}

/// Shadda/sukun consonant `[БбВвГгДдЖжЗзЙйКкЛлМмНнПпРрСсТтФфХхЦцЧчШшЎў]`
/// (both cases, no `ь`).
fn is_ar_cons(b0: u8, b1: u8) -> bool {
    matches!(
        (b0, b1),
        // Lowercase.
        (0xD0, 0xB1) // б
            | (0xD0, 0xB2) // в
            | (0xD0, 0xB3) // г
            | (0xD0, 0xB4) // д
            | (0xD0, 0xB6) // ж
            | (0xD0, 0xB7) // з
            | (0xD0, 0xB9) // й
            | (0xD0, 0xBA) // к
            | (0xD0, 0xBB) // л
            | (0xD0, 0xBC) // м
            | (0xD0, 0xBD) // н
            | (0xD0, 0xBF) // п
            | (0xD1, 0x80) // р
            | (0xD1, 0x81) // с
            | (0xD1, 0x82) // т
            | (0xD1, 0x84) // ф
            | (0xD1, 0x85) // х
            | (0xD1, 0x86) // ц
            | (0xD1, 0x87) // ч
            | (0xD1, 0x88) // ш
            | (0xD1, 0x9E) // ў
            // Uppercase.
            | (0xD0, 0x91) // Б
            | (0xD0, 0x92) // В
            | (0xD0, 0x93) // Г
            | (0xD0, 0x94) // Д
            | (0xD0, 0x96) // Ж
            | (0xD0, 0x97) // З
            | (0xD0, 0x99) // Й
            | (0xD0, 0x9A) // К
            | (0xD0, 0x9B) // Л
            | (0xD0, 0x9C) // М
            | (0xD0, 0x9D) // Н
            | (0xD0, 0x9F) // П
            | (0xD0, 0xA0) // Р
            | (0xD0, 0xA1) // С
            | (0xD0, 0xA2) // Т
            | (0xD0, 0xA4) // Ф
            | (0xD0, 0xA5) // Х
            | (0xD0, 0xA6) // Ц
            | (0xD0, 0xA7) // Ч
            | (0xD0, 0xA8) // Ш
            | (0xD0, 0x8E) // Ў
    )
}

/// Lam-alif vowel `[АаЯя]` at `i` (2 bytes)? Returns 2 when present.
fn lam_vow_len(b: &[u8], i: usize) -> usize {
    if i + 2 <= b.len()
        && ((b[i] == 0xD0 && matches!(b[i + 1], 0x90 | 0xB0 | 0xAF))
            || (b[i] == 0xD1 && b[i + 1] == 0x8F))
    {
        2
    } else {
        0
    }
}

/// Entries 0–1: ` [Лл][АаЯя]→␣لا`, `[Лл][АаЯя]→ـلا` (fused: outputs hold no
/// Cyrillic, and the space rules the two patterns disjoint).
fn ar_lam(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if bytes[i] == b' '
            && i + 5 <= len
            && bytes[i + 1] == 0xD0
            && matches!(bytes[i + 2], 0x9B | 0xBB)
            && lam_vow_len(bytes, i + 3) == 2
        {
            emit!(out, text, flush, i, " لا", 5);
            continue;
        }
        if i + 4 <= len
            && bytes[i] == 0xD0
            && matches!(bytes[i + 1], 0x9B | 0xBB)
            && lam_vow_len(bytes, i + 2) == 2
        {
            emit!(out, text, flush, i, "ـلا", 4);
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

/// Entry 2: `([CONS]|[Дд][ЗзЖж])\1→$1\u{651}` (single-char branch first,
/// like the regex alternation; the sukun comes from entry 3).
fn ar_shadda(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + 4 <= len && is_ar_cons(bytes[i], bytes[i + 1]) {
            // Single-letter double.
            if bytes[i + 2] == bytes[i] && bytes[i + 3] == bytes[i + 1] {
                out.push_str(&text[flush..i]);
                out.push_str(&text[i..i + 2]);
                out.push('ّ');
                flush = i + 4;
                i += 4;
                continue;
            }
            // `д[зж]` digraph double (exact bytes, like `\1`).
            let pair = bytes[i] == 0xD0
                && matches!(bytes[i + 1], 0xB4 | 0x94)
                && bytes[i + 2] == 0xD0
                && matches!(bytes[i + 3], 0xB7 | 0x97 | 0xB6 | 0x96);
            if pair
                && i + 8 <= len
                && bytes[i + 4] == bytes[i]
                && bytes[i + 5] == bytes[i + 1]
                && bytes[i + 6] == bytes[i + 2]
                && bytes[i + 7] == bytes[i + 3]
            {
                out.push_str(&text[flush..i]);
                out.push_str(&text[i..i + 4]);
                out.push('ّ');
                flush = i + 8;
                i += 8;
                continue;
            }
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

/// Entry 3: `([CONS])→$1\u{652}`.
fn ar_sukun(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + 2 <= len && is_ar_cons(bytes[i], bytes[i + 1]) {
            out.push_str(&text[flush..i]);
            out.push_str(&text[i..i + 2]);
            out.push('ْ');
            flush = i + 2;
            i += 2;
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

/// Entry 4: `[Аа]→اа` (the Cyrillic `а` stays for the fatha entry).
fn ar_alif_a(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + 2 <= len && bytes[i] == 0xD0 && matches!(bytes[i + 1], 0x90 | 0xB0) {
            emit!(out, text, flush, i, "اа", 2);
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

/// Entry 5: ` (?=[ЕеЭэЫыУуОо])→␣ا` (space kept, vowel only peeked).
fn ar_space_alif(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if bytes[i] == b' '
            && i + 3 <= len
            && matches!(
                (bytes[i + 1], bytes[i + 2]),
                (0xD0, 0x95)
                    | (0xD0, 0xB5)
                    | (0xD0, 0xAD)
                    | (0xD1, 0x8D)
                    | (0xD0, 0xAB)
                    | (0xD1, 0x8B)
                    | (0xD0, 0xA3)
                    | (0xD1, 0x83)
                    | (0xD0, 0x9E)
                    | (0xD0, 0xBE)
            )
        {
            emit!(out, text, flush, i, " ا", 1);
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

/// Softening lookahead `[еёіюяь]` (lowercase only, like the JSON).
fn is_soft_look(b0: u8, b1: u8) -> bool {
    matches!(
        (b0, b1),
        (0xD0, 0xB5) | (0xD1, 0x91) | (0xD1, 0x96) | (0xD1, 0x8E) | (0xD1, 0x8F) | (0xD1, 0x8C)
    )
}

/// Entries 6–10 fused: `д\u{652}з\u{652}(?=look)→ࢮ`, `з/к/с/т\u{652}(?=look)`
/// → `ز/ك/ث/ت` (lowercase only; outputs are Arabic so they re-match
/// nothing here — tzIi/presoft run next and see them).
fn ar_soft(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        // `д\u{652}з\u{652}(?=[еёіюяь])` → `ࢮ` (before lone `з`, dict order).
        if i + 10 <= len
            && bytes[i] == 0xD0
            && bytes[i + 1] == 0xB4
            && bytes[i + 2] == 0xD9
            && bytes[i + 3] == 0x92
            && bytes[i + 4] == 0xD0
            && bytes[i + 5] == 0xB7
            && bytes[i + 6] == 0xD9
            && bytes[i + 7] == 0x92
            && is_soft_look(bytes[i + 8], bytes[i + 9])
        {
            emit!(out, text, flush, i, "ࢮ", 8);
            continue;
        }
        // `з/к/с/т\u{652}(?=[еёіюяь])`.
        if i + 6 <= len
            && bytes[i + 2] == 0xD9
            && bytes[i + 3] == 0x92
            && is_soft_look(bytes[i + 4], bytes[i + 5])
        {
            let rep = match (bytes[i], bytes[i + 1]) {
                (0xD0, 0xB7) => Some("ز"),
                (0xD0, 0xBA) => Some("ك"),
                (0xD1, 0x81) => Some("ث"),
                (0xD1, 0x82) => Some("ت"),
                _ => None,
            };
            if let Some(r) = rep {
                emit!(out, text, flush, i, r, 4);
                continue;
            }
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

/// Entry 11: `([تزكث])[Іі]→ы`.
fn ar_tzii(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + 4 <= len
            && ((bytes[i] == 0xD8 && matches!(bytes[i + 1], 0xAA | 0xB2 | 0xAB))
                || (bytes[i] == 0xD9 && bytes[i + 1] == 0x83))
            && ((bytes[i + 2] == 0xD0 && bytes[i + 3] == 0x86)
                || (bytes[i + 2] == 0xD1 && bytes[i + 3] == 0x96))
        {
            emit!(out, text, flush, i, "ы", 4);
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

/// Byte length of a presoft head `[تزكثࢮбвгджзйклмнпрстфхцчшў]` at `i`
/// (lowercase Cyrillic only, like the JSON): 3 for `ࢮ`, 2 else, 0 if none.
fn presoft_head_len(b: &[u8], i: usize) -> usize {
    if i + 3 <= b.len() && b[i] == 0xE0 && b[i + 1] == 0xA2 && b[i + 2] == 0xAE {
        return 3;
    }
    if i + 2 > b.len() {
        return 0;
    }
    match (b[i], b[i + 1]) {
        // Arabic `ت ز ث ك`.
        (0xD8, 0xAA) | (0xD8, 0xB2) | (0xD8, 0xAB) | (0xD9, 0x83) => 2,
        // Lowercase `бвгджзйклмнп`.
        (0xD0, 0xB1)
        | (0xD0, 0xB2)
        | (0xD0, 0xB3)
        | (0xD0, 0xB4)
        | (0xD0, 0xB6)
        | (0xD0, 0xB7)
        | (0xD0, 0xB9)
        | (0xD0, 0xBA)
        | (0xD0, 0xBB)
        | (0xD0, 0xBC)
        | (0xD0, 0xBD)
        | (0xD0, 0xBF) => 2,
        // Lowercase `рстфхцчшў`.
        (0xD1, 0x80)
        | (0xD1, 0x81)
        | (0xD1, 0x82)
        | (0xD1, 0x84)
        | (0xD1, 0x85)
        | (0xD1, 0x86)
        | (0xD1, 0x87)
        | (0xD1, 0x88)
        | (0xD1, 0x9E) => 2,
        _ => 0,
    }
}

/// Presoft vowel mark (lowercase only): `[аяэе]→fatha`, `[іы]→kasra`,
/// `[оёую]→damma`.
fn presoft_mark(b0: u8, b1: u8) -> Option<&'static str> {
    Some(match (b0, b1) {
        (0xD0, 0xB0) | (0xD1, 0x8F) | (0xD1, 0x8D) | (0xD0, 0xB5) => "َ",
        (0xD1, 0x96) | (0xD1, 0x8B) => "ِ",
        (0xD0, 0xBE) | (0xD1, 0x91) | (0xD1, 0x83) | (0xD1, 0x8E) => "ُ",
        _ => return None,
    })
}

/// Entries 12, 13, 15 fused: `HEAD\u{652}?(\u{651}?)[vow]→HEAD(shadda?)mark`.
/// The three vowel classes are disjoint and outputs (marks) re-match
/// nothing here; entry 14 (` [Iі] `) is mutually exclusive (spaces) and
/// runs next.
fn ar_presoft(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        let hl = presoft_head_len(bytes, i);
        if hl > 0 {
            let mut j = i + hl;
            // Optional sukun, consumed like `ْ?`.
            if j + 2 <= len && bytes[j] == 0xD9 && bytes[j + 1] == 0x92 {
                j += 2;
            }
            // Optional shadda, preserved like `(\u{651}?)`.
            let mut sh = 0usize;
            if j + 2 <= len && bytes[j] == 0xD9 && bytes[j + 1] == 0x91 {
                sh = 2;
            }
            if j + sh + 2 <= len {
                if let Some(m) = presoft_mark(bytes[j + sh], bytes[j + sh + 1]) {
                    out.push_str(&text[flush..i]);
                    out.push_str(&text[i..i + hl]);
                    if sh > 0 {
                        out.push_str(&text[j..j + 2]);
                    }
                    out.push_str(m);
                    flush = j + sh + 2;
                    i = flush;
                    continue;
                }
            }
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

/// Entry 14: ` [Iі] → اِ ` (Latin `I` or Cyrillic `і`, spaces kept).
fn ar_space_i(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if bytes[i] == b' ' && i + 3 <= len {
            let ilen = if bytes[i + 1] == 0x49 {
                1
            } else if bytes[i + 1] == 0xD1 && i + 4 <= len && bytes[i + 2] == 0x96 {
                2
            } else {
                0
            };
            if ilen > 0 && bytes[i + 1 + ilen] == b' ' {
                emit!(out, text, flush, i, " اِ ", 2 + ilen);
                continue;
            }
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

/// Entries 16–47 fused into one scan (entry order kept for the overlaps
/// `[Іі]`→`يِ` before `[ЫыІі]`→` kasra`, `д\u{652}ж`→`ج` before `Д→د`):
/// no output holds Cyrillic/`ʼ`/`,`/`?`, so nothing re-matches here.
fn ar_rest(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        // `ʼ` (U+02BC) → `ع`.
        if i + 2 <= len && b == 0xCA && bytes[i + 1] == 0xBC {
            emit!(out, text, flush, i, "ع", 2);
            continue;
        }
        if i + 2 <= len {
            // `д\u{652}ж` → `ج` before bare `Д→د`.
            if b == 0xD0
                && bytes[i + 1] == 0xB4
                && i + 6 <= len
                && bytes[i + 2] == 0xD9
                && bytes[i + 3] == 0x92
                && bytes[i + 4] == 0xD0
                && bytes[i + 5] == 0xB6
            {
                emit!(out, text, flush, i, "ج", 6);
                continue;
            }
            let rep: Option<&'static str> = match (b, bytes[i + 1]) {
                // `ь`/`Ь` delete.
                (0xD1, 0x8C) | (0xD0, 0xAC) => Some(""),
                // Vowels.
                (0xD0, 0xAF) | (0xD1, 0x8F) | (0xD0, 0x95) | (0xD0, 0xB5) => Some("يَ"),
                (0xD0, 0x86) | (0xD1, 0x96) => Some("يِ"),
                (0xD0, 0x81) | (0xD1, 0x91) | (0xD0, 0xAE) | (0xD1, 0x8E) => Some("يُ"),
                (0xD0, 0x90) | (0xD0, 0xB0) | (0xD0, 0xAD) | (0xD1, 0x8D) => Some("َ"),
                // `[ЫыІі]` only ever sees `Ы/ы` here (`І/і` went above).
                (0xD0, 0xAB) | (0xD1, 0x8B) => Some("ِ"),
                (0xD0, 0x9E) | (0xD0, 0xBE) | (0xD0, 0xA3) | (0xD1, 0x83) => Some("ُ"),
                // Consonants.
                (0xD0, 0xB1) | (0xD0, 0x91) => Some("ب"),
                (0xD0, 0xB2) | (0xD0, 0x92) | (0xD0, 0x8E) | (0xD1, 0x9E) => Some("و"),
                (0xD0, 0xB3) | (0xD0, 0x93) => Some("ه"),
                (0xD0, 0xB9) | (0xD0, 0x99) => Some("ي"),
                (0xD0, 0xBA) | (0xD0, 0x9A) => Some("ق"),
                (0xD0, 0xBB) | (0xD0, 0x9B) => Some("ل"),
                (0xD0, 0xBC) | (0xD0, 0x9C) => Some("م"),
                (0xD0, 0xBD) | (0xD0, 0x9D) => Some("ن"),
                (0xD0, 0xBF) | (0xD0, 0x9F) => Some("پ"),
                (0xD1, 0x80) | (0xD0, 0xA0) => Some("ر"),
                (0xD1, 0x81) | (0xD0, 0xA1) => Some("ص"),
                (0xD1, 0x82) | (0xD0, 0xA2) => Some("ط"),
                (0xD1, 0x84) | (0xD0, 0xA4) => Some("ف"),
                (0xD1, 0x85) | (0xD0, 0xA5) => Some("ح"),
                (0xD1, 0x86) | (0xD0, 0xA6) => Some("ࢯ"),
                (0xD1, 0x87) | (0xD0, 0xA7) => Some("چ"),
                (0xD1, 0x88) | (0xD0, 0xA8) => Some("ش"),
                (0xD0, 0xB4) | (0xD0, 0x94) => Some("د"),
                (0xD0, 0xB6) | (0xD0, 0x96) => Some("ژ"),
                (0xD0, 0xB7) | (0xD0, 0x97) => Some("ض"),
                (0xD2, 0x90) | (0xD2, 0x91) => Some("غ"),
                _ => None,
            };
            if let Some(r) = rep {
                emit!(out, text, flush, i, r, 2);
                continue;
            }
        }
        // `,` → `،`, `?` → `؟`.
        if b == b',' {
            emit!(out, text, flush, i, "،", 1);
            continue;
        }
        if b == b'?' {
            emit!(out, text, flush, i, "؟", 1);
            continue;
        }
        i += if b < 0x80 { 1 } else { utf8_char_len(b) };
    }
    out.push_str(&text[flush..]);
    out
}

/// arabic entry point (the JSON holds a lower table only).
pub(crate) fn convert_arabic(text: &str) -> String {
    ar_rest(&ar_space_i(&ar_presoft(&ar_tzii(&ar_soft(
        &ar_space_alif(&ar_alif_a(&ar_sukun(&ar_shadda(&ar_lam(text))))),
    )))))
}

#[cfg(test)]
mod tests {
    use super::super::test_oracle::{old_abc, OldAbc};
    use super::*;
    use crate::config::Alphabet;
    use std::sync::LazyLock;

    static OLD_AR: LazyLock<OldAbc> = LazyLock::new(|| old_abc(Alphabet::Arabic));

    fn check(input: &str) {
        assert_eq!(convert_arabic(input), OLD_AR.lower(input), "{input:?}");
    }

    #[test]
    fn singles_and_passthrough() {
        let mut inputs = Vec::new();
        for cp in 0x0400u32..0x0460 {
            inputs.push(char::from_u32(cp).unwrap().to_string());
        }
        for cp in 0x20u32..0x7F {
            inputs.push(char::from_u32(cp).unwrap().to_string());
        }
        for s in [
            "ʼ", "́", "ł", "Ł", "04", "«»", "—", " ", "\n", "\t", "\u{a0}", "т", "Т", "ز", "ي",
            "\u{652}", "\u{651}",
        ] {
            inputs.push(s.to_string());
        }
        for input in &inputs {
            check(input);
            check(&format!("а{input}о"));
        }
        // Documented spot checks from the reference implementation.
        assert_eq!(convert_arabic("Я"), "يَ");
        assert_eq!(convert_arabic("ю"), "يُ");
        assert_eq!(convert_arabic("ь"), "");
        assert_eq!(convert_arabic("ʼ"), "ع");
        assert_eq!(convert_arabic(",?"), "،؟");
        assert_eq!(convert_arabic("т"), "ط\u{652}");
    }

    #[test]
    fn shadda_and_sukun() {
        // Doubles of every shadda-class letter, both cases.
        for c in [
            'б', 'в', 'г', 'д', 'ж', 'з', 'й', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'т', 'ф', 'х',
            'ц', 'ч', 'ш', 'ў', 'Б', 'Д', 'Н',
        ] {
            check(&format!("{c}{c}"));
            check(&format!("{c}{c}{c}"));
            check(&format!("а{c}{c}о"));
        }
        // `д[зж]` pairs, all case combos, doubled or not.
        for a in ['д', 'Д'] {
            for b in ['з', 'З', 'ж', 'Ж'] {
                check(&format!("{a}{b}"));
                check(&format!("{a}{b}{a}{b}"));
                check(&format!("{a}{b}{b}"));
            }
        }
        assert_eq!(convert_arabic("ББ"), "ب\u{652}\u{651}");
        assert_eq!(convert_arabic("нн"), "ن\u{652}\u{651}");
    }

    #[test]
    fn lam_alif_and_alif() {
        for a in ['А', 'а', 'Я', 'я'] {
            for l in ['Л', 'л'] {
                check(&format!("{l}{a}"));
                check(&format!(" {l}{a}"));
                check(&format!("x{l}{a}y"));
            }
        }
        for s in ["ле", "Ло", "лу", "ла"] {
            check(s);
        }
        assert_eq!(convert_arabic("ла"), "ـلا");
        assert_eq!(convert_arabic("ма"), "م\u{652}اَ");
    }

    #[test]
    fn space_alif() {
        for v in ['Е', 'е', 'Э', 'э', 'Ы', 'ы', 'У', 'у', 'О', 'о'] {
            check(&format!(" {v}"));
            check(&format!("x {v}y"));
        }
        for v in ['а', 'я', 'і', 'А', ' '] {
            check(&format!(" {v}"));
        }
    }

    #[test]
    fn softening() {
        // Each soft letter × lookahead (vowel/ь/EOS/other), lower + upper.
        for c in ['д', 'з', 'к', 'с', 'т'] {
            for look in ["е", "ё", "і", "ю", "я", "ь", "а", "о", " ", ""] {
                check(&format!("{c}\u{652}{look}"));
            }
        }
        check("д\u{652}зе");
        check("з\u{652}і");
        // Uppercase soft triggers stay literal (JSON is case-sensitive;
        // the JS `gi` flags were dropped by the export — kept as-is).
        check("ДЖ");
        check("ЗІ");
        check("д\u{652}Же");
        assert_eq!(convert_arabic("зі"), "ِ");
        assert_eq!(convert_arabic("дж"), "ج\u{652}");
    }

    #[test]
    fn tz_presoft_spacei() {
        for t in ['ت', 'ز', 'ك', 'ث'] {
            for v in ['І', 'і'] {
                check(&format!("{t}{v}"));
            }
            check(&format!("{t}а"));
        }
        // Presoft heads × sukun/shadda × vowels.
        for h in ["т", "з", "б", "ࢮ", "Д"] {
            for mid in ["", "\u{652}", "\u{652}\u{651}", "\u{651}"] {
                for v in ["а", "е", "і", "ы", "о", "у", "ё", "ю", "я", "э"] {
                    check(&format!("{h}{mid}{v}"));
                }
            }
        }
        for s in [" I ", " і ", " І ", " I", "I ", "і"] {
            check(s);
        }
    }

    #[test]
    fn vowels_and_rest() {
        for c in [
            'Я', 'я', 'Е', 'е', 'І', 'і', 'Ё', 'ё', 'Ю', 'ю', 'А', 'а', 'Э', 'э', 'Ы', 'ы', 'О',
            'о', 'У', 'у',
        ] {
            check(&c.to_string());
            check(&format!("м{c}"));
        }
        for s in [
            "д\u{652}ж",
            "дж",
            "ДЖ",
            "Планета",
            "надзіманне",
            "Прывет",
            "ЯЕ",
            "ее",
            "ЕЕ",
            "эІ",
            "оЁ",
            "зья",
            "Ць",
            "Ее",
        ] {
            check(s);
        }
        assert_eq!(convert_arabic("Планета"), "پ\u{652}ـلانَط\u{652}اَ");
    }

    /// Seeded fuzz over a mixed soup (Cyrillic, Arabic, marks, punct).
    #[test]
    fn fuzz_arabic() {
        let mut abc: Vec<char> = (0x0400u32..0x0460).filter_map(char::from_u32).collect();
        abc.extend(
            [
                ' ', '(', ')', '.', ',', '?', '2', '́', 'ْ', 'ّ', 'ز', 'e', 'ت', 'ي', 'ا', 'I', 'і',
            ]
            .iter()
            .copied(),
        );
        let mut seed = 0x51ED270Bu64;
        let mut next = move || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) as usize
        };
        for _ in 0..1500 {
            let n = 1 + next() % 10;
            let s: String = (0..n).map(|_| abc[next() % abc.len()]).collect();
            check(&s);
        }
    }
}
