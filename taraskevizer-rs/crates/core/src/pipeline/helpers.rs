use crate::config::Alphabet;

pub(crate) use crate::text::soften;

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

/// Compile a `regex` pattern once and reuse it across all calls.
///
/// `regex::Regex::new` is surprisingly expensive (it builds Unicode
/// property automata, e.g. for `\p{P}|\p{S}|\d+`), so recompiling it on every
/// `regex_replace_all` call — which happens once per chunk in the parallel
/// pipeline — dominated `step_prepare`/`step_finalize`. This cache compiles
/// each distinct pattern a single time for the whole process and returns a
/// cheaply-cloneable `Arc` handle.
fn compiled_regex(pattern: &str) -> Arc<regex::Regex> {
    static CACHE: LazyLock<Mutex<HashMap<String, Arc<regex::Regex>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    let mut cache = CACHE.lock().unwrap();
    if let Some(re) = cache.get(pattern) {
        return Arc::clone(re);
    }
    let re = Arc::new(regex::Regex::new(pattern).unwrap());
    cache.insert(pattern.to_string(), Arc::clone(&re));
    re
}

/// Lower-case alphabet conversion, borrowing the input when the alphabet
/// needs no conversion (cyrillic) so hot call sites pay no allocation.
/// Converters for other alphabets must build a new string (`Cow::Owned`).
pub(crate) fn apply_abc_lower(text: &str, abc: Alphabet) -> Cow<'_, str> {
    match abc {
        Alphabet::Cyrillic => Cow::Borrowed(text),
        Alphabet::Latin => Cow::Owned(crate::text::convert_latin_lower(text)),
        Alphabet::LatinJi => Cow::Owned(crate::text::convert_latin_ji_lower(text)),
        Alphabet::Arabic => Cow::Owned(crate::text::convert_arabic(text)),
    }
}

/// Manual upper-case alphabet conversion; `None` when the alphabet has no
/// upper table (cyrillic, arabic) — replaces the `has_entries()` check.
pub(crate) fn apply_abc_upper(text: &str, abc: Alphabet) -> Option<String> {
    match abc {
        Alphabet::Latin => Some(crate::text::convert_latin_upper(text)),
        Alphabet::LatinJi => Some(crate::text::convert_latin_ji_upper(text)),
        Alphabet::Cyrillic | Alphabet::Arabic => None,
    }
}

pub(crate) fn find_unescaped_gt(s: &str) -> Option<usize> {
    let mut chars = s.char_indices();
    while let Some((byte_idx, c)) = chars.next() {
        if c == '\\' {
            chars.next();
        } else if c == '>' {
            return Some(byte_idx);
        }
    }
    None
}

pub(crate) fn regex_replace_all_with(
    text: &str,
    pattern: &str,
    mut callback: impl FnMut(&regex::Captures) -> String,
) -> String {
    let re = compiled_regex(pattern);
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;
    for cap in re.captures_iter(text) {
        let m = match cap.get(0) {
            Some(m) => m,
            None => continue,
        };
        result.push_str(&text[last_end..m.start()]);
        result.push_str(&callback(&cap));
        last_end = m.end();
    }
    result.push_str(&text[last_end..]);
    result
}

