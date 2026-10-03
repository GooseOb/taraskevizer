use super::{match_iwords_len, utf8_char_len};

/// Consonant cluster of the `не`/`без` rules: `[бвгджзйклмнпрстфхцчшўьʼ]`.
fn is_zw_cons(ch: char) -> bool {
    matches!(
        ch,
        'б' | 'в'
            | 'г'
            | 'д'
            | 'ж'
            | 'з'
            | 'й'
            | 'к'
            | 'л'
            | 'м'
            | 'н'
            | 'п'
            | 'р'
            | 'с'
            | 'т'
            | 'ф'
            | 'х'
            | 'ц'
            | 'ч'
            | 'ш'
            | 'ў'
            | 'ь'
            | 'ʼ'
    )
}

/// Vowel of the `не`/`без` rules: `[аеёіоуыэюя]`.
fn is_zw_vow(ch: char) -> bool {
    matches!(
        ch,
        'а' | 'е' | 'ё' | 'і' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я'
    )
}

/// Byte length of one cluster consonant at the start of `s`, or 0 when absent.
///
/// `ʼ` (U+02BC) is 2 bytes; every Cyrillic member is 2 bytes.
fn cons_char_len(s: &str) -> usize {
    match s.chars().next() {
        Some(
            'б' | 'в' | 'г' | 'д' | 'ж' | 'з' | 'й' | 'к' | 'л' | 'м' | 'н' | 'п' | 'р' | 'с' | 'т'
            | 'ф' | 'х' | 'ц' | 'ч' | 'ш' | 'ў' | 'ь' | 'ʼ',
        ) => 2,
        _ => 0,
    }
}

/// `[бвгджзйклмнпрстфхцчшўьʼ]*[оё]` prefix length in bytes, if present.
///
/// The star is greedy but deterministic: consonants exclude `о`/`ё`, so the
/// run extends to the first non-cluster char, which must be `о`/`ё` (an
/// empty run matches bare `о`/`ё`, like the regex).
fn cons_o_len(s: &str) -> Option<usize> {
    let mut i = 0;
    while cons_char_len(&s[i..]) > 0 {
        i += cons_char_len(&s[i..]);
    }
    if s[i..].starts_with('о') || s[i..].starts_with('ё') {
        Some(i + 'о'.len_utf8())
    } else {
        None
    }
}

/// `карт\S{0,4}[ \)]` / `мап\S{0,4}[ \)]` match length in bytes, if present.
///
/// `\S{0,4}` is greedy with backtracking over the longest-first `k = 4..=0`
/// whose `k` chars are all non-whitespace and char `k` is a space or `)`.
/// (`\S` is `!char::is_whitespace`, the same class `fancy_regex` uses.)
fn kartmap_len(s: &str) -> Option<usize> {
    let pre = if s.starts_with("карт") {
        "карт"
    } else if s.starts_with("мап") {
        "мап"
    } else {
        return None;
    };
    let rest: Vec<char> = s[pre.len()..].chars().take(5).collect();
    for k in (0..=4).rev() {
        if k >= rest.len() {
            continue;
        }
        if rest[..k].iter().all(|c| !c.is_whitespace()) && (rest[k] == ' ' || rest[k] == ')') {
            let n: usize = rest[..k].iter().map(|c| c.len_utf8()).sum();
            return Some(pre.len() + n + 1);
        }
    }
    None
}

/// First matching literal's byte length, in pattern order.
fn prefix_len(s: &str, pats: &[&str]) -> Option<usize> {
    for p in pats {
        if s.starts_with(p) {
            return Some(p.len());
        }
    }
    None
}

