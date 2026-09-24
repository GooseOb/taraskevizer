use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DictEntry {
    #[serde(alias = "p")]
    pub pattern: String,
    #[serde(alias = "r")]
    pub result: String,
}

enum Entry {
    Literal(String, String),
    Fancy(FancyRegex, String),
    Std(Regex, String),
}

/// A compiled dictionary that processes entries sequentially in their
/// original JSON order. For each entry:
/// - If it has no regex metacharacters → plain `str::replace`
/// - If it has lookarounds/backrefs → `fancy_regex` replacement
/// - Otherwise → `taraskevizer-matcher` (single-pattern DFA-based)
pub struct CompiledDict {
    entries: Vec<Entry>,
}

impl CompiledDict {
    pub fn new(entries: &[DictEntry]) -> Self {
        let mut compiled: Vec<Entry> = Vec::with_capacity(entries.len());

        for entry in entries {
            if !has_regex_meta(&entry.pattern) {
                // Plain string replacement (no regex metacharacters).
                let expanded = entry.result.replace("$&", &entry.pattern);
                compiled.push(Entry::Literal(entry.pattern.clone(), expanded));
            } else if needs_fancy_regex(&entry.pattern) {
                // Look-arounds / backreferences the default `regex` crate
                // cannot compile: use `fancy_regex`.
                if let Ok(re) = FancyRegex::new(&entry.pattern) {
                    compiled.push(Entry::Fancy(re, entry.result.clone()));
                }
            } else if let Ok(re) = Regex::new(&entry.pattern) {
                // Default `regex` crate, applied sequentially per entry.
                compiled.push(Entry::Std(re, entry.result.clone()));
            } else if let Ok(re) = FancyRegex::new(&entry.pattern) {
                // Fallback for anything the default crate rejects.
                compiled.push(Entry::Fancy(re, entry.result.clone()));
            }
        }

        Self { entries: compiled }
    }

    /// Apply all entries sequentially in their original JSON order.
    /// Short-circuits: skips entries that don't match, avoiding unnecessary
    /// full-string traversals.
    pub fn replace_all(&self, text: &str) -> String {
        let mut result = text.to_string();
        for entry in &self.entries {
            match entry {
                Entry::Literal(pattern, replacement) => {
                    if result.contains(pattern) {
                        result = result.replace(pattern, replacement);
                    }
                }
                Entry::Fancy(re, result_tpl) => {
                    if re.is_match(&result).unwrap_or(false) {
                        result = fancy_replace_all(re, &result, result_tpl);
                    }
                }
                Entry::Std(re, result_tpl) => {
                    // NOTE: delegating to `re.replace_all(&result, &str)` uses the
                    // `regex` crate's `&str` Replacer, which (in the current
                    // `regex` 1.13.x) mis-expands a backreference `$N` that is
                    // immediately followed by another letter (e.g. `$1JI` -> ""),
                    // silently dropping characters. Expand manually instead so
                    // `$N` + literal text works correctly.
                    if re.is_match(&result) {
                        result = replace_all_std(re, &result, result_tpl);
                    }
                }
            }
        }
        result
    }

    pub fn has_entries(&self) -> bool {
        !self.entries.is_empty()
    }

    /// Iterative soften: apply all entries repeatedly until stable.
    pub fn soften(&self, text: &str) -> String {
        let mut result = text.to_string();
        loop {
            let prev = result.clone();
            result = self.replace_all(&result);
            if result == prev {
                return result;
            }
        }
    }
}

/// Check if a pattern needs fancy-regex (has lookarounds, backrefs, or
/// other constructs not supported by regex-automata).
fn needs_fancy_regex(pattern: &str) -> bool {
    let bytes = pattern.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'\\' && bytes[i + 1].is_ascii_digit() {
            return true; // backreference \1, \2, etc.
        }
        if bytes[i] == b'(' && i + 2 < bytes.len() && bytes[i + 1] == b'?' {
            match bytes[i + 2] {
                b'=' | b'!' => return true, // lookahead (?= or (?!)
                b'<' if i + 3 < bytes.len() => match bytes[i + 3] {
                    b'=' | b'!' => return true, // lookbehind (?<= or (?<!)
                    _ => {}
                },
                b'>' => return true, // atomic group (?>)
                _ => {}
            }
        }
        i += 1;
    }
    false
}

/// Custom `replace_all` that manually expands `$1`, `$2` etc. backreferences
/// in the replacement string, because `fancy_regex::Regex::replace_all()`
/// does not properly handle them.
pub(crate) fn fancy_replace_all(re: &FancyRegex, text: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;
    for cap in re.captures_iter(text) {
        let cap = match cap {
            Ok(c) => c,
            Err(_) => continue,
        };
        let m = match cap.get(0) {
            Some(m) => m,
            None => continue,
        };
        result.push_str(&text[last_end..m.start()]);
        result.push_str(&expand_replacement(replacement, &cap));
        last_end = m.end();
    }
    result.push_str(&text[last_end..]);
    result
}