pub(crate) fn replace_g_str(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        out.push(match ch {
            'Ґ' => 'Г',
            'ґ' => 'г',
            _ => ch,
        })
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

pub(crate) fn initcap(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

pub(crate) fn initcap_var(word: &str) -> String {
    regex_replace_all_with(word, r"[^(|)]*[|)]", |caps| {
        let m = caps.get(0).map(|m| m.as_str()).unwrap_or("");
        initcap(m)
    })
}

pub fn apply_highlight_diff(
    word: &str,
    o_word: &str,
    is_cyrillic: bool,
    fix: &dyn Fn(&str) -> String,
) -> String {
    let word_h = if is_cyrillic {
        replace_g_str(word)
    } else {
        word.to_string()
    };
    highlight_diff_word(word, o_word, &word_h, is_cyrillic, fix)
}

pub(crate) fn highlight_diff_word(
    word: &str,
    o_word: &str,
    word_h: &str,
    is_cyrillic: bool,
    highlight: &dyn Fn(&str) -> String,
) -> String {
    let wchars: Vec<char> = word.chars().collect();
    let ochars: Vec<char> = o_word.chars().collect();
    let hchars: Vec<char> = word_h.chars().collect();
    let wlen = wchars.len();
    let olen = ochars.len();

    if !word.contains('(') && wlen == olen && wlen == hchars.len() {
        let mut result = String::new();
        let mut j = 0;
        while j < wlen {
            while j < wlen && hchars[j] == ochars[j] {
                result.push(wchars[j]);
                j += 1;
            }
            if j == wlen {
                break;
            }
            let first = j;
            while j < wlen && hchars[j] != ochars[j] {
                j += 1;
            }
            let diff: String = wchars[first..j].iter().collect();
            result.push_str(&highlight(&diff));
        }
        return result;
    }

    if is_cyrillic && !word.contains('(') {
        let no_soft_word: String = word.chars().filter(|&c| c != 'ь').collect();
        if o_word == no_soft_word {
            let mut result = String::new();
            for c in word.chars() {
                if c == 'ь' {
                    result.push_str(&highlight("ь"));
                } else {
                    result.push(c);
                }
            }
            return result;
        }
        let mut no_soft_word_plus_ь = no_soft_word.clone();
        no_soft_word_plus_ь.push('ь');
        if o_word == no_soft_word_plus_ь {
            let mut result = String::new();
            let wchars: Vec<char> = word.chars().collect();
            for (j, &c) in wchars.iter().enumerate() {
                if c == 'ь' && j != wchars.len() - 1 {
                    result.push_str(&highlight("ь"));
                } else {
                    result.push(c);
                }
            }
            return result;
        }
    }

    highlight_diff_variable(&wchars, &ochars, &hchars, highlight)
}

fn highlight_diff_variable(
    word: &[char],
    o_word: &[char],
    word_h: &[char],
    highlight: &dyn Fn(&str) -> String,
) -> String {
    let wlen = word.len();
    let olen = o_word.len();

    if wlen == 0 {
        return String::new();
    }

    let mut last_i = wlen as isize - 1;
    let mut last_oi = olen as isize - 1;
    while last_i >= 0 && last_oi >= 0 && word_h[last_i as usize] == o_word[last_oi as usize] {
        last_i -= 1;
        last_oi -= 1;
    }

    if last_i < 0 {
        let first: String = word[..1].iter().collect();
        let rest: String = word[1..].iter().collect();
        return format!("{}{}", highlight(&first), rest);
    }

    let mut first_i = 0;
    while first_i < wlen && first_i < olen && word_h[first_i] == o_word[first_i] {
        first_i += 1;
    }

    let last_i_u = last_i as usize;
    let last_oi_u = last_oi as usize;

    if first_i == wlen {
        let prefix: String = word[..last_i_u].iter().collect();
        let last: String = word[last_i_u..last_i_u + 1].iter().collect();
        return format!("{}{}", prefix, highlight(&last));
    }

    if first_i == 0 && last_oi_u == olen.wrapping_sub(1) {
        let whole: String = word.iter().collect();
        return highlight(&whole);
    }

    let last_i_exclusive = last_i_u + 1;
    if first_i == last_i_exclusive {
        let first_i = if first_i > 0 { first_i - 1 } else { 0 };
        let last_i_exclusive = if last_i_exclusive < wlen {
            last_i_exclusive + 1
        } else {
            wlen
        };
        let prefix: String = word[..first_i].iter().collect();
        let diff: String = word[first_i..last_i_exclusive].iter().collect();
        let suffix: String = word[last_i_exclusive..].iter().collect();
        return format!("{}{}{}", prefix, highlight(&diff), suffix);
    }

    // The matched prefix and suffix can overlap when `wlen != olen` and the only
    // difference is made of repeated characters (e.g. "aaa" vs "aa"). In that
    // case there is no distinct difference region to highlight, so return the
    // word unchanged. This also guards against inverted slice indices that would
    // otherwise panic (`word[first_i..last_i_exclusive]` with first_i > end).
    if first_i > last_i_exclusive {
        let whole: String = word.iter().collect();
        return whole;
    }

    let prefix: String = word[..first_i].iter().collect();
    let diff: String = word[first_i..last_i_exclusive].iter().collect();
    let suffix: String = word[last_i_exclusive..].iter().collect();
    format!("{}{}{}", prefix, highlight(&diff), suffix)
}
