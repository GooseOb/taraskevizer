use std::io::{self, IsTerminal, Read, Write};

use clap::Parser;
use rayon::prelude::*;
use taraskevizer_core::config::{Alphabet, JMode, TaraskConfig, VariationMode};
use taraskevizer_core::text::split_into_chunks;
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
                // Everything else passes through untouched: single-char
                // flags clap handles natively (`-l`, `-a`, `-h`, `-V`) and
                // unrecognized input (which clap reports).
                _ => out.push(arg),
            }
        }
    }
    out
}

// ── CLI args ────────────────────────────────────────────────────

/// Flat bools are the idiomatic clap design (one field per flag); grouping
/// them would only obscure the CLI definition.
#[allow(clippy::struct_excessive_bools)]
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

    /// Use `LatinJi` alphabet
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
    // Bounded parallel group tuning (see the grouping loop below).
    const GROUP_MAX_CHUNKS: usize = 256;
    const GROUP_MAX_BYTES: usize = 8 << 20;
    const BIG_CHUNK: usize = 256 << 10;
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
        // Zero-copy when stdin is valid UTF-8 (the common case for XML dumps).
        // Shrink first: read_to_end doubles capacity (up to ~2x input size).
        let input = if io::stdin().read_to_end(&mut bytes).is_ok() {
            bytes.shrink_to_fit();
            String::from_utf8(bytes)
                .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned())
        } else {
            String::new()
        };
        if !input.is_empty() {
            const CHUNK_SIZE: usize = 16_000;
            let nchunks = input.len().div_ceil(CHUNK_SIZE);
            // Borrowed ranges: same boundaries as before, no per-chunk copies.
            let ranges = split_into_chunks(&input, nchunks);
            let nchars = input.len();
            // Stream results out as chunks complete: peak memory stays ~1x
            // input (input string only) instead of ~3x (input + chunk copies
            // + collected results), so multi-GB dumps no longer OOM.
            let stdout = io::stdout();
            let mut out = io::BufWriter::with_capacity(1 << 20, stdout.lock());
            if !cli.single_thread && ranges.len() > 1 {
                let _ = io::stderr().write_fmt(format_args!(
                    "Processing {} chars in {} chunks... ",
                    nchars,
                    ranges.len(),
                ));
                let _ = io::stderr().flush();
                let start = std::time::Instant::now();

                // Bounded parallel groups: full rayon speed with O(group)
                // extra memory, output order preserved. Groups are bounded
                // by count AND input bytes; a group containing an oversized
                // chunk runs sequentially (one big chunk's transient working
                // set fits easily, but 8 threads' worth does not — this
                // OOM-killed the full-dump run at ~10GB).
                let mut gstart = 0;
                while gstart < ranges.len() {
                    let mut gend = gstart;
                    let mut gbytes = 0usize;
                    while gend < ranges.len()
                        && gend - gstart < GROUP_MAX_CHUNKS
                        && gbytes <= GROUP_MAX_BYTES
                    {
                        let (s, e) = ranges[gend];
                        gbytes += e - s;
                        gend += 1;
                    }
                    let group = &ranges[gstart..gend];
                    let results: Vec<String> = if group.iter().any(|&(s, e)| e - s > BIG_CHUNK) {
                        group
                            .iter()
                            .map(|&(s, e)| run_mode(mode, &input[s..e], &cfg))
                            .collect()
                    } else {
                        group
                            .into_par_iter()
                            .map(|&(s, e)| run_mode(mode, &input[s..e], &cfg))
                            .collect()
                    };
                    for r in &results {
                        let _ = out.write_all(r.as_bytes());
                    }
                    gstart = gend;
                }

                let _ = io::stderr().write_fmt(format_args!(
                    "done in {:.2}s.\n",
                    start.elapsed().as_secs_f64()
                ));
            } else {
                let debug_chunks = std::env::var("TARASK_DEBUG_CHUNKS").is_ok();
                for (i, &(s, e)) in ranges.iter().enumerate() {
                    if debug_chunks {
                        eprintln!("chunk {i} [{s}..{e}] len {}", e - s);
                    }
                    let result = run_mode(mode, &input[s..e], &cfg);
                    let _ = out.write_all(result.as_bytes());
                }
            }
            let _ = out.flush();
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
