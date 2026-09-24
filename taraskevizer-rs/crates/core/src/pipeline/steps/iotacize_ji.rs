use crate::{pipeline::PipelineContext, text::iotacize_ji};

pub fn step_iotacize_ji(ctx: &mut PipelineContext) {
    let text = std::mem::take(&mut ctx.text);
    ctx.text = iotacize_ji(&text);
}
