//! `regex`-based replace-all where the callback appends directly to the
//! output buffer instead of returning an intermediate `String`.

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

/// `regex`-based replace where the callback appends directly to the output
/// buffer instead of returning an intermediate `String` (one alloc saved per
/// match — the variations step can have hundreds per chunk).
pub(crate) fn regex_replace_all_with(
    text: &str,
    pattern: &str,
    mut callback: impl FnMut(&regex::Captures, &mut String),
) -> String {
    let re = compiled_regex(pattern);
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;
    for cap in re.captures_iter(text) {
        let Some(m) = cap.get(0) else { continue };
        result.push_str(&text[last_end..m.start()]);
        callback(&cap, &mut result);
        last_end = m.end();
    }
    result.push_str(&text[last_end..]);
    result
}

#[cfg(test)]
mod tests {
    use super::regex_replace_all_with;

    #[test]
    fn replaces_all_matches() {
        let out = regex_replace_all_with("a1b22c", r"\d+", |caps, dst| {
            dst.push('[');
            dst.push_str(&caps[0]);
            dst.push(']');
        });
        assert_eq!(out, "a[1]b[22]c");
    }

    #[test]
    fn no_match_passthrough() {
        let out = regex_replace_all_with("abc", r"\d+", |caps, dst| {
            dst.push_str(&caps[0]);
        });
        assert_eq!(out, "abc");
    }
}
