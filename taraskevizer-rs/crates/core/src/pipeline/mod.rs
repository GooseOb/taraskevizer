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

/// Per-step timing record for the benchmark-only instrumented runner.
///
/// Production runners pass `timings: None`, which compiles down to a single
/// predictable branch per step with no allocation.
#[derive(Debug, Clone, Copy)]
pub struct StepTiming {
    pub name: &'static str,
    pub duration: std::time::Duration,
}

/// Evaluate `$call`, and when `$timings` is `Some`, record how long it took
/// under `$name`. Returns the value of `$call`, so both unit steps
/// (`timed!(t, "trim", { step_trim(&mut ctx); })`) and value-producing stages
/// (`timed!(t, "collapse", collapse_whitespaces(&src))`) work. With `None`
/// this expands to a plain `$call` — the check lives only in benchmark runs.
macro_rules! timed {
    ($timings:expr, $name:literal, $call:expr) => {{
        match $timings.as_mut() {
            Some(__t) => {
                let __start = ::std::time::Instant::now();
                let __v = $call;
                __t.push(StepTiming {
                    name: $name,
                    duration: __start.elapsed(),
                });
                __v
            }
            None => $call,
        }
    }};
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

/// Run the shared core of the tarask / phonetic pipelines, optionally recording
/// the duration of every step into `timings` (benchmark only).
fn run_base_pipeline_timed<F>(
    text: &str,
    cfg: &TaraskConfig,
    sub: F,
    mut timings: Option<&mut Vec<StepTiming>>,
) -> String
where
    F: FnOnce(&mut PipelineContext),
{
    let mut ctx = PipelineContext::new(text, cfg);
    timed!(timings, "trim", {
        step_trim(&mut ctx);
    });
    timed!(timings, "resolve_special_syntax", {
        step_resolve_special_syntax(&mut ctx);
    });
    timed!(timings, "prepare", {
        step_prepare(&mut ctx);
    });
    // See `run_alphabetic`: `ws_src` must outlive the borrowed `spaces`.
    let ws_src = std::mem::take(&mut ctx.text);
    let (collapsed, spaces) = timed!(
        timings,
        "whitespaces_to_spaces",
        collapse_whitespaces(&ws_src)
    );
    ctx.text = collapsed;
    timed!(timings, "store_splitted_abc_converted_orig", {
        step_store_splitted_abc_converted_orig(&mut ctx);
    });
    timed!(timings, "to_lower_case", {
        step_to_lower_case(&mut ctx);
    });
    // `sub` is the mode-specific step (taraskevize / phonetize+iotacize_ji).
    timed!(timings, "mode_sub", {
        sub(&mut ctx);
    });
    timed!(timings, "replace_i_by_j", {
        step_replace_i_by_j(&mut ctx);
    });
    timed!(timings, "convert_alphabet_lower", {
        step_convert_alphabet_lower(&mut ctx);
    });
    timed!(timings, "store_splitted_text", {
        step_store_splitted_text(&mut ctx);
    });
    timed!(timings, "restore_case", {
        step_restore_case(&mut ctx);
    });
    timed!(timings, "highlight_diff", {
        step_highlight_diff(&mut ctx);
    });
    timed!(timings, "escape_left_angle_bracket", {
        step_escape_left_angle_bracket(&mut ctx);
    });
    timed!(timings, "join_splitted_text", {
        step_join_splitted_text(&mut ctx);
    });
    let restored = timed!(
        timings,
        "restore_whitespaces",
        restore_whitespaces(&ctx.text, &spaces)
    );
    ctx.text = restored;
    timed!(timings, "apply_g", {
        step_apply_g(&mut ctx);
    });
    timed!(timings, "apply_variations", {
        step_apply_variations(&mut ctx);
    });
    timed!(timings, "unspace", {
        step_unspace(&mut ctx);
    });
    timed!(timings, "apply_no_fix", {
        step_apply_no_fix(&mut ctx);
    });
    timed!(timings, "finalize", {
        step_finalize(&mut ctx);
    });
    timed!(timings, "untrim", {
        step_untrim(&mut ctx);
    });
    ctx.text
}

/// Run the taraskevization pipeline.
pub fn run_tarask(text: &str, cfg: &TaraskConfig) -> String {
    run_base_pipeline_timed(text, cfg, step_taraskevize, None)
}

/// Benchmark-only instrumented counterpart of [`run_tarask`]: same behavior,
/// plus per-step timings pushed into `timings` when `Some`.
pub fn run_tarask_timed(
    text: &str,
    cfg: &TaraskConfig,
    timings: Option<&mut Vec<StepTiming>>,
) -> String {
    run_base_pipeline_timed(text, cfg, step_taraskevize, timings)
}

pub fn run_phonetic(text: &str, cfg: &TaraskConfig) -> String {
    run_base_pipeline_timed(
        text,
        cfg,
        |ctx| {
            step_phonetize(ctx);
            step_iotacize_ji(ctx);
        },
        None,
    )
}
