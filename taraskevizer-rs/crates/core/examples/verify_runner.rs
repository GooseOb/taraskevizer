// Differential-test runner for the Rust rewrite.
//
// Reads verify/corpus.json (produced by gen_corpus.ts from the original `..`),
// runs each entry through the Rust core, and writes verify/out_rs.json.
//
// Run: cargo run --example verify_runner --release -- [path/to/corpus.json] [path/to/out_rs.json]

use std::fs;
use std::panic::{self, UnwindSafe};

use taraskevizer_core::config::{Alphabet, JMode, TaraskConfig, VariationMode};
use taraskevizer_core::{
    alphabetic, apply_highlight_diff, phonetic, tarask, ANSI_COLOR_WRAPPERS, HTML_WRAPPERS,
};

#[derive(serde::Deserialize)]
struct Cfg {
    abc: Option<String>,
    j: Option<String>,
    do_escape_capitalized: Option<bool>,
    wrappers: Option<String>,
    g: Option<bool>,
    variations: Option<String>,
    new_line: Option<String>,
    left_angle_bracket: Option<String>,
}

#[derive(serde::Deserialize)]
struct Entry {
    pipeline: String,
    config: Cfg,
    input: String,
    #[serde(default)]
    word: Option<String>,
}

#[derive(serde::Serialize)]
struct Out {
    out: Option<String>,
    err: Option<String>,
}

fn build_config(c: &Cfg) -> TaraskConfig {
    let mut cfg = TaraskConfig::default();
    if let Some(abc) = &c.abc {
        cfg.abc = match abc.as_str() {
            "latin" => Alphabet::Latin,
            "latinJi" => Alphabet::LatinJi,
            "arabic" => Alphabet::Arabic,
            _ => Alphabet::Cyrillic,
        };
    }
    if let Some(j) = &c.j {
        cfg.j = match j.as_str() {
            "always" => JMode::Always,
            "random" => JMode::Random,
            _ => JMode::Never,
        };
    }
    if let Some(v) = c.do_escape_capitalized {
        cfg.do_escape_capitalized = v;
    }
    if let Some(w) = &c.wrappers {
        cfg.wrappers = match w.as_str() {
            "html" => Some(HTML_WRAPPERS),
            "ansi" => Some(ANSI_COLOR_WRAPPERS),
            _ => None,
        };
    }
    if let Some(g) = c.g {
        cfg.g = g;
    }
    if let Some(v) = &c.variations {
        cfg.variations = match v.as_str() {
            "no" => VariationMode::No,
            "first" => VariationMode::First,
            _ => VariationMode::All,
        };
    }
    if let Some(nl) = &c.new_line {
        cfg.new_line = nl.clone();
    }
    if let Some(l) = &c.left_angle_bracket {
        cfg.left_angle_bracket = l.clone();
    }
    cfg
}

fn catch<F: FnOnce() -> String + UnwindSafe>(f: F) -> Out {
    match panic::catch_unwind(f) {
        Ok(s) => Out {
            out: Some(s),
            err: None,
        },
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "panic".to_string()
            };
            Out {
                out: None,
                err: Some(msg),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let corpus_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("verify/corpus.json");
    let out_path = args
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or("verify/out_rs.json");

    let corpus_text = fs::read_to_string(corpus_path).expect("read corpus");
    let entries: Vec<Entry> = serde_json::from_str(&corpus_text).expect("parse corpus");

    let mut results = Vec::with_capacity(entries.len());
    for e in &entries {
        let cfg = build_config(&e.config);
        if e.pipeline == "highlight" {
            let word = e.word.clone().unwrap_or_default();
            let orig = e.input.clone();
            let out = catch(move || {
                apply_highlight_diff(&word, &orig, true, &|s: &str| format!("[{s}]"))
            });
            results.push(out);
        } else if e.pipeline == "alphabetic" {
            let input = e.input.clone();
            let out = catch(move || alphabetic(&input, &cfg));
            results.push(out);
        } else if e.pipeline == "phonetic" {
            let input = e.input.clone();
            let out = catch(move || phonetic(&input, &cfg));
            results.push(out);
        } else {
            let input = e.input.clone();
            let out = catch(move || tarask(&input, &cfg));
            results.push(out);
        }
    }

    let json = serde_json::to_string(&results).expect("serialize");
    fs::write(out_path, json).expect("write out");
    let panics = results.iter().filter(|r| r.err.is_some()).count();
    println!(
        "Rust runner: {} entries, {} panics -> {}",
        results.len(),
        panics,
        out_path
    );
}
