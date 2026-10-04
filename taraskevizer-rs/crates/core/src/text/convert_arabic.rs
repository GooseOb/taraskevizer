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

use super::{byte_pair, utf8_char_len};

/// Advance-bytes helper: copy `text[flush..i]` then skip `n` bytes.
macro_rules! emit {
    ($out:expr, $text:expr, $flush:expr, $i:expr, $s:expr, $n:expr) => {{
        $out.push_str(&$text[$flush..$i]);
        $out.push_str($s);
        $flush = $i + $n;
        $i += $n;
    }};
}

// ---------------------------------------------------------------------------
// Byte patterns spelled with the character literals (`byte_pair` /
// `as_bytes` are const): the hot loops below read as characters but compile
// to the same raw byte compares as hand-written hex — no per-position
// decoding or `Pattern` overhead (measured parity with hex literals).
// ---------------------------------------------------------------------------

/// `(lead, second)` byte pair of a 2-byte literal: `P_A` is `('А' as bytes)`.
macro_rules! pairs {
    ($($name:ident = $lit:literal;)*) => {
        $(const $name: (u8, u8) = byte_pair($lit);)*
    };
}

pairs! {
    P_A_UP = "А"; P_A_LO = "а"; P_YA_UP = "Я"; P_YA_LO = "я";
    P_YE_UP = "Е"; P_YE_LO = "е"; P_E_UP = "Э"; P_E_LO = "э";
    P_YERU_UP = "Ы"; P_YERU_LO = "ы"; P_U_UP = "У"; P_U_LO = "у";
    P_O_UP = "О"; P_O_LO = "о"; P_YO_UP = "Ё"; P_YO_LO = "ё";
    P_YU_UP = "Ю"; P_YU_LO = "ю"; P_II_UP = "І"; P_II_LO = "і";
    P_L_UP = "Л"; P_L_LO = "л";
    P_BE_LO = "б"; P_VE_LO = "в"; P_GHE_LO = "г"; P_DE_LO = "д";
    P_ZHE_LO = "ж"; P_ZE_LO = "з"; P_JOT_LO = "й"; P_KA_LO = "к";
    P_EL_LO = "л"; P_EM_LO = "м"; P_EN_LO = "н"; P_PE_LO = "п";
    P_ER_LO = "р"; P_ES_LO = "с"; P_TE_LO = "т"; P_EF_LO = "ф";
    P_KHA_LO = "х"; P_TSE_LO = "ц"; P_CHE_LO = "ч"; P_SHA_LO = "ш";
    P_USHORT_LO = "ў"; P_SOFT_LO = "ь";
    P_BE_UP = "Б"; P_VE_UP = "В"; P_GHE_UP = "Г"; P_DE_UP = "Д";
    P_ZHE_UP = "Ж"; P_ZE_UP = "З"; P_JOT_UP = "Й"; P_KA_UP = "К";
    P_EM_UP = "М"; P_EN_UP = "Н"; P_PE_UP = "П";
    P_ER_UP = "Р"; P_ES_UP = "С"; P_TE_UP = "Т"; P_EF_UP = "Ф";
    P_KHA_UP = "Х"; P_TSE_UP = "Ц"; P_CHE_UP = "Ч"; P_SHA_UP = "Ш";
    P_USHORT_UP = "Ў"; P_SOFT_UP = "Ь";
    P_TA_AR = "ت"; P_ZA_AR = "ز"; P_THA_AR = "ث"; P_KAF_AR = "ك";
    P_APOS_MOD = "ʼ"; P_GHE_DESC_UP = "Ґ"; P_GHE_DESC_LO = "ґ";
}

/// Fixed multi-byte patterns as byte slices (slice `starts_with` is a plain
/// `memcmp`, unlike `str` `Pattern` matching).
const S_DZHE_SUKUN: &[u8] = "д\u{652}ж".as_bytes();
const S_SUKUN: &[u8] = "ْ".as_bytes();
const P_SUKUN: (u8, u8) = byte_pair("ْ");
const S_SHADDA: &[u8] = "ّ".as_bytes();
const S_DZ_SOFT: &[u8] = "ࢮ".as_bytes();
const S_SPACE_I_LAT: &[u8] = " I ".as_bytes();
const S_SPACE_I_CYR: &[u8] = " і ".as_bytes();

