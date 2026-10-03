use aho_corasick::{AhoCorasick, MatchKind};
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DictEntry {
    #[serde(alias = "p")]
    pub pattern: String,
    #[serde(alias = "r")]
    pub result: String,
}

enum SequentialEntry {
    Literal(String, String),
    Std(Regex, String),
}

/// One literal batch applied in a single left-to-right pass.
///
/// All patterns are plain strings (no regex metacharacters) and are replaced
/// simultaneously: the replacement text is never rescanned within the same
/// batch. Priority on overlap is dict order (`LeftmostFirst`), mirroring the
/// old sequential order for the common disjoint case.
struct LiteralBatch {
    ac: AhoCorasick,
    replacements: Vec<String>,
}

impl LiteralBatch {
    fn new(entries: &[DictEntry]) -> Self {
        let patterns: Vec<&str> = entries.iter().map(|e| e.pattern.as_str()).collect();
        let replacements: Vec<String> =
            entries.iter().map(|e| e.result.clone()).collect();
        let ac = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostFirst)
            .build(patterns)
            .expect("failed to build Aho-Corasick batch");
        Self { ac, replacements }
    }

    fn replace_all(&self, text: &str) -> String {
        if self.ac.is_match(text) {
            self.ac.replace_all(text, &self.replacements)
        } else {
            text.to_string()
        }
    }

    fn is_empty(&self) -> bool {
        self.replacements.is_empty()
    }
}

/// One regex batch applied in a single left-to-right pass.
///
/// The batch compiles to ONE combined regex
/// `(?P<__b0__>p0)|(?P<__b1__>p1)|…` so the regex engine traverses the text
/// once. Dispatch is by which `__b{i}__` wrapper participated; `$N`
/// backreferences in each replacement are rewritten once at construction to
/// the combined group numbers (`N + wrapper_id`), then expanded with the
/// same routine as the sequential path.
struct RegexBatch {
    re: Regex,
    /// `(wrapper_group_id, rewritten_replacement)` per alternative, in order.
    alternatives: Vec<(usize, String)>,
}

impl RegexBatch {
    fn new(entries: &[DictEntry]) -> Self {
        assert!(
            !entries.is_empty(),
            "RegexBatch::new called with empty entries"
        );
        // Group counts per pattern (excluding the wrapper we are about to add).
        let mut group_counts: Vec<usize> = Vec::with_capacity(entries.len());
        for entry in entries {
            let re = Regex::new(&entry.pattern).unwrap_or_else(|_| {
                panic!(
                    "Failed to compile regex pattern: {:?} (result: {:?})",
                    entry.pattern, entry.result
                )
            });
            // `captures_len` includes group 0.
            group_counts.push(re.captures_len().saturating_sub(1));
        }

        let mut combined = String::new();
        let mut alternatives: Vec<(usize, String)> = Vec::with_capacity(entries.len());
        // wrapper_id for pattern i = 1 + sum_{j<i}(1 + g_j)
        let mut next_id: usize = 1;
        for (i, entry) in entries.iter().enumerate() {
            if i > 0 {
                combined.push('|');
            }
            let wrapper = next_id;
            combined.push_str(&format!("(?P<__b{i}__>{})", entry.pattern));
            let rewritten = rewrite_replacement(&entry.result, wrapper);
            alternatives.push((wrapper, rewritten));
            next_id += 1 + group_counts[i];
        }
        let re = Regex::new(&combined).unwrap_or_else(|_| {
            panic!("Failed to compile combined regex batch ({} entries)", entries.len())
        });
        Self { re, alternatives }
    }

    fn replace_all(&self, text: &str) -> String {
        if !self.re.is_match(text) {
            return text.to_string();
        }
        let mut result = String::with_capacity(text.len());
        let mut last_end = 0;
        for cap in self.re.captures_iter(text) {
            let m = match cap.get(0) {
                Some(m) => m,
                None => continue,
            };
            result.push_str(&text[last_end..m.start()]);
            // Exactly one wrapper participates; first hit wins (dict order).
            let mut dispatched: Option<&str> = None;
            for (wrapper_id, rewritten) in &self.alternatives {
                if cap.get(*wrapper_id).is_some() {
                    dispatched = Some(rewritten.as_str());
                    break;
                }
            }
            if let Some(tpl) = dispatched {
                result.push_str(&expand_replacement_std(tpl, &cap));
            } else {
                // Should be unreachable; copy the match verbatim to stay total.
                result.push_str(m.as_str());
            }
            last_end = m.end();
        }
        result.push_str(&text[last_end..]);
        result
    }
}

