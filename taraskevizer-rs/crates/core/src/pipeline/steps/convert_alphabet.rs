use crate::pipeline::{
    helpers::{apply_abc_lower, apply_abc_upper},
    PipelineContext,
};

pub fn step_convert_alphabet(ctx: &mut PipelineContext) {
    let lowered = apply_abc_lower(&ctx.text, ctx.cfg.abc);
    ctx.text = apply_abc_upper(&lowered, ctx.cfg.abc).unwrap_or(lowered);
}

pub fn step_convert_alphabet_lower(ctx: &mut PipelineContext) {
    ctx.text = apply_abc_lower(&ctx.text, ctx.cfg.abc);
}
