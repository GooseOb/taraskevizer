use crate::pipeline::{join_text_words, PipelineContext};

pub fn step_join_splitted_text(ctx: &mut PipelineContext) {
    ctx.text = join_text_words(&ctx.text_arr, &ctx.text);
}
