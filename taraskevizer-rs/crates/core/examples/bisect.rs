//! Bisect the flat wordlist into few single-pass batches with zero behavior change.
//!
//! ```sh
//! # from taraskevizer-rs/:
//! cargo run --release --example bisect -- restore   # write safe-baseline wordlist.rs
//! cargo run --release --example bisect -- bisect    # full bisect, writes final wordlist.rs
//! ```
//!
//! Entries come straight from the compiled `WORD_LIST` (flattened in order),
//! so there is no source parsing. The oracle replicates production exactly:
//! CLI chunking plus the pre-wordlist pipeline prefix, comparing candidate
//! vs safe-baseline `CompiledDict` output on what the wordlist stage actually
//! sees (safe baseline proven identical to output.reference.txt end-to-end
//! via run.sh). Everything runs in-process: no recompiles, no Python.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::io::Write as _;
use taraskevizer_core::config::{Alphabet, JMode, TaraskConfig, VariationMode};
use taraskevizer_core::dict::{CompiledDict, DictEntry, WORD_LIST};
use taraskevizer_core::pipeline::{
    collapse_whitespaces, step_prepare, step_resolve_special_syntax,
    step_store_splitted_abc_converted_orig, step_to_lower_case, step_trim, PipelineContext,
};
use taraskevizer_core::tarask;
use taraskevizer_core::text::split_into_chunks;

const WORDLIST_RS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/dict/wordlist.rs");
const DEFAULT_DUMP: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../test/texts/bewiki-20251101-pages-articles-multistream_30M.xml"
);
const N: usize = 1416;
/// Chunk size mirroring production (`cli/src/main.rs`).
const CHUNK_SIZE: usize = 16_000;

/// Flat entries in dict order, borrowed from the compiled `WORD_LIST`.
fn flat_entries() -> Vec<(&'static str, &'static str)> {
    let flat: Vec<(&'static str, &'static str)> =
        WORD_LIST.iter().flat_map(|b| b.iter().copied()).collect();
    assert_eq!(flat.len(), N, "flat entry count");
    for (p, r) in &flat {
        assert!(!p.contains('"') && !r.contains('"'), "raw-literal-safe");
    }
    flat
}

fn to_batches(entries: &[(&'static str, &'static str)], cuts: &[usize]) -> Vec<Vec<DictEntry>> {
    let mut batches = Vec::with_capacity(cuts.len() - 1);
    for w in cuts.windows(2) {
        batches.push(
            entries[w[0]..w[1]]
                .iter()
                .map(|(p, r)| DictEntry {
                    pattern: p.to_string(),
                    result: r.to_string(),
                })
                .collect(),
        );
    }
    batches
}

/// Isolated test cuts: merge [rs, re) into one batch, everything else
/// singleton, except the proven-safe tail merges. The last cut [1415, 1416)
/// stays sequential, matching `dict_batches!` semantics.
fn candidate_cuts(rs: usize, re: usize) -> Vec<usize> {
    let mut remove: BTreeSet<usize> = BTreeSet::new();
    for (a, b) in [(rs, re), (1268usize, 1271usize), (1271, 1302), (1329, 1415)] {
        for x in (a + 1)..b {
            remove.insert(x);
        }
    }
    (0..=N).filter(|x| !remove.contains(x)).collect()
}

fn dict_for(entries: &[(&'static str, &'static str)], cuts: &[usize]) -> CompiledDict {
    CompiledDict::new_batched_entries(&to_batches(entries, cuts))
}

/// Production chunking lives in core (`text::split_into_chunks`) so the
/// oracle can never drift from it again.
///
/// Candidate partition matches the reference on every production chunk.
fn matches_reference(dict: &CompiledDict, chunks: &[String], reference_chunks: &[String]) -> bool {
    for (chunk, expected) in chunks.iter().zip(reference_chunks.iter()) {
        if dict.replace_all(chunk) != *expected {
            return false;
        }
    }
    true
}

