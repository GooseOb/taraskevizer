//! Performance benchmark for the `run_tarask` pipeline.
//!
//! Sweeps a range of `CHUNK_SIZE` values (mirroring how `cli/src/main.rs`
//! splits stdin into parallel chunks) and measures total wall time and
//! throughput for the tarask pipeline.
//!
//! Only the tarask pipeline is measured (not phonetic / alphabetic).
//!
//! Usage:
//!   cargo run -p taraskevizer-core --example bench_pipeline [PATH]
//!
//! `PATH` defaults to the 50M Wikipedia dump XML:
//!   ../test/texts/bewiki-20251101-pages-articles-multistream_50M.xml

use std::time::{Duration, Instant};

use rayon::prelude::*;
use taraskevizer_core::config::TaraskConfig;
use taraskevizer_core::pipeline::run_tarask;

fn main() {
    // ── Resolve source path ─────────────────────────────────────
    let args: Vec<String> = std::env::args().collect();
    let default_path = "../test/texts/bewiki-20251101-pages-articles-multistream_50M.xml";
    let candidates: Vec<String> = if args.len() > 1 {
        vec![args[1].clone()]
    } else {
        vec![
            default_path.to_string(),
            "../../../test/texts/bewiki-20251101-pages-articles-multistream_50M.xml".to_string(),
            "/home/gooseob/projects/taraskevizer/test/texts/bewiki-20251101-pages-articles-multistream_50M.xml".to_string(),
        ]
    };

    let path = candidates
        .into_iter()
        .find(|p| std::path::Path::new(p).exists())
        .unwrap_or_else(|| {
            eprintln!("Source file not found. Tried: {default_path} (and variants). Pass a path as the first argument.");
            std::process::exit(1);
        });

    eprintln!("Reading source: {path}");
    // Mirror cli/src/main.rs: read raw bytes and fall back to lossy UTF-8 so
    // files with a few invalid bytes (e.g. the Wikipedia dump) still process.
    let bytes = std::fs::read(&path).expect("read file");
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let total_bytes = text.len();
    eprintln!(
        "Loaded {} bytes (~{:.1} MB).",
        total_bytes,
        total_bytes as f64 / 1e6
    );

    let cfg = TaraskConfig::default();

    // ── Warm up: force dict compilation (LazyLocks) + JIT warm ──
    eprint!("Warming up...");
    let warm_start = Instant::now();
    {
        let mut warm_len = total_bytes.min(500_000);
        while warm_len > 0 && !text.is_char_boundary(warm_len) {
            warm_len -= 1;
        }
        let warm = &text[..warm_len];
        let _ = run_tarask(warm, &cfg);
    }
    eprintln!(" done in {:.2}s\n", warm_start.elapsed().as_secs_f64());

    // ── CHUNK_SIZE sweep ────────────────────────────────────────
    // Mirrors cli/src/main.rs: nchunks = ceil(len / CHUNK_SIZE); chunks are
    // processed in parallel with rayon when there is more than one chunk.
    let chunk_sizes: &[usize] = &[
        4_000, 16_000, 32_000, 64_000, 128_000, 256_000, 512_000,
        1_048_576,
        // >= file size ⇒ single sequential chunk (baseline)
    ];

    eprintln!(
        "=== tarask pipeline benchmark ===\n\
         source: {path}\n\
         size:   {total_bytes} bytes\n\
         threads: {} (rayon default)\n",
        rayon::current_num_threads()
    );

    let mut results: Vec<RunResult> = Vec::with_capacity(chunk_sizes.len());

    for &chunk_size in chunk_sizes {
        let nchunks = total_bytes.div_ceil(chunk_size);
        let chunks = split_into_chunks(&text, nchunks);

        eprintln!("[CHUNK_SIZE={chunk_size:>10}] splitting into {nchunks} chunk(s)...",);

        let start = Instant::now();
        // The output string is dropped (we only need the measurement);
        // the computation still runs fully.
        if chunks.len() > 1 {
            chunks
                .into_par_iter()
                .for_each(|chunk| {
                    let _out = run_tarask(&chunk, &cfg);
                });
        } else {
            chunks
                .into_iter()
                .for_each(|chunk| {
                    let _out = run_tarask(&chunk, &cfg);
                });
        };
        let total = start.elapsed();

        results.push(RunResult {
            chunk_size,
            nchunks,
            total,
        });

        let mb_s = total_bytes as f64 / total.as_secs_f64() / 1e6;
        eprintln!(
            "  total={:.3}s  throughput={:.2} MB/s",
            total.as_secs_f64(),
            mb_s
        );
    }

    // ── Print consolidated report ───────────────────────────────
    print_sweep_table(&results, total_bytes);
}

struct RunResult {
    chunk_size: usize,
    nchunks: usize,
    total: Duration,
}

fn print_sweep_table(results: &[RunResult], total_bytes: usize) {
    println!("\n{:=<78}", "");
    println!("CHUNK_SIZE SWEEP — run_tarask (total wall time)");
    println!("{:=<78}", "");
    println!(
        "{:>12} {:>10} {:>12} {:>16}",
        "CHUNK_SIZE", "CHUNKS", "TOTAL(s)", "THROUGHPut(MB/s)"
    );
    println!("{:-<78}", "");
    for r in results {
        let mb_s = total_bytes as f64 / r.total.as_secs_f64() / 1e6;
        println!(
            "{:>12} {:>10} {:>12.3} {:>16.2}",
            r.chunk_size,
            r.nchunks,
            r.total.as_secs_f64(),
            mb_s
        );
    }
    println!("{:=<78}", "");
}

/// Split `text` into `n` chunks at newline boundaries (mirrors cli/src/main.rs).
fn split_into_chunks(text: &str, n: usize) -> Vec<String> {
    if n <= 1 || text.is_empty() {
        return vec![text.to_string()];
    }
    let target = text.len().div_ceil(n);
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