// IA-ne word lists, pattern order, grouped by first char.
// The `i`-group (Latin i + iwords) is handled via `match_iwords_len`.
static NE_D0B1: &[&str] = &[
    "бача",
    "бачу",
    "бачы",
    "бегл",
    "бега",
    "блытаю",
    "блытац",
    "брала",
    "бралі",
    "будзе ",
    "будуць ",
    "будучы ",
    "буду ",
    "болш",
    "болей",
];
static NE_D0B2: &[&str] = &[
    "выбача",
    "выбачу",
    "выбачы",
    "выбегл",
    "выбега",
    "выблытаю",
    "выблытац",
    "выбрала",
    "выбралі",
    "выбудзе ",
    "выбудуць ",
    "выбудучы ",
    "выбуду ",
    "вываш",
    "выведала",
    "выведалі",
    "выведаць",
    "выведаюць",
    "выведаючы",
    "выведаю",
    "выведаў",
    "выдумала",
    "выдумалі",
    "выдумаць",
    "выдумаюць",
    "выдумаючы",
    "выдумаю",
    "выдумаў",
    "выдура",
    "выдуру",
    "выдуры",
    "выдурай",
    "выдурань",
    "выдурняў",
    "выдурням",
    "выдурня",
    "выдур ",
    "вызнае",
    "вызнала",
    "вызналі",
    "вызнаць",
    "вызнаюць",
    "вызнаючы",
    "вызнаю",
    "выклад ",
    "выклал",
    "выклаў",
    "вымае",
    "вымаючы",
    "вымаюць",
    "вымаю",
    "вымел",
    "вымеў",
    "вымець",
    "вымыц",
    "вымыла",
    "вымылі",
    "вынейк",
    "выпісацц",
    "выпісачы",
    "выпісаць ",
    "выпіса",
    "выпішуцц",
    "выпішучы",
    "выпішуць ",
    "выпішу",
    "выўпэўн",
    "выпэўн",
    "выскажа",
    "выскажуць",
    "высказа ",
    "выспала",
    "выспалі",
    "высправ",
    "выспраў",
    "вытрэба ",
    "вычуе",
    "вычул",
    "вычуў",
    "вычуц",
    "вычую",
    "ваш",
    "ведала",
    "ведалі",
    "ведаць",
    "ведаюць",
    "ведаючы",
    "ведаю",
    "ведаў",
    "веды",
    "веліч",
    "выраш",
    "выгар",
    "выпад",
    "выдале",
    "выдалі",
    "выдаля",
    "выдалю",
    "выпале",
    "выпалі",
    "выпаля",
    "выпалю",
];
static NE_D0B3: &[&str] = &["горш", "горай", "гук", "гучн", "густ"];
static NE_D0B4: &[&str] = &[
    "думала",
    "думалі",
    "думаць",
    "думаюць",
    "думаючы",
    "думаю",
    "думаў",
    "дура",
    "дуру",
    "дуры",
    "дурай",
    "дурань",
    "дурняў",
    "дурням",
    "дурня",
    "дур ",
    "дрэнн",
];
static NE_D0B7: &[&str] = &[
    "знае",
    "знала",
    "зналі",
    "знаць",
    "знаюць",
    "знаючы",
    "знаю",
    "зьбег",
];
static NE_D0BA: &[&str] = &[
    "клад ",
    "клал",
    "клаў",
    "кашай ",
    "каша ",
    "кашамі ",
    "кашу ",
    "кашы ",
];
static NE_D0BB: &[&str] = &["лёгк", "літар", "лішн", "лепш", "лепей"];
static NE_D0BC: &[&str] = &[
    "мае",
    "маючы",
    "маюць",
    "маю",
    "мел",
    "меў",
    "мець",
    "мыц",
    "мыла",
    "мылі",
    "менш",
    "меней",
    "медз",
    "мякк",
];
static NE_D0BD: &[&str] = &["нейк", "нашая", "нашыя", "нашую", "нашых"];
static NE_D0BF: &[&str] = &[
    "пісацц",
    "пісачы",
    "пісаць ",
    "піса",
    "пішуцц",
    "пішучы",
    "пішуць ",
    "пішу",
    "пэўн",
    "прыйдзеш",
];
static NE_D180: &[&str] = &["руша", "рушы"];
static NE_D181: &[&str] = &[
    "скажа",
    "скажуць",
    "сказа ",
    "спала",
    "спалі",
    "справ",
    "спраў",
    "стаў",
    "стане",
    "стануць",
    "стала",
    "сталі",
];
static NE_D182: &[&str] = &["трэба ", "таго", "тое", "тыя"];
static NE_D184: &[&str] = &["фарбаў ", "фарбамі ", "фарба ", "фарбу ", "фарбы ", "фарб "];
static NE_D187: &[&str] = &["чуе", "чул", "чуў", "чуц", "чую"];
static NE_D18F: &[&str] = &["ян "];
static NE_D196: &[&str] = &["іхн"];
static NE_D19E: &[&str] = &["ўпэўн", "ўвод", "ўсе", "ўсё", "ўся"];

// IA-bez word lists, pattern order, grouped by first char.
// The `i`-group (Latin i + iwords) is handled via `match_iwords_len`.
static BEZ_D0B1: &[&str] = &["бол"];
static BEZ_D0B2: &[&str] = &[
    "выклада",
    "выкладу",
    "вымела ",
    "вымытых",
    "выпісан",
    "выпісац",
    "выскажа ",
    "выскажуць ",
    "высказа ",
    "выспала",
    "выспалі",
    "вычуе",
    "вычул",
    "вычуў",
    "вычуц",
    "вычую",
    "ваш",
    "ведаў",
    "велічы",
    "выраш",
    "выдале",
    "выпале",
    "выпад",
];
static BEZ_D0B3: &[&str] = &["горш", "горай", "гук", "гучн", "густ"];
static BEZ_D0B4: &[&str] = &["дрэнн"];
static BEZ_D0B5: &[&str] = &["ей"];
static BEZ_D0BA: &[&str] = &["клада", "кладу", "кашы ", "кашаў "];
static BEZ_D0BB: &[&str] = &["лёгк", "літар", "лішн", "леп"];
static BEZ_D0BC: &[&str] = &["мела ", "мытых", "медз", "мен", "мякк"];
static BEZ_D0BD: &[&str] = &["нейк", "нашую", "нашых"];
static BEZ_D0BF: &[&str] = &["пісан", "пісац", "пішучы"];
static BEZ_D180: &[&str] = &["руша"];
static BEZ_D181: &[&str] = &["скажа ", "скажуць ", "сказа ", "спала", "спалі", "стаў"];
static BEZ_D182: &[&str] = &["таго"];
static BEZ_D184: &[&str] = &["фарбаў ", "фарбы ", "фарб "];
static BEZ_D186: &[&str] = &["цукра ", "цукру "];
static BEZ_D187: &[&str] = &["чуе", "чул", "чуў", "чуц", "чую"];
static BEZ_D188: &[&str] = &["ш"];
static BEZ_D18F: &[&str] = &["ян"];
static BEZ_D196: &[&str] = &["іхн"];
static BEZ_D19E: &[&str] = &["ўвод", "ўсе", "ўсё", "ўся"];

