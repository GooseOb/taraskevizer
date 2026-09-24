use crate::pipeline::PipelineContext;

pub fn step_escape_left_angle_bracket(ctx: &mut PipelineContext) {
    if ctx.cfg.left_angle_bracket == "<" {
        return;
    }
    for word in &mut ctx.text_arr {
        if word == "<" {
            *word = ctx.cfg.left_angle_bracket.clone();
        }
    }
}
