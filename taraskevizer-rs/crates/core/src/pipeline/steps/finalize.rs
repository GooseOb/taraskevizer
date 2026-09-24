use crate::pipeline::PipelineContext;

pub fn step_finalize(ctx: &mut PipelineContext) {
    let text = std::mem::take(&mut ctx.text);
    let mut t = text.replace("&nbsp;", " ");
    if ctx.cfg.new_line != "\n" {
        t = t.replace('\n', &ctx.cfg.new_line);
    }
    ctx.text = t.trim().to_string();
}
