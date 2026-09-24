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
/// Replicating the string comparison keeps the Rust port faithful to the
/// reference `restoreCase` logic.
fn is_upper_str(s: &str) -> bool {
    s == &s.to_uppercase()
}

pub fn step_restore_case(ctx: &mut PipelineContext) {
    for i in 0..ctx.text_arr.len() {
        let word = &ctx.text_arr[i];
        let o_word = &ctx.orig_arr[i];
        if word == o_word {
            continue;
        }
        if word.to_lowercase() == o_word.to_lowercase() {
            ctx.text_arr[i] = o_word.clone();
            continue;
        }
        if o_word.is_empty() || !is_upper_str(&o_word.chars().next().unwrap().to_string()) {
            continue;
        }
        if word == "зь" {
            ctx.text_arr[i] = if i + 1 < ctx.orig_arr.len() && is_upper_str(&ctx.orig_arr[i + 1]) {
                "ЗЬ".to_string()
            } else {
                "Зь".to_string()
            };
        } else {
            let last = o_word.chars().last().unwrap();
            if is_upper_str(&last.to_string()) {
                ctx.text_arr[i] = word.to_uppercase();
            } else if word.starts_with('(') {
                ctx.text_arr[i] = initcap_var(word);
            } else {
                ctx.text_arr[i] = initcap(word);
            }
        }
    }
}
