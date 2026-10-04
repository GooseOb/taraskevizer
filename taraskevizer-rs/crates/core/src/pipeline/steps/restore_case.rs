use crate::{
    pipeline::{PipelineContext, TextWord},
    text::{initcap, initcap_var},
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
/// Streaming per-char comparison: `str::to_uppercase` is defined as
/// `chars().flat_map(char::to_uppercase)`, so this matches the old
/// `s == s.to_uppercase()` exactly with no allocation.
fn is_upper_str(s: &str) -> bool {
    s.chars().all(is_upper_char)
}
fn is_upper_char(c: char) -> bool {
    c.to_uppercase().eq(std::iter::once(c))
}

/// `word == o_word.to_lowercase()` without allocating the lowercased copy:
/// `str::to_lowercase` is `chars().flat_map(char::to_lowercase)`, so a
/// streaming comparison is exact and short-circuits on the first mismatch.
fn eq_lower(word: &str, o_word: &str) -> bool {
    let mut w = word.chars();
    for oc in o_word.chars() {
        for lc in oc.to_lowercase() {
            if w.next() != Some(lc) {
                return false;
            }
        }
    }
    w.next().is_none()
}

pub fn step_restore_case(ctx: &mut PipelineContext) {
    restore_case_words(&mut ctx.text_arr, &ctx.text, &ctx.orig_text);
}

/// Mirrors the JS `restoreCase`, shared by the pipeline step and the
/// caps-escape stash in `resolve_syntax` (JS `convertAlphavet`).
///
/// Per word: identical words stay; a word equal to the lowercased original
/// takes the original (this preserves letters like Turkish `İ`, which a
/// plain `to_uppercase` would decompose to `I` + combining dot); otherwise
/// an uppercase-initial original fully uppercases a fully-uppercase
/// original word, else `initcap`s it.
///
/// `orig_text` is split lazily in lockstep with `text` (same word count by
/// construction): the original words borrow instead of costing one `String`
/// alloc each. `text` holds copy-on-write words over `text_buf`; only
/// actually changed words become owned.
pub(crate) fn restore_case_words(text: &mut [TextWord], text_buf: &str, orig_text: &str) {
    let mut orig_it = orig_text.split(' ').peekable();
    for slot in text.iter_mut() {
        // Fused iterator: stays `None`; remaining words are unchanged,
        // exactly like the old `orig.get(i) → None → continue`.
        let Some(o_word) = orig_it.next() else { break };
        let word = slot.as_str(text_buf);
        if word == o_word {
            continue;
        }
        if eq_lower(word, o_word) {
            *slot = TextWord::Owned(o_word.to_string());
            continue;
        }
        if o_word.is_empty() || !is_upper_char(o_word.chars().next().unwrap()) {
            continue;
        }
        if word == "зь" {
            *slot = TextWord::Owned(if orig_it.peek().is_some_and(|n| is_upper_str(n)) {
                "ЗЬ".to_string()
            } else {
                "Зь".to_string()
            });
        } else {
            // Compute the replacement first (borrows `slot`); assign after,
            // so no clone of unchanged words is ever needed.
            let new: String = {
                let w = slot.as_str(text_buf);
                let last = o_word.chars().last().unwrap();
                if is_upper_char(last) {
                    w.to_uppercase()
                } else if w.starts_with('(') {
                    initcap_var(w)
                } else {
                    initcap(w)
                }
            };
            *slot = TextWord::Owned(new);
        }
    }
}
