pub mod config;
pub mod dict;
pub mod pipeline;
pub mod wrappers;

pub use config::TaraskConfig;
pub use pipeline::{
    apply_highlight_diff, run_alphabetic, run_phonetic, run_tarask, run_tarask_timed, StepTiming,
};
pub use wrappers::{html_config_options, ANSI_COLOR_WRAPPERS, HTML_WRAPPERS};

/// Run the full taraskevization pipeline.
pub fn tarask(text: &str, cfg: &TaraskConfig) -> String {
    run_tarask(text, cfg)
}

/// Run the full taraskevization pipeline, optionally recording per-step timings.
pub fn tarask_timed(
    text: &str,
    cfg: &TaraskConfig,
    timings: Option<&mut Vec<StepTiming>>,
) -> String {
    run_tarask_timed(text, cfg, timings)
}

/// Run alphabet-only pipeline.
pub fn alphabetic(text: &str, cfg: &TaraskConfig) -> String {
    run_alphabetic(text, cfg)
}

/// Run phonetic pipeline.
pub fn phonetic(text: &str, cfg: &TaraskConfig) -> String {
    run_phonetic(text, cfg)
}