// ================= alt_len functions ================
/// `IA_WORDS` не-entry alternation match length.
///
/// Pattern order: `[cons]*[оё]` first, then words (the `i`-branch
/// last — disjoint from the rest, so dispatch position is free).
fn ia_ne_alt_len(s: &str) -> Option<usize> {
    if let Some(n) = cons_o_len(s) {
        return Some(n);
    }
    match s.chars().next() {
        Some('б') => prefix_len(s, NE_D0B1),
        Some('в') => prefix_len(s, NE_D0B2),
        Some('г') => prefix_len(s, NE_D0B3),
        Some('д') => prefix_len(s, NE_D0B4),
        Some('з') => prefix_len(s, NE_D0B7),
        Some('к') => {
            // Pattern-ordered before literals, but disjoint from them
            // (3rd byte т/п vs ш/л/н/с/б/д/е/я), so position is free.
            if let Some(n) = kartmap_len(s) {
                return Some(n);
            }
            prefix_len(s, NE_D0BA)
        }
        Some('л') => prefix_len(s, NE_D0BB),
        Some('м') => {
            // Pattern-ordered before literals, but disjoint from them
            // (3rd byte т/п vs ш/л/н/с/б/д/е/я), so position is free.
            if let Some(n) = kartmap_len(s) {
                return Some(n);
            }
            prefix_len(s, NE_D0BC)
        }
        Some('н') => prefix_len(s, NE_D0BD),
        Some('п') => prefix_len(s, NE_D0BF),
        Some('р') => prefix_len(s, NE_D180),
        Some('с') => prefix_len(s, NE_D181),
        Some('т') => prefix_len(s, NE_D182),
        Some('ф') => prefix_len(s, NE_D184),
        Some('ч') => prefix_len(s, NE_D187),
        Some('я') => prefix_len(s, NE_D18F),
        Some('і') => prefix_len(s, NE_D196),
        Some('ў') => prefix_len(s, NE_D19E),
        // Latin `i` + iwords (last branch of the pattern).
        Some('i') => {
            if let Some(n) = match_iwords_len(&s[1..]) {
                return Some(1 + n);
            }
            None
        }
        _ => None,
    }
}