/// `regex` (std) analogue of [`fancy_replace_all`], applying `re` and
/// expanding `$1`, `$2`, ... backreferences with surrounding literal text.
/// Uses a manual `Captures`-based expansion to avoid the broken `&str`
/// Replacer behaviour (see [`CompiledDict::replace_all`]).
fn replace_all_std(re: &Regex, text: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;
    for cap in re.captures_iter(text) {
        let m = match cap.get(0) {
            Some(m) => m,
            None => continue,
        };
        result.push_str(&text[last_end..m.start()]);
        result.push_str(&expand_replacement_std(replacement, &cap));
        last_end = m.end();
    }
    result.push_str(&text[last_end..]);
    result
}

fn expand_replacement_std(replacement: &str, cap: &regex::Captures) -> String {
    let mut result = String::new();
    let mut chars = replacement.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('$') => {
                    result.push('$');
                    chars.next();
                }
                Some('&') => {
                    if let Some(m) = cap.get(0) {
                        result.push_str(m.as_str());
                    }
                    chars.next();
                }
                Some('0'..='9') => {
                    let mut num = String::new();
                    while let Some(d) = chars.peek() {
                        if d.is_ascii_digit() {
                            num.push(*d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let idx: usize = num.parse().unwrap_or(0);
                    if let Some(m) = cap.get(idx) {
                        result.push_str(m.as_str());
                    }
                }
                _ => {
                    result.push('$');
                }
            }
        } else if ch == '\\' {
            match chars.peek() {
                Some('$') => {
                    result.push('$');
                    chars.next();
                }
                _ => result.push(ch),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn expand_replacement(replacement: &str, cap: &fancy_regex::Captures) -> String {
    let mut result = String::new();
    let mut chars = replacement.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('$') => {
                    result.push('$');
                    chars.next();
                }
                Some('&') => {
                    if let Some(m) = cap.get(0) {
                        result.push_str(m.as_str());
                    }
                    chars.next();
                }
                Some('0'..='9') => {
                    let mut num = String::new();
                    while let Some(d) = chars.peek() {
                        if d.is_ascii_digit() {
                            num.push(*d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let idx: usize = num.parse().unwrap_or(0);
                    if let Some(m) = cap.get(idx) {
                        result.push_str(m.as_str());
                    }
                }
                _ => {
                    result.push('$');
                }
            }
        } else if ch == '\\' {
            match chars.peek() {
                Some('$') => {
                    result.push('$');
                    chars.next();
                }
                _ => result.push(ch),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Check if a pattern string contains regex metacharacters.
pub(crate) fn has_regex_meta(pattern: &str) -> bool {
    pattern.contains('\\')
        || pattern.contains('(')
        || pattern.contains(')')
        || pattern.contains('[')
        || pattern.contains(']')
        || pattern.contains('.')
        || pattern.contains('*')
        || pattern.contains('+')
        || pattern.contains('?')
        || pattern.contains('^')
        || pattern.contains('$')
        || pattern.contains('|')
        || pattern.contains('{')
        || pattern.contains('}')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(p: &str, r: &str) -> DictEntry {
        DictEntry {
            pattern: p.to_string(),
            result: r.to_string(),
        }
    }

    #[test]
    fn test_literal_simple() {
        let entries = vec![make_entry("планета", "плянэта")];
        let dict = CompiledDict::new(&entries);
        assert_eq!(dict.replace_all("планета"), "плянэта");
    }

    #[test]
    fn test_regex_simple() {
        let entries = vec![make_entry("се(?:к?)(ц)ы[ія]", "сэ$1ыя")];
        let dict = CompiledDict::new(&entries);
        assert_eq!(dict.replace_all("секцыя"), "сэцыя");
    }

    #[test]
    fn test_mixed_literal_and_regex() {
        let entries = vec![
            make_entry("планета", "плянэта"),
            make_entry("се(?:к?)(ц)ы[ія]", "сэ$1ыя"),
        ];
        let dict = CompiledDict::new(&entries);
        assert_eq!(dict.replace_all("планета секцыя"), "плянэта сэцыя");
    }

    #[test]
    fn test_soften_loop() {
        let entries = vec![
            make_entry("пэндзлік", "пэндзлік"),
            make_entry("дз(?=[еёіюяь])", "дзь"),
        ];
        let dict = CompiledDict::new(&entries);
        assert_eq!(dict.replace_all("пэндзлік"), "пэндзлік");
        assert_eq!(dict.replace_all("дзі"), "дзьі");
    }

    #[test]
    fn test_no_meta_check() {
        assert!(!has_regex_meta("планета"));
        assert!(!has_regex_meta(" гера"));
        assert!(!has_regex_meta("Гродна"));
        assert!(has_regex_meta("ге(?! )"));
        assert!(has_regex_meta("се(?:к?)(ц)ы[ія]"));
        assert!(has_regex_meta("абанемен(?=[тц])"));
    }

    #[test]
    fn test_order_preserved() {
        // Simulate the ганконг case: regex then literal AC
        let entries = vec![
            make_entry(" ган(?=к|ак )", " ґан"),
            make_entry("ґанконг", "ганконґ"),
        ];
        let dict = CompiledDict::new(&entries);
        let result = dict.replace_all(" ганконг ");
        assert_eq!(result, " ганконґ ", "got: {result:?}");
    }
}
