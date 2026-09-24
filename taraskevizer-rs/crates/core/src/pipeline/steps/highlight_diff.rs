use crate::config::Alphabet;
use crate::pipeline::helpers::{highlight_diff_word, replace_g_str};
use crate::pipeline::PipelineContext;

pub fn step_highlight_diff(ctx: &mut PipelineContext) {
    let fix = ctx.cfg.wrappers.as_ref().and_then(|w| w.fix);
    let fix = match fix {
        Some(f) => f,
        None => return,
    };
    let is_cyrillic = ctx.cfg.abc == Alphabet::Cyrillic;

    for i in 0..ctx.text_arr.len() {
        let word = ctx.text_arr[i].clone();
        let o_word = &ctx.orig_arr[i];
        if o_word == &word {
            continue;
        }
        let word_h = if is_cyrillic {
            replace_g_str(&word)
        } else {
            word.clone()
        };
        if o_word == &word_h {
            continue;
        }
        ctx.text_arr[i] = highlight_diff_word(&word, o_word, &word_h, is_cyrillic, &fix);
    }
}
