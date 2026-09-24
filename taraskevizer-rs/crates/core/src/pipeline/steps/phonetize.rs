use crate::pipeline::{
    helpers::{end_z_soften_and_nia_biaz, soften},
    PipelineContext, IA_WORDS, PHONETIC,
};

pub fn step_phonetize(ctx: &mut PipelineContext) {
    let mut text = std::mem::take(&mut ctx.text);
    text = soften(&text);
    text = IA_WORDS.replace_all(&text);
    text = PHONETIC.replace_all(&text);
    text = end_z_soften_and_nia_biaz(&text);
    ctx.text = text;
}
