//! Manual Belarusian → Latin / Latin-Ji conversion (no regex engine).
//!
//! The `alphabets.json` entries applied sequentially cost a full string scan
//! *per entry* (48–57 passes, most through `fancy_regex` backtracking).
//! The replacements below merge each dict *group* into one left-to-right
//! scan. Groups stay sequential where an output feeds a later pattern:
//!
//! * `latin/lower` is one pass: jefication runs first in the dict, so its
//!   lookbehind sees the input (tracked as the previous input char); later
//!   outputs (Latin) never re-match Cyrillic patterns, except `ʼ`-deletion
//!   enabling `ць`-style clusters — handled by skipping an `ʼ`-run when it
//!   is followed by `ь`.
//! * `latinJi/lower` is three passes (`Vow`+`iwords`, jefication, rest):
//!   the `Vow`-rule output (`й`) sits in later jefication lookbehinds, and
//!   jefication output (`e`/`o`/`u`/`a`) sits in later `(..)і` contexts —
//!   merging either would diverge (e.g. `а і ўе` → `a j uie`).
//! * upper conversions keep dict-group order (spaced-`Е`, captures, …):
//!   capture `$1` echoes output context (e.g. `АЕЁ` → `AJEJO`), so the four
//!   capture vowels stay four passes.

use super::{byte_pair, is_decimal_number, is_ll, is_lu, is_punct, utf8_char_len};

// ---------- shared: jefication (`е→je` after vowels etc.) ----------

/// Base of the `е/ё/ю/я` lookbehind (before per-vowel Latin extras).
///
/// From `(?<=[аеёіоуўыэюяьʼ| >АЕЁІОУЎЫЭЮЯЬ]|^)`: lowercase vowels (incl. `ў`,
/// excl. `й`), `ь`, `ʼ`, literal `|`, space, `>`, and the uppercase vowels.
fn is_jef_base(c: char) -> bool {
    matches!(
        c,
        'а' | 'е'
            | 'ё'
            | 'і'
            | 'о'
            | 'у'
            | 'ў'
            | 'ы'
            | 'э'
            | 'ю'
            | 'я'
            | 'ь'
            | 'ʼ'
            | '|'
            | ' '
            | '>'
            | 'А'
            | 'Е'
            | 'Ё'
            | 'І'
            | 'О'
            | 'У'
            | 'Ў'
            | 'Ы'
            | 'Э'
            | 'Ю'
            | 'Я'
            | 'Ь'
    )
}

/// `е→je`, `ё→jo`, `ю→ju`, `я→ja` with lookbehind `prev` (`None` is `^`).
///
/// The lookbehind classes are the base above plus Latin extras per vowel
/// (`ё` adds `e`, `ю` adds `e`/`o`, `я` adds `e`/`o`/`u`).
fn jeficate(ch: char, prev: Option<char>) -> Option<&'static str> {
    let ok = match prev {
        None => true,
        Some(p) => {
            is_jef_base(p)
                || match ch {
                    'ё' => p == 'e',
                    'ю' => p == 'e' || p == 'o',
                    'я' => p == 'e' || p == 'o' || p == 'u',
                    _ => false,
                }
        }
    };
    if !ok {
        return None;
    }
    match ch {
        'е' => Some("je"),
        'ё' => Some("jo"),
        'ю' => Some("ju"),
        'я' => Some("ja"),
        _ => None,
    }
}

// ---------- shared: lower single letters ----------

/// Single lowercase Cyrillic letter → Latin. `None` = passthrough
/// (uppercase, `ь`, digits, punctuation, already-Latin, …).
fn latin_single_lower(c: char) -> Option<&'static str> {
    Some(match c {
        'а' => "a",
        'б' => "b",
        'в' => "v",
        'г' => "h",
        'ґ' => "g",
        'д' => "d",
        'е' => "ie",
        'ё' => "io",
        'ж' => "ž",
        'з' => "z",
        'і' => "i",
        'й' => "j",
        'к' => "k",
        'л' => "ł",
        'м' => "m",
        'н' => "n",
        'о' => "o",
        'п' => "p",
        'р' => "r",
        'с' => "s",
        'т' => "t",
        'у' => "u",
        'ў' => "ŭ",
        'ф' => "f",
        'х' => "ch",
        'ц' => "c",
        'ч' => "č",
        'ш' => "š",
        'ы' => "y",
        'э' => "e",
        'ю' => "iu",
        'я' => "ia",
        _ => return None,
    })
}

/// `C + ь` soft clusters (dict order: `ць зь сь нь ль`).
fn soft_cluster(c: char) -> Option<&'static str> {
    Some(match c {
        'ц' => "ć",
        'з' => "ź",
        'с' => "ś",
        'н' => "ń",
        'л' => "l",
        _ => return None,
    })
}

/// `л + vowel` clusters (dict order after the soft ones: `лі ля лё лю ле`).
fn la_cluster(c: char) -> Option<&'static str> {
    Some(match c {
        'і' => "li",
        'я' => "la",
        'ё' => "lo",
        'ю' => "lu",
        'е' => "le",
        _ => return None,
    })
}

// ---------- latin/lower: single pass ----------

/// Lowercase latinization in one scan (jefication, `ʼі`, `ʼ`-deletion with
/// `ь`-cluster lookahead, clusters, singles — dict order at each position).
/// latin lower entry point: jefication + rest in one scan.
pub(crate) fn convert_latin_lower(text: &str) -> String {
    lower_pass(text, false, true)
}

