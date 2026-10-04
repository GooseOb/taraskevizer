use crate::{
    pipeline::PipelineContext,
    text::{apply_abc_lower, apply_abc_upper},
};

pub fn step_store_splitted_abc_converted_orig(ctx: &mut PipelineContext) {
    let lowered = apply_abc_lower(&ctx.text, ctx.cfg.abc);
    let converted = apply_abc_upper(&lowered, ctx.cfg.abc).into_owned();
    // One backing string; consumers split it lazily (borrowed `&str`, zero
    // per-word allocs) instead of materializing a `String` per word.
    ctx.orig_text = converted;
}
