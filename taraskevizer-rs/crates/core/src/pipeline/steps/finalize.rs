use crate::pipeline::PipelineContext;

pub fn step_finalize(ctx: &mut PipelineContext) {
    // `str::replace` allocates unconditionally; skip the full copy when the
    // pattern is absent (the common case).
    if ctx.text.contains("&nbsp;") {
        ctx.text = ctx.text.replace("&nbsp;", " ");
    }
    if ctx.cfg.new_line != "\n" {
        ctx.text = ctx.text.replace('\n', &ctx.cfg.new_line);
    }
    // Trim in place (no allocation): `drain`/`truncate` memmove instead of
    // cloning. This trims the `" {} "` padding added by `step_trim`.
    // (`tlen == 0` covers all-whitespace, where `end < start` would
    // underflow the naive `end - start`.)
    let tlen = ctx.text.trim().len();
    if tlen == 0 {
        ctx.text.clear();
        return;
    }
    let start = ctx.text.len() - ctx.text.trim_start().len();
    if start > 0 {
        ctx.text.drain(..start);
    }
    ctx.text.truncate(tlen);
}
