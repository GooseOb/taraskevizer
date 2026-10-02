use super::utf8_char_len;

/// Whether `c` is in `[аеёіоуыэюя\u0301]`.
fn is_vowel_ji(c: char) -> bool {
    matches!(
        c,
        'а' | 'е' | 'ё' | 'і' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я' | '́'
    )
}

/// Whether `s` (starting right after `і`) matches the `iwords` alternation.
///
/// Equivalent to the lookahead in `/ і(?=iwords)/`, anchored at the start:
/// prefix match, no word boundary. Dispatched on the first char so the
/// common case checks only a handful of `starts_with` (memcmp) probes.
///
/// The literals below are the exact expansion of the original pattern:
/// `\u{301}|біс|бсэн|в[аеоы] |верс|вал[гз]|гар|грышч|грэк|дал|дыш|жыц|`
/// `канапіс|кань?н|ка[цўл]|каў[кц]|кл(ыя?|а([яей]|га|му)|ую) |ксі|леус|`
/// `л(іст| )|лістас|льк|м |мант|мась?ц|мбры[кч]|менна |мідж|мпар[тц]|`
/// `мпульс[аеуы]|нахадз|нды([ійюя] |ев)|ндэкс(а(ў|мі?)? |[еуыі])|`
/// `н[еі][ейяю]|нк([аіу])|нтэрым|нфікс|нфімум|ншась?ц|`
/// `нш(а[ейя]?|ага|аму|ась?ц|ую|ы(мі?|х|я)?) |псілан|`
/// `р([аыу]|а[мхйў]|амі|) |рад|рбіс|рмас|рха|рыс |скарк|скарак|скра|`
/// `скравец|скрачк|ста |с[нт]ась?ц|сь?ці[нк]|та[р ]|тры|`
/// `х(ны[хя]?|ную|на[яей])?|ць?він|шыяс`
pub(crate) fn matches_iwords(s: &str) -> bool {
    // `́` (U+0301) alone matches.
    if s.starts_with('́') {
        return true;
    }
    match s.chars().next() {
        // біс | бсэн
        Some('б') => s.starts_with("біс") || s.starts_with("бсэн"),
        // в[аеоы]␣ | верс | вал[гз]
        Some('в') => {
            s.starts_with("ва ")
                || s.starts_with("ве ")
                || s.starts_with("во ")
                || s.starts_with("вы ")
                || s.starts_with("верс")
                || s.starts_with("валг")
                || s.starts_with("валз")
        }
        // гар | грышч | грэк
        Some('г') => s.starts_with("гар") || s.starts_with("грышч") || s.starts_with("грэк"),
        // дал | дыш
        Some('д') => s.starts_with("дал") || s.starts_with("дыш"),
        // жыц
        Some('ж') => s.starts_with("жыц"),
        // канапіс | кань?н | ка[цўл] | каў[кц] | кл(… )␣ | ксі
        Some('к') => {
            s.starts_with("канапіс")
                || s.starts_with("канн")
                || s.starts_with("каньн")
                || s.starts_with("кац")
                || s.starts_with("каў")
                || s.starts_with("кал")
                || s.starts_with("каўк")
                || s.starts_with("каўц")
                || s.starts_with("клы ")
                || s.starts_with("клыя ")
                || s.starts_with("клая ")
                || s.starts_with("клае ")
                || s.starts_with("клай ")
                || s.starts_with("клага ")
                || s.starts_with("кламу ")
                || s.starts_with("клую ")
                || s.starts_with("ксі")
        }
        // леус | л(іст|␣) | лістас | льк
        Some('л') => {
            s.starts_with("леус")
                || s.starts_with("ліст")
                || s.starts_with("л ")
                || s.starts_with("лістас")
                || s.starts_with("льк")
        }
        // м␣ | мант | мась?ц | мбры[кч] | менна␣ | мідж | мпар[тц] | мпульс[аеуы]
        Some('м') => {
            s.starts_with("м ")
                || s.starts_with("мант")
                || s.starts_with("масц")
                || s.starts_with("масьц")
                || s.starts_with("мбрык")
                || s.starts_with("мбрыч")
                || s.starts_with("менна ")
                || s.starts_with("мідж")
                || s.starts_with("мпарт")
                || s.starts_with("мпарц")
                || s.starts_with("мпульса")
                || s.starts_with("мпульсе")
                || s.starts_with("мпульсу")
                || s.starts_with("мпульсы")
        }
        // нахадз | нды(…) | ндэкс(…) | н[еі][ейяю] | нк(…) | нтэрым |
        // нфікс | нфімум | ншась?ц | нш(… )␣
        Some('н') => {
            s.starts_with("нахадз")
                || s.starts_with("ндыі ")
                || s.starts_with("ндый ")
                || s.starts_with("ндыю ")
                || s.starts_with("ндыя ")
                || s.starts_with("ндыев")
                || s.starts_with("ндэкса ")
                || s.starts_with("ндэксаў ")
                || s.starts_with("ндэксам ")
                || s.starts_with("ндэксамі ")
                || s.starts_with("ндэксе")
                || s.starts_with("ндэксу")
                || s.starts_with("ндэксы")
                || s.starts_with("ндэксі")
                || s.starts_with("нее")
                || s.starts_with("ней")
                || s.starts_with("нея")
                || s.starts_with("нею")
                || s.starts_with("ніе")
                || s.starts_with("ній")
                || s.starts_with("нія")
                || s.starts_with("нію")
                || s.starts_with("нка")
                || s.starts_with("нкі")
                || s.starts_with("нку")
                || s.starts_with("нтэрым")
                || s.starts_with("нфікс")
                || s.starts_with("нфімум")
                || s.starts_with("ншасц")
                || s.starts_with("ншасьц")
                || s.starts_with("нша ")
                || s.starts_with("ншае ")
                || s.starts_with("ншай ")
                || s.starts_with("ншая ")
                || s.starts_with("ншага ")
                || s.starts_with("ншаму ")
                || s.starts_with("ншасц ")
                || s.starts_with("ншасьц ")
                || s.starts_with("ншую ")
                || s.starts_with("ншы ")
                || s.starts_with("ншым ")
                || s.starts_with("ншымі ")
                || s.starts_with("ншых ")
                || s.starts_with("ншыя ")
        }
        // псілан
        Some('п') => s.starts_with("псілан"),
        // р(… )␣ | рад | рбіс | рмас | рха | рыс␣
        Some('р') => {
            s.starts_with("ра ")
                || s.starts_with("ры ")
                || s.starts_with("ру ")
                || s.starts_with("рам ")
                || s.starts_with("рах ")
                || s.starts_with("рай ")
                || s.starts_with("раў ")
                || s.starts_with("рамі ")
                || s.starts_with("р ")
                || s.starts_with("рад")
                || s.starts_with("рбіс")
                || s.starts_with("рмас")
                || s.starts_with("рха")
                || s.starts_with("рыс ")
        }
        // скарк | скарак | скра… | ста␣ | с[нт]ась?ц | сь?ці[нк]
        Some('с') => {
            s.starts_with("скарк")
                || s.starts_with("скарак")
                || s.starts_with("скра")
                || s.starts_with("скравец")
                || s.starts_with("скрачк")
                || s.starts_with("ста ")
                || s.starts_with("снасц")
                || s.starts_with("снасьц")
                || s.starts_with("стасц")
                || s.starts_with("стасьц")
                || s.starts_with("сцін")
                || s.starts_with("сцік")
                || s.starts_with("сьцін")
                || s.starts_with("сьцік")
        }
        // та[р␣] | тры
        Some('т') => s.starts_with("тар") || s.starts_with("та ") || s.starts_with("тры"),
        // х(ны[хя]?|ную|на[яей])? — bare "х" alone matches.
        Some('х') => true,
        // ць?він
        Some('ц') => s.starts_with("цвін") || s.starts_with("цьвін"),
        // шыяс
        Some('ш') => s.starts_with("шыяс"),
        _ => false,
    }
}