fn write_wordlist(entries: &[(&'static str, &'static str)], cuts: &[usize]) {
    let mut out = String::new();
    out.push_str(
        "//! Batched wordlist: few single-pass batches + sequential tail; identical to reference.\n",
    );
    out.push_str("crate::dict_batches! {\n");
    out.push_str("    pub WORD_LIST:\n");
    for w in cuts.windows(2) {
        let (s, e) = (w[0], w[1]);
        if e == N {
            let _ = writeln!(out, "    sequential [ // SEQ [{s}..{e})");
        } else {
            let _ = writeln!(out, "    batch [ // [{s}..{e}) ({})", e - s);
        }
        for (p, r) in &entries[s..e] {
            let _ = writeln!(out, "        (r\"{p}\", r\"{r}\"),");
        }
        out.push_str("    ],\n");
    }
    out.push_str("}\n");
    std::fs::write(WORDLIST_RS, out).expect("write wordlist.rs");
}

fn safe_baseline_cuts() -> Vec<usize> {
    let mut cuts = vec![0, 101];
    cuts.extend(102..917);
    cuts.push(983);
    cuts.extend(984..1269);
    cuts.extend([1271, 1302]);
    cuts.extend(1303..1330);
    cuts.extend([1415, 1416]);
    cuts
}

fn log(msg: &str) {
    println!("{msg}");
    let _ = std::io::stdout().flush();
}

/// Combine accepted merges + mandatory boundaries; split the rest into singletons.
fn combine(accepted: &[(usize, usize)]) -> Vec<usize> {
    let mut bounds: BTreeSet<usize> = BTreeSet::from([0, N]);
    for (a, b) in accepted {
        bounds.insert(*a);
        bounds.insert(*b);
    }
    for b in [
        101, 145, 257, 400, 534, 612, 916, 983, 1048, 1268, 1271, 1302, 1329, 1415,
    ] {
        bounds.insert(b);
    }
    let acc_set: BTreeSet<(usize, usize)> = accepted.iter().copied().collect();
    let flat: Vec<usize> = bounds.into_iter().collect();
    let mut cuts = vec![flat[0]];
    for w in flat.windows(2) {
        let (s, e) = (w[0], w[1]);
        if e - s > 1 && !acc_set.contains(&(s, e)) {
            cuts.extend((s + 1)..=e);
        } else {
            cuts.push(e);
        }
    }
    cuts
}

/// CLI `-nc` config (cyrillic, no color, full variations).
fn nc_cfg() -> TaraskConfig {
    TaraskConfig {
        abc: Alphabet::Cyrillic,
        j: JMode::Never,
        do_escape_capitalized: true,
        wrappers: None,
        g: true,
        variations: VariationMode::All,
        ..Default::default()
    }
}

/// Run each production chunk through the exact pre-wordlist prefix
/// (trim → `resolve_special_syntax` → prepare → `collapse_whitespaces` →
/// `store_splitted_abc_converted_orig` → `to_lower_case`), mirroring
/// `run_base_pipeline_timed`. The wordlist stage sees precisely these strings
/// in production, so partition equality HERE implies end-to-end equality
/// (everything downstream is deterministic per chunk).
fn wordlist_inputs(input: &str, ranges: &[(usize, usize)], cfg: &TaraskConfig) -> Vec<String> {
    ranges
        .iter()
        .map(|&(s, e)| {
            let mut ctx = PipelineContext::new(&input[s..e], cfg);
            step_trim(&mut ctx);
            step_resolve_special_syntax(&mut ctx);
            step_prepare(&mut ctx);
            let ws_src = std::mem::take(&mut ctx.text);
            let (collapsed, _spaces) = collapse_whitespaces(&ws_src);
            ctx.text = collapsed;
            step_store_splitted_abc_converted_orig(&mut ctx);
            step_to_lower_case(&mut ctx);
            ctx.text
        })
        .collect()
}

/// Replicate the CLI (`-nc`: no color, cyrillic, variations) exactly with the
/// COMPILED-IN wordlist and compare against output.reference.txt.
/// Usage: `bisect verify [dump] [reference]`
fn verify(dump: &str, reference_path: &str) {
    let cfg = nc_cfg();
    let bytes = std::fs::read(dump).expect("read dump");
    let input = String::from_utf8_lossy(&bytes).into_owned();
    let ranges = split_into_chunks(&input, input.len().div_ceil(CHUNK_SIZE));
    log(&format!(
        "verifying {} chunks against reference...",
        ranges.len()
    ));
    let mut out = String::with_capacity(input.len());
    for &(s, e) in &ranges {
        out.push_str(&tarask(&input[s..e], &cfg));
    }
    let expected = std::fs::read_to_string(reference_path).expect("read reference");
    // CLI writes each chunk without added separators; compare exact bytes.
    if out == expected {
        log("VERIFY: IDENTICAL");
    } else {
        let a = out.lines().count();
        let b = expected.lines().count();
        log(&format!(
            "VERIFY: DIFFERENT (got {a} lines, want {b} lines)"
        ));
        std::process::exit(1);
    }
}

