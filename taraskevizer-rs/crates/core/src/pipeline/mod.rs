mod helpers;
mod steps;

pub use self::helpers::apply_highlight_diff;
pub use self::steps::*;

use crate::{
    config::TaraskConfig,
    dict::{CompiledDict, DictEntry},
};

fn build_dict_matcher(json: &str) -> CompiledDict {
    let entries: Vec<DictEntry> = serde_json::from_str(json).unwrap();
    CompiledDict::new(&entries)
}

static WORD_LIST: std::sync::LazyLock<CompiledDict> =
    std::sync::LazyLock::new(|| build_dict_matcher(include_str!("../dict/data/wordlist.json")));
static SOFTEN: std::sync::LazyLock<CompiledDict> =
    std::sync::LazyLock::new(|| build_dict_matcher(include_str!("../dict/data/soften.json")));
static PHONETIC: std::sync::LazyLock<CompiledDict> =
    std::sync::LazyLock::new(|| build_dict_matcher(include_str!("../dict/data/phonetic.json")));

pub struct PipelineContext<'a> {
    pub text: String,
    pub cfg: &'a TaraskConfig,
    pub trim_before: String,
    pub trim_after: String,
    pub text_arr: Vec<String>,
    pub orig_arr: Vec<String>,
    pub no_fix_arr: Vec<String>,
}

impl<'a> PipelineContext<'a> {
    pub fn new(text: &str, cfg: &'a TaraskConfig) -> Self {
        Self {
            text: text.to_string(),
            cfg,
            trim_before: String::new(),
            trim_after: String::new(),
            text_arr: Vec::new(),
            orig_arr: Vec::new(),
            no_fix_arr: Vec::new(),
        }
    }
}

pub fn run_alphabetic(text: &str, cfg: &TaraskConfig) -> String {
    let mut ctx = PipelineContext::new(text, cfg);
    step_trim(&mut ctx);
    step_resolve_special_syntax(&mut ctx);
    step_prepare(&mut ctx);
    // Borrowed run slices: `ws_src` owns the pre-collapse text and must
    // outlive `spaces`, so it is moved out of `ctx` here (`take` is required
    // for the move, unlike the plain `&ctx.text` steps) and dropped only at
    // the end of this scope, after restore.
    let ws_src = std::mem::take(&mut ctx.text);
    let (collapsed, spaces) = collapse_whitespaces(&ws_src);
    ctx.text = collapsed;
    step_convert_alphabet(&mut ctx);
    ctx.text = restore_whitespaces(&ctx.text, &spaces);
    step_unspace(&mut ctx);
    step_apply_no_fix(&mut ctx);
    step_finalize(&mut ctx);
    step_untrim(&mut ctx);
    ctx.text
}

/// Run the shared core of the tarask / phonetic pipelines.
fn run_base_pipeline<F>(text: &str, cfg: &TaraskConfig, sub: F) -> String
where
    F: FnOnce(&mut PipelineContext),
{
    let mut ctx = PipelineContext::new(text, cfg);
    step_trim(&mut ctx);
    step_resolve_special_syntax(&mut ctx);
    step_prepare(&mut ctx);
    // See `run_alphabetic`: `ws_src` must outlive the borrowed `spaces`.
    let ws_src = std::mem::take(&mut ctx.text);
    let (collapsed, spaces) = collapse_whitespaces(&ws_src);
    ctx.text = collapsed;
    step_store_splitted_abc_converted_orig(&mut ctx);
    step_to_lower_case(&mut ctx);
    // `sub` is the mode-specific step (taraskevize / phonetize+iotacize_ji).
    sub(&mut ctx);
    step_replace_i_by_j(&mut ctx);
    step_convert_alphabet_lower(&mut ctx);
    step_store_splitted_text(&mut ctx);
    step_restore_case(&mut ctx);
    step_highlight_diff(&mut ctx);
    step_escape_left_angle_bracket(&mut ctx);
    step_join_splitted_text(&mut ctx);
    ctx.text = restore_whitespaces(&ctx.text, &spaces);
    step_apply_g(&mut ctx);
    step_apply_variations(&mut ctx);
    step_unspace(&mut ctx);
    step_apply_no_fix(&mut ctx);
    step_finalize(&mut ctx);
    step_untrim(&mut ctx);
    ctx.text
}

/// Run the taraskevization pipeline.
pub fn run_tarask(text: &str, cfg: &TaraskConfig) -> String {
    run_base_pipeline(text, cfg, step_taraskevize)
}

pub fn run_phonetic(text: &str, cfg: &TaraskConfig) -> String {
    run_base_pipeline(text, cfg, |ctx| {
        step_phonetize(ctx);
        step_iotacize_ji(ctx);
    })
}
