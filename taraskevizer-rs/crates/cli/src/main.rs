use std::io::{self, IsTerminal, Read, Write};

use clap::Parser;
use rayon::prelude::*;
use taraskevizer_core::config::*;
use taraskevizer_core::wrappers::{ANSI_COLOR_WRAPPERS, HTML_WRAPPERS};
use taraskevizer_core::{alphabetic, phonetic, tarask};

// Wrapper sets (ANSI_COLOR_WRAPPERS / HTML_WRAPPERS) and the `variation_*`
// helpers now live in `taraskevizer_core::wrappers` (mirroring `src/wrappers.ts`).

// ── Short-arg expansion ─────────────────────────────────────────
// Match JS parse-args.ts behavior: multi-char short flags like -nc, -nec, -abc
// These are expanded to their long equivalents before clap parsing.

fn expand_args(raw: Vec<String>) -> Vec<String> {
    let mut out = Vec::with_capacity(raw.len());
    for (i, arg) in raw.into_iter().enumerate() {
        if i == 0 || !arg.starts_with('-') || arg.starts_with("--") {
            out.push(arg);
        } else {
            match arg.as_str() {
                // Pass through — clap handles these natively
                "-l" | "-a" | "-h" | "-V" => out.push(arg),
                // Multi-char short forms → expand to long flags
                "-lj" => out.push("--latin-ji".into()),
                "-jr" => out.push("--jrandom".into()),
                "-ja" => out.push("--jalways".into()),
                "-nec" => out.push("--no-escape-caps".into()),
                "-nv" => out.push("--no-variations".into()),
                "-fv" => out.push("--first-variation".into()),
                "-nc" => out.push("--no-color".into()),
                "-html" => out.push("--html".into()),
                "-abc" => out.push("--alphabet-only".into()),
                "-ph" => out.push("--phonetic".into()),
                "-st" => out.push("--single-thread".into()),
                _ => out.push(arg), // unrecognized → let clap handle it
            }
        }
    }
    out
}

// ── CLI args ────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "tarask",
    version = "10.5.0",
    about = "Канвэртацыя акадэмічнага правапісу ў клясычны",
    long_about = "Belarusian orthography converter (Narkamauka → Taraskevica)\n\
                   Read text from stdin or provide it as arguments."
)]
struct Cli {
    /// Use Latin alphabet
    #[arg(long, short = 'l')]
    latin: bool,

    /// Use LatinJi alphabet
    #[arg(long)]
    latin_ji: bool,

    /// Use Arabic alphabet
    #[arg(long, short = 'a')]
    arabic: bool,

    /// Replace і→й after vowels randomly
    #[arg(long)]
    jrandom: bool,

    /// Always replace і→й after vowels
    #[arg(long)]
    jalways: bool,

    /// Disable escaping of capitalized words (acronyms may change)
    #[arg(long)]
    no_escape_caps: bool,

    /// Disable ґ→г conversion
    #[arg(long = "h")]
    disable_g: bool,

    /// Disable variation output — show main form only
    #[arg(long)]
    no_variations: bool,

    /// Show first variation instead of all
    #[arg(long)]
    first_variation: bool,

    /// Disable ANSI color highlighting
    #[arg(long)]
    no_color: bool,

    /// Use HTML wrappers for changes
    #[arg(long)]
    html: bool,

    /// Alphabet-only conversion mode (no Taraskevization)
    #[arg(long)]
    alphabet_only: bool,

    /// Phonetic conversion mode
    #[arg(long)]
    phonetic: bool,

    /// Disable parallel processing
    #[arg(long)]
    single_thread: bool,

    /// Text to convert (if not provided, reads from stdin)
    #[arg(trailing_var_arg = true, hide = true)]
    text: Vec<String>,
}

// ── Main ────────────────────────────────────────────────────────