/// Recombine from a previous run's `ACCEPTED:` log line plus the pre-proven
/// safe tail merges, without re-running the bisect:
/// `bisect refinalize /tmp/bisect_rs.log`
fn refinalize(entries: &[(&'static str, &'static str)], log_path: &str, dump: &str) {
    let log_text = std::fs::read_to_string(log_path).expect("read bisect log");
    let line = log_text
        .lines()
        .find(|l| l.starts_with("ACCEPTED:"))
        .expect("ACCEPTED line");
    let nums: Vec<usize> = line
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("int"))
        .collect();
    assert!(nums.len().is_multiple_of(2), "pairs");
    let mut accepted: Vec<(usize, usize)> = nums.chunks_exact(2).map(|c| (c[0], c[1])).collect();
    // Pre-proven safe (each merged alone with zero diffs; also baked into
    // every candidate test): keep them merged in the final file.
    for t in [(1268, 1271), (1271, 1302), (1329, 1415)] {
        if !accepted.contains(&t) {
            accepted.push(t);
        }
    }
    accepted.sort_unstable();
    log(&format!("ACCEPTED: {accepted:?}"));
    let cuts = combine(&accepted);
    log(&format!("final intervals: {}", cuts.len() - 1));
    write_wordlist(entries, &cuts);

    let cfg = nc_cfg();
    let bytes = std::fs::read(dump).expect("read dump");
    let input = String::from_utf8_lossy(&bytes).into_owned();
    let ranges = split_into_chunks(&input, input.len().div_ceil(CHUNK_SIZE));
    let wl_inputs = wordlist_inputs(&input, &ranges, &cfg);
    let ref_dict = dict_for(entries, &safe_baseline_cuts());
    let reference_chunks: Vec<String> = wl_inputs.iter().map(|c| ref_dict.replace_all(c)).collect();
    let got_ok = matches_reference(&dict_for(entries, &cuts), &wl_inputs, &reference_chunks);
    log(if got_ok {
        "FINAL: IDENTICAL"
    } else {
        "FINAL: DIFFERENT"
    });
    if !got_ok {
        std::process::exit(1);
    }
}

