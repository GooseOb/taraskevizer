use crate::{
    pipeline::PipelineContext,
    text::{
        normalize_apostrophes, replace_g_apostrophe, space_out_punct_sym_digits,
        unspace_punct_sym_digits,
    },
};

pub fn step_prepare(ctx: &mut PipelineContext) {
    let mut t = replace_g_apostrophe(&ctx.text);
    t = t.replace(
        &ctx.cfg.left_angle_bracket,
        &format!(" {} ", ctx.cfg.left_angle_bracket),
    );
    t = normalize_apostrophes(&t);
    t = space_out_punct_sym_digits(&t);
    ctx.text = t;
}

pub fn step_unspace(ctx: &mut PipelineContext) {
    let mut t = unspace_punct_sym_digits(&ctx.text);
    t = t.replace(
        &format!(" {} ", ctx.cfg.left_angle_bracket),
        ctx.cfg.left_angle_bracket.as_str(),
    );
    ctx.text = t
}
