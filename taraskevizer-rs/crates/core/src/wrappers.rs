//! Predefined change-wrapping strategies.
//!
//! This mirrors `src/wrappers.ts` in the reference implementation: the wrapper
//! sets (`html`, `ansiColor`) live in the core library rather than in a binary,
//! so they can be shared and referenced from a predefined configuration.

use crate::config::{TaraskConfig, VariationWrappers, Wrappers};
use std::fmt::Display;

// ── ANSI color wrappers (mirror `ansiColor` in `src/wrappers.ts`) ──

fn ansi_fix(s: &str) -> String {
    format!("\x1b[32m{s}\x1b[0m")
}

fn ansi_letter_h<T: Display>(s: T) -> String {
    format!("\x1b[35m{s}\x1b[0m")
}

fn ansi_var_all(s: &str) -> String {
    ansi_letter_h(s)
}

fn ansi_var_first(s: &str) -> String {
    ansi_letter_h(variation_first(s))
}

fn ansi_var_no(s: &str) -> String {
    ansi_letter_h(variation_no(s))
}

// ── HTML wrappers (mirror `html` in `src/wrappers.ts`) ──────────

fn html_fix(s: &str) -> String {
    format!("<tarF>{s}</tarF>")
}

fn html_letter_h(ch: char) -> String {
    format!("<tarH>{ch}</tarH>")
}

fn html_var_all(s: &str) -> String {
    let inner = &s[1..s.len().saturating_sub(1)];
    let parts: Vec<&str> = inner.split('|').collect();
    let main = parts.first().unwrap_or(&"");
    let data: Vec<&str> = parts.iter().skip(1).copied().collect();
    if data.is_empty() {
        main.to_string()
    } else {
        format!("<tarL data-l='{}'>{main}</tarL>", data.join(","))
    }
}

fn html_var_first(s: &str) -> String {
    let inner = &s[1..s.len().saturating_sub(1)];
    let parts: Vec<&str> = inner.split('|').collect();
    if parts.len() >= 2 {
        let first = parts[0];
        let main = parts[1];
        let rest: Vec<&str> = parts.iter().skip(2).copied().collect();
        let mut data = rest;
        data.push(first);
        format!("<tarL data-l='{}'>{main}</tarL>", data.join(","))
    } else {
        html_var_all(s)
    }
}

fn html_var_no(s: &str) -> String {
    variation_no(s)
}

/// Shared variation helpers (mirror `defaultVariation` in the reference).
pub fn variation_no(s: &str) -> String {
    let after_paren = s.trim_start_matches('(');
    let end = after_paren
        .find(|c| c == '|' || c == ')')
        .unwrap_or(after_paren.len());
    after_paren[..end].to_string()
}

pub fn variation_first(s: &str) -> String {
    let after_paren = s.trim_start_matches('(');
    if let Some(pipe) = after_paren.find('|') {
        let after_pipe = &after_paren[pipe + 1..];
        let end = after_pipe
            .find(|c| c == '|' || c == ')')
            .unwrap_or(after_pipe.len());
        after_pipe[..end].to_string()
    } else {
        after_paren
            .find(|c| c == ')')
            .map_or(after_paren, |end| &after_paren[..end])
            .to_string()
    }
}

/// ANSI color wrappers.
pub const ANSI_COLOR_WRAPPERS: Wrappers = Wrappers {
    fix: Some(ansi_fix),
    letter_h: Some(ansi_letter_h),
    variable: VariationWrappers {
        all: ansi_var_all,
        first: ansi_var_first,
        no: ansi_var_no,
    },
};

/// HTML wrappers.
pub const HTML_WRAPPERS: Wrappers = Wrappers {
    fix: Some(html_fix),
    letter_h: Some(html_letter_h),
    variable: VariationWrappers {
        all: html_var_all,
        first: html_var_first,
        no: html_var_no,
    },
};

/// Predefined configuration for HTML output.
///
/// Mirrors `htmlConfigOptions` in the reference `src/config.ts`: wraps changes
/// with [`HTML_WRAPPERS`], disables ґ→г conversion, uses `<br>` for newlines
/// and `&lt` for `<`.
pub fn html_config_options() -> TaraskConfig {
    TaraskConfig {
        wrappers: Some(HTML_WRAPPERS),
        g: false,
        new_line: "<br>".into(),
        left_angle_bracket: "&lt".into(),
        ..Default::default()
    }
}
