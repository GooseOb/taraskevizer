use crate::{
    pipeline::PipelineContext,
    text::{
        normalize_apostrophes, replace_g_apostrophe, space_out_punct_sym_digits,
        unspace_punct_sym_digits,
    },
};

pub fn step_prepare(ctx: &mut PipelineContext) {
    let mut t = replace_g_apostrophe(&ctx.text);
    // `str::replace` (and the `format!` below) allocate unconditionally; skip
    // when the bracket is absent. With the default `<` this still fires only
    // on real tags.
    if t.contains(ctx.cfg.left_angle_bracket.as_str()) {
        t = t.replace(
            &ctx.cfg.left_angle_bracket,
            &format!(" {} ", ctx.cfg.left_angle_bracket),
        );
    }
    t = normalize_apostrophes(&t);
    t = space_out_punct_sym_digits(&t);
    ctx.text = t;
}

pub fn step_unspace(ctx: &mut PipelineContext) {
    let mut t = unspace_punct_sym_digits(&ctx.text);
    // Mirror of `step_prepare`: skip the full copy when absent. The cheap
    // `contains` pre-check avoids even the small `format!` for tag-free text.
    if t.contains(ctx.cfg.left_angle_bracket.as_str()) {
        let spaced = format!(" {} ", ctx.cfg.left_angle_bracket);
        if t.contains(spaced.as_str()) {
            t = t.replace(&spaced, ctx.cfg.left_angle_bracket.as_str());
        }
    }
    ctx.text = t;
}
