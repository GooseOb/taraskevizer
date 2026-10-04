use std::borrow::Cow;

use crate::config::Alphabet;
use crate::text::{highlight_diff_word, replace_g_str};
use crate::pipeline::{PipelineContext, TextWord};

pub fn step_highlight_diff(ctx: &mut PipelineContext) {
    let fix = ctx.cfg.wrappers.as_ref().and_then(|w| w.fix);
    let Some(fix) = fix else { return };
    let is_cyrillic = ctx.cfg.abc == Alphabet::Cyrillic;

    // `orig_text` splits lazily in lockstep (same word count by construction);
    // disjoint field borrows keep this allocation-free apart from real diffs.
    // Spans resolve against `ctx.text`, which is stable until the join step.
    let mut orig_it = ctx.orig_text.split(' ');
    let text_buf = &ctx.text;
    for slot in &mut ctx.text_arr {
        let Some(o_word) = orig_it.next() else { break };
        let word = slot.as_str(text_buf);
        if o_word == word {
            continue;
        }
        // Changed word: materialize before mutating (borrow ends at `new`).
        let new: String = {
            let w = slot.as_str(text_buf);
            let word_h: Cow<str> = if is_cyrillic {
                replace_g_str(w)
            } else {
                Cow::Borrowed(w)
            };
            let word_h: &str = &word_h;
            if o_word == word_h {
                continue;
            }
            highlight_diff_word(w, o_word, word_h, &fix)
        };
        *slot = TextWord::Owned(new);
    }
}
