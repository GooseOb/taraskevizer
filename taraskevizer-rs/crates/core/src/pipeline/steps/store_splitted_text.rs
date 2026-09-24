use crate::pipeline::PipelineContext;

pub fn step_store_splitted_text(ctx: &mut PipelineContext) {
    ctx.text_arr = ctx.text.split(' ').map(|s| s.to_string()).collect();
    if ctx.text_arr.len() != ctx.orig_arr.len() {
        panic!(
            "Word count mismatch: text={}, orig={}",
            ctx.text_arr.len(),
            ctx.orig_arr.len()
        );
    }
}
