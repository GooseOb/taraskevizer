use crate::pipeline::{
    helpers::{end_z_soften_and_nia_biaz, soften},
    PipelineContext, WORD_LIST,
};

pub fn step_taraskevize(ctx: &mut PipelineContext) {
    let mut text = std::mem::take(&mut ctx.text);
    text = WORD_LIST.replace_all(&text);
    text = soften(&text);
    text = end_z_soften_and_nia_biaz(&text);
    ctx.text = text;
}