enum SinglePassBatch {
    Literal(LiteralBatch),
    Regex(RegexBatch),
}

impl SinglePassBatch {
    /// Build a single-pass batch, auto-selecting the engine:
    /// all-literal → `Aho-Corasick` (no regex engine), otherwise one combined
    /// regex. Returns `None` for empty input (no pass needed).
    fn new(entries: &[DictEntry]) -> Option<Self> {
        if entries.is_empty() {
            return None;
        }
        if entries.iter().all(|e| !has_regex_meta(&e.pattern)) {
            Some(SinglePassBatch::Literal(LiteralBatch::new(entries)))
        } else {
            Some(SinglePassBatch::Regex(RegexBatch::new(entries)))
        }
    }
}

/// A compiled dictionary: a few ordered single-pass batches followed by one
/// ordered sequential tail.
///
/// * Batches `0..n-1` each traverse the text once, replacing several patterns
///   simultaneously (no rescanning of replacement text within the batch).
/// * The last batch (`sequential`) keeps the historical per-entry
///   `str::replace` / `regex` loop for patterns that cannot be batched
///   (order-sensitive exceptions, cascading rules).
///
/// JSON shape is `{p, r}[][]`: outer vec = batches, last inner vec =
/// sequential entries.
pub struct CompiledDict {
    single_pass: Vec<SinglePassBatch>,
    sequential: Vec<SequentialEntry>,
}

impl CompiledDict {
    /// Flat sequential dictionary (historical behavior): every entry runs in
    /// order, literals via `str::replace`, regexes via the `regex` engine.
    /// Used for tests; production dicts use the batched constructors below.
    pub fn new(entries: &[DictEntry]) -> Self {
        Self {
            single_pass: Vec::new(),
            sequential: compile_sequential(entries),
        }
    }

    /// Batched dictionary from `&[(&str, &str)]` batches (static wordlist).
    ///
    /// `batches[..len-1]` each become ONE single-pass pass (Aho-Corasick when
    /// all-literal, otherwise one combined regex); `batches[len-1]` stays
    /// sequential. Empty batches are skipped. Empty outer slice → empty dict.
    pub fn new_batched_str(batches: &[&[(&str, &str)]]) -> Self {
        let owned: Vec<Vec<DictEntry>> = batches
            .iter()
            .map(|b| {
                b.iter()
                    .map(|(p, r)| DictEntry {
                        pattern: (*p).to_string(),
                        result: (*r).to_string(),
                    })
                    .collect()
            })
            .collect();
        Self::new_batched_entries(&owned)
    }

    /// Batched dictionary from owned entry batches (JSON `{p, r}[][]`).
    /// Same contract as [`CompiledDict::new_batched_str`].
    pub fn new_batched_entries(batches: &[Vec<DictEntry>]) -> Self {
        if batches.is_empty() {
            return Self {
                single_pass: Vec::new(),
                sequential: Vec::new(),
            };
        }
        let (head, tail) = batches.split_at(batches.len() - 1);
        let mut single_pass = Vec::with_capacity(head.len());
        for batch in head {
            if let Some(b) = SinglePassBatch::new(batch) {
                single_pass.push(b);
            }
        }
        Self {
            single_pass,
            sequential: compile_sequential(&tail[0]),
        }
    }