/// Shadda/sukun consonant `[БбВвГгДдЖжЗзЙйКкЛлМмНнПпРрСсТтФфХхЦцЧчШшЎў]`
/// (both cases, no `ь`).
fn is_ar_cons(b0: u8, b1: u8) -> bool {
    matches!(
        (b0, b1),
        P_BE_LO
            | P_VE_LO
            | P_GHE_LO
            | P_DE_LO
            | P_ZHE_LO
            | P_ZE_LO
            | P_JOT_LO
            | P_KA_LO
            | P_EL_LO
            | P_EM_LO
            | P_EN_LO
            | P_PE_LO
            | P_ER_LO
            | P_ES_LO
            | P_TE_LO
            | P_EF_LO
            | P_KHA_LO
            | P_TSE_LO
            | P_CHE_LO
            | P_SHA_LO
            | P_USHORT_LO
            | P_BE_UP
            | P_VE_UP
            | P_GHE_UP
            | P_DE_UP
            | P_ZHE_UP
            | P_ZE_UP
            | P_JOT_UP
            | P_KA_UP
            | P_L_UP
            | P_EM_UP
            | P_EN_UP
            | P_PE_UP
            | P_ER_UP
            | P_ES_UP
            | P_TE_UP
            | P_EF_UP
            | P_KHA_UP
            | P_TSE_UP
            | P_CHE_UP
            | P_SHA_UP
            | P_USHORT_UP
    )
}