/// Lowercase latinization core, dict order at each position.
///
/// `rule5` enables the latinJi `(..)і → ..ji` rule (checked before
/// `ʼ`-deletion); plain latin passes `false`. `jef` enables jefication —
/// plain latin single-passes with it, while latinJi runs jefication as its
/// own pass first (its lookbehind must see `Vow`-rule output) and calls
/// this with `jef: false`.
fn lower_pass(text: &str, rule5: bool, jef: bool) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + len / 2);
    let mut flush = 0usize;
    let mut i = 0usize;
    // Previous *input* char (jefication runs first in its pass, so its
    // lookbehind sees the input — tracked across consumed chars).
    let mut prev: Option<char> = None;
    while i < len {
        let b = bytes[i];
        // No pattern starts with ASCII, except latinJi rule5 (`e`/`o`/`u`/`a`).
        if b < 0x80 && !(rule5 && matches!(b, b'e' | b'o' | b'u' | b'a')) {
            prev = Some(b as char);
            i += 1;
            continue;
        }
        let ch = if b < 0x80 {
            b as char
        } else {
            text[i..].chars().next().unwrap()
        };
        let cl = ch.len_utf8();
        if jef && matches!(ch, 'е' | 'ё' | 'ю' | 'я') {
            if let Some(rep) = jeficate(ch, prev) {
                out.push_str(&text[flush..i]);
                out.push_str(rep);
                flush = i + cl;
                prev = Some(ch);
                i += cl;
                continue;
            }
        }
        // latinJi `(class-char spaces*)і → $1ji`, before `ʼ`-deletion.
        if rule5 && is_rule5_ctx(ch) {
            if let Some(end) = rule5_end(text, i) {
                out.push_str(&text[flush..i]);
                push_rule5_ctx(&mut out, ch);
                out.push_str(&text[i + cl..end]);
                out.push_str("ji");
                flush = end + 'і'.len_utf8();
                prev = Some('і');
                i = flush;
                continue;
            }
        }
        if ch == 'ʼ' {
            let after = i + cl;
            if text[after..].starts_with('і') {
                // `ʼі → ji` (latin only; latinJi is covered by rule5 above).
                if !rule5 {
                    out.push_str(&text[flush..i]);
                    out.push_str("ji");
                    flush = after + 'і'.len_utf8();
                    prev = Some('і');
                    i = flush;
                    continue;
                }
            }
            // Lone `ʼ` deletes (flush the pending span, skip the mark).
            out.push_str(&text[flush..i]);
            flush = after;
            prev = Some('ʼ');
            i = after;
            continue;
        }
        // Soft clusters, with an `ʼ`-run allowed before `ь`
        // (`цʼь` → `ć`, like deletion-then-cluster sequentially).
        if matches!(ch, 'ц' | 'з' | 'с' | 'н' | 'л') {
            let mut j = i + cl;
            while text[j..].starts_with('ʼ') {
                j += 'ʼ'.len_utf8();
            }
            if j < len {
                let c2 = text[j..].chars().next().unwrap();
                if c2 == 'ь' {
                    if let Some(rep) = soft_cluster(ch) {
                        out.push_str(&text[flush..i]);
                        out.push_str(rep);
                        flush = j + 'ь'.len_utf8();
                        prev = Some('ь');
                        i = flush;
                        continue;
                    }
                }
            }
        }
        // `л + vowel` clusters (after the soft ones, like the dict).
        if ch == 'л' && i + cl < len {
            let c2 = text[i + cl..].chars().next().unwrap();
            if let Some(rep) = la_cluster(c2) {
                // rule5 (`([CTX] *)і`, entry 13, latinJi only) runs before
                // the clusters: every `л`-vowel is a rule5 context char,
                // so a vowel starting a rule5 match keeps `Vji`
                // (`Аўстраліі` → `..liji`, not `..lii`).
                let c2pos = i + cl;
                if rule5 {
                    if let Some(rend) = rule5_end(text, c2pos) {
                        out.push_str(&text[flush..i]);
                        out.push_str(rep);
                        out.push_str(&text[c2pos + c2.len_utf8()..rend]);
                        out.push_str("ji");
                        flush = rend + 'і'.len_utf8();
                        prev = Some('і');
                        i = flush;
                        continue;
                    }
                }
                out.push_str(&text[flush..i]);
                out.push_str(rep);
                flush = i + cl + c2.len_utf8();
                prev = Some(c2);
                i = flush;
                continue;
            }
        }
        if let Some(rep) = latin_single_lower(ch) {
            out.push_str(&text[flush..i]);
            out.push_str(rep);
            flush = i + cl;
            prev = Some(ch);
            i += cl;
            continue;
        }
        prev = Some(ch);
        i += cl;
    }
    out.push_str(&text[flush..]);
    out
}

/// `([eouaаеёіоуыэюяʼАЕЁІОУЫЭЮЯЬ] *)` context char (latinJi rule5).
fn is_rule5_ctx(c: char) -> bool {
    matches!(
        c,
        'e' | 'o'
            | 'u'
            | 'a'
            | 'а'
            | 'е'
            | 'ё'
            | 'і'
            | 'о'
            | 'у'
            | 'ы'
            | 'э'
            | 'ю'
            | 'я'
            | 'ʼ'
            | 'А'
            | 'Е'
            | 'Ё'
            | 'І'
            | 'О'
            | 'У'
            | 'Ы'
            | 'Э'
            | 'Ю'
            | 'Я'
            | 'Ь'
    )
}

/// Emit a rule5 `$1` context char mapped as the singles pass would:
/// lowercase vowels through [`latin_single_lower`], `ʼ` dropped (deleted
/// later sequentially — dropped here directly), the rest verbatim.
fn push_rule5_ctx(out: &mut String, c: char) {
    if c == 'ʼ' {
        return;
    }
    if let Some(rep) = latin_single_lower(c) {
        out.push_str(rep);
    } else {
        out.push(c);
    }
}

/// End (byte index of `і`) of a rule5 match starting at context `i`,
/// or `None`: class char at `i`, then ` *`, then `і`.
fn rule5_end(text: &str, i: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut j = i + text[i..].chars().next().unwrap().len_utf8();
    while j < bytes.len() && bytes[j] == b' ' {
        j += 1;
    }
    if text[j..].starts_with('і') {
        Some(j)
    } else {
        None
    }
}

// ---------- latinJi/lower: Vow + iwords pass ----------

/// Whether `c` is in `[аеёіоуыэюяАЕЁІОУЫЭЮЯ]`
/// (the latinJi `V` — note: no `ў`/`Ў`, no acute, unlike iotacize).
#[inline]
fn is_ji_vow(c: char) -> bool {
    matches!(
        c,
        'а' | 'е'
            | 'ё'
            | 'і'
            | 'о'
            | 'у'
            | 'ы'
            | 'э'
            | 'ю'
            | 'я'
            | 'А'
            | 'Е'
            | 'Ё'
            | 'І'
            | 'О'
            | 'У'
            | 'Ы'
            | 'Э'
            | 'Ю'
            | 'Я'
    )
}