fn main() {
    let raw: Vec<String> = std::env::args().collect();
    let expanded = expand_args(raw);
    let cli = Cli::parse_from(expanded);

    let alphabet = if cli.latin_ji {
        Alphabet::LatinJi
    } else if cli.latin {
        Alphabet::Latin
    } else if cli.arabic {
        Alphabet::Arabic
    } else {
        Alphabet::Cyrillic
    };

    let j = if cli.jalways {
        JMode::Always
    } else if cli.jrandom {
        JMode::Random
    } else {
        JMode::Never
    };

    let variations = if cli.no_variations {
        VariationMode::No
    } else if cli.first_variation {
        VariationMode::First
    } else {
        VariationMode::All
    };

    let wrappers = if cli.html {
        Some(HTML_WRAPPERS)
    } else if cli.no_color {
        None
    } else {
        Some(ANSI_COLOR_WRAPPERS)
    };

    let mut cfg = TaraskConfig {
        abc: alphabet,
        j,
        do_escape_capitalized: !cli.no_escape_caps,
        wrappers,
        g: !cli.disable_g,
        variations,
        ..Default::default()
    };

    if cli.html {
        // Mirror `...htmlConfigOptions` in the reference: <br> for newlines and
        // &lt for `<`. The `g` flag above still wins over htmlConfigOptions.g.
        cfg.new_line = "<br>".into();
        cfg.left_angle_bracket = "&lt".into();
    }

    let mode = if cli.alphabet_only {
        "alphabetic"
    } else if cli.phonetic {
        "phonetic"
    } else {
        "tarask"
    };

    if !cli.text.is_empty() {
        let input = cli.text.join(" ");
        let result = run_mode(mode, &input, &cfg);
        let _ = io::stdout().write_all(result.as_bytes());
        let _ = io::stdout().write_all(b"\n");
        return;
    }

    if io::stdin().is_terminal() {
        let _ = io::stderr().write_all(b"Enter the text: ");
        let _ = io::stderr().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let input = input.trim_end_matches('\n');
            let result = run_mode(mode, input, &cfg);
            let _ = io::stdout().write_all(result.as_bytes());
        }
    } else {
        let mut bytes = Vec::new();
        let input = if io::stdin().read_to_end(&mut bytes).is_ok() {
            String::from_utf8_lossy(&bytes).into_owned()
        } else {
            String::new()
        };
        if !input.is_empty() {
            const CHUNK_SIZE: usize = 64_000;
            let nchunks = (input.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
            let chunks = split_into_chunks(&input, nchunks);
            let nchars = input.len();
            if !cli.single_thread && chunks.len() > 1 {
                let _ = io::stderr().write_fmt(format_args!(
                    "Processing {} chars in {} chunks... ",
                    nchars, nchunks,
                ));
                let _ = io::stderr().flush();
                let start = std::time::Instant::now();

                let results: Vec<String> = chunks
                    .into_par_iter()
                    .map(|chunk| run_mode(mode, &chunk, &cfg))
                    .collect();

                let _ = io::stderr().write_fmt(format_args!(
                    "done in {:.2}s.\n",
                    start.elapsed().as_secs_f64()
                ));

                for r in &results {
                    let _ = io::stdout().write_all(r.as_bytes());
                }
            } else {
                for chunk in &chunks {
                    let result = run_mode(mode, chunk, &cfg);
                    let _ = io::stdout().write_all(result.as_bytes());
                }
            }
        }
    }
}

fn run_mode(mode: &str, text: &str, cfg: &TaraskConfig) -> String {
    match mode {
        "alphabetic" => alphabetic(text, cfg),
        "phonetic" => phonetic(text, cfg),
        _ => tarask(text, cfg),
    }
}

fn split_into_chunks(text: &str, n: usize) -> Vec<String> {
    if n <= 1 || text.is_empty() {
        return vec![text.to_string()];
    }
    let target = (text.len() + n - 1) / n;
    let mut chunks = Vec::with_capacity(n);
    let mut start = 0;
    for i in 0..n {
        if start >= text.len() {
            break;
        }
        let mut end = (start + target).min(text.len());
        if i < n - 1 && end < text.len() {
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            let forward = text[end..].find('\n').map(|p| end + p + 1);
            let backward = text[..end].rfind('\n').map(|p| p + 1);
            match (forward, backward) {
                (Some(f), Some(b)) => {
                    if f - end < end - b {
                        end = f;
                    } else {
                        end = b;
                    }
                }
                (Some(f), None) => end = f,
                (None, Some(b)) => end = b,
                (None, None) => {
                    while !text.is_char_boundary(end) {
                        end -= 1;
                    }
                }
            }
        }
        if i == n - 1 {
            end = text.len();
        }
        chunks.push(text[start..end].to_string());
        start = end;
    }
    chunks
}