/// Length (in bytes) of the `iwords` prefix of `s`, if any.
///
/// Same alternation as [`matches_iwords`], but returns the match length
/// with the original alternation order (first listed wins, like the regex
/// engine): e.g. `скра` wins over `скравец`, bare `х` loses to `хны…`.
/// Needed where the match is consumed, not just tested (IA-words capture).
/// `None` ⟺ [`matches_iwords`] is false.
pub(crate) fn match_iwords_len(s: &str) -> Option<usize> {
    // Pattern order; mutually-exclusive neighbors make most positions
    // order-insensitive, but prefix pairs (скра|скравец, каў|каўк,
    // ншасц|ншасц␣) and greedy optionals (каньн|канн, хны…|х) need it.
    const WORDS: &[&str] = &[
        "́",
        "біс",
        "бсэн",
        "ва ",
        "ве ",
        "во ",
        "вы ",
        "верс",
        "валг",
        "валз",
        "гар",
        "грышч",
        "грэк",
        "дал",
        "дыш",
        "жыц",
        "канапіс",
        "каньн",
        "канн",
        "кац",
        "каў",
        "кал",
        "каўк",
        "каўц",
        "клы ",
        "клыя ",
        "клая ",
        "клае ",
        "клай ",
        "клага ",
        "кламу ",
        "клую ",
        "ксі",
        "леус",
        "ліст",
        "л ",
        "лістас",
        "льк",
        "м ",
        "мант",
        "масьц",
        "масц",
        "мбрык",
        "мбрыч",
        "менна ",
        "мідж",
        "мпарт",
        "мпарц",
        "мпульса",
        "мпульсе",
        "мпульсу",
        "мпульсы",
        "нахадз",
        "ндыі ",
        "ндый ",
        "ндыю ",
        "ндыя ",
        "ндыев",
        "ндэксаў ",
        "ндэксамі ",
        "ндэксам ",
        "ндэкса ",
        "ндэксе",
        "ндэксу",
        "ндэксы",
        "ндэксі",
        "нее",
        "ней",
        "нея",
        "нею",
        "ніе",
        "ній",
        "нія",
        "нію",
        "нка",
        "нкі",
        "нку",
        "нтэрым",
        "нфікс",
        "нфімум",
        "ншасц",
        "ншасьц",
        "нша ",
        "ншае ",
        "ншай ",
        "ншая ",
        "ншага ",
        "ншаму ",
        "ншасц ",
        "ншасьц ",
        "ншую ",
        "ншы ",
        "ншым ",
        "ншымі ",
        "ншых ",
        "ншыя ",
        "псілан",
        "ра ",
        "ры ",
        "ру ",
        "рам ",
        "рах ",
        "рай ",
        "раў ",
        "рамі ",
        "р ",
        "рад",
        "рбіс",
        "рмас",
        "рха",
        "рыс ",
        "скарк",
        "скарак",
        "скра",
        "скравец",
        "скрачк",
        "ста ",
        "снасц",
        "снасьц",
        "стасц",
        "стасьц",
        "сцін",
        "сцік",
        "сьцін",
        "сьцік",
        "тар",
        "та ",
        "тры",
        "хных",
        "хныя",
        "хны",
        "хную",
        "хная",
        "хнае",
        "хнай",
        "х",
        "цьвін",
        "цвін",
        "шыяс",
    ];
    for w in WORDS {
        if s.starts_with(w) {
            return Some(w.len());
        }
    }
    None
}

