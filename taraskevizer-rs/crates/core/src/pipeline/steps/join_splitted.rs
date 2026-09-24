use crate::pipeline::PipelineContext;

pub fn step_join_splitted_text(ctx: &mut PipelineContext) {
    ctx.text = ctx.text_arr.join(" ");
}
