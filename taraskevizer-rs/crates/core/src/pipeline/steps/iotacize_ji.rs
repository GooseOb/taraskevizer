use crate::{pipeline::PipelineContext, text::iotacize_ji};

pub fn step_iotacize_ji(ctx: &mut PipelineContext) {
    ctx.text = iotacize_ji(&ctx.text);
}
