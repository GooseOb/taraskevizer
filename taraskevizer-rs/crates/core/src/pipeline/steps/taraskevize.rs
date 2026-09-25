use crate::{
    pipeline::{helpers::soften, PipelineContext, WORD_LIST},
    text::end_z_soften_and_nia_biaz,
};

pub fn step_taraskevize(ctx: &mut PipelineContext) {
    let mut text = WORD_LIST.replace_all(&ctx.text);
    text = soften(&text);
    text = end_z_soften_and_nia_biaz(&text);
    ctx.text = text;
}
