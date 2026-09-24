use crate::pipeline::{
    helpers::{alphabet_dict, apply_alphabet},
    PipelineContext,
};

pub fn step_convert_alphabet(ctx: &mut PipelineContext) {
    let lowered = apply_alphabet(&ctx.text, ctx.cfg.abc, "lower");
    let upper_dict = alphabet_dict(ctx.cfg.abc, "upper");
    let result = if upper_dict.has_entries() {
        upper_dict.replace_all(&lowered)
    } else {
        lowered
    };
    ctx.text = result;
}

pub fn step_convert_alphabet_lower(ctx: &mut PipelineContext) {
    ctx.text = apply_alphabet(&ctx.text, ctx.cfg.abc, "lower");
}