/// Uppercase `iwords` alternation (`iwords.toUpperCase()` in JS), bool only.
/// Rare path (uppercase `І`): flat prefix scan in pattern order.
fn matches_iwords_upper(s: &str) -> bool {
    if s.starts_with('́') {
        return true;
    }
    if s.starts_with('Б') {
        if s.starts_with("БІС") {
            return true;
        }
        if s.starts_with("БСЭН") {
            return true;
        }
    }
    if s.starts_with('В') {
        if s.starts_with("ВА ") {
            return true;
        }
        if s.starts_with("ВЕ ") {
            return true;
        }
        if s.starts_with("ВО ") {
            return true;
        }
        if s.starts_with("ВЫ ") {
            return true;
        }
        if s.starts_with("ВЕРС") {
            return true;
        }
        if s.starts_with("ВАЛГ") {
            return true;
        }
        if s.starts_with("ВАЛЗ") {
            return true;
        }
    }
    if s.starts_with('Г') {
        if s.starts_with("ГАР") {
            return true;
        }
        if s.starts_with("ГРЫШЧ") {
            return true;
        }
        if s.starts_with("ГРЭК") {
            return true;
        }
    }
    if s.starts_with('Д') {
        if s.starts_with("ДАЛ") {
            return true;
        }
        if s.starts_with("ДЫШ") {
            return true;
        }
    }
    if s.starts_with("ЖЫЦ") {
        return true;
    }
    if s.starts_with('К') {
        if s.starts_with("КАНАПІС") {
            return true;
        }
        if s.starts_with("КАНЬН") {
            return true;
        }
        if s.starts_with("КАНН") {
            return true;
        }
        if s.starts_with("КАЦ") {
            return true;
        }
        if s.starts_with("КАЎ") {
            return true;
        }
        if s.starts_with("КАЛ") {
            return true;
        }
        if s.starts_with("КАЎК") {
            return true;
        }
        if s.starts_with("КАЎЦ") {
            return true;
        }
        if s.starts_with("КЛЫ ") {
            return true;
        }
        if s.starts_with("КЛЫЯ ") {
            return true;
        }
        if s.starts_with("КЛАЯ ") {
            return true;
        }
        if s.starts_with("КЛАЕ ") {
            return true;
        }
        if s.starts_with("КЛАЙ ") {
            return true;
        }
        if s.starts_with("КЛАГА ") {
            return true;
        }
        if s.starts_with("КЛАМУ ") {
            return true;
        }
        if s.starts_with("КЛУЮ ") {
            return true;
        }
        if s.starts_with("КСІ") {
            return true;
        }
    }
    if s.starts_with('Л') {
        if s.starts_with("ЛЕУС") {
            return true;
        }
        if s.starts_with("ЛІСТ") {
            return true;
        }
        if s.starts_with("Л ") {
            return true;
        }
        if s.starts_with("ЛІСТАС") {
            return true;
        }
        if s.starts_with("ЛЬК") {
            return true;
        }
    }
    if s.starts_with('М') {
        if s.starts_with("М ") {
            return true;
        }
        if s.starts_with("МАНТ") {
            return true;
        }
        if s.starts_with("МАСЬЦ") {
            return true;
        }
        if s.starts_with("МАСЦ") {
            return true;
        }
        if s.starts_with("МБРЫК") {
            return true;
        }
        if s.starts_with("МБРЫЧ") {
            return true;
        }
        if s.starts_with("МЕННА ") {
            return true;
        }
        if s.starts_with("МІДЖ") {
            return true;
        }
        if s.starts_with("МПАРТ") {
            return true;
        }
        if s.starts_with("МПАРЦ") {
            return true;
        }
        if s.starts_with("МПУЛЬСА") {
            return true;
        }
        if s.starts_with("МПУЛЬСЕ") {
            return true;
        }
        if s.starts_with("МПУЛЬСУ") {
            return true;
        }
        if s.starts_with("МПУЛЬСЫ") {
            return true;
        }
    }
    if s.starts_with('Н') {
        if s.starts_with("НАХАДЗ") {
            return true;
        }
        if s.starts_with("НДЫІ ") {
            return true;
        }
        if s.starts_with("НДЫЙ ") {
            return true;
        }
        if s.starts_with("НДЫЮ ") {
            return true;
        }
        if s.starts_with("НДЫЯ ") {
            return true;
        }
        if s.starts_with("НДЫЕВ") {
            return true;
        }
        if s.starts_with("НДЭКСАЎ ") {
            return true;
        }
        if s.starts_with("НДЭКСАМІ ") {
            return true;
        }
        if s.starts_with("НДЭКСАМ ") {
            return true;
        }
        if s.starts_with("НДЭКСА ") {
            return true;
        }
        if s.starts_with("НДЭКСЕ") {
            return true;
        }
        if s.starts_with("НДЭКСУ") {
            return true;
        }
        if s.starts_with("НДЭКСЫ") {
            return true;
        }
        if s.starts_with("НДЭКСІ") {
            return true;
        }
        if s.starts_with("НЕЕ") {
            return true;
        }
        if s.starts_with("НЕЙ") {
            return true;
        }
        if s.starts_with("НЕЯ") {
            return true;
        }
        if s.starts_with("НЕЮ") {
            return true;
        }
        if s.starts_with("НІЕ") {
            return true;
        }
        if s.starts_with("НІЙ") {
            return true;
        }
        if s.starts_with("НІЯ") {
            return true;
        }
        if s.starts_with("НІЮ") {
            return true;
        }
        if s.starts_with("НКА") {
            return true;
        }
        if s.starts_with("НКІ") {
            return true;
        }
        if s.starts_with("НКУ") {
            return true;
        }
        if s.starts_with("НТЭРЫМ") {
            return true;
        }
        if s.starts_with("НФІКС") {
            return true;
        }
        if s.starts_with("НФІМУМ") {
            return true;
        }
        if s.starts_with("НШАСЦ") {
            return true;
        }
        if s.starts_with("НШАСЬЦ") {
            return true;
        }
        if s.starts_with("НША ") {
            return true;
        }
        if s.starts_with("НШАЕ ") {
            return true;
        }
        if s.starts_with("НШАЙ ") {
            return true;
        }
        if s.starts_with("НШАЯ ") {
            return true;
        }
        if s.starts_with("НШАГА ") {
            return true;
        }
        if s.starts_with("НШАМУ ") {
            return true;
        }
        if s.starts_with("НШАСЦ ") {
            return true;
        }
        if s.starts_with("НШАСЬЦ ") {
            return true;
        }
        if s.starts_with("НШУЮ ") {
            return true;
        }
        if s.starts_with("НШЫ ") {
            return true;
        }
        if s.starts_with("НШЫМ ") {
            return true;
        }
        if s.starts_with("НШЫМІ ") {
            return true;
        }
        if s.starts_with("НШЫХ ") {
            return true;
        }
        if s.starts_with("НШЫЯ ") {
            return true;
        }
    }
    if s.starts_with("ПСІЛАН") {
        return true;
    }
    if s.starts_with('Р') {
        if s.starts_with("РА ") {
            return true;
        }
        if s.starts_with("РЫ ") {
            return true;
        }
        if s.starts_with("РУ ") {
            return true;
        }
        if s.starts_with("РАМ ") {
            return true;
        }
        if s.starts_with("РАХ ") {
            return true;
        }
        if s.starts_with("РАЙ ") {
            return true;
        }
        if s.starts_with("РАЎ ") {
            return true;
        }
        if s.starts_with("РАМІ ") {
            return true;
        }
        if s.starts_with("Р ") {
            return true;
        }
        if s.starts_with("РАД") {
            return true;
        }
        if s.starts_with("РБІС") {
            return true;
        }
        if s.starts_with("РМАС") {
            return true;
        }
        if s.starts_with("РХА") {
            return true;
        }
        if s.starts_with("РЫС ") {
            return true;
        }
    }
    if s.starts_with('С') {
        if s.starts_with("СКАРК") {
            return true;
        }
        if s.starts_with("СКАРАК") {
            return true;
        }
        if s.starts_with("СКРА") {
            return true;
        }
        if s.starts_with("СКРАВЕЦ") {
            return true;
        }
        if s.starts_with("СКРАЧК") {
            return true;
        }
        if s.starts_with("СТА ") {
            return true;
        }
        if s.starts_with("СНАСЦ") {
            return true;
        }
        if s.starts_with("СНАСЬЦ") {
            return true;
        }
        if s.starts_with("СТАСЦ") {
            return true;
        }
        if s.starts_with("СТАСЬЦ") {
            return true;
        }
        if s.starts_with("СЦІН") {
            return true;
        }
        if s.starts_with("СЦІК") {
            return true;
        }
        if s.starts_with("СЬЦІН") {
            return true;
        }
        if s.starts_with("СЬЦІК") {
            return true;
        }
    }
    if s.starts_with('Т') {
        if s.starts_with("ТАР") {
            return true;
        }
        if s.starts_with("ТА ") {
            return true;
        }
        if s.starts_with("ТРЫ") {
            return true;
        }
    }
    if s.starts_with('Х') {
        if s.starts_with("ХНЫХ") {
            return true;
        }
        if s.starts_with("ХНЫЯ") {
            return true;
        }
        if s.starts_with("ХНЫ") {
            return true;
        }
        if s.starts_with("ХНУЮ") {
            return true;
        }
        if s.starts_with("ХНАЯ") {
            return true;
        }
        if s.starts_with("ХНАЕ") {
            return true;
        }
        if s.starts_with("ХНАЙ") {
            return true;
        }
        if s.starts_with("Х") {
            return true;
        }
    }
    if s.starts_with('Ц') {
        if s.starts_with("ЦЬВІН") {
            return true;
        }
        if s.starts_with("ЦВІН") {
            return true;
        }
    }
    if s.starts_with("ШЫЯС") {
        return true;
    }
    false
}

