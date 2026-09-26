use crate::pipeline::{
    helpers::{initcap, initcap_var},
    PipelineContext,
};

/// Mirrors the JS `isUpperCase` helper: a string is considered uppercase when
/// uppercasing it leaves it unchanged (`str === str.toUpperCase()`).
///
/// This differs from `char::is_uppercase` in two ways that cause divergence
/// with the reference implementation:
///   * combining marks such as U+0301 (combining acute) are *not* reported as
///     uppercase by `char::is_uppercase`, but JS treats them as uppercase
///     because the character is unchanged by `toUpperCase`; and
///   * non-letters that are unchanged by uppercasing (digits, some punctuation)
///     are treated as uppercase by JS for the same reason.
///
/// Replicating the string comparison keeps the Rust port faithful to the
/// reference `restoreCase` logic.
fn is_upper_str(s: &str) -> bool {
    s == s.to_uppercase()
}
fn is_upper_char(c: char) -> bool {
    c.to_uppercase().eq(std::iter::once(c))
}

pub fn step_restore_case(ctx: &mut PipelineContext) {
    restore_case_words(&mut ctx.text_arr, &ctx.orig_arr);
}

/// Mirrors the JS `restoreCase`, shared by the pipeline step and the
/// caps-escape stash in `resolve_syntax` (JS `convertAlphavet`).
///
/// Per word: identical words stay; a word equal to the lowercased original
/// takes the original (this preserves letters like Turkish `İ`, which a
/// plain `to_uppercase` would decompose to `I` + combining dot); otherwise
/// an uppercase-initial original fully uppercases a fully-uppercase
/// original word, else `initcap`s it.
pub(crate) fn restore_case_words(text: &mut [String], orig: &[String]) {
    for (i, word) in text.iter_mut().enumerate() {
        let o_word = match orig.get(i) {
            Some(o) => o,
            None => continue,
        };
        if word == o_word {
            continue;
        }
        if *word == o_word.to_lowercase() {
            *word = o_word.clone();
            continue;
        }
        if o_word.is_empty() || !is_upper_char(o_word.chars().next().unwrap()) {
            continue;
        }
        if word == "зь" {
            *word = if orig.get(i + 1).is_some_and(|n| is_upper_str(n)) {
                "ЗЬ".to_string()
            } else {
                "Зь".to_string()
            };
        } else {
            let last = o_word.chars().last().unwrap();
            if is_upper_char(last) {
                *word = word.to_uppercase();
            } else if word.starts_with('(') {
                *word = initcap_var(word);
            } else {
                *word = initcap(word);
            }
        }
    }
}
