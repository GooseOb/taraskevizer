use crate::pipeline::{split_text_words, PipelineContext};

pub fn step_store_splitted_text(ctx: &mut PipelineContext) {
    ctx.text_arr = split_text_words(&ctx.text);
    // `orig_text` is split lazily by consumers; count (allocation-free scan)
    // only for the invariant check.
    let orig_len = ctx.orig_text.split(' ').count();
    assert!(
        ctx.text_arr.len() == orig_len,
        "Word count mismatch: text={}, orig={}",
        ctx.text_arr.len(),
        orig_len
    );
}
