use crate::{
    pipeline::{helpers::soften, PipelineContext, PHONETIC},
    text::{end_z_soften_and_nia_biaz, ia_words},
};

pub fn step_phonetize(ctx: &mut PipelineContext) {
    let mut text = soften(&ctx.text);
    text = ia_words(&text);
    // Borrowed when the phonetic dict is a no-op (no per-batch clones).
    let dict = PHONETIC.replace_all_cow(&text);
    text = end_z_soften_and_nia_biaz(dict.as_ref());
    ctx.text = text;
}