/// Iotacize `і` → `й`/`йі` in phonetic contexts.
///
/// Equivalent to the four sequential replacements in `step_iotacize_ji`:
/// `/([V] )і ў/ → $1й у`, `/([V] )і / → $1й␣`,
/// `/([V] ?)і/ → $1йі`, `/ і(?=iwords)/ → ␣йі`
/// (where `V=[аеёіоуыэюя\u0301]`), but without the regex engine.
///
/// Two passes, like the original: the vowel pass first, then the `iwords`
/// pass. They can't merge into one: e.g. ` індыі ` contains `ыі`, which
/// the vowel pass turns into `ыйі`, breaking the `ндыі␣` lookahead —
/// checking the original text would wrongly match. The reverse needs no
/// loop (`й` ∉ `V`, inserted `і` is always preceded by `й`).
/// Each pass is a single scan, one allocation (output grows only by
/// inserted `й`, 2B each).
pub(crate) fn iotacize_ji(text: &str) -> String {
    iotacize_iwords(&iotacize_vowel_ji(text))
}

/// Vowel part: `/([V] )і ў/ → $1й у`, then `/([V] )і / → $1й␣`,
/// then `/([V] ?)і/ → $1йі` — three sequential passes in dict order.
///
/// They must not fuse: e1 and e2 matches can overlap at different starts
/// (`італіі і …`: e1 `і і ` at one position, e2 `іі` starting a char
/// earlier), and the earlier dict entry wins its full pass first. Fusing
/// lets the earlier-starting e2 destroy e1's match (`йімператар` bug).
fn iotacize_vowel_ji(text: &str) -> String {
    iotacize_opt_space_ji(&iotacize_sp_ji(&iotacize_sp_u_ji(text)))
}

