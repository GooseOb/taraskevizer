use crate::{
    config::Alphabet,
    pipeline::{
        helpers::{replace_g_str, replace_g_with_map},
        PipelineContext,
    },
};

pub fn step_apply_g(ctx: &mut PipelineContext) {
    if ctx.cfg.abc != Alphabet::Cyrillic {
        return;
    }
    let wrap = ctx.cfg.wrappers.as_ref().and_then(|w| w.letter_h);
    if let Some(wrap) = wrap {
        ctx.text = replace_g_with_map(&ctx.text, |ch| {
            wrap(if ctx.cfg.g {
                ch
            } else {
                match ch {
                    'Ґ' => 'Г',
                    'ґ' => 'г',
                    _ => ch,
                }
            })
        });
    } else if !ctx.cfg.g {
        ctx.text = replace_g_str(&ctx.text)
    }
}
