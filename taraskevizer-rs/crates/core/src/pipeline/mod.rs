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
static IA_WORDS: std::sync::LazyLock<CompiledDict> =
    std::sync::LazyLock::new(|| build_dict_matcher(include_str!("../dict/data/iawords.json")));
static ALPHABETS: std::sync::LazyLock<serde_json::Value> = std::sync::LazyLock::new(|| {
    serde_json::from_str(include_str!("../dict/data/alphabets.json")).unwrap()
});

pub struct PipelineContext<'a> {
    pub text: String,
    pub cfg: &'a TaraskConfig,
    pub trim_before: String,
    pub trim_after: String,
    pub spaces: Vec<String>,
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
            spaces: Vec::new(),
            text_arr: Vec::new(),
            orig_arr: Vec::new(),
            no_fix_arr: Vec::new(),
        }
    }
}

/// Per-step timing record produced by the instrumented pipeline runners.
///
/// Each entry corresponds to one pipeline step and holds the wall-clock
/// duration that step took to process its input.
#[derive(Debug, Clone, Copy)]
pub struct StepTiming {
    pub name: &'static str,
    pub duration: std::time::Duration,
}

/// Wrap a single pipeline step with optional timing collection.
///
/// When `timings` is `Some`, records the wall-clock duration of `$call` under
/// the given step `$name`. When `None`, this expands to a plain `$call` with no
/// measurable overhead, so the non-instrumented runners stay allocation- and
/// branch-free on the hot path.
macro_rules! timed_step {
    ($timings:expr, $name:literal, $call:expr) => {{
        match $timings.as_mut() {
            Some(__t) => {
                let __start = ::std::time::Instant::now();
                $call;
                __t.push(StepTiming {
                    name: $name,
                    duration: __start.elapsed(),
                });
            }
            None => {
                $call;
            }
        }
    }};
}

pub fn run_alphabetic(text: &str, cfg: &TaraskConfig) -> String {
    let mut ctx = PipelineContext::new(text, cfg);
    step_trim(&mut ctx);
    step_resolve_special_syntax(&mut ctx);
    step_prepare(&mut ctx);
    step_whitespaces_to_spaces(&mut ctx);
    step_convert_alphabet(&mut ctx);
    step_restore_whitespaces(&mut ctx);
    step_apply_no_fix(&mut ctx);
    step_finalize(&mut ctx);
    step_untrim(&mut ctx);
    ctx.text
}

/// Run the shared core of the tarask / phonetic pipelines, optionally recording
/// the duration of every step into `timings`.
///
/// Pass `timings: None` for a hot path with no measurable overhead.
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
    timed_step!(timings, "trim", step_trim(&mut ctx));
    timed_step!(
        timings,
        "resolve_special_syntax",
        step_resolve_special_syntax(&mut ctx)
    );
    timed_step!(timings, "prepare", step_prepare(&mut ctx));
    timed_step!(
        timings,
        "whitespaces_to_spaces",
        step_whitespaces_to_spaces(&mut ctx)
    );
    timed_step!(
        timings,
        "store_splitted_abc_converted_orig",
        step_store_splitted_abc_converted_orig(&mut ctx)
    );
    timed_step!(timings, "to_lower_case", step_to_lower_case(&mut ctx));
    // `sub` is the mode-specific step (taraskevize / phonetize+iotacize_ji).
    timed_step!(timings, "mode_sub", sub(&mut ctx));
    timed_step!(timings, "replace_i_by_j", step_replace_i_by_j(&mut ctx));
    timed_step!(
        timings,
        "convert_alphabet_lower",
        step_convert_alphabet_lower(&mut ctx)
    );
    timed_step!(
        timings,
        "store_splitted_text",
        step_store_splitted_text(&mut ctx)
    );
    timed_step!(timings, "restore_case", step_restore_case(&mut ctx));
    timed_step!(timings, "highlight_diff", step_highlight_diff(&mut ctx));
    timed_step!(
        timings,
        "escape_left_angle_bracket",
        step_escape_left_angle_bracket(&mut ctx)
    );
    timed_step!(
        timings,
        "join_splitted_text",
        step_join_splitted_text(&mut ctx)
    );
    timed_step!(
        timings,
        "restore_whitespaces",
        step_restore_whitespaces(&mut ctx)
    );
    timed_step!(timings, "apply_g", step_apply_g(&mut ctx));
    timed_step!(timings, "apply_variations", step_apply_variations(&mut ctx));
    timed_step!(timings, "apply_no_fix", step_apply_no_fix(&mut ctx));
    timed_step!(timings, "finalize", step_finalize(&mut ctx));
    timed_step!(timings, "untrim", step_untrim(&mut ctx));
    ctx.text
}

/// Run the taraskevization pipeline.
///
/// To also capture per-step timings, use [`run_tarask_timed`].
pub fn run_tarask(text: &str, cfg: &TaraskConfig) -> String {
    run_tarask_timed(text, cfg, None)
}

/// Run the taraskevization pipeline, optionally recording the duration of every
/// step into `timings`.
///
/// This is the instrumented counterpart of [`run_tarask`]; pass `None` to get
/// the exact same behavior and performance as `run_tarask`. The step timings
/// cover only the tarask pipeline (i.e. with `step_taraskevize` as the
/// mode-specific sub-step), not the phonetic or alphabetic pipelines.
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
