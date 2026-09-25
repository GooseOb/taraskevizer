use crate::{
    config::{Alphabet, JMode},
    pipeline::PipelineContext,
    text::replace_i_by_j,
};

pub fn step_replace_i_by_j(ctx: &mut PipelineContext) {
    if ctx.cfg.j == JMode::Never || ctx.cfg.abc == Alphabet::LatinJi {
        return;
    }
    ctx.text = replace_i_by_j(&ctx.text, ctx.cfg.j == JMode::Always);
}
