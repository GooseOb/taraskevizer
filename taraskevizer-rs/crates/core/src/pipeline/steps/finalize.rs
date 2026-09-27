use crate::pipeline::PipelineContext;

pub fn step_finalize(ctx: &mut PipelineContext) {
    let mut t = ctx.text.replace("&nbsp;", " ");
    if ctx.cfg.new_line != "\n" {
        t = t.replace('\n', &ctx.cfg.new_line);
    }
    ctx.text = t.trim().to_string();
}