/// `IA_WORDS` без-entry alternation match length.
///
/// Pattern order: `[cons]*[оё]` first, then words (the `i`-branch
/// last — disjoint from the rest, so dispatch position is free).
fn ia_bez_alt_len(s: &str) -> Option<usize> {
    if let Some(n) = cons_o_len(s) {
        return Some(n);
    }
    match s.chars().next() {
        Some('б') => prefix_len(s, BEZ_D0B1),
        Some('в') => prefix_len(s, BEZ_D0B2),
        Some('г') => prefix_len(s, BEZ_D0B3),
        Some('д') => prefix_len(s, BEZ_D0B4),
        Some('е') => prefix_len(s, BEZ_D0B5),
        Some('к') => {
            // Pattern-ordered before literals, but disjoint from them
            // (3rd byte т/п vs ш/л/н/с/б/д/е/я), so position is free.
            if let Some(n) = kartmap_len(s) {
                return Some(n);
            }
            prefix_len(s, BEZ_D0BA)
        }
        Some('л') => prefix_len(s, BEZ_D0BB),
        Some('м') => {
            // Pattern-ordered before literals, but disjoint from them
            // (3rd byte т/п vs ш/л/н/с/б/д/е/я), so position is free.
            if let Some(n) = kartmap_len(s) {
                return Some(n);
            }
            prefix_len(s, BEZ_D0BC)
        }
        Some('н') => prefix_len(s, BEZ_D0BD),
        Some('п') => prefix_len(s, BEZ_D0BF),
        Some('р') => prefix_len(s, BEZ_D180),
        Some('с') => prefix_len(s, BEZ_D181),
        Some('т') => prefix_len(s, BEZ_D182),
        Some('ф') => prefix_len(s, BEZ_D184),
        Some('ц') => prefix_len(s, BEZ_D186),
        Some('ч') => prefix_len(s, BEZ_D187),
        Some('ш') => prefix_len(s, BEZ_D188),
        Some('я') => prefix_len(s, BEZ_D18F),
        Some('і') => prefix_len(s, BEZ_D196),
        Some('ў') => prefix_len(s, BEZ_D19E),
        // Latin `i` + iwords (last branch of the pattern).
        Some('i') => {
            if let Some(n) = match_iwords_len(&s[1..]) {
                return Some(1 + n);
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
const ALL_NE_GROUPS: &[&[&str]] = &[
    NE_D0B1, NE_D0B2, NE_D0B3, NE_D0B4, NE_D0B7, NE_D0BA, NE_D0BB, NE_D0BC, NE_D0BD, NE_D0BF,
    NE_D180, NE_D181, NE_D182, NE_D184, NE_D187, NE_D18F, NE_D196, NE_D19E,
];
#[cfg(test)]
const ALL_BEZ_GROUPS: &[&[&str]] = &[
    BEZ_D0B1, BEZ_D0B2, BEZ_D0B3, BEZ_D0B4, BEZ_D0B5, BEZ_D0BA, BEZ_D0BB, BEZ_D0BC, BEZ_D0BD,
    BEZ_D0BF, BEZ_D180, BEZ_D181, BEZ_D182, BEZ_D184, BEZ_D186, BEZ_D187, BEZ_D188, BEZ_D18F,
    BEZ_D196, BEZ_D19E,
];

/// ` ...[ая]ў | ну␣` lookahead of the `б[ея]з` / `(пра|цера)?з` rules.
///
/// ` і\S*[ая]ў`: after ` і`, some non-whitespace run containing `[ая]ў`
/// (greedy `\S*` backtracks, so any position works); `ну ` is literal.
fn has_nu_imlau(s: &str) -> bool {
    if s.starts_with("ну ") {
        return true;
    }
    if !s.starts_with(" і") {
        return false;
    }
    // After ` і` (space + 2-byte `і`): scan a non-whitespace run for `[ая]ў`.
    let mut i = 1 + 'і'.len_utf8();
    while i < s.len() {
        let ch = s[i..].chars().next().unwrap_or('\0');
        if ch.is_whitespace() {
            break;
        }
        if ch == 'а' || ch == 'я' {
            let j = i + ch.len_utf8();
            if s[j..].starts_with('ў') {
                return true;
            }
        }
        i += ch.len_utf8();
    }
    false
}

/// `[бвгджзйклмнпрстфхцчшўьʼ]*.\u{301}` lookahead: a stress mark preceded by
/// one arbitrary (`\n`-less) char after a consonant run.
///
/// The regex backtracks the run, so this holds iff an acute sits at
/// char-index `m` with `1 <= m <= run+1` and a non-`\n` char before it.
///
/// Streaming: only chars `0..=run+1` are ever inspected, so this runs in
/// O(run) time with O(1) memory. (The old version `collect()`ed the whole
/// remainder into a `Vec<char>` per call — O(chunk²) on `не`-heavy text.)
fn has_acute(s: &str) -> bool {
    let mut run = 0usize;
    let mut open = true;
    let mut prev = '\0';
    for (idx, ch) in s.chars().enumerate() {
        if idx >= 1 {
            // While the run is still open, `run == idx`, so the check
            // below always applies; once closed, `run` is final.
            let r = if open { idx } else { run };
            if idx > r + 1 {
                break;
            }
            if ch == '́' && prev != '\n' {
                return true;
            }
        }
        if open {
            if is_zw_cons(ch) {
                run += 1;
            } else {
                open = false;
            }
        }
        prev = ch;
    }
    false
}

/// `[бвгджзйклмнпрстфхцчшўьʼ]*[аеёіоуыэюя][бвгджзйклмнпрстфхцчшўьʼ]*␣`
/// lookahead: a phonetic word (consonants, vowel, consonants, space).
///
/// Deterministic left-to-right: the sets are disjoint, so no backtracking
/// can ever succeed where the greedy run fails.
fn has_phonetic_word(s: &str) -> bool {
    let mut it = s.chars();
    let mut c = it.next();
    while matches!(c, Some(ch) if is_zw_cons(ch)) {
        c = it.next();
    }
    if !matches!(c, Some(ch) if is_zw_vow(ch)) {
        return false;
    }
    c = it.next();
    while matches!(c, Some(ch) if is_zw_cons(ch)) {
        c = it.next();
    }
    matches!(c, Some(' '))
}

/// Preposition blacklist of the generic `не → ня` rule.
///
/// `(?!а[бд]? |б[ея]зь? |[дз]а |д?ля |дзеля |[нп]ад? |пр[аы] |празь? |у `
/// `|церазь? )`, expanded to literals.
fn is_prep(s: &str) -> bool {
    const PREPS: &[&str] = &[
        "а ",
        "аб ",
        "ад ",
        "без ",
        "безь ",
        "бяз ",
        "бязь ",
        "да ",
        "за ",
        "ля ",
        "для ",
        "дзеля ",
        "на ",
        "над ",
        "па ",
        "пад ",
        "пра ",
        "пры ",
        "праз ",
        "празь ",
        "у ",
        "цераз ",
        "церазь ",
    ];
    PREPS.iter().any(|p| s.starts_with(p))
}

/// One `IA_WORDS` entry as a manual pass: ` не`/` без` + ` \(?ALT` capture.
///
/// Equivalent to the dict entry (` ня$1` / ` бяз$1`): the head becomes
/// `repl`, the captured ` ␣[(]?ALT` is copied verbatim, and scanning
/// resumes after it (consuming semantics — a later anchor inside the
/// capture is skipped by this pass, like `replace_all`).
///
/// `IA_WORDS` without the regex engine or the JSON file: `не`-entry, then
/// `без`-entry, like the compiled dict order.
///
/// Both entries run in ONE scan: the anchors (` не` vs ` без`) are
/// disjoint (2nd byte `н` vs `б`), replacements create no new anchors
/// (` ня`/` бяз` contain neither head), and a match consumes its whole
/// capture in both orders — so merged == sequential, minus one full
/// scan plus the intermediate `String`.
///
/// At a `не`-before-`без` position the entry order is preserved anyway.
///
/// One head entry: (anchor, replacement, tail-length probe).
type HeadRule<'a> = (&'a str, &'a str, fn(&str) -> Option<usize>);

pub(crate) fn ia_words(text: &str) -> String {
    const HEADS: &[HeadRule<'_>] = &[
        (" не", " ня", ia_ne_alt_len),
        (" без", " бяз", ia_bez_alt_len),
    ];
    if !HEADS.iter().any(|(head, _, _)| text.contains(head)) {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if bytes[i] == b' ' {
            let mut done = false;
            for (head, repl, alt_len) in HEADS {
                if !text[i..].starts_with(head) {
                    continue;
                }
                // Disjoint 2nd byte: the other head cannot match at `i`,
                // so a failed tail means no match here at all.
                let after = i + head.len();
                if after < len && bytes[after] == b' ' {
                    let mut p = after + 1;
                    if p < len && bytes[p] == b'(' {
                        p += 1;
                    }
                    if let Some(n) = alt_len(&text[p..]) {
                        let end = p + n;
                        out.push_str(&text[flush..i]);
                        out.push_str(repl);
                        out.push_str(&text[after..end]);
                        flush = end;
                        i = end;
                        done = true;
                    }
                }
                break;
            }
            if done {
                continue;
            }
            i += 1;
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

/// The four generic rules merged into one scan over spaces.
///
/// Anchors are disjoint (`не␣`, `без`/`бяз`, `(пра|цера)?з`), and at a
/// ` без` position the original order (`без → бяз`, then `б[ея]з → бязь`)
/// is chained: after `без → бяз`, the same spot is re-tested for `бязь`.
/// Every other anchor maps to exactly one rule, so one pass equals the four
/// sequential `replace_all` calls.
fn explicit_pass(text: &str) -> String {
    if !text.contains(' ') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush = 0usize;
    let mut i = 0usize;
    while i < len {
        if bytes[i] != b' ' {
            i += if bytes[i] < 0x80 {
                1
            } else {
                utf8_char_len(bytes[i])
            };
            continue;
        }
        // ` не ` → ` ня ` with phonetic lookahead + preposition guard.
        // ` не ` is space(1) + `н`(2) + `е`(2) + space(1) = 6 bytes.
        if text[i..].starts_with(" не ") {
            let after = i + " не ".len();
            if (has_acute(&text[after..]) || has_phonetic_word(&text[after..]))
                && !is_prep(&text[after..])
            {
                out.push_str(&text[flush..i]);
                out.push_str(" ня ");
                flush = after;
                i = after;
                continue;
            }
        }
        // ` без` / ` бяз` (+ optional `ь` for `без`).
        // Both heads are space(1) + 3×2 bytes = 7 bytes.
        if text[i..].starts_with(" без") || text[i..].starts_with(" бяз") {
            let is_bez = text[i..].starts_with(" без");
            let after = i + " без".len();
            if is_bez {
                // ` без(?=ь? (?:acute|phonetic))` → ` бяз`, then chain `бязь`.
                let mut p = after;
                if text[p..].starts_with('ь') {
                    p += 'ь'.len_utf8();
                }
                if text[p..].starts_with(' ')
                    && (has_acute(&text[p + 1..]) || has_phonetic_word(&text[p + 1..]))
                {
                    out.push_str(&text[flush..i]);
                    out.push_str(" бяз");
                    flush = after;
                    if has_nu_imlau(&text[after..]) {
                        out.push('ь');
                    }
                    i = after;
                    continue;
                }
            }
            // ` б[ея]з(?= і\S*[ая]ў|ну )` → ` бязь`.
            if has_nu_imlau(&text[after..]) {
                out.push_str(&text[flush..i]);
                out.push_str(" бязь");
                flush = after;
                i = after;
                continue;
            }
        }
        // ` (?:пра|цера)?з(?= і\S*[ая]ў|ну )` → `$0ь`.
        let zlen = if text[i..].starts_with(" цераз") {
            " цераз".len() // 11
        } else if text[i..].starts_with(" праз") {
            " праз".len() // 9
        } else if text[i..].starts_with(" з") {
            " з".len() // 3
        } else {
            0
        };
        if zlen > 0 && has_nu_imlau(&text[i + zlen..]) {
            out.push_str(&text[flush..i + zlen]);
            out.push('ь');
            flush = i + zlen;
            i += zlen;
            continue;
        }
        i += 1;
    }
    out.push_str(&text[flush..]);
    out
}

/// `end_z_soften_and_nia_biaz` without the regex engine or `iawords.json`.
///
/// `IA_WORDS` entries first (word-list `не → ня`, `без → бяз`), then the
/// four generic rules — the same order as the original sequential
/// `replace_all` calls.
pub(crate) fn end_z_soften_and_nia_biaz(text: &str) -> String {
    explicit_pass(&ia_words(text))
}

#[cfg(test)]
mod tests {
    use super::{
        cons_o_len, end_z_soften_and_nia_biaz, has_acute, has_nu_imlau, has_phonetic_word,
        ia_bez_alt_len, ia_ne_alt_len, is_prep, kartmap_len, ALL_BEZ_GROUPS, ALL_NE_GROUPS,
    };

    // Oracle: original impl (fancy IA dict + 4 explicit regexes),
    // inlined so tests survive `iawords.json` removal.
    fn fancy_end_z(text: &str) -> String {
        use super::super::fancy_test::{fancy_replace_all, FancyDict};
        static P0: &str = r" не( \(?(?:[бвгджзйклмнпрстфхцчшўьʼ]*[оё]|(?:вы)?(?:бач[ауы]|бег[ла]|блыта[юц]|брал[аі]|буд(?:зе|у(?:ць|чы)?) |ваш|веда(?:л[аі]|ць|ю(?:ць|чы)?|ў)|дума(?:л[аі]|ць|ю(?:ць|чы)?|ў)|дур(?:[ауы]|ай|ань|ня[ўм]?| )|зна(?:е|л[аі]|ць|ю(?:ць|чы)?)|кла(?:д |л|ў)|ма(?:е|ючы|ю(?:ць|чы)?)|ме(?:л|ў|ць)|мы(?:ц|л[аі])|нейк|пі(?:са|шу)(?:цц|чы|ць )?|ў?пэўн|ска(?:ж(?:а|уць)|за )|спал[аі]|спра[вў]|трэба |чу[елўцю])|веды|веліч|вы(?:раш|гар|пад|[дп]ал[еіяю])|гор(?:ш|ай)|гу(?:к|чн|ст)|дрэнн|зьбег|іхн|карт\S{0,4}[ \)]|каш(?:ай?|амі|у|ы) |лёгк|літар|лішн|(?:мен|бол|леп)(?:ш|ей)|мап\S{0,4}[ \)]|медз|мякк|наш(?:[аы]я|ую|ых)|прыйдзеш|руш[аы]|ста(?:ў|не|нуць|л[аі])|таго|тое|тыя|ўвод|ўс[еёя]|фарб(?:а(?:ў|мі)?|у|ы)? |ян |i(?:́|біс|бсэн|в[аеоы] |верс|вал[гз]|гар|грышч|грэк|дал|дыш|жыц|канапіс|кань?н|ка[цўл]|каў[кц]|кл(ыя?|а([яей]|га|му)|ую) |ксі|леус|л(іст| )|лістас|льк|м |мант|мась?ц|мбры[кч]|менна |мідж|мпар[тц]|мпульс[аеуы]|нахадз|нды([ійюя] |ев)|ндэкс(а(ў|мі?)? |[еуыі])|н[еі][ейяю]|нк([аіу])|нтэрым|нфікс|нфімум|ншась?ц|нш(а[ейя]?|ага|аму|ась?ц|ую|ы(мі?|х|я)?) |псілан|р([аыу]|а[мхйў]|амі|) |рад|рбіс|рмас|рха|рыс |скарк|скарак|скра|скравец|скрачк|ста |с[нт]ась?ц|сь?ці[нк]|та[р ]|тры|х(ны[хя]?|ную|на[яей])?|ць?він|шыяс)))";
        static P1: &str = r" без( \(?(?:[бвгджзйклмнпрстфхцчшўьʼ]*[оё]|(?:вы)?(?:клад[ау]|мела |мытых|піса[нц]|ска(?:жа|жуць|за) |спал[аі]|чу[елўцю])|ваш|ведаў|велічы|выраш|вы[дп]але|выпад|гор(?:ш|ай)|гу(?:к|чн|ст)|дрэнн|іхн|карт\S{0,4}[ \)]|каш(?:ы|аў) |лёгк|літар|лішн|медз|(?:мен|бол|леп|ш|ей)|мап\S{0,4}[ \)]|мякк|нейк|наш(?:ую|ых)|пішучы|руша|стаў|таго|ўвод|ўс[еёя]|фарб(?:аў|ы)? |цукр[ау] |ян|i(?:́|біс|бсэн|в[аеоы] |верс|вал[гз]|гар|грышч|грэк|дал|дыш|жыц|канапіс|кань?н|ка[цўл]|каў[кц]|кл(ыя?|а([яей]|га|му)|ую) |ксі|леус|л(іст| )|лістас|льк|м |мант|мась?ц|мбры[кч]|менна |мідж|мпар[тц]|мпульс[аеуы]|нахадз|нды([ійюя] |ев)|ндэкс(а(ў|мі?)? |[еуыі])|н[еі][ейяю]|нк([аіу])|нтэрым|нфікс|нфімум|ншась?ц|нш(а[ейя]?|ага|аму|ась?ц|ую|ы(мі?|х|я)?) |псілан|р([аыу]|а[мхйў]|амі|) |рад|рбіс|рмас|рха|рыс |скарк|скарак|скра|скравец|скрачк|ста |с[нт]ась?ц|сь?ці[нк]|та[р ]|тры|х(ны[хя]?|ную|на[яей])?|ць?він|шыяс)))";
        let dict = FancyDict::new(&[(P0, " ня$1"), (P1, " бяз$1")]);
        let mut t = dict.replace_all(text);
        for (pat, rep) in [
            (
                r" не (?=[бвгджзйклмнпрстфхцчшўьʼ]*.\u{301}|[бвгджзйклмнпрстфхцчшўьʼ]*[аеёіоуыэюя][бвгджзйклмнпрстфхцчшўьʼ]* )(?!а[бд]? |б[ея]зь? |[дз]а |д?ля |дзеля |[нп]ад? |пр[аы] |празь? |у |церазь? )",
                " ня ",
            ),
            (
                r" без(?=ь? (?:[бвгджзйклмнпрстфхцчшўьʼ]*.\u{301}|[бвгджзйклмнпрстфхцчшўьʼ]*[аеёіоуыэюя][бвгджзйклмнпрстфхцчшўьʼ]* ))",
                " бяз",
            ),
            (r" б[ея]з(?= і\S*[ая]ў|ну )", " бязь"),
            (r" (?:пра|цера)?з(?= і\S*[ая]ў|ну )", "$0ь"),
        ] {
            let re = fancy_regex::Regex::new(pat).unwrap();
            t = fancy_replace_all(&re, &t, rep);
        }
        t
    }

    fn check(input: &str) {
        assert_eq!(
            end_z_soften_and_nia_biaz(input),
            fancy_end_z(input),
            "input: {input:?}"
        );
    }

    #[test]
    fn all_ia_ne_literals() {
        for group in ALL_NE_GROUPS {
            for w in *group {
                check(&format!("не {w}"));
                check(&format!("не ({w}"));
                check(&format!("x не {w} y"));
            }
        }
    }

    #[test]
    fn all_ia_bez_literals() {
        for group in ALL_BEZ_GROUPS {
            for w in *group {
                check(&format!("без {w}"));
                check(&format!("без ({w}"));
                check(&format!("x без {w} y"));
            }
        }
    }

    #[test]
    fn ia_branch1_stressed_o() {
        // `[cons]*[оё]`: consonant run (possibly empty) + о/ё.
        for w in [
            "во",
            "го",
            "до",
            "о",
            "ё",
            "сон",
            "мост",
            "восень",
            "(во",
            "зло",
        ] {
            check(&format!("не {w}"));
            check(&format!("без {w}"));
        }
        // Not о/ё after the run: no branch-1 match (words may still match).
        for w in ["ва", "ве", "на", "ма", "ба"] {
            check(&format!("не {w}"));
            check(&format!("без {w}"));
        }
    }

    #[test]
    fn ia_kart_map_specials() {
        for w in [
            "карта ",
            "карт ",
            "карт)",
            "карт12 ",
            "карт1234 ",
            "мапа ",
            "мап)",
            "мап12 ",
            "мап1234 ",
        ] {
            check(&format!("не {w}x"));
            check(&format!("без {w}x"));
        }
        // 5+ non-spaces: `\S{0,4}` can't reach the delimiter.
        for w in ["карт12345 ", "мап12345 ", "карт12)"] {
            check(&format!("не {w}x"));
            check(&format!("без {w}x"));
        }
        // `карт1234 ` SHOULD match (4 \S + space).
        check("не карт1234 x");
    }

    #[test]
    fn ia_latin_i_branch() {
        // Latin `i` + iwords (note: Cyrillic `і` does NOT take this branch).
        for w in ["iбіс", "iх", "iва ", "iм ", "iр ", "iхны", "i\u{301}"] {
            check(&format!("не {w}"));
            check(&format!("без {w}"));
        }
        for w in ["iб", "ix", "iмама", "ікус"] {
            check(&format!("не {w}"));
            check(&format!("без {w}"));
        }
    }

    #[test]
    fn ia_no_match_basics() {
        for input in [
            "",
            "не",
            "без",
            "не ",
            "без ",
            "небо",
            "безь",
            " ане ",
            "не а",
            "не ма",
            "без ма",
            "не  веды",
        ] {
            check(input);
        }
    }

    #[test]
    fn explicit_ne_nia() {
        for input in [
            "не маю часу",
            "не ма",
            "не мост",
            "не ста́ну",
            "не ста\u{301}ну",
            "не веды",
            "не буду",
            "не буду ",
        ] {
            check(input);
        }
    }

    #[test]
    fn explicit_prep_blacklist() {
        // Prepositions block the generic `не → ня`.
        for prep in [
            "а",
            "аб",
            "ад",
            "без",
            "безь",
            "бяз",
            "бязь",
            "да",
            "за",
            "ля",
            "для",
            "дзеля",
            "на",
            "над",
            "па",
            "пад",
            "пра",
            "пры",
            "праз",
            "празь",
            "у",
            "цераз",
            "церазь",
        ] {
            check(&format!("не {prep} x"));
        }
        // ...but IA words still fire through the blacklist.
        check("не веды");
        check("не буду ");
    }

    #[test]
    fn explicit_bez_biaz() {
        for input in [
            "без вокнаў",
            "без ма",
            "без мост",
            "без ведаў",
            "безь ма ",
            "безь ведаў",
            "без клубных",
            "без  ма",
        ] {
            check(input);
        }
    }

    #[test]
    fn bez_imlau_and_nu() {
        for input in [
            "без імглаў",
            "без імяў",
            "без іаў",
            "без іх",
            "без ну ",
            "без ну",
            "бяз імглаў",
            "з імглаў",
            "з імяў",
            "з ну ",
            "праз імглаў",
            "праз ну ",
            "цераз ну ",
            "цераз імяў",
            "праз іх",
            "з іх",
            "праца",
            "праз",
            "без",
        ] {
            check(input);
        }
    }

    #[test]
    fn consumption_order() {
        // Trailing-space captures are consumed: the inner anchor is skipped
        // by the same pass (later passes still see it — like the original).
        for input in [
            "не буду не ма",
            "не буду без ведаў",
            "без ведаў не ма",
            "не веды не веды",
            "не буду не буду не ма",
            "без мела без ведаў",
            "не трэба не ма",
            "не кашамі не ма",
        ] {
            check(input);
        }
    }

    #[test]
    fn phonetic_regression() {
        // Cases from the integration suite's phonetic pipeline.
        for input in [
            "я і смяяўся",
            "не маю часу",
            "без вокнаў",
            "без імглаў",
            "праз імглаў",
            "бязь імглаў",
            "не дурань",
            "не справімся",
            "не выгарыць",
            "не збегчы",
            "не стану",
            "без клубных",
            "не ста\u{301}ну",
            "без клу\u{301}бных",
        ] {
            check(input);
        }
    }

    #[test]
    fn helpers() {
        assert_eq!(cons_o_len("во"), Some(4));
        assert_eq!(cons_o_len("о"), Some(2));
        assert_eq!(cons_o_len("ё"), Some(2));
        assert_eq!(cons_o_len("ва"), None);
        assert_eq!(cons_o_len("в"), None);
        assert_eq!(cons_o_len(""), None);
        assert_eq!(cons_o_len("сон"), Some(4));
        assert_eq!(kartmap_len("карта "), Some(11));
        assert_eq!(kartmap_len("карт)"), Some(9));
        assert_eq!(kartmap_len("карт1234 "), Some(13));
        assert_eq!(kartmap_len("карт12345 "), None);
        assert_eq!(kartmap_len("карт"), None);
        assert_eq!(kartmap_len("мап)"), Some(7));
        assert_eq!(kartmap_len("мапа"), None);
        assert!(has_acute("ма\u{301}"));
        assert!(has_acute("вма\u{301}x"));
        assert!(!has_acute("ма"));
        assert!(!has_acute(""));
        assert!(!has_acute("́"));
        assert!(has_phonetic_word("ма "));
        assert!(has_phonetic_word("мост "));
        assert!(has_phonetic_word("а "));
        assert!(!has_phonetic_word("ма"));
        assert!(!has_phonetic_word("ведаў "));
        assert!(!has_phonetic_word(""));
        assert!(is_prep("а x"));
        assert!(is_prep("безь x"));
        assert!(is_prep("церазь x"));
        assert!(!is_prep("ма x"));
        assert!(!is_prep(""));
        assert!(has_nu_imlau("ну "));
        assert!(has_nu_imlau(" імяў"));
        assert!(has_nu_imlau(" іаў"));
        assert!(!has_nu_imlau("ну"));
        assert!(!has_nu_imlau(" іх"));
        assert!(!has_nu_imlau(""));
        assert_eq!(ia_ne_alt_len("веды"), Some(8));
        assert_eq!(ia_ne_alt_len("во"), Some(4));
        assert_eq!(ia_ne_alt_len("(во"), None);
        assert_eq!(ia_ne_alt_len("ма"), None);
        assert_eq!(ia_bez_alt_len("ведаў"), Some(10));
        assert_eq!(ia_bez_alt_len("ш"), Some(2));
        assert_eq!(ia_bez_alt_len("ма"), None);
    }
}
