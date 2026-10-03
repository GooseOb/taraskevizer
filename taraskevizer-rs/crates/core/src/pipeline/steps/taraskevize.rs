use crate::{
    pipeline::{helpers::soften, PipelineContext, WORD_LIST},
    text::end_z_soften_and_nia_biaz,
};

pub fn step_taraskevize(ctx: &mut PipelineContext) {
    // Borrowed when the 100+ batch dict is a no-op (no per-batch clones).
    let dict = WORD_LIST.replace_all_cow(&ctx.text);
    let mut text = soften(dict.as_ref());
    text = end_z_soften_and_nia_biaz(&text);
    ctx.text = text;
}
