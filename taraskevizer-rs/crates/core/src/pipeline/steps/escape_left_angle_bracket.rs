use crate::pipeline::{PipelineContext, TextWord};

pub fn step_escape_left_angle_bracket(ctx: &mut PipelineContext) {
    if ctx.cfg.left_angle_bracket == "<" {
        return;
    }
    let text_buf = &ctx.text;
    for slot in &mut ctx.text_arr {
        if slot.as_str(text_buf) == "<" {
            *slot = TextWord::Owned(ctx.cfg.left_angle_bracket.clone());
        }
    }
}
