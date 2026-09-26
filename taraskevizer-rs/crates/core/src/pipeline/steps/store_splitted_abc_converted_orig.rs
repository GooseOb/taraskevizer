use crate::pipeline::{
    helpers::{apply_abc_lower, apply_abc_upper},
    PipelineContext,
};

pub fn step_store_splitted_abc_converted_orig(ctx: &mut PipelineContext) {
    let lowered = apply_abc_lower(&ctx.text, ctx.cfg.abc);
    let converted = apply_abc_upper(&lowered, ctx.cfg.abc).unwrap_or_else(|| lowered.into_owned());
    ctx.orig_arr = converted.split(' ').map(|s| s.to_string()).collect();
}
