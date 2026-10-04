mod steps;

pub use self::steps::*;
pub use crate::text::apply_highlight_diff;

use crate::{
    config::TaraskConfig,
    dict::{CompiledDict, PHONETIC as PHONETIC_ENTRIES, WORD_LIST as WORD_LIST_ENTRIES},
};

fn build_batched_dict_matcher(batches: &[&[(&'static str, &'static str)]]) -> CompiledDict {
    CompiledDict::new_batched_str(batches)
}

static WORD_LIST: std::sync::LazyLock<CompiledDict> =
    std::sync::LazyLock::new(|| build_batched_dict_matcher(WORD_LIST_ENTRIES));
static PHONETIC: std::sync::LazyLock<CompiledDict> =
    std::sync::LazyLock::new(|| build_batched_dict_matcher(PHONETIC_ENTRIES));

/// One working word: either a byte range into the backing buffer (no
/// allocation) or an owned replacement for words changed by
/// `restore_case` / `highlight_diff` / `escape` (copy-on-write).
///
/// Spans borrow `PipelineContext::text`, which is stable from
/// `step_store_splitted_text` until `step_join_splitted_text` overwrites it.
/// Stored as offsets (not `&str`) so the struct stays non-self-referential.
#[derive(Debug, Clone)]
pub enum TextWord {
    Span(u32, u32),
    Owned(String),
}

impl TextWord {
    /// Resolve to `&str` via the backing buffer (`buf` is `ctx.text`, or the
    /// local `conv`/`word` string at the caps-escape call site).
    pub fn as_str<'x>(&'x self, buf: &'x str) -> &'x str {
        match self {
            TextWord::Span(s, e) => &buf[*s as usize..*e as usize],
            TextWord::Owned(o) => o.as_str(),
        }
    }
}

/// `words.join(" ")` over resolved spans (single exact-sized allocation).
pub fn join_text_words(words: &[TextWord], buf: &str) -> String {
    let mut len = 0usize;
    for (i, w) in words.iter().enumerate() {
        if i > 0 {
            len += 1;
        }
        len += w.as_str(buf).len();
    }
    let mut out = String::with_capacity(len);
    for (i, w) in words.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(w.as_str(buf));
    }
    out
}

/// Split `buf` on `' '` into span words (one `Vec` allocation, no per-word
/// `String`s). Mirrors `buf.split(' ')` exactly, including leading/trailing
/// empty parts from padding spaces.
///
/// Byte offsets are stored as `u32`: pipeline inputs are chunked to ≤1 MiB
/// (and multi-gigabyte direct inputs are out of scope), so the `as u32`
/// casts below cannot truncate in practice; the `debug_assert`s pin that.
#[allow(clippy::cast_possible_truncation)]
pub fn split_text_words(buf: &str) -> Vec<TextWord> {
    // Exact reserve via SIMD space count: `split(' ')` yields spaces + 1.
    let n = memchr::memchr_iter(b' ', buf.as_bytes()).count() + 1;
    let mut out = Vec::with_capacity(n);
    let mut start = 0u32;
    for (i, _) in buf.match_indices(' ') {
        debug_assert!(u32::try_from(i).is_ok());
        out.push(TextWord::Span(start, i as u32));
        start = i as u32 + 1;
    }
    debug_assert!(u32::try_from(buf.len()).is_ok());
    out.push(TextWord::Span(start, buf.len() as u32));
    out
}

pub struct PipelineContext<'a> {
    pub text: String,
    pub cfg: &'a TaraskConfig,
    pub trim_before: String,
    pub trim_after: String,
    pub text_arr: Vec<TextWord>,
    /// ABC-converted original text; word-split lazily (`split(' ')` borrows,
    /// no per-word allocation) by `step_restore_case` / `step_highlight_diff`.
    /// Replaces the old `orig_arr: Vec<String>` (one `String` alloc per word).
    pub orig_text: String,
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
            orig_text: String::new(),
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
    let restored = restore_whitespaces(&ctx.text, &spaces);
    ctx.text = restored;
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