/// `(V )і/І Ў|ў|␣` trailers (dict order, entries 0–5).
///
/// Emits `j`/`J` (+ mapped trailer ` U`/` u`/space, `$1` kept for the
/// singles passes). Runs before `ji_iwords`: later entries match the
/// spaces it emits (`а і іншыя` → `а j іншыя` → `а j jinшыя`).
fn ji_trailers(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        // No trailer starts with ASCII (`V` is always 2-byte Cyrillic).
        if b < 0x80 {
            i += 1;
            continue;
        }
        // `V␣і` / `V␣І`? `V` is 2 bytes, so offsets below stay on boundaries.
        if let Some(v) = text[i..].chars().next() {
            if is_ji_vow(v)
                && (text[i + v.len_utf8()..].starts_with(" і")
                    || text[i + v.len_utf8()..].starts_with(" І"))
            {
                // Re-decode to learn which `і` case matched (both 2 bytes).
                let after_v = &text[i + v.len_utf8() + 1..];
                let is_lower = after_v.starts_with('і');
                let is_upper = after_v.starts_with('І');
                if is_lower || is_upper {
                    // After `і`/`І` (2 bytes): trailer ` Ў` / ` ў` / ` `.
                    let after_i = &after_v[2..];
                    let trailer: Option<&str> = if after_i.starts_with(" Ў") {
                        Some(" U")
                    } else if after_i.starts_with(" ў") {
                        Some(" u")
                    } else if after_i.starts_with(' ') {
                        Some(" ")
                    } else {
                        None
                    };
                    if let Some(t) = trailer {
                        // `V`(2) + space(1): copy through, then `j`/`J` + trailer.
                        let v_end = i + v.len_utf8() + 1;
                        out.push_str(&text[flush..v_end]);
                        out.push(if is_lower { 'j' } else { 'J' });
                        out.push_str(t);
                        // Consumed: V(2) + space + і(2) + trailer(1 or 3).
                        let end = if t == " " {
                            v_end + 'і'.len_utf8() + 1
                        } else {
                            v_end + 'і'.len_utf8() + 3
                        };
                        flush = end;
                        i = end;
                        continue;
                    }
                }
            }
        }
        i += if b < 0x80 { 1 } else { utf8_char_len(b) };
    }
    out.push_str(&text[flush..]);
    out
}

/// ` і/І(?=iwords)` (dict order, entries 6–8) over `ji_trailers` output,
/// whose emitted spaces can start new matches. `і` (lower, acute in both
/// lists) before `І` → `Ji`, then uppercase-only `JI`.
fn ji_iwords(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        // Both patterns start with a space.
        if b != b' ' {
            i += if b.is_ascii() { 1 } else { utf8_char_len(b) };
            continue;
        }
        // ` і` + iwords lookahead (` і` is space + 2-byte `і`).
        if text[i..].starts_with(" і") && crate::text::matches_iwords(&text[i + 3..]) {
            out.push_str(&text[flush..i + 1]);
            out.push_str("ji");
            flush = i + 3;
            i += 3;
            continue;
        }
        if text[i..].starts_with(" І") {
            if crate::text::matches_iwords(&text[i + 3..]) {
                out.push_str(&text[flush..i + 1]);
                out.push_str("Ji");
                flush = i + 3;
                i += 3;
                continue;
            }
            if matches_iwords_upper(&text[i + 3..]) {
                out.push_str(&text[flush..i + 1]);
                out.push_str("JI");
                flush = i + 3;
                i += 3;
                continue;
            }
        }
        i += if b < 0x80 { 1 } else { utf8_char_len(b) };
    }
    out.push_str(&text[flush..]);
    out
}

// ---------- latinJi/lower: jefication + rest ----------

/// Jefication over `ji_iwords` output (lookbehind sees that output — the
/// `Vow`-rule `й` outputs sit in later lookbehinds, so this is its own pass).
fn ji_jef(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    let mut prev: Option<char> = None;
    while i < len {
        let b = bytes[i];
        if b < 0x80 {
            prev = Some(b as char);
            i += 1;
            continue;
        }
        let ch = text[i..].chars().next().unwrap();
        let cl = ch.len_utf8();
        if matches!(ch, 'е' | 'ё' | 'ю' | 'я') {
            if let Some(rep) = jeficate(ch, prev) {
                out.push_str(&text[flush..i]);
                out.push_str(rep);
                flush = i + cl;
                prev = Some(ch);
                i += cl;
                continue;
            }
        }
        prev = Some(ch);
        i += cl;
    }
    out.push_str(&text[flush..]);
    out
}

/// latinJi lower entry point: `Vow/iwords` → jefication → rule5+rest.
pub(crate) fn convert_latin_ji_lower(text: &str) -> String {
    lower_pass(&ji_jef(&ji_iwords(&ji_trailers(text))), true, false)
}

// ---------- upper: ` [ЕЁЮЯ]` + lowercase lookahead (shared) ----------

/// `[ \p{P}\d]*\p{Lu}?\p{Ll}` from `s`: spaces, *punctuation* (not symbols
/// — e.g. `>` is Sm and blocks, like the regex), decimal digits, then
/// optional uppercase + required lowercase.
///
/// Deterministic: the skipped set is disjoint from Lu/Ll, so only the
/// maximal skip can succeed — no backtracking needed.
fn has_lower_after(s: &str) -> bool {
    let mut it = s.chars();
    let mut c = it.next();
    while matches!(c, Some(ch) if ch == ' ' || is_punct(ch as u32) || is_decimal_number(ch as u32))
    {
        c = it.next();
    }
    match c {
        Some(ch) if is_lu(ch as u32) => matches!(it.next(), Some(n) if is_ll(n as u32)),
        Some(ch) => is_ll(ch as u32),
        None => false,
    }
}