/// Entry 0: `(V )і ў` → `$1й у` (`V` + ` і ў`).
fn iotacize_sp_u_ji(text: &str) -> String {
    if !text.contains('і') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    // Trailer ` і ў` is space + `і` + space + `ў` (6 bytes).
    const TRAILER: &str = " і ў";
    while i < len {
        let b = bytes[i];
        // `V` is non-ASCII: ASCII bytes never start a match.
        if b.is_ascii() {
            i += 1;
            continue;
        }
        if let Some(v) = text[i..].chars().next() {
            if is_vowel_ji(v) && text[i + v.len_utf8()..].starts_with(TRAILER) {
                // `V` + space kept, then `й у`.
                let v_end = i + v.len_utf8() + 1;
                out.push_str(&text[flush_from..v_end]);
                out.push('й');
                out.push(' ');
                out.push('у');
                flush_from = i + v.len_utf8() + TRAILER.len();
                i = flush_from;
                continue;
            }
        }
        i += if bytes[i] < 0x80 {
            1
        } else {
            utf8_char_len(bytes[i])
        };
    }
    out.push_str(&text[flush_from..]);
    out
}

/// Entry 1: `(V )і␣` → `$1й␣` (`V` + ` і `, trailing space consumed).
fn iotacize_sp_ji(text: &str) -> String {
    if !text.contains('і') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    // Trailer ` і ` is space + `і` + space (4 bytes).
    const TRAILER: &str = " і ";
    while i < len {
        let b = bytes[i];
        // `V` is non-ASCII: ASCII bytes never start a match.
        if b.is_ascii() {
            i += 1;
            continue;
        }
        if let Some(v) = text[i..].chars().next() {
            if is_vowel_ji(v) && text[i + v.len_utf8()..].starts_with(TRAILER) {
                let v_end = i + v.len_utf8() + 1;
                out.push_str(&text[flush_from..v_end]);
                out.push('й');
                out.push(' ');
                flush_from = i + v.len_utf8() + TRAILER.len();
                i = flush_from;
                continue;
            }
        }
        i += if bytes[i] < 0x80 {
            1
        } else {
            utf8_char_len(bytes[i])
        };
    }
    out.push_str(&text[flush_from..]);
    out
}

