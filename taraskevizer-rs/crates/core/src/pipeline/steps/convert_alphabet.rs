use crate::pipeline::{
    helpers::{apply_abc_lower, apply_abc_upper},
    PipelineContext,
};

pub fn step_convert_alphabet(ctx: &mut PipelineContext) {
    // Cyrillic lower is a borrow and upper is `None`, so the fallback below
    // would be a full self-copy (`ctx.text = ctx.text.clone()`). Skip it.
    if ctx.cfg.abc == crate::config::Alphabet::Cyrillic {
        return;
    }
    let lowered = apply_abc_lower(&ctx.text, ctx.cfg.abc);
    ctx.text = apply_abc_upper(&lowered, ctx.cfg.abc).into_owned();
}

pub fn step_convert_alphabet_lower(ctx: &mut PipelineContext) {
    // Cyrillic lower conversion borrows (identity); the text is already
    // lowercase from `step_to_lower_case`, so cloning here is pure waste.
    if ctx.cfg.abc == crate::config::Alphabet::Cyrillic {
        return;
    }
    ctx.text = apply_abc_lower(&ctx.text, ctx.cfg.abc).into_owned();
}
