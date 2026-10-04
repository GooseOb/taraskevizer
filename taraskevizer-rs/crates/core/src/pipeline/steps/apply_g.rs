use std::borrow::Cow;

use crate::{
    config::Alphabet,
    pipeline::PipelineContext,
    text::{replace_g_str, replace_g_with_map},
};

pub fn step_apply_g(ctx: &mut PipelineContext) {
    if ctx.cfg.abc != Alphabet::Cyrillic {
        return;
    }
    let wrap = ctx.cfg.wrappers.as_ref().and_then(|w| w.letter_h);
    if let Some(wrap) = wrap {
        // Like `replace_g_str` below: skip rewriting when nothing matched.
        if let Cow::Owned(mapped) = replace_g_with_map(&ctx.text, |ch| {
            wrap(if ctx.cfg.g {
                ch
            } else {
                match ch {
                    'Ґ' => 'Г',
                    'ґ' => 'г',
                    _ => ch,
                }
            })
        }) {
            ctx.text = mapped;
        }
    } else if !ctx.cfg.g {
        // `replace_g_str` borrows when there is nothing to map: only
        // overwrite the backing string when a `ґ` was actually mapped.
        if let Cow::Owned(mapped) = replace_g_str(&ctx.text) {
            ctx.text = mapped;
        }
    }
}
