use crate::pipeline::PipelineContext;

pub fn step_trim(ctx: &mut PipelineContext) {
    let trimmed = ctx.text.trim();
    let before = &ctx.text[..ctx.text.len() - ctx.text.trim_start().len()];
    let after = &ctx.text[ctx.text.trim_end().len()..];
    ctx.trim_before = before.to_string();
    ctx.trim_after = after.to_string();
    ctx.text = format!(" {} ", trimmed);
}

pub fn step_untrim(ctx: &mut PipelineContext) {
    let trimmed = ctx.text.trim();
    ctx.text = format!("{}{}{}", ctx.trim_before, trimmed, ctx.trim_after);
}