/// ` [VOWEL](?=[ \p{P}\d]*\p{Lu}?\p{Ll})` → ` J*`, one vowel per pass.
/// Separate passes in dict order (Е Ё Ю Я): later vowels' lookaheads see
/// earlier passes' output (`А Ю. Е. t`: ` Е` → ` Je` first, then ` Ю`
/// + `Je` lookahead → ` Ju`).
///
/// `vow` is the vowel's byte pair — `byte_pair("Е")` at the call site, so
/// the source reads as the character while the loop compares immediates.
/// All four vowels are 2-byte Cyrillic, hence the literal `3` below.
fn spaced_vow_upper(text: &str, vow: (u8, u8), je: &'static str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        // ` ${vow}`: space + 2-byte vowel.
        if bytes[i] == b' '
            && i + 3 <= len
            && (bytes[i + 1], bytes[i + 2]) == vow
            && has_lower_after(&text[i + 3..])
        {
            out.push_str(&text[flush..i]);
            out.push_str(je);
            flush = i + 3;
            i += 3;
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

// ---------- upper: `([CLASS]\(?)VOWEL` captures (4 passes) ----------

/// Byte length of the `$1` class char ending at byte `end` (a boundary),
/// or 0: 1-byte ASCII (` |` + Latin extras) or 2-byte Cyrillic base.
fn cap_class_len(text: &str, end: usize, extra: &[char]) -> usize {
    let Some(c) = text[..end].chars().next_back() else {
        return 0;
    };
    if c == ' ' || c == '|' || extra.contains(&c) {
        return c.len_utf8();
    }
    if matches!(
        c,
        'А' | 'Е' | 'Ё' | 'І' | 'О' | 'У' | 'Ў' | 'Ы' | 'Э' | 'Ю' | 'Я' | 'Ь'
    ) {
        return c.len_utf8();
    }
    0
}

/// One `([CLASS]\(?)VOWEL → $1J*` pass. `$1` (class char + optional `(`)
/// is echoed from the scanned text, so behind-context is exact — which is
/// why the four vowels stay four passes (`АЕЁ` → `AJEJO`: the second `$1`
/// is the first pass's output `E`).
///
/// `vow` is the vowel's byte pair (`byte_pair("Е")` at the call site).
/// All four vowels are 2-byte Cyrillic, hence the literal `2` below.
fn capture_upper_pass(
    text: &str,
    vow: (u8, u8),
    extra_latin: &[char],
    je: &'static str,
) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + 2 <= len && (bytes[i], bytes[i + 1]) == vow {
            // `$1` = class char, with an optional `(` between it and VOWEL.
            let mut k = i;
            if k >= 1 && bytes[k - 1] == b'(' {
                k -= 1;
            }
            let cl = cap_class_len(text, k, extra_latin);
            if cl > 0 {
                let start = k - cl;
                // `$1` reaching into an already-consumed match can never
                // fire: `replace_all` resumes searching past it.
                if start < flush {
                    i += 2;
                    continue;
                }
                out.push_str(&text[flush..start]);
                out.push_str(&text[start..i]);
                out.push_str(je);
                flush = i + 2;
                i += 2;
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

// ---------- upper: latinJi `І` rules (own passes, dict order) ----------

/// `І` as a byte pair (spelled with the literal; hot loops compare bytes).
const II_PAIR: (u8, u8) = byte_pair("І");
/// `І` length (all uses below are 2-byte).
const II_LEN: usize = 2;

/// `([eoua] *)І(?=[ \p{P}\d]*\p{Lu}?\p{Ll})` → `$1Ji` (latinJi only).
fn eoua_i_upper(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + II_LEN <= len && (bytes[i], bytes[i + 1]) == II_PAIR && has_lower_after(&text[i + II_LEN..]) {
            let mut k = i;
            while k > 0 && bytes[k - 1] == b' ' {
                k -= 1;
            }
            if k > 0 && matches!(bytes[k - 1], b'e' | b'o' | b'u' | b'a') {
                let start = k - 1;
                if start < flush {
                    i += II_LEN;
                    continue;
                }
                out.push_str(&text[flush..start]);
                out.push_str(&text[start..i]);
                out.push_str("Ji");
                flush = i + II_LEN;
                i += II_LEN;
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

/// `([AOEUАЕЁІОУЎЫЭЮЯ][( ]*)І` → `$1JI` (latinJi only, no lookahead).
fn aoeu_i_upper(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if i + II_LEN <= len && (bytes[i], bytes[i + 1]) == II_PAIR {
            let mut k = i;
            while k > 0 && (bytes[k - 1] == b'(' || bytes[k - 1] == b' ') {
                k -= 1;
            }
            let start = if k >= 1 && matches!(bytes[k - 1], b'A' | b'O' | b'E' | b'U') {
                Some(k - 1)
            } else if let Some(c) = text[..k].chars().next_back() {
                if matches!(
                    c,
                    'А' | 'Е' | 'Ё' | 'І' | 'О' | 'У' | 'Ў' | 'Ы' | 'Э' | 'Ю' | 'Я'
                ) {
                    Some(k - c.len_utf8())
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(s) = start {
                if s < flush {
                    i += II_LEN;
                    continue;
                }
                out.push_str(&text[flush..s]);
                out.push_str(&text[s..i]);
                out.push_str("JI");
                flush = i + II_LEN;
                i += II_LEN;
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

// ---------- upper: singles + clusters + `Х` (shared, dict order) ----------

/// Uppercase single letter → Latin (`None` = passthrough).
fn latin_single_upper(c: char) -> Option<&'static str> {
    Some(match c {
        'А' => "A",
        'Б' => "B",
        'В' => "V",
        'Г' => "H",
        'Ґ' => "G",
        'Д' => "D",
        'Ж' => "Ž",
        'З' => "Z",
        'І' => "I",
        'Й' => "J",
        'К' => "K",
        'Л' => "Ł",
        'М' => "M",
        'Н' => "N",
        'О' => "O",
        'П' => "P",
        'Р' => "R",
        'С' => "S",
        'Т' => "T",
        'У' => "U",
        'Ў' => "Ŭ",
        'Ф' => "F",
        'Х' => "CH",
        'Ц' => "C",
        'Ч' => "Č",
        'Ш' => "Š",
        'Ы' => "Y",
        'Э' => "E",
        _ => return None,
    })
}

/// Lead byte of 2-byte Cyrillic (`А`–`я`, `Ё`… — all `D0`/`D1` block chars
/// used here share it except `Ґ`), spelled with the literal.
const LEAD_D0: u8 = "А".as_bytes()[0];
/// Lead byte of `Ґ`/`ґ` (the only `D2`-lead letter used here).
const LEAD_D2: u8 = "Ґ".as_bytes()[0];
/// Second bytes of the `ЕЁЮЯ` singles and the `ЦЗСНЛ` soft heads below —
/// each spelled with its character, so the matches read as characters.
const YE_2ND: u8 = "Е".as_bytes()[1];
const YO_2ND: u8 = "Ё".as_bytes()[1];
const YU_2ND: u8 = "Ю".as_bytes()[1];
const YA_2ND: u8 = "Я".as_bytes()[1];
const TSE_2ND: u8 = "Ц".as_bytes()[1];
const ZE_2ND: u8 = "З".as_bytes()[1];
const ES_2ND: u8 = "С".as_bytes()[1];
const EN_2ND: u8 = "Н".as_bytes()[1];
const EL_2ND: u8 = "Л".as_bytes()[1];
/// ` Х`, `Ь`, `ь`, `Ґ` as byte slices for `memcmp`-style matching.
const KHA_SP: &[u8] = " Х".as_bytes();
const SOFT_UPPER: &[u8] = "Ь".as_bytes();
const SOFT_LOWER: &[u8] = "ь".as_bytes();

/// Upper singles/clusters/`Х` in dict order: `ЕЁЮЯ`, `Ц[Ьь]`…`Л[Ьь]`,
/// `А`…`Э`, ` Х(?=[\p{Ll} ])` before `Х`, then `ЦЧШЫЭ`.
fn upper_singles(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        if b.is_ascii() {
            // Only ` Х` starts with ASCII here (a space).
            if b != b' ' {
                i += 1;
                continue;
            }
            // ` Х` is space(1) + `Х`(2).
            if bytes[i..].starts_with(KHA_SP) {
                let after = i + KHA_SP.len();
                let ll = if after < len {
                    if bytes[after] == b' ' {
                        true
                    } else {
                        match text[after..].chars().next() {
                            Some(ch) => is_ll(ch as u32),
                            None => false,
                        }
                    }
                } else {
                    false
                };
                if ll {
                    out.push_str(&text[flush..i]);
                    out.push_str(" Ch");
                    flush = after;
                    i = after;
                    continue;
                }
            }
            i += 1;
            continue;
        }
        // All remaining singles are `D0`/`D2`-lead 2-byte Cyrillic:
        // anything else advances untouched.
        if (b != LEAD_D0 && b != LEAD_D2) || i + 2 > len {
            i += utf8_char_len(b);
            continue;
        }
        if b == LEAD_D0 {
            // `ЕЁЮЯ` singles.
            let rep: Option<&'static str> = match bytes[i + 1] {
                YE_2ND => Some("IE"),
                YO_2ND => Some("IO"),
                YU_2ND => Some("IU"),
                YA_2ND => Some("IA"),
                _ => None,
            };
            if let Some(r) = rep {
                out.push_str(&text[flush..i]);
                out.push_str(r);
                flush = i + 2;
                i += 2;
                continue;
            }
            // `Ц[Ьь]` etc.: soft sign either case (dict order Ц З С Н Л).
            let soft: Option<&'static str> = match bytes[i + 1] {
                TSE_2ND => Some("Ć"),
                ZE_2ND => Some("Ź"),
                ES_2ND => Some("Ś"),
                EN_2ND => Some("Ń"),
                EL_2ND => Some("L"),
                _ => None,
            };
            if let Some(r) = soft {
                let after_head = &bytes[i + 2..];
                if after_head.starts_with(SOFT_UPPER) || after_head.starts_with(SOFT_LOWER)
                {
                    out.push_str(&text[flush..i]);
                    out.push_str(r);
                    flush = i + 4;
                    i += 4;
                    continue;
                }
            }
            // Plain uppercase singles (incl. `Х→CH` fallback here).
            if let Some(ch) = text[i..].chars().next() {
                if let Some(r) = latin_single_upper(ch) {
                    let cl = ch.len_utf8();
                    out.push_str(&text[flush..i]);
                    out.push_str(r);
                    flush = i + cl;
                    i += cl;
                    continue;
                }
            }
        } else {
            // `D2` lead: only `Ґ` (handled by the singles table).
            if let Some(ch) = text[i..].chars().next() {
                if let Some(r) = latin_single_upper(ch) {
                    let cl = ch.len_utf8();
                    out.push_str(&text[flush..i]);
                    out.push_str(r);
                    flush = i + cl;
                    i += cl;
                    continue;
                }
            }
        }
        i += if b < 0x80 { 1 } else { utf8_char_len(b) };
    }
    out.push_str(&text[flush..]);
    out
}

// ---------- upper: `Ł` fixes (+ latinJi ` JIŁ -`) ----------

/// ` JIŁ -` fix pattern as bytes (spelled with the literal).
const JI_LFIX: &[u8] = " JIŁ -".as_bytes();
/// ` IŁ -` replacement (same length as the pattern).
const I_LFIX: &str = " IŁ -";
/// `Ł` as a byte pair (spelled with the literal).
const STROKE_PAIR: (u8, u8) = byte_pair("Ł");
/// `Ł` length (all uses below are 2-byte).
const STROKE_LEN: usize = 2;

/// `Ł[Ii](?=[AEOUaeou])` → `L`, `Ł(?=[Ii])` → `L`; latinJi additionally
/// ` JIŁ -` → ` IŁ -`. Runs after the singles pass (needs converted `Ł`).
fn sharp_l_fix(text: &str, ji_fix: bool) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        if b.is_ascii() {
            // Only ` JIŁ -` starts with ASCII here (a space).
            if b == b' ' && ji_fix && bytes[i..].starts_with(JI_LFIX) {
                out.push_str(&text[flush..i]);
                out.push_str(I_LFIX);
                flush = i + JI_LFIX.len();
                i += JI_LFIX.len();
                continue;
            }
            i += 1;
            continue;
        }
        // `Ł` is 2-byte: anything else (ASCII included) advances.
        if i + STROKE_LEN > len || (bytes[i], bytes[i + 1]) != STROKE_PAIR {
            i += if b.is_ascii() { 1 } else { utf8_char_len(b) };
            continue;
        }
        {
            let next = bytes.get(i + STROKE_LEN).copied().unwrap_or(0);
            if next == b'I' || next == b'i' {
                let foll = bytes
                    .get(i + STROKE_LEN + 1)
                    .copied()
                    .unwrap_or(0);
                if matches!(
                    foll,
                    b'A' | b'E' | b'O' | b'U' | b'a' | b'e' | b'o' | b'u'
                ) {
                    // Fix 1 consumes `ŁI`/`Łi`.
                    out.push_str(&text[flush..i]);
                    out.push('L');
                    flush = i + STROKE_LEN + 1;
                    i += STROKE_LEN + 1;
                    continue;
                }
                // Fix 2 consumes only `Ł`.
                out.push_str(&text[flush..i]);
                out.push('L');
                flush = i + STROKE_LEN;
                i += STROKE_LEN;
                continue;
            }
            i += STROKE_LEN;
        }
    }
    out.push_str(&text[flush..]);
    out
}

// ---------- entry points ----------

/// latin lower → upper (dict order: spaced-`ЕЁЮЯ`, captures, singles, `Ł`-fix).
pub(crate) fn convert_latin_upper(text: &str) -> String {
    let t = spaced_vow_upper(text, byte_pair("Е"), " Je");
    let t = spaced_vow_upper(&t, byte_pair("Ё"), " Jo");
    let t = spaced_vow_upper(&t, byte_pair("Ю"), " Ju");
    let t = spaced_vow_upper(&t, byte_pair("Я"), " Ja");
    let t = capture_upper_pass(&t, byte_pair("Е"), &[], "JE");
    let t = capture_upper_pass(&t, byte_pair("Ё"), &['E'], "JO");
    let t = capture_upper_pass(&t, byte_pair("Ю"), &['E', 'O'], "JU");
    let t = capture_upper_pass(&t, byte_pair("Я"), &['E', 'O', 'U'], "JA");
    sharp_l_fix(&upper_singles(&t), false)
}

/// latinJi lower → upper (+ `І` rules and the ` JIŁ -` fix).
pub(crate) fn convert_latin_ji_upper(text: &str) -> String {
    let t = spaced_vow_upper(text, byte_pair("Е"), " Je");
    let t = spaced_vow_upper(&t, byte_pair("Ё"), " Jo");
    let t = spaced_vow_upper(&t, byte_pair("Ю"), " Ju");
    let t = spaced_vow_upper(&t, byte_pair("Я"), " Ja");
    let t = eoua_i_upper(&t);
    let t = capture_upper_pass(&t, byte_pair("Е"), &[], "JE");
    let t = capture_upper_pass(&t, byte_pair("Ё"), &['E'], "JO");
    let t = capture_upper_pass(&t, byte_pair("Ю"), &['E', 'O'], "JU");
    let t = capture_upper_pass(&t, byte_pair("Я"), &['E', 'O', 'U'], "JA");
    let t = aoeu_i_upper(&t);
    sharp_l_fix(&upper_singles(&t), true)
}
#[cfg(test)]
mod tests {
    use super::super::test_oracle::{old_abc, OldAbc};
    use super::*;
    use crate::config::Alphabet;
    use std::sync::LazyLock;

    static OLD_LAT: LazyLock<OldAbc> = LazyLock::new(|| old_abc(Alphabet::Latin));
    static OLD_JI: LazyLock<OldAbc> = LazyLock::new(|| old_abc(Alphabet::LatinJi));

    fn check_lat(text: &str) {
        assert_eq!(
            convert_latin_lower(text),
            OLD_LAT.lower(text),
            "latin-lower {text:?}"
        );
    }

    fn check_ji(text: &str) {
        assert_eq!(
            convert_latin_ji_lower(text),
            OLD_JI.lower(text),
            "ji-lower {text:?}"
        );
    }

    fn check_lat_full(text: &str) {
        assert_eq!(
            convert_latin_upper(&convert_latin_lower(text)),
            OLD_LAT.full(text),
            "latin-full {text:?}"
        );
    }

    fn check_ji_full(text: &str) {
        assert_eq!(
            convert_latin_ji_upper(&convert_latin_ji_lower(text)),
            OLD_JI.full(text),
            "ji-full {text:?}"
        );
    }

    #[test]
    fn latin_lower_letters() {
        // Every Cyrillic codepoint around the alphabet + ASCII + specials.
        let mut inputs = Vec::new();
        for cp in 0x0400u32..0x0460 {
            inputs.push(char::from_u32(cp).unwrap().to_string());
        }
        for cp in 0x20u32..0x7F {
            inputs.push(char::from_u32(cp).unwrap().to_string());
        }
        for s in [
            "ʼ", "ь", "Ь", "`", "’", "́", "ł", "Ł", "č", "ž", "ŭ", "ć", "ś", "ź", "ń", "04", "«»",
            "—", "…", "\n", "\u{a0}", "\t", "і", "І", "ї", "Ј",
        ] {
            inputs.push(s.to_string());
        }
        for input in &inputs {
            check_lat(input);
            check_lat(&format!("x{input}y"));
            check_lat(&format!("а{input}"));
        }
    }

    #[test]
    fn latin_lower_clusters() {
        for c in [
            "ць", "зь", "сь", "нь", "ль", "лі", "ля", "лё", "лю", "ле", "Ць", "ЦЬ", "ЗЬ", "ЛЬ",
            "ЛІ", "цЬ", "Ль", "льʼь", "цʼь", "цʼʼь", "лʼь", "лʼі", "ʼі", "ʼʼі", "ʼ", "цʼа", "лʼ",
            "льʼь", "ньʼ",
        ] {
            check_lat(c);
            check_lat(&format!("а{c}о"));
            check_lat(&format!("{c} {c}"));
        }
    }

    #[test]
    fn latin_lower_jefication() {
        let behinds = [
            "", "а", "е", "ё", "і", "о", "у", "ў", "ы", "э", "ю", "я", "ь", "ʼ", "|", " ", ">",
            "А", "Е", "Ё", "І", "О", "У", "Ў", "Ы", "Э", "Ю", "Я", "Ь", "й", "б", "к", "е", "о",
            "у", "1", ".",
        ];
        for ch in ['е', 'ё', 'ю', 'я'] {
            for b in behinds {
                check_lat(&format!("{b}{ch}"));
                check_lat(&format!("{b}{ch} "));
            }
        }
        // Documented behaviors from the reference implementation.
        assert_eq!(convert_latin_lower("еі"), "jei");
        assert_eq!(convert_latin_lower("ае"), "aje");
        assert_eq!(convert_latin_lower("Ее"), "Еje");
        assert_eq!(convert_latin_lower("ее"), "jeje");
        assert_eq!(convert_latin_lower("лʼі"), "łji");
        assert_eq!(convert_latin_lower("цʼь"), "ć");
    }

    #[test]
    fn ji_vow_rules() {
        let vowels = [
            'а', 'е', 'ё', 'і', 'о', 'у', 'ы', 'э', 'ю', 'я', 'А', 'Е', 'Ё', 'І', 'О', 'У', 'Ы',
            'Э', 'Ю', 'Я',
        ];
        for v in vowels {
            for mid in ['і', 'І'] {
                for tr in ["Ў", "ў", " ", ""] {
                    check_ji(&format!("{v} {mid}{tr}"));
                    check_ji(&format!("x{v} {mid}{tr}y"));
                    // Spaced trailers `V␣і/І␣Ў|ў|␣` (dict order).
                    check_ji(&format!("{v} {mid} {tr}"));
                    check_ji(&format!("x{v} {mid} {tr}y"));
                }
            }
        }
        // No `ў`/`Ў` in the latinJi vowel class (unlike iotacize).
        check_ji("ў і ў");
        check_ji("а і");
        check_ji("а  і");
        // Regression: spaced `ў`/`Ў` trailers (integration case).
        assert_eq!(convert_latin_ji_lower("я і ўваліўся"), "ja j uvaliŭsia");
        assert_eq!(convert_latin_ji_lower("а і Ўб"), "a j Ub");
        assert_eq!(convert_latin_ji_lower("а і ўб"), "a j ub");
        // Regression: trailer-emitted spaces feed `і→ji`
        // (`а і іншыя` → `а j іншыя` → `а j jinшыя`).
        for s in ["а і іншыя", " яна і іншыя ", " і іншыя", "а і біс"] {
            check_ji(s);
        }
        assert_eq!(convert_latin_ji_lower(" яна і іншыя "), " jana j jinšyja ");
    }

    #[test]
    fn ji_iwords_rules() {
        for w in [
            "біс", "х", "м ", "р ", "ва ", "тар", "тары", "хны", "цвін", "шыяс", "б", "мама", "і",
            "л ",
        ] {
            check_ji(&format!(" і{w}"));
            check_ji(&format!(" І{w}"));
        }
        // Uppercase iwords only fires on uppercase `І`.
        for w in ["БІС", "Х", "М ", "ВА ", "ТАР", "біс", "х"] {
            check_ji(&format!(" І{w}"));
            check_ji(&format!(" і{w}"));
        }
    }

    #[test]
    fn ji_rule5_and_rest() {
        // Every rule5 context char + `і`, with 0–2 spaces.
        for c in [
            'e', 'o', 'u', 'a', 'а', 'е', 'ё', 'і', 'о', 'у', 'ы', 'э', 'ю', 'я', 'ʼ', 'А', 'Е',
            'Ё', 'І', 'О', 'У', 'Ы', 'Э', 'Ю', 'Я', 'Ь',
        ] {
            check_ji(&format!("{c}і"));
            check_ji(&format!("{c} і"));
            check_ji(&format!("{c}  і"));
            check_ji(&format!("x{c}іy"));
        }
        // Non-context chars stay single-mapped.
        for c in ['б', 'й', 'ў', 'ь', 'к', ' ', '1'] {
            check_ji(&format!("{c}і"));
        }
        assert_eq!(convert_latin_ji_lower("поіць"), "pojić");
        assert_eq!(convert_latin_ji_lower("еі"), "jeji");
        assert_eq!(convert_latin_ji_lower("аі"), "aji");
        assert_eq!(convert_latin_ji_lower("а і"), "a ji");
        assert_eq!(convert_latin_ji_lower("ʼі"), "ji");
        // rule5 wins over `л`-clusters whose vowel is its context
        // (entry 13 runs before the clusters).
        for v in ['і', 'я', 'ё', 'ю', 'е'] {
            check_ji(&format!("л{v}і"));
            check_ji(&format!("л{v} і"));
            check_ji(&format!("л{v}  і"));
            check_ji(&format!("xл{v}іy"));
            check_ji(&format!("л{v}x"));
        }
        assert_eq!(convert_latin_ji_lower("Аўстраліі"), "Аŭstraliji");
        assert_eq!(
            convert_latin_ji_lower("для інтэрнэт-крамы"),
            "dla jinternet-kramy"
        );
        assert_eq!(convert_latin_ji_lower("Асамблеі"), "Аsambleji");
    }

    #[test]
    fn upper_spaced_vowels() {
        for v in ["Е", "Ё", "Ю", "Я"] {
            for after in [
                "а", "a", "Z", "2a", ",а", " Аа", "а ", "5", ".", " Еа", "е", "",
            ] {
                check_lat_full(&format!(" {v}{after}"));
                check_ji_full(&format!(" {v}{after}"));
            }
        }
        assert_eq!(convert_latin_upper(&convert_latin_lower(" Еа")), " Jea");
        assert_eq!(convert_latin_upper(&convert_latin_lower("ЕЕ")), "IEJE");
        assert_eq!(convert_latin_upper(&convert_latin_lower("АЕ")), "AJE");
        assert_eq!(
            convert_latin_upper(&convert_latin_ji_lower(" Е2а")),
            " Je2a"
        );
        // Spaced entries chain in dict order: `Е.` → `Je` first, then
        // `Ю` + `Je` lookahead → `Ju` (one fused scan would miss it).
        for s in ["А Ю. Е. t", "А Ю. Ё. x", "А Я. Е. z", " Ю. Е. Foo"] {
            check_ji_full(s);
            check_lat_full(s);
        }
        assert_eq!(
            convert_latin_ji_upper(&convert_latin_ji_lower("А Ю. Е. t")),
            "A Ju. Je. t"
        );
    }

    #[test]
    fn upper_captures() {
        // Behind variants: start (miss), vowel, space, `|`, paren, Latin EOU.
        for v in ["Е", "Ё", "Ю", "Я"] {
            for b in ["А", " ", "|", "(", "А(", "E", " І", "б"] {
                check_lat_full(&format!("{b}{v}"));
                check_lat_full(&format!("x{b}{v}y"));
            }
        }
        // Output-behind chaining (`АЕЁ` → `AJEJO`) forces separate passes.
        assert_eq!(convert_latin_upper(&convert_latin_lower("АЕЁ")), "AJEJO");
        assert_eq!(convert_latin_upper(&convert_latin_lower("ЕЕ")), "IEJE");
    }

    #[test]
    fn upper_singles_and_x() {
        for c in [
            'А', 'Б', 'В', 'Г', 'Ґ', 'Д', 'Ж', 'З', 'І', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р',
            'С', 'Т', 'У', 'Ў', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Ы', 'Э', 'Ь',
        ] {
            check_lat_full(&c.to_string());
            check_lat_full(&format!("а{c}"));
        }
        for s in [
            "Ць", "ЦЬ", "ЗЬ", "СЬ", "НЬ", "ЛЬ", " Хa", " Х ", "Х", " ХА", " ХA", "Хa",
        ] {
            check_lat_full(s);
            check_ji_full(s);
        }
        assert_eq!(convert_latin_upper(&convert_latin_lower(" Хa")), " Cha");
        assert_eq!(convert_latin_upper(&convert_latin_lower("ЛІ")), "LI");
        assert_eq!(convert_latin_upper(&convert_latin_lower("ЛІА")), "LA");
    }

    #[test]
    fn ji_upper_extras() {
        for s in [
            "eІа", "ouaІx", "ЕІа", "oІ", "АЕЁ", " ІА", "AІ", "(І", " (І", "ЬІ", " JIŁ -", " JIŁI",
            " JIŁ -x",
        ] {
            check_ji_full(s);
            check_ji_full(&format!("x{s}y"));
        }
        assert_eq!(
            convert_latin_ji_upper(&convert_latin_ji_lower("eІа")),
            "eJia"
        );
        assert_eq!(
            convert_latin_ji_upper(&convert_latin_ji_lower("ЕІа")),
            "IEJIa"
        );
        assert_eq!(
            convert_latin_ji_upper(&convert_latin_ji_lower(" JIŁ -")),
            " IŁ -"
        );
    }

    #[test]
    fn iwords_upper_trie_vs_fancy() {
        // ` І(?=UPPER)` from latinJi/lower, inlined from the JSON.
        static UPPER: &str = r"́|БІС|БСЭН|В[АЕОЫ] |ВЕРС|ВАЛ[ГЗ]|ГАР|ГРЫШЧ|ГРЭК|ДАЛ|ДЫШ|ЖЫЦ|КАНАПІС|КАНЬ?Н|КА[ЦЎЛ]|КАЎ[КЦ]|КЛ(ЫЯ?|А([ЯЕЙ]|ГА|МУ)|УЮ) |КСІ|ЛЕУС|Л(ІСТ| )|ЛІСТАС|ЛЬК|М |МАНТ|МАСЬ?Ц|МБРЫ[КЧ]|МЕННА |МІДЖ|МПАР[ТЦ]|МПУЛЬС[АЕУЫ]|НАХАДЗ|НДЫ([ІЙЮЯ] |ЕВ)|НДЭКС(А(Ў|МІ?)? |[ЕУЫІ])|Н[ЕІ][ЕЙЯЮ]|НК([АІУ])|НТЭРЫМ|НФІКС|НФІМУМ|НШАСЬ?Ц|НШ(А[ЕЙЯ]?|АГА|АМУ|АСЬ?Ц|УЮ|Ы(МІ?|Х|Я)?) |ПСІЛАН|Р([АЫУ]|А[МХЙЎ]|АМІ|) |РАД|РБІС|РМАС|РХА|РЫС |СКАРК|СКАРАК|СКРА|СКРАВЕЦ|СКРАЧК|СТА |С[НТ]АСЬ?Ц|СЬ?ЦІ[НК]|ТА[Р ]|ТРЫ|Х(НЫ[ХЯ]?|НУЮ|НА[ЯЕЙ])?|ЦЬ?ВІН|ШЫЯС";
        let re = fancy_regex::Regex::new(&format!(" І(?={UPPER})")).unwrap();
        // Zero-width lookahead: does ` І` + following match at pos 0?
        let fancy_hit = |s: &str| {
            re.captures(&format!(" І{s}"))
                .ok()
                .flatten()
                .and_then(|c| c.get(0))
                .is_some_and(|m| m.start() == 0 && m.end() == 3)
        };
        let abc = [
            'А', 'Б', 'В', 'І', 'М', 'Н', 'Р', 'С', 'Т', 'Х', ' ', '́', 'а', 'б',
        ];
        let mut seed = 0x9E3779B9u64;
        let mut next = move || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) as usize
        };
        for _ in 0..1500 {
            let n = 1 + next() % 7;
            let s: String = (0..n).map(|_| abc[next() % abc.len()]).collect();
            assert_eq!(matches_iwords_upper(&s), fancy_hit(&s), "fuzz {s:?}");
        }
        // Every generated literal must hit (both bare and extended).
        for line in ["БІС", "Х", "М ", "ВА ", "ТАР", "ХНЫ", "ЦВІН", "ШЫЯС"] {
            assert!(matches_iwords_upper(line), "{line:?}");
            assert!(fancy_hit(line), "oracle {line:?}");
        }
    }

    /// Deterministic fuzz over a mixed soup, all six converters at once.
    #[test]
    fn fuzz_all_converters() {
        let mut abc: Vec<char> = (0x0400u32..0x0460).filter_map(char::from_u32).collect();
        abc.extend(
            [
                'e', 'o', 'u', 'a', 'I', 'i', 'J', 'Ł', ' ', '(', ')', '|', '>', '.', ',', '2',
                '-', '́', 'ʼ', 'č', 'ž', 'š',
            ]
            .iter()
            .copied(),
        );
        let mut seed = 0x12345678u64;
        let mut next = move || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 33) as usize
        };
        for _ in 0..1500 {
            let n = 1 + next() % 8;
            let s: String = (0..n).map(|_| abc[next() % abc.len()]).collect();
            check_lat(&s);
            check_ji(&s);
            check_lat_full(&s);
            check_ji_full(&s);
        }
    }

    #[test]
    fn latin_words_regression() {
        // Lower-only expectations (uppercase survives; restore_case runs later).
        for (input, expected) in [
            ("У яе ёсьць плянэта", "У jaje jość planeta"),
            ("Харошы", "Хarošy"),
            ("верабі", "vierabi"),
            ("грошы", "hrošy"),
            ("Я 77-ы", "Я 77-y"),
        ] {
            assert_eq!(convert_latin_lower(input), expected, "{input:?}");
            check_lat(input);
        }
        for (input, expected) in [
            ("пры інстытуце", "pry jinstytucie"),
            ("поіць", "pojić"),
            ("у ім", "u jim"),
        ] {
            assert_eq!(convert_latin_ji_lower(input), expected, "{input:?}");
            check_ji(input);
        }
    }
}