/// Entry 2: `(V ?)і` → `$1йі` (optional single space kept in `$1`).
fn iotacize_opt_space_ji(text: &str) -> String {
    if !text.contains('і') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        // `V` is non-ASCII: ASCII bytes never start a match.
        if b.is_ascii() {
            i += 1;
            continue;
        }
        if let Some(v) = text[i..].chars().next() {
            if is_vowel_ji(v) {
                let mut j = i + v.len_utf8();
                if text[j..].starts_with(' ') {
                    j += 1;
                }
                if text[j..].starts_with('і') {
                    out.push_str(&text[flush_from..j]);
                    out.push('й');
                    out.push('і');
                    flush_from = j + 'і'.len_utf8();
                    i = flush_from;
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
    out.push_str(&text[flush_from..]);
    out
}

/// `iwords` part: `/ і(?=iwords)/ → ␣йі`, single scan over `␣і`
/// occurrences with trie lookahead. Must run *after* the vowel pass:
/// e.g. ` індыі ` contains `ыі` which the vowel pass turns into `ыйі`,
/// breaking the `ндыі␣` lookahead — checking the original would wrongly match.
fn iotacize_iwords(text: &str) -> String {
    if !text.contains('і') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    // ` і` is space(1) + `і`(2).
    while i < len {
        let b = bytes[i];
        // The pattern starts with a space.
        if b != b' ' {
            i += if b.is_ascii() { 1 } else { utf8_char_len(b) };
            continue;
        }
        if text[i..].starts_with(" і") && matches_iwords(&text[i + 3..]) {
            out.push_str(&text[flush_from..i + 1]);
            out.push('й');
            out.push('і');
            flush_from = i + 3;
            i += 3;
            continue;
        }
        i += if bytes[i] < 0x80 {
            1
        } else {
            utf8_char_len(bytes[i])
        };
    }
    out.push_str(&text[flush_from..]);
    out
}

#[cfg(test)]
mod tests {
    use super::{iotacize_ji, matches_iwords};

    fn check(input: &str, expected: &str) {
        assert_eq!(iotacize_ji(input), expected, "input: {input:?}");
    }

    fn fancy_iotacize(text: &str) -> String {
        use super::super::fancy_test::FancyDict;
        FancyDict::new(&[
            (r"([аеёіоуыэюя́] )і ў", "$1й у"),
            (r"([аеёіоуыэюя́] )і ", "$1й "),
            (r"([аеёіоуыэюя́] ?)і", "$1йі"),
            (r" і(?=́|біс|бсэн|в[аеоы] |верс|вал[гз]|гар|грышч|грэк|дал|дыш|жыц|канапіс|кань?н|ка[цўл]|каў[кц]|кл(ыя?|а([яей]|га|му)|ую) |ксі|леус|л(іст| )|лістас|льк|м |мант|мась?ц|мбры[кч]|менна |мідж|мпар[тц]|мпульс[аеуы]|нахадз|нды([ійюя] |ев)|ндэкс(а(ў|мі?)? |[еуыі])|н[еі][ейяю]|нк([аіу])|нтэрым|нфікс|нфімум|ншась?ц|нш(а[ейя]?|ага|аму|ась?ц|ую|ы(мі?|х|я)?) |псілан|р([аыу]|а[мхйў]|амі|) |рад|рбіс|рмас|рха|рыс |скарк|скарак|скра|скравец|скрачк|ста |с[нт]ась?ц|сь?ці[нк]|та[р ]|тры|х(ны[хя]?|ную|на[яей])?|ць?він|шыяс)", " йі"),
        ])
        .replace_all(text)
    }

    #[test]
    fn vowel_space_iu() {
        check("а і ў", "а й у");
        check("е і ў б", "е й у б");
        check("о і ў", "о й у");
        // No vowel before → unchanged by rules 1-3.
        check("б і ў", "б і ў");
        check(" і ў", " і ў");
    }

    #[test]
    fn vowel_space_i_space() {
        check("а і ", "а й ");
        check("а і б", "а й б");
        check("е і ", "е й ");
        check("б і ", "б і ");
    }

    #[test]
    fn vowel_opt_space_i() {
        check("аі", "айі");
        check("а і", "а йі");
        check("еі", "ейі");
        check("оі", "ойі");
        check("эі", "эйі");
        check("ббі", "ббі");
        check("аіў", "айіў");
    }

    #[test]
    fn iwords_basic() {
        check(" ібіс", " йібіс");
        check(" іксі", " йіксі");
        check(" іх", " йіх");
        check(" і́", " йі́");
        check(" іб", " іб");
        check(" імама", " імама");
        check(" ім ", " йім ");
        check(" ір ", " йір ");
        check(" іл ", " йіл ");
        check(" іва ", " йіва ");
        check(" ітары", " йітары");
        check(" ітар", " йітар");
        check(" ітры", " йітры");
        check(" іхны", " йіхны");
        check(" іцвін", " йіцвін");
        check(" іцьвін", " йіцьвін");
        check(" ішыяс", " йішыяс");
    }

    #[test]
    fn overlap_non_overlapping() {
        // `ііі`: first `іі` → `ійі`, third stays (regex skips consumed `і`).
        check("ііі", "ійіі");
        check("а іі", "а йіі");
        // Trailing-space priority: `V␣і␣` wins over `V␣і`.
        check("а і ", "а й ");
        // `V␣і␣ў` wins over `V␣і␣`.
        check("а і ў", "а й у");
        // Cross-entry overlap: e1 (`V␣і␣`) wins its full pass over e2
        // (`V ?і`) starting a char earlier (`йімператар` bug).
        check("італіі і імператар", "італійі й імператар");
        check("а і і б", "а й і б");
    }

    #[test]
    fn matches_fancy_regex() {
        let inputs = [
            "",
            "і",
            "іі",
            "ііі",
            "а",
            "аі",
            "а і",
            "а і ",
            "а і ў",
            "а і ў ",
            "е і ў",
            "б і ў",
            " і ў",
            "а і б",
            "аіў",
            "аі",
            "італіі і імператар",
            "а і і б",
            "оі",
            "эі",
            "ббі",
            " ібіс",
            " ібсэн",
            " іва ",
            " іверс",
            " івалг",
            " івалз",
            " ігар",
            " ігрышч",
            " ігрэк",
            " ідал",
            " ідыш",
            " іжыц",
            " іканапіс",
            " іканн",
            " іканьн",
            " ікац",
            " ікаў",
            " ікал",
            " ікаўк",
            " ікаўц",
            " іклы ",
            " іклыя ",
            " іклая ",
            " іклае ",
            " іклай ",
            " іклага ",
            " ікламу ",
            " іклую ",
            " іксі",
            " ілеус",
            " іліст",
            " іл ",
            " ілістас",
            " ільк",
            " ім ",
            " імант",
            " імасц",
            " імасьц",
            " імбрык",
            " імбрыч",
            " іменна ",
            " імідж",
            " імпарт",
            " імпарц",
            " імпульса",
            " імпульсе",
            " інахадз",
            " індыі ",
            " індый ",
            " індыю ",
            " індыя ",
            " інды ев",
            " індэкса ",
            " індэксаў ",
            " індэксам ",
            " індэксамі ",
            " індэксе",
            " індэксу",
            " індэксы",
            " індэксі",
            " інее",
            " іней",
            " інія",
            " інка",
            " інкі",
            " інку",
            " інтэрым",
            " інфікс",
            " інфімум",
            " іншасц",
            " іншасьц",
            " інша ",
            " іншае ",
            " іншую ",
            " іншы ",
            " іншым ",
            " іпсілан",
            " іра ",
            " ір ",
            " ірамі ",
            " ірад",
            " ірбіс",
            " ірмас",
            " ірха",
            " ірыс ",
            " іскарк",
            " іскарак",
            " іскра",
            " іскравец",
            " іскрачк",
            " іста ",
            " існасц",
            " ісцін",
            " ісьцін",
            " ітар",
            " іта ",
            " ітры",
            " іх",
            " іхны",
            " іхную",
            " іцвін",
            " іцьвін",
            " ішыяс",
            " і́",
            "а ібіс",
            "а іксі",
            "а іх",
            "б ібіс",
            " ібіс іксі",
            " ібіс ",
            "аі ібіс",
            "Яны ідуць",
            "але і ён",
            "я і смяяўся",
            "з іншага",
            "а і ў ібіс",
            " і м іксі",
            "emoji 😀 ібіс end",
            "а́і",
            "а́ і",
            "а́ і ў",
        ];
        for input in inputs {
            let expected = fancy_iotacize(input);
            assert_eq!(iotacize_ji(input), expected, "input: {input:?}");
        }
        // iwords matcher spot checks.
        assert!(matches_iwords("біс"));
        assert!(matches_iwords("бсэн"));
        assert!(matches_iwords("ва "));
        assert!(!matches_iwords("ва"));
        assert!(matches_iwords("х"));
        assert!(matches_iwords("хны"));
        assert!(matches_iwords("́"));
        assert!(!matches_iwords("б"));
        assert!(!matches_iwords(""));
        assert!(!matches_iwords(" м"));
    }
}
