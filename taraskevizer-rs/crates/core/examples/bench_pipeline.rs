//! Performance benchmark for the `run_tarask` pipeline.
//!
//! This is a *pure benchmarking* harness — it makes no changes to the CLI or
//! any app-facing behavior. It:
//!   1. Sweeps a range of `CHUNK_SIZE` values (mirroring how `cli/src/main.rs`
//!      splits stdin into parallel chunks) and measures total wall time and
//!      throughput for the tarask pipeline.
//!   2. Captures the duration of every individual step of `run_tarask`
//!      (aggregated across all chunks) so the bottleneck step is visible.
//!
//! Only the tarask pipeline is measured (not phonetic / alphabetic).
//!
//! Usage:
//!   cargo run -p taraskevizer-core --example `bench_pipeline` [PATH]
//!
//! `PATH` defaults to the 50M Wikipedia dump XML:
//!   ../test/texts/bewiki-20261001-pages-articles-multistream_50M.xml

use std::time::{Duration, Instant};

use rayon::prelude::*;
use taraskevizer_core::config::TaraskConfig;
use taraskevizer_core::pipeline::{run_tarask_timed, StepTiming};
use taraskevizer_core::text::split_into_chunks;

fn main() {
    // ── Resolve source path ─────────────────────────────────────
    let args: Vec<String> = std::env::args().collect();
    let default_path = "../test/texts/bewiki-20261001-pages-articles-multistream_10M.xml";
    let candidates: Vec<String> = if args.len() > 1 {
        vec![args[1].clone()]
    } else {
        vec![
            default_path.to_string(),
            "../../../test/texts/bewiki-20261001-pages-articles-multistream_10M.xml".to_string(),
            "/home/gooseob/projects/taraskevizer/test/texts/bewiki-20261001-pages-articles-multistream_10M.xml".to_string(),
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
        let mut timings = Vec::new();
        let _ = run_tarask_timed(warm, &cfg, Some(&mut timings));
    }
    eprintln!(" done in {:.2}s\n", warm_start.elapsed().as_secs_f64());

    // ── CHUNK_SIZE sweep ────────────────────────────────────────
    // Mirrors cli/src/main.rs: nchunks = ceil(len / CHUNK_SIZE); chunks are
    // processed in parallel with rayon when there is more than one chunk.
    let chunk_sizes: &[usize] = &[
        4_000, 8_000, 16_000, 32_000, 64_000, 128_000, 256_000, 512_000,
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
        let ranges = split_into_chunks(&text, nchunks);

        eprintln!("[CHUNK_SIZE={chunk_size:>10}] splitting into {nchunks} chunk(s)...");

        let start = Instant::now();
        // Each chunk is run with the instrumented runner so we also get the
        // per-step breakdown. The output string is dropped (we only need the
        // measurement); the computation still runs fully.
        let per_chunk: Vec<Vec<StepTiming>> = if ranges.len() > 1 {
            ranges
                .into_par_iter()
                .map(|(s, e)| {
                    let mut t = Vec::new();
                    let _out = run_tarask_timed(&text[s..e], &cfg, Some(&mut t));
                    t
                })
                .collect()
        } else {
            ranges
                .into_iter()
                .map(|(s, e)| {
                    let mut t = Vec::new();
                    let _out = run_tarask_timed(&text[s..e], &cfg, Some(&mut t));
                    t
                })
                .collect()
        };
        let total = start.elapsed();

        let steps = aggregate_steps(&per_chunk);
        results.push(RunResult {
            chunk_size,
            nchunks,
            total,
            steps,
        });

        let mb_s = total_bytes as f64 / total.as_secs_f64() / 1e6;
        eprintln!(
            "  total={:.3}s  throughput={:.2} MB/s",
            total.as_secs_f64(),
            mb_s
        );
    }

    // ── Print consolidated reports ──────────────────────────────
    print_sweep_table(&results, total_bytes);
    print_step_tables(&results);
}

struct RunResult {
    chunk_size: usize,
    nchunks: usize,
    total: Duration,
    /// Aggregated per-step durations (CPU time summed across all chunks).
    steps: Vec<(String, Duration)>,
}

/// Sum per-step durations across chunks, preserving step order.
fn aggregate_steps(per_chunk: &[Vec<StepTiming>]) -> Vec<(String, Duration)> {
    let mut agg: Vec<(String, Duration)> = Vec::new();
    for chunk_steps in per_chunk {
        for (i, st) in chunk_steps.iter().enumerate() {
            if i >= agg.len() {
                agg.push((st.name.to_string(), Duration::ZERO));
            }
            agg[i].1 += st.duration;
        }
    }
    agg
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

fn print_step_tables(results: &[RunResult]) {
    for r in results {
        let step_sum: Duration = r.steps.iter().map(|(_, d)| *d).sum();
        println!(
            "\n{:=<78}",
            format!(
                "CHUNK_SIZE={}  ({} chunk(s), wall={:.3}s, summed-step CPU={:.3}s)",
                r.chunk_size,
                r.nchunks,
                r.total.as_secs_f64(),
                step_sum.as_secs_f64()
            )
        );
        println!("{:>28} {:>14} {:>10}", "STEP", "TIME(s)", "% of steps");
        println!("{:-<78}", "");
        for (name, d) in &r.steps {
            let pct = if step_sum.as_secs_f64() > 0.0 {
                d.as_secs_f64() / step_sum.as_secs_f64() * 100.0
            } else {
                0.0
            };
            println!("{:>28} {:>14.4} {:>9.1}%", name, d.as_secs_f64(), pct);
        }
    }
}
