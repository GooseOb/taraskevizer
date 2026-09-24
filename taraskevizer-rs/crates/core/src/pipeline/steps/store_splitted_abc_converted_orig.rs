use crate::pipeline::{
    helpers::{alphabet_dict, apply_alphabet},
    PipelineContext,
};

pub fn step_store_splitted_abc_converted_orig(ctx: &mut PipelineContext) {
    let lowered = apply_alphabet(&ctx.text, ctx.cfg.abc, "lower");
    let upper_dict = alphabet_dict(ctx.cfg.abc, "upper");
    let converted = if upper_dict.has_entries() {
        upper_dict.replace_all(&lowered)
    } else {
        lowered
    };
    ctx.orig_arr = converted.split(' ').map(|s| s.to_string()).collect();
}
