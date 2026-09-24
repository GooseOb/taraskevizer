use super::utf8_char_len;

/// Whether the 2-byte sequence is in `[аеёіоуыэюя\u0301]`.
///
/// All members are 2-byte UTF-8:
/// а=D0 B0, е=D0 B5, ё=D1 91, і=D1 96, о=D0 BE, у=D1 83,
/// ы=D1 8B, э=D1 8D, ю=D1 8E, я=D1 8F, ́=CC 81.
#[inline]
fn is_vowel_ji(b1: u8, b2: u8) -> bool {
    matches!(
        (b1, b2),
        (0xD0, 0xB0)
            | (0xD0, 0xB5)
            | (0xD1, 0x91)
            | (0xD1, 0x96)
            | (0xD0, 0xBE)
            | (0xD1, 0x83)
            | (0xD1, 0x8B)
            | (0xD1, 0x8D)
            | (0xD1, 0x8E)
            | (0xD1, 0x8F)
            | (0xCC, 0x81)
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
fn matches_iwords(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 2 {
        return false;
    }
    // ́ U+0301 (CC 81) alone matches.
    if b[0] == 0xCC && b[1] == 0x81 {
        return true;
    }
    if b[0] != 0xD0 && b[0] != 0xD1 {
        return false;
    }
    match (b[0], b[1]) {
        // біс | бсэн
        (0xD0, 0xB1) => s.starts_with("біс") || s.starts_with("бсэн"),
        // в[аеоы]␣ | верс | вал[гз]
        (0xD0, 0xB2) => {
            s.starts_with("ва ")
                || s.starts_with("ве ")
                || s.starts_with("во ")
                || s.starts_with("вы ")
                || s.starts_with("верс")
                || s.starts_with("валг")
                || s.starts_with("валз")
        }
        // гар | грышч | грэк
        (0xD0, 0xB3) => {
            s.starts_with("гар") || s.starts_with("грышч") || s.starts_with("грэк")
        }
        // дал | дыш
        (0xD0, 0xB4) => s.starts_with("дал") || s.starts_with("дыш"),
        // жыц
        (0xD0, 0xB6) => s.starts_with("жыц"),
        // канапіс | кань?н | ка[цўл] | каў[кц] | кл(… )␣ | ксі
        (0xD0, 0xBA) => {
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
        (0xD0, 0xBB) => {
            s.starts_with("леус")
                || s.starts_with("ліст")
                || s.starts_with("л ")
                || s.starts_with("лістас")
                || s.starts_with("льк")
        }
        // м␣ | мант | мась?ц | мбры[кч] | менна␣ | мідж | мпар[тц] | мпульс[аеуы]
        (0xD0, 0xBC) => {
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
        (0xD0, 0xBD) => {
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
        (0xD0, 0xBF) => s.starts_with("псілан"),
        // р(… )␣ | рад | рбіс | рмас | рха | рыс␣
        (0xD1, 0x80) => {
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
        (0xD1, 0x81) => {
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
        (0xD1, 0x82) => {
            s.starts_with("тар") || s.starts_with("та ") || s.starts_with("тры")
        }
        // х(ны[хя]?|ную|на[яей])? — bare "х" alone matches.
        (0xD1, 0x85) => true,
        // ць?він
        (0xD1, 0x86) => s.starts_with("цвін") || s.starts_with("цьвін"),
        // шыяс
        (0xD1, 0x88) => s.starts_with("шыяс"),
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
        "́", "біс", "бсэн", "ва ", "ве ", "во ", "вы ", "верс", "валг", "валз",
        "гар", "грышч", "грэк", "дал", "дыш", "жыц", "канапіс", "каньн", "канн",
        "кац", "каў", "кал", "каўк", "каўц", "клы ", "клыя ", "клая ", "клае ",
        "клай ", "клага ", "кламу ", "клую ", "ксі", "леус", "ліст", "л ",
        "лістас", "льк", "м ", "мант", "масьц", "масц", "мбрык", "мбрыч",
        "менна ", "мідж", "мпарт", "мпарц", "мпульса", "мпульсе", "мпульсу",
        "мпульсы", "нахадз", "ндыі ", "ндый ", "ндыю ", "ндыя ", "ндыев",
        "ндэксаў ", "ндэксамі ", "ндэксам ", "ндэкса ", "ндэксе", "ндэксу",
        "ндэксы", "ндэксі", "нее", "ней", "нея", "нею", "ніе", "ній", "нія",
        "нію", "нка", "нкі", "нку", "нтэрым", "нфікс", "нфімум", "ншасц",
        "ншасьц", "нша ", "ншае ", "ншай ", "ншая ", "ншага ", "ншаму ",
        "ншасц ", "ншасьц ", "ншую ", "ншы ", "ншым ", "ншымі ", "ншых ",
        "ншыя ", "псілан", "ра ", "ры ", "ру ", "рам ", "рах ", "рай ",
        "раў ", "рамі ", "р ", "рад", "рбіс", "рмас", "рха", "рыс ", "скарк",
        "скарак", "скра", "скравец", "скрачк", "ста ", "снасц", "снасьц",
        "стасц", "стасьц", "сцін", "сцік", "сьцін", "сьцік", "тар", "та ",
        "тры", "хных", "хныя", "хны", "хную", "хная", "хнае", "хнай", "х",
        "цьвін", "цвін", "шыяс",
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

/// Vowel part: `/([V] )і ў/ → $1й у`, `/([V] )і / → $1й␣`,
/// `/([V] ?)і/ → $1йі` merged into one scan.
///
/// `V␣і␣ў`/`V␣і␣` take priority over plain `V[␣]?і`. Replacements never
/// create new matches (`й` ∉ `V`), and trailing lookaheads (`␣ў`/`␣`)
/// contain no `і`, so sequential passes equal one prioritized pass.
/// Non-overlapping: a consumed `і` can't serve as the next `V`.
fn iotacize_vowel_ji(text: &str) -> String {
    if !text.contains('і') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + 16);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    // `i` always stays on a char boundary, so all slicing is safe.
    while i < len {
        let b = bytes[i];
        // Vowel start? All V are 2 bytes.
        if i + 2 <= len && is_vowel_ji(b, bytes[i + 1]) {
            // `V␣і`?
            if i + 5 <= len
                && bytes[i + 2] == b' '
                && bytes[i + 3] == 0xD1
                && bytes[i + 4] == 0x96
            {
                let j = i + 5; // after `і`
                // Case 1: `V␣і␣ў` → `V␣й␣у`.
                if j + 3 <= len
                    && bytes[j] == b' '
                    && bytes[j + 1] == 0xD1
                    && bytes[j + 2] == 0x9E
                {
                    out.push_str(&text[flush_from..i + 3]);
                    out.push('й');
                    out.push(' ');
                    out.push('у');
                    flush_from = j + 3;
                    i = j + 3;
                    continue;
                }
                // Case 2: `V␣і␣` → `V␣й␣`.
                if j < len && bytes[j] == b' ' {
                    out.push_str(&text[flush_from..i + 3]);
                    out.push('й');
                    out.push(' ');
                    flush_from = j + 1;
                    i = j + 1;
                    continue;
                }
                // Case 3 with space: `V␣і` → `V␣йі`.
                out.push_str(&text[flush_from..i + 3]);
                out.push('й');
                out.push('і');
                flush_from = i + 5;
                i += 5;
                continue;
            }
            // Case 3 without space: `Vі` → `Vйі`.
            if i + 4 <= len && bytes[i + 2] == 0xD1 && bytes[i + 3] == 0x96 {
                out.push_str(&text[flush_from..i + 2]);
                out.push('й');
                out.push('і');
                flush_from = i + 4;
                i += 4;
                continue;
            }
            // Bare vowel, no `і` after.
            i += 2;
            continue;
        }
        i += if b < 0x80 { 1 } else { utf8_char_len(b) };
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
    while i < len {
        if bytes[i] == b' '
            && i + 3 <= len
            && bytes[i + 1] == 0xD1
            && bytes[i + 2] == 0x96
            && matches_iwords(&text[i + 3..])
        {
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
        let mut t = text.to_string();
        for (pat, rep) in [
            (r"([аеёіоуыэюя́] )і ў", "$1й у"),
            (r"([аеёіоуыэюя́] )і ", "$1й "),
            (r"([аеёіоуыэюя́] ?)і", "$1йі"),
        ] {
            let re = fancy_regex::Regex::new(pat).unwrap();
            t = crate::dict::types::fancy_replace_all(&re, &t, rep);
        }
        let re4 = fancy_regex::Regex::new(r" і(?=́|біс|бсэн|в[аеоы] |верс|вал[гз]|гар|грышч|грэк|дал|дыш|жыц|канапіс|кань?н|ка[цўл]|каў[кц]|кл(ыя?|а([яей]|га|му)|ую) |ксі|леус|л(іст| )|лістас|льк|м |мант|мась?ц|мбры[кч]|менна |мідж|мпар[тц]|мпульс[аеуы]|нахадз|нды([ійюя] |ев)|ндэкс(а(ў|мі?)? |[еуыі])|н[еі][ейяю]|нк([аіу])|нтэрым|нфікс|нфімум|ншась?ц|нш(а[ейя]?|ага|аму|ась?ц|ую|ы(мі?|х|я)?) |псілан|р([аыу]|а[мхйў]|амі|) |рад|рбіс|рмас|рха|рыс |скарк|скарак|скра|скравец|скрачк|ста |с[нт]ась?ц|сь?ці[нк]|та[р ]|тры|х(ны[хя]?|ную|на[яей])?|ць?він|шыяс)").unwrap();
        t = crate::dict::types::fancy_replace_all(&re4, &t, " йі");
        t
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