    /// Apply batches in order: each single-pass batch traverses once, then
    /// the sequential tail runs per-entry. Short-circuits non-matching
    /// batches/passes like the old loop did.
    pub fn replace_all(&self, text: &str) -> String {
        let mut result = text.to_string();
        for batch in &self.single_pass {
            match batch {
                SinglePassBatch::Literal(b) => {
                    if !b.is_empty() {
                        result = b.replace_all(&result);
                    }
                }
                SinglePassBatch::Regex(b) => {
                    result = b.replace_all(&result);
                }
            }
        }
        for entry in &self.sequential {
            match entry {
                SequentialEntry::Literal(pattern, replacement) => {
                    if result.contains(pattern) {
                        result = result.replace(pattern, replacement);
                    }
                }
                SequentialEntry::Std(re, result_tpl) => {
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
        !self.single_pass.is_empty() || !self.sequential.is_empty()
    }

    /// Number of single-pass batches (excludes the sequential tail).
    pub fn batch_count(&self) -> usize {
        self.single_pass.len()
    }
}

fn compile_sequential(entries: &[DictEntry]) -> Vec<SequentialEntry> {
    let mut compiled: Vec<SequentialEntry> = Vec::with_capacity(entries.len());
    for entry in entries {
        if !has_regex_meta(&entry.pattern) {
            compiled.push(SequentialEntry::Literal(
                entry.pattern.clone(),
                entry.result.clone(),
            ));
        } else if let Ok(re) = Regex::new(&entry.pattern) {
            compiled.push(SequentialEntry::Std(re, entry.result.clone()));
        } else {
            panic!(
                "Failed to compile regex pattern: {:?} (result: {:?})",
                entry.pattern, entry.result
            );
        }
    }
    compiled
}

/// Rewrite `$N` (N > 0) in `replacement` to `$(N + base)` for a combined
/// regex where this alternative's wrapper is group `base` and its own group
/// `k` lives at `base + k`. `$0`, `$&`, `$$` and `\$` pass through untouched
/// (whole-match semantics are identical in the combined regex).
fn rewrite_replacement(replacement: &str, base: usize) -> String {
    let mut out = String::with_capacity(replacement.len() + 4);
    let mut chars = replacement.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('$') => {
                    out.push_str("$$");
                    chars.next();
                }
                Some('&') => {
                    out.push_str("$&");
                    chars.next();
                }
                Some('{') => {
                    // `${N}` form (not used by the current dict, but handle
                    // it so combined batches stay total).
                    let mut clone = chars.clone();
                    clone.next(); // '{'
                    let mut num = String::new();
                    for d in clone.by_ref() {
                        if d.is_ascii_digit() {
                            num.push(d);
                        } else {
                            break;
                        }
                    }
                    if !num.is_empty() {
                        // Consume '{', digits, and optional '}'.
                        chars.next();
                        for _ in 0..num.len() {
                            chars.next();
                        }
                        if chars.peek() == Some(&'}') {
                            chars.next();
                        }
                        let n: usize = num.parse().unwrap_or(0);
                        if n == 0 {
                            out.push_str("$0");
                        } else {
                            out.push_str(&format!("${}", n + base));
                        }
                    } else {
                        out.push('$');
                    }
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
                    let n: usize = num.parse().unwrap_or(0);
                    if n == 0 {
                        out.push_str("$0");
                    } else {
                        out.push_str(&format!("${}", n + base));
                    }
                }
                _ => {
                    out.push('$');
                }
            }
        } else if ch == '\\' {
            match chars.peek() {
                Some('$') => {
                    out.push('\\');
                    out.push('$');
                    chars.next();
                }
                _ => out.push(ch),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

/// Custom `replace_all` applying `re` and expanding `$1`, `$2`, ...
/// backreferences with surrounding literal text.
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
        let dict =
            crate::text::FancyDict::new(&[("пэндзлік", "пэндзлік"), ("дз(?=[еёіюяь])", "дзь")]);
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
        let dict =
            crate::text::FancyDict::new(&[(" ган(?=к|ак )", " ґан"), ("ґанконг", "ганконґ")]);
        let result = dict.replace_all(" ганконг ");
        assert_eq!(result, " ганконґ ", "got: {result:?}");
    }

    fn make_batches(raw: &[&[(&str, &str)]]) -> Vec<Vec<DictEntry>> {
        raw.iter()
            .map(|b| b.iter().map(|(p, r)| make_entry(p, r)).collect())
            .collect()
    }

    #[test]
    fn test_batched_literal_single_pass() {
        // Two literals in one batch replace simultaneously in one pass.
        let batches = make_batches(&[&[("аахен", "аахэн"), (" абасід", " абасыд")], &[("x", "y")]]);
        let dict = CompiledDict::new_batched_entries(&batches);
        assert_eq!(dict.batch_count(), 1);
        assert_eq!(
            dict.replace_all("аахен  абасід x"),
            "аахэн  абасыд y"
        );
    }

    #[test]
    fn test_batched_literal_no_rescan() {
        // Simultaneous: replacement text is not rescanned within the batch.
        // Sequential would double-expand брэстам → ((брэста|…)…); single-pass
        // keeps one variation level.
        let batches = make_batches(&[&[
            ("брэстам", "(брэста|берасьце)м"),
            ("брэста", "(брэста|берасьця)"),
        ], &[("q", "q")]]);
        let dict = CompiledDict::new_batched_entries(&batches);
        assert_eq!(
            dict.replace_all("брэстам"),
            "(брэста|берасьце)м"
        );
    }

    #[test]
    fn test_batched_regex_backrefs() {
        // Combined regex batch rewrites $N to combined group numbers.
        let batches = make_batches(&[&[
            ("абанен([тц])", "абанэн$1"),
            (" абвер(а[мў]? |[ыу] | )", " абвэр$1"),
        ], &[("z", "z")]]);
        let dict = CompiledDict::new_batched_entries(&batches);
        assert_eq!(dict.replace_all("абанент"), "абанэнт");
        assert_eq!(dict.replace_all(" абвера "), " абвэра ");
        assert_eq!(
            dict.replace_all("абаненц  абверы "),
            "абанэнц  абвэры "
        );
    }

    #[test]
    fn test_batched_cross_batch_cascade() {
        // General → exception across batches preserves cascading (village
        // protection): batch 1 converts, batch 2 reverts.
        let batches = make_batches(&[
            &[(" берн", " бэрн")],
            &[(" бэрнік", " бернік")],
            &[("w", "w")],
        ]);
        let dict = CompiledDict::new_batched_entries(&batches);
        assert_eq!(dict.batch_count(), 2);
        // " бернік" (е): batch 1 fires (берн prefix), batch 2 reverts.
        assert_eq!(dict.replace_all(" бернік"), " бернік");
        // " бэрнік" (э): batch 1 misses, batch 2 converts.
        assert_eq!(dict.replace_all(" бэрнік"), " бернік");
    }

    #[test]
    fn test_batched_gankong_split() {
        // Ганконг needs general (with backref) in an earlier batch than the
        // exception; same-batch simultaneous would stop after the first step.
        let batches = make_batches(&[
            &[(" ган(к|ак )", " ґан$1")],
            &[("ґанконг", "ганконґ")],
            &[("e", "e")],
        ]);
        let dict = CompiledDict::new_batched_entries(&batches);
        assert_eq!(dict.replace_all(" ганконг"), " ганконґ");
    }

    #[test]
    fn test_batched_str_constructor() {
        let dict = CompiledDict::new_batched_str(&[
            &[("a", "b"), ("c", "d")],
            &[("e", "f")],
        ]);
        assert_eq!(dict.batch_count(), 1);
        assert_eq!(dict.replace_all("a c e"), "b d f");
    }

    #[test]
    fn test_rewrite_replacement_offsets() {
        // Wrapper for pattern 0 is group 1; its $1 → $2. Pattern 1 wrapper is
        // group 1+1+g0; with g0=1, wrapper=3, $1 → $4.
        assert_eq!(rewrite_replacement("абанэн$1", 1), "абанэн$2");
        assert_eq!(rewrite_replacement(" абвэр$1", 3), " абвэр$4");
        assert_eq!(rewrite_replacement("$0 $& $$ $1x", 5), "$0 $& $$ $6x");
        assert_eq!(rewrite_replacement("\\$1 $2", 10), "\\$1 $12");
    }
}
