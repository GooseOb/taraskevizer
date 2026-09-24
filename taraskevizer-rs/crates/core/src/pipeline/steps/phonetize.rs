use crate::{
    pipeline::{helpers::soften, PipelineContext, PHONETIC},
    text::{end_z_soften_and_nia_biaz, ia_words},
};

pub fn step_phonetize(ctx: &mut PipelineContext) {
    let mut text = std::mem::take(&mut ctx.text);
    text = soften(&text);
    text = ia_words(&text);
    text = PHONETIC.replace_all(&text);
    text = end_z_soften_and_nia_biaz(&text);
    ctx.text = text;
}
