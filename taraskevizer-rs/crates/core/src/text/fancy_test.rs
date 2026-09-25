//! Test-only `fancy-regex` helpers (the crate is a dev-dependency).
//!
//! [`FancyDict`] replicates the pre-manual-rewiring production dictionary
//! semantics — sequential entries, literal `str::replace` (with `$&`
//! expansion) for patterns without metacharacters, `fancy_replace_all`
//! otherwise — so oracles keep verifying against the exact old behavior.

use fancy_regex::Regex as FancyRegex;

use crate::dict::types::has_regex_meta;

enum FancyEntry {
    Literal(String, String),
    Fancy(FancyRegex, String),
}

/// Test-only compiled dictionary with the original (fancy-backed) behavior.
pub(crate) struct FancyDict {
    entries: Vec<FancyEntry>,
}

impl FancyDict {
    pub(crate) fn new(entries: &[(&str, &str)]) -> Self {
        let mut compiled = Vec::with_capacity(entries.len());
        for (pattern, result) in entries {
            if !has_regex_meta(pattern) {
                // Plain string replacement (no regex metacharacters).
                let expanded = result.replace("$&", pattern);
                compiled.push(FancyEntry::Literal(
                    (*pattern).to_string(),
                    expanded,
                ));
            } else if let Ok(re) = FancyRegex::new(pattern) {
                compiled.push(FancyEntry::Fancy(re, (*result).to_string()));
            } else {
                panic!(
                    "Failed to compile fancy pattern: {pattern:?} (result: {result:?})"
                );
            }
        }
        Self { entries: compiled }
    }

    /// Apply all entries sequentially in the given order.
    pub(crate) fn replace_all(&self, text: &str) -> String {
        let mut result = text.to_string();
        for entry in &self.entries {
            match entry {
                FancyEntry::Literal(pattern, replacement) => {
                    if result.contains(pattern) {
                        result = result.replace(pattern, replacement);
                    }
                }
                FancyEntry::Fancy(re, result_tpl) => {
                    if re.is_match(&result).unwrap_or(false) {
                        result = fancy_replace_all(re, &result, result_tpl);
                    }
                }
            }
        }
        result
    }
}

/// Custom `replace_all` that manually expands `$1`, `$2` etc. backreferences
/// in the replacement string, because `fancy_regex::Regex::replace_all()`
/// does not properly handle them.
pub(crate) fn fancy_replace_all(
    re: &FancyRegex,
    text: &str,
    replacement: &str,
) -> String {
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