/// Lam-alif vowel `[АаЯя]` at `i` (2 bytes)? Returns 2 when present.
fn lam_vow_len(b: &[u8], i: usize) -> usize {
    if i + 2 <= b.len() && matches!((b[i], b[i + 1]), P_A_UP | P_A_LO | P_YA_UP | P_YA_LO) {
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
            && matches!((bytes[i + 1], bytes[i + 2]), P_L_UP | P_L_LO)
            && lam_vow_len(bytes, i + 3) == 2
        {
            emit!(out, text, flush, i, " لا", 5);
            continue;
        }
        if i + 4 <= len
            && matches!((bytes[i], bytes[i + 1]), P_L_UP | P_L_LO)
            && lam_vow_len(bytes, i + 2) == 2
        {
            emit!(out, text, flush, i, "ـلا", 4);
            continue;
        }
        i += if bytes[i].is_ascii() {
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
            let pair = matches!((bytes[i], bytes[i + 1]), P_DE_LO | P_DE_UP)
                && matches!(
                    (bytes[i + 2], bytes[i + 3]),
                    P_ZE_LO | P_ZE_UP | P_ZHE_LO | P_ZHE_UP
                );
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
        i += if bytes[i].is_ascii() {
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
        i += if bytes[i].is_ascii() {
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
        if i + 2 <= len && matches!((bytes[i], bytes[i + 1]), P_A_UP | P_A_LO) {
            emit!(out, text, flush, i, "اа", 2);
            continue;
        }
        i += if bytes[i].is_ascii() {
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
        // Only a space can start this pattern.
        if bytes[i] != b' ' {
            i += if bytes[i].is_ascii() {
                1
            } else {
                utf8_char_len(bytes[i])
            };
            continue;
        }
        if i + 3 <= len
            && matches!(
                (bytes[i + 1], bytes[i + 2]),
                P_YE_UP
                    | P_YE_LO
                    | P_E_UP
                    | P_E_LO
                    | P_YERU_UP
                    | P_YERU_LO
                    | P_U_UP
                    | P_U_LO
                    | P_O_UP
                    | P_O_LO
            )
        {
            emit!(out, text, flush, i, " ا", 1);
            continue;
        }
        i += 1;
    }
    out.push_str(&text[flush..]);
    out
}

/// Softening lookahead `[еёіюяь]` (lowercase only, like the JSON).
fn is_soft_look(b0: u8, b1: u8) -> bool {
    matches!(
        (b0, b1),
        P_YE_LO | P_YO_LO | P_II_LO | P_YU_LO | P_YA_LO | P_SOFT_LO
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
    // `ْ` is U+0652; all heads here are 2-byte non-ASCII.
    while i < len {
        // `д\u{652}з\u{652}(?=[еёіюяь])` → `ࢮ` (before lone `з`, dict order).
        if i + 10 <= len
            && matches!((bytes[i], bytes[i + 1]), P_DE_LO)
            && matches!((bytes[i + 2], bytes[i + 3]), P_SUKUN)
            && matches!((bytes[i + 4], bytes[i + 5]), P_ZE_LO)
            && matches!((bytes[i + 6], bytes[i + 7]), P_SUKUN)
            && is_soft_look(bytes[i + 8], bytes[i + 9])
        {
            emit!(out, text, flush, i, "ࢮ", 8);
            continue;
        }
        // `з/к/с/т\u{652}(?=[еёіюяь])`.
        if i + 6 <= len
            && matches!((bytes[i + 2], bytes[i + 3]), P_SUKUN)
            && is_soft_look(bytes[i + 4], bytes[i + 5])
        {
            let rep = match (bytes[i], bytes[i + 1]) {
                P_ZE_LO => Some("ز"),
                P_KA_LO => Some("ك"),
                P_ES_LO => Some("ث"),
                P_TE_LO => Some("ت"),
                _ => None,
            };
            if let Some(r) = rep {
                emit!(out, text, flush, i, r, 4);
                continue;
            }
        }
        i += if bytes[i].is_ascii() {
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
            && matches!(
                (bytes[i], bytes[i + 1]),
                P_TA_AR | P_ZA_AR | P_THA_AR | P_KAF_AR
            )
            && matches!((bytes[i + 2], bytes[i + 3]), P_II_UP | P_II_LO)
        {
            // Both sides are 2 bytes: 2 + 2.
            emit!(out, text, flush, i, "ы", 4);
            continue;
        }
        i += if bytes[i].is_ascii() {
            1
        } else {
            utf8_char_len(bytes[i])
        };
    }
    out.push_str(&text[flush..]);
    out
}

/// Presoft head `[تزكثࢮбвгджзйклмнпрстфхцчшў]` at `b[i]`
/// (lowercase Cyrillic only, like the JSON): byte length, 0 if none.
/// `ࢮ` is 3 bytes, everything else is 2 bytes.
fn presoft_head_len(b: &[u8], i: usize) -> usize {
    // `ࢮ` first: its lead differs from every 2-byte head.
    if i + S_DZ_SOFT.len() <= b.len() && b[i..].starts_with(S_DZ_SOFT) {
        return S_DZ_SOFT.len();
    }
    if i + 2 > b.len() {
        return 0;
    }
    match (b[i], b[i + 1]) {
        P_TA_AR | P_ZA_AR | P_THA_AR | P_KAF_AR | P_BE_LO | P_VE_LO | P_GHE_LO | P_DE_LO
        | P_ZHE_LO | P_ZE_LO | P_JOT_LO | P_KA_LO | P_L_LO | P_EM_LO | P_EN_LO | P_PE_LO
        | P_ER_LO | P_ES_LO | P_TE_LO | P_EF_LO | P_KHA_LO | P_TSE_LO | P_CHE_LO | P_SHA_LO
        | P_USHORT_LO => 2,
        _ => 0,
    }
}

/// Presoft vowel mark (lowercase only): `[аяэе]→fatha`, `[іы]→kasra`,
/// `[оёую]→damma`.
fn presoft_mark(b0: u8, b1: u8) -> Option<&'static str> {
    Some(match (b0, b1) {
        P_A_LO | P_YA_LO | P_E_LO | P_YE_LO => "َ",
        P_II_LO | P_YERU_LO => "ِ",
        P_O_LO | P_YO_LO | P_U_LO | P_YU_LO => "ُ",
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
            // Optional sukun (`ْ` U+0652), consumed like `ْ?`.
            if bytes[j..].starts_with(S_SUKUN) {
                j += S_SUKUN.len();
            }
            // Optional shadda (`ّ` U+0651), preserved like `(\u{651}?)`.
            let mut sh = 0usize;
            if bytes[j..].starts_with(S_SHADDA) {
                sh = S_SHADDA.len();
            }
            if j + sh + 2 <= len {
                if let Some(m) = presoft_mark(bytes[j + sh], bytes[j + sh + 1]) {
                    // All presoft vowels are 2 bytes.
                    out.push_str(&text[flush..i]);
                    out.push_str(&text[i..i + hl]);
                    if sh > 0 {
                        out.push_str(&text[j..j + sh]);
                    }
                    out.push_str(m);
                    flush = j + sh + 2;
                    i = flush;
                    continue;
                }
            }
        }
        i += if bytes[i].is_ascii() {
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
        // Both patterns start with a space.
        if bytes[i] != b' ' {
            i += if bytes[i].is_ascii() {
                1
            } else {
                utf8_char_len(bytes[i])
            };
            continue;
        }
        // ` I ` is 3 bytes, ` і ` is 4 bytes (`і` is 2 bytes).
        if bytes[i..].starts_with(S_SPACE_I_LAT) {
            emit!(out, text, flush, i, " اِ ", 3);
            continue;
        }
        if bytes[i..].starts_with(S_SPACE_I_CYR) {
            emit!(out, text, flush, i, " اِ ", 4);
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
        if b.is_ascii() {
            // Only `,`/`?` match ASCII here; everything else passes through.
            if b == b',' {
                emit!(out, text, flush, i, "،", 1);
                continue;
            }
            if b == b'?' {
                emit!(out, text, flush, i, "؟", 1);
                continue;
            }
            i += 1;
            continue;
        }
        // `ʼ` → `ع`.
        if i + 2 <= len && (b, bytes[i + 1]) == P_APOS_MOD {
            emit!(out, text, flush, i, "ع", 2);
            continue;
        }
        // `д\u{652}ж` → `ج` before bare `д/Д→د`.
        if bytes[i..].starts_with(S_DZHE_SUKUN) {
            emit!(out, text, flush, i, "ج", 6);
            continue;
        }
        if i + 2 <= len {
            let rep: Option<&'static str> = match (b, bytes[i + 1]) {
                // `ь`/`Ь` delete.
                P_SOFT_LO | P_SOFT_UP => Some(""),
                // Vowels.
                P_YA_UP | P_YA_LO | P_YE_UP | P_YE_LO => Some("يَ"),
                P_II_UP | P_II_LO => Some("يِ"),
                P_YO_UP | P_YO_LO | P_YU_UP | P_YU_LO => Some("يُ"),
                P_A_UP | P_A_LO | P_E_UP | P_E_LO => Some("َ"),
                // `[ЫыІі]` only ever sees `Ы/ы` here (`І/і` went above).
                P_YERU_UP | P_YERU_LO => Some("ِ"),
                P_O_UP | P_O_LO | P_U_UP | P_U_LO => Some("ُ"),
                // Consonants.
                P_BE_LO | P_BE_UP => Some("ب"),
                P_VE_LO | P_VE_UP | P_USHORT_UP | P_USHORT_LO => Some("و"),
                P_GHE_LO | P_GHE_UP => Some("ه"),
                P_JOT_LO | P_JOT_UP => Some("ي"),
                P_KA_LO | P_KA_UP => Some("ق"),
                P_L_LO | P_L_UP => Some("ل"),
                P_EM_LO | P_EM_UP => Some("م"),
                P_EN_LO | P_EN_UP => Some("ن"),
                P_PE_LO | P_PE_UP => Some("پ"),
                P_ER_LO | P_ER_UP => Some("ر"),
                P_ES_LO | P_ES_UP => Some("ص"),
                P_TE_LO | P_TE_UP => Some("ط"),
                P_EF_LO | P_EF_UP => Some("ف"),
                P_KHA_LO | P_KHA_UP => Some("ح"),
                P_TSE_LO | P_TSE_UP => Some("ࢯ"),
                P_CHE_LO | P_CHE_UP => Some("چ"),
                P_SHA_LO | P_SHA_UP => Some("ش"),
                P_DE_LO | P_DE_UP => Some("د"),
                P_ZHE_LO | P_ZHE_UP => Some("ژ"),
                P_ZE_LO | P_ZE_UP => Some("ض"),
                P_GHE_DESC_UP | P_GHE_DESC_LO => Some("غ"),
                _ => None,
            };
            if let Some(r) = rep {
                emit!(out, text, flush, i, r, 2);
                continue;
            }
        }
        i += utf8_char_len(b);
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
