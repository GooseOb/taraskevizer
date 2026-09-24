use crate::pipeline::PipelineContext;

pub fn step_to_lower_case(ctx: &mut PipelineContext) {
    ctx.text = ctx.text.to_lowercase();
}