fn bisect(
    entries: &[(&'static str, &'static str)],
    chunks: &[String],
    reference_chunks: &[String],
    accepted: &mut Vec<(usize, usize)>,
    rs: usize,
    re: usize,
    depth: usize,
) {
    let n = re - rs;
    if n <= 1 {
        return;
    }
    let pad = "  ".repeat(depth);
    log(&format!("{pad}try [{rs}..{re}) ({n})..."));
    let dict = dict_for(entries, &candidate_cuts(rs, re));
    if matches_reference(&dict, chunks, reference_chunks) {
        log(&format!("{pad}  ACCEPT [{rs}..{re})"));
        accepted.push((rs, re));
        return;
    }
    if n == 2 {
        log(&format!("{pad}  SPLIT into singletons (keep cut)"));
        return;
    }
    log(&format!("{pad}  differs, splitting"));
    let mid = rs.midpoint(re);
    bisect(
        entries,
        chunks,
        reference_chunks,
        accepted,
        rs,
        mid,
        depth + 1,
    );
    bisect(
        entries,
        chunks,
        reference_chunks,
        accepted,
        mid,
        re,
        depth + 1,
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map_or("bisect", |s| s.as_str());
    let dump = args.get(2).map_or(DEFAULT_DUMP, |s| s.as_str());

    let entries = flat_entries();
    log(&format!(
        "flattened {} entries from WORD_LIST",
        entries.len()
    ));

    if mode == "restore" {
        write_wordlist(&entries, &safe_baseline_cuts());
        log("wrote safe baseline");
        return;
    }

    if mode == "chunkinfo" {
        let bytes = std::fs::read(dump).expect("read dump");
        let input = String::from_utf8_lossy(&bytes).into_owned();
        let ranges = split_into_chunks(&input, input.len().div_ceil(CHUNK_SIZE));
        let max = ranges.iter().map(|&(s, e)| e - s).max().unwrap_or(0);
        log(&format!(
            "input {} bytes -> {} chunks (max {max})",
            input.len(),
            ranges.len()
        ));
        return;
    }

    if mode == "verify" {
        let dump = args.get(2).map_or(DEFAULT_DUMP, |s| s.as_str());
        let reference = args.get(3).map_or(
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../output.reference.txt"),
            |s| s.as_str(),
        );
        verify(dump, reference);
        return;
    }

    if mode == "probe" {
        let rs: usize = args.get(2).expect("rs").parse().expect("int");
        let re: usize = args.get(3).expect("re").parse().expect("int");
        let dump = args.get(4).map_or(DEFAULT_DUMP, |s| s.as_str());
        let cfg = nc_cfg();
        let bytes = std::fs::read(dump).expect("read dump");
        let input = String::from_utf8_lossy(&bytes).into_owned();
        let ranges = split_into_chunks(&input, input.len().div_ceil(CHUNK_SIZE));
        let wl_inputs = wordlist_inputs(&input, &ranges, &cfg);
        let ref_dict = dict_for(&entries, &safe_baseline_cuts());
        let reference_chunks: Vec<String> =
            wl_inputs.iter().map(|c| ref_dict.replace_all(c)).collect();
        let dict = dict_for(&entries, &candidate_cuts(rs, re));
        let mut bad = 0;
        for (i, (w, e)) in wl_inputs.iter().zip(reference_chunks.iter()).enumerate() {
            if dict.replace_all(w) != *e {
                if bad < 5 {
                    log(&format!("chunk {i} differs"));
                }
                bad += 1;
            }
        }
        log(&format!("probe [{rs}..{re}): {bad} differing chunks"));
        return;
    }

    if mode == "refinalize" {
        let log_path = args.get(2).map_or("/tmp/bisect_rs.log", |s| s.as_str());
        let dump = args.get(3).map_or(DEFAULT_DUMP, |s| s.as_str());
        refinalize(&entries, log_path, dump);
        return;
    }

    // CLI-faithful input handling: bytes -> lossy string, then the same chunking,
    // then the exact pre-wordlist prefix: dicts are compared on what the
    // wordlist stage actually sees in production.
    let cfg = nc_cfg();
    let bytes = std::fs::read(dump).expect("read dump");
    let input = String::from_utf8_lossy(&bytes).into_owned();
    let ranges = split_into_chunks(&input, input.len().div_ceil(CHUNK_SIZE));
    log(&format!(
        "input {} bytes in {} chunks",
        input.len(),
        ranges.len()
    ));
    let wl_inputs = wordlist_inputs(&input, &ranges, &cfg);
    log("compiling reference dict...");
    let ref_dict = dict_for(&entries, &safe_baseline_cuts());
    let reference_chunks: Vec<String> = wl_inputs.iter().map(|c| ref_dict.replace_all(c)).collect();
    log("reference chunks ready");

    // Pre-proven safe merges (each merged alone with zero diffs): kept, and
    // the tail ones are also baked into every candidate test.
    let mut accepted: Vec<(usize, usize)> = vec![
        (0, 101),
        (916, 983),
        (1268, 1271),
        (1271, 1302),
        (1329, 1415),
    ];
    let regions = [
        (101usize, 145usize),
        (145, 257),
        (257, 400),
        (400, 534),
        (534, 612),
        (612, 916),
        (983, 1048),
        (1048, 1268),
        (1302, 1329),
    ];
    for (rs, re) in regions {
        log(&format!("=== region [{rs}..{re}) ==="));
        bisect(
            &entries,
            &wl_inputs,
            &reference_chunks,
            &mut accepted,
            rs,
            re,
            0,
        );
    }
    accepted.sort_unstable();
    log(&format!("ACCEPTED: {accepted:?}"));
    let cuts = combine(&accepted);
    log(&format!("final intervals: {}", cuts.len() - 1));
    log(&format!("final cuts: {cuts:?}"));
    write_wordlist(&entries, &cuts);

    // In-process final verification.
    let final_dict = dict_for(&entries, &cuts);
    let ok = matches_reference(&final_dict, &wl_inputs, &reference_chunks);
    log(if ok {
        "FINAL: IDENTICAL"
    } else {
        "FINAL: DIFFERENT"
    });
    if !ok {
        std::process::exit(1);
    }
}
