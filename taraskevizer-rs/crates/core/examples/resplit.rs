//! Re-split the flat (sequential) wordlist into fully independent single-pass
//! batches (Rust-only heavy computation).
//!
//! ```sh
//! cargo run --release --example resplit
//! ```
//!
//! Source: `crate::dict::flat_wordlist::WORD_LIST` (user-fixed flat list,
//! order preserved). Output: overwrites `src/dict/wordlist.rs` with
//! `dict_batches!` batches where every batch is internally equivalent to its
//! sequential application on all test inputs (corpus chunks + pairwise
//! synthetic overlap/adjacency witnesses + tricky words), plus a nested-paren
//! check for single words.
//!
//! Independence model: two entries share a batch only if simultaneous
//! single-pass application == sequential application (no overlapping matches,
//! no earlier->later cascade, no space sharing) on the whole witness set.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use taraskevizer_core::dict::{flat_wordlist::WORD_LIST as FLAT, CompiledDict, DictEntry};

const WORDLIST_RS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/dict/wordlist.rs");
const CORPUS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../test/texts/bewiki-20261001-pages-articles-multistream_2M.xml"
);
const CHUNK: usize = 16_000;
const MAX_EXAMPLES: usize = 12;
const MAX_EX_PER_PAIR: usize = 4;

fn has_meta(p: &str) -> bool {
    p.contains('\\')
        || p.contains('(')
        || p.contains(')')
        || p.contains('[')
        || p.contains(']')
        || p.contains('.')
        || p.contains('*')
        || p.contains('+')
        || p.contains('?')
        || p.contains('^')
        || p.contains('$')
        || p.contains('|')
        || p.contains('{')
        || p.contains('}')
}

/// Patterns that can never share a batch: infinite/variable-length classes,
/// anchors, wildcards. Everything else is finite (literals, char classes,
/// groups, `|` alternatives, `?` optionals) and gets enumerated.
fn force_singleton(p: &str) -> bool {
    // \S \s \d \D \w \W \b and friends (but not \uXXXX unicode escapes)
    let b = p.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 1 < b.len() {
            let n = b[i + 1];
            if matches!(n, b'S' | b's' | b'D' | b'd' | b'W' | b'w' | b'b' | b'B') {
                return true;
            }
            // \uXXXX / \u{...} / \x.. : finite single char, skip escape
            if n == b'u' || n == b'x' {
                i += 2;
                continue;
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    if p.contains("[^") {
        return true;
    }
    p.contains('*')
        || p.contains('+')
        || p.contains('{')
        || p.contains('}')
        || p.contains('^')
        || p.contains('$')
        || p.contains('.')
}

// ---------- finite-regex example expander ----------

struct Exp<'a> {
    chars: Vec<char>,
    pos: usize,
    _m: std::marker::PhantomData<&'a ()>,
}

fn expand_pattern(p: &str) -> Vec<String> {
    let chars: Vec<char> = p.chars().collect();
    let mut st = Exp {
        chars,
        pos: 0,
        _m: std::marker::PhantomData,
    };
    let out = parse_alt(&mut st);
    let mut v: Vec<String> = out
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    v.truncate(MAX_EXAMPLES);
    if v.is_empty() {
        v.push(String::new());
    }
    v
}

fn parse_alt(st: &mut Exp) -> Vec<String> {
    let mut out = parse_seq(st);
    while st.pos < st.chars.len() && st.chars[st.pos] == '|' {
        st.pos += 1;
        let rhs = parse_seq(st);
        out.extend(rhs);
        if out.len() > MAX_EXAMPLES * 4 {
            out.truncate(MAX_EXAMPLES * 4);
        }
    }
    out
}

fn parse_seq(st: &mut Exp) -> Vec<String> {
    let mut acc = vec![String::new()];
    while st.pos < st.chars.len() && st.chars[st.pos] != '|' && st.chars[st.pos] != ')' {
        let atom = parse_atom(st);
        // `?` optional only (callers guarantee no */+/{})
        let optional = st.pos < st.chars.len() && st.chars[st.pos] == '?';
        if optional {
            st.pos += 1;
        }
        let mut next = Vec::new();
        for prefix in acc.iter() {
            for a in atom.iter() {
                let mut s = prefix.clone();
                s.push_str(a);
                next.push(s);
                if next.len() > MAX_EXAMPLES * 4 {
                    break;
                }
            }
            if optional {
                next.push(prefix.clone());
            }
            if next.len() > MAX_EXAMPLES * 4 {
                break;
            }
        }
        acc = next;
    }
    acc
}

fn parse_atom(st: &mut Exp) -> Vec<String> {
    if st.pos >= st.chars.len() {
        return vec![String::new()];
    }
    let c = st.chars[st.pos];
    if c == '(' {
        // (?:...) or (...)
        st.pos += 1;
        if st.pos + 1 < st.chars.len() && st.chars[st.pos] == '?' && st.chars[st.pos + 1] == ':' {
            st.pos += 2;
        }
        let inner = parse_alt(st);
        // consume ')'
        if st.pos < st.chars.len() && st.chars[st.pos] == ')' {
            st.pos += 1;
        }
        return inner;
    }
    if c == '[' {
        return parse_class(st);
    }
    if c == '\\' {
        st.pos += 1;
        if st.pos >= st.chars.len() {
            return vec!["\\".to_string()];
        }
        let e = st.chars[st.pos];
        st.pos += 1;
        if e == 'u' {
            // \uXXXX or \u{...}
            if st.pos < st.chars.len() && st.chars[st.pos] == '{' {
                st.pos += 1;
                let mut hex = String::new();
                while st.pos < st.chars.len() && st.chars[st.pos] != '}' {
                    hex.push(st.chars[st.pos]);
                    st.pos += 1;
                }
                if st.pos < st.chars.len() {
                    st.pos += 1;
                }
                if let Ok(cp) = u32::from_str_radix(&hex, 16) {
                    if let Some(ch) = char::from_u32(cp) {
                        return vec![ch.to_string()];
                    }
                }
                return vec![String::new()];
            } else {
                let mut hex = String::new();
                for _ in 0..4 {
                    if st.pos < st.chars.len() && st.chars[st.pos].is_ascii_hexdigit() {
                        hex.push(st.chars[st.pos]);
                        st.pos += 1;
                    } else {
                        break;
                    }
                }
                if hex.len() == 4 {
                    if let Ok(cp) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = char::from_u32(cp) {
                            return vec![ch.to_string()];
                        }
                    }
                }
                return vec![e.to_string()];
            }
        }
        // other escapes: literal char (regex \. \$ etc. — none in dict, be safe)
        return vec![e.to_string()];
    }
    st.pos += 1;
    vec![c.to_string()]
}

fn parse_class(st: &mut Exp) -> Vec<String> {
    // st.chars[pos] == '['
    st.pos += 1;
    let mut negated = false;
    if st.pos < st.chars.len() && st.chars[st.pos] == '^' {
        negated = true;
        st.pos += 1;
    }
    let mut items: Vec<char> = Vec::new();
    let mut ranges: Vec<(char, char)> = Vec::new();
    while st.pos < st.chars.len() && st.chars[st.pos] != ']' {
        if st.chars[st.pos] == '\\' {
            st.pos += 1;
            if st.pos < st.chars.len() {
                items.push(st.chars[st.pos]);
                st.pos += 1;
            }
            continue;
        }
        // range a-z ?
        if st.pos + 2 < st.chars.len() && st.chars[st.pos + 1] == '-' && st.chars[st.pos + 2] != ']'
        {
            ranges.push((st.chars[st.pos], st.chars[st.pos + 2]));
            st.pos += 3;
            continue;
        }
        items.push(st.chars[st.pos]);
        st.pos += 1;
    }
    if st.pos < st.chars.len() {
        st.pos += 1; // consume ']'
    }
    if negated {
        // Cannot enumerate: caller should have forced singleton; return placeholder.
        return vec!["\u{10FFFF}".to_string()];
    }
    let mut out: Vec<String> = items.iter().map(|c| c.to_string()).collect();
    for (lo, hi) in ranges {
        // expand small ranges fully, cap large ones to endpoints + middle
        let mut lo_u = lo as u32;
        let hi_u = hi as u32;
        if hi_u.saturating_sub(lo_u) > 12 {
            out.push(lo.to_string());
            out.push(hi.to_string());
            if let Some(mid) = char::from_u32(lo_u + (hi_u - lo_u) / 2) {
                out.push(mid.to_string());
            }
            let _ = lo_u;
        } else {
            while lo_u <= hi_u {
                if let Some(ch) = char::from_u32(lo_u) {
                    out.push(ch.to_string());
                }
                lo_u += 1;
            }
        }
    }
    out.sort();
    out.dedup();
    out.truncate(16);
    out
}

// ---------- witness generation ----------

fn bare_cores(full_examples: &[String]) -> Vec<String> {
    let mut v = Vec::new();
    for e in full_examples {
        // strip edge spaces (pipeline padding); keep interior intact
        let t = e.trim_matches(' ');
        if !t.is_empty() {
            v.push(t.to_string());
        }
    }
    v.sort();
    v.dedup();
    v.truncate(MAX_EX_PER_PAIR);
    v
}

/// Synthetic inputs exercising pair (cores_a, cores_b): adjacency both orders,
/// spaceless concatenation both orders, char-level border overlaps, containment.
fn pair_inputs(cores_a: &[String], cores_b: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for ba in cores_a.iter().take(MAX_EX_PER_PAIR) {
        for bb in cores_b.iter().take(MAX_EX_PER_PAIR) {
            let ca: Vec<char> = ba.chars().collect();
            let cb: Vec<char> = bb.chars().collect();
            if ca.is_empty() || cb.is_empty() {
                continue;
            }
            out.push(format!(" {ba} {bb} "));
            out.push(format!(" {bb} {ba} "));
            out.push(format!(" {ba}{bb} "));
            out.push(format!(" {bb}{ba} "));
            // border overlaps
            let maxk = ca.len().min(cb.len()).min(12);
            for k in 1..=maxk {
                if ca[ca.len() - k..] == cb[..k] {
                    let s: String =
                        ca.iter().collect::<String>() + &cb[k..].iter().collect::<String>();
                    out.push(format!(" {s} "));
                }
                if cb[cb.len() - k..] == ca[..k] {
                    let s: String =
                        cb.iter().collect::<String>() + &ca[k..].iter().collect::<String>();
                    out.push(format!(" {s} "));
                }
            }
            // containment carriers
            if ba.contains(bb.as_str()) {
                out.push(format!(" {ba} "));
            }
            if bb.contains(ba.as_str()) {
                out.push(format!(" {bb} "));
            }
            if out.len() > 40 {
                break;
            }
        }
    }
    out.sort();
    out.dedup();
    out.truncate(48);
    out
}

/// Cascade probes for ordered pair (earlier A -> later B): strings S where
/// applying A creates (or extends into) a B match, so sequential applies B
/// but simultaneous single-pass misses it. Built by unapplying: wherever A's
/// output fragment occurs in (or border-overlaps) B's example, swap it back
/// to A's input fragment.
fn cascade_inputs(inout_a: &[(String, String)], full_b: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for (a_in, a_out) in inout_a.iter() {
        let ai = a_in.trim_matches(' ');
        let ao = a_out.trim_matches(' ');
        if ai.is_empty() || ao.is_empty() || ai == ao {
            continue;
        }
        let ao_c: Vec<char> = ao.chars().collect();
        for b_full in full_b.iter() {
            let b = b_full.trim_matches(' ');
            if b.is_empty() {
                continue;
            }
            // pure containment: B's example contains A's output -> pre-image
            if b.contains(ao) && ao.chars().count() >= 2 {
                let s = b.replacen(ao, ai, 1);
                out.push(format!(" {s} "));
            }
            // border overlaps between A's output and B's example
            let bc: Vec<char> = b.chars().collect();
            let maxk = ao_c.len().min(bc.len()).min(10);
            for k in 1..=maxk {
                if k < 2 && (ao_c.len() < 2 || bc.len() < 2) {
                    continue;
                }
                // suffix(output_A) == prefix(B): T completes B; S unapplies
                if ao_c[ao_c.len() - k..] == bc[..k] {
                    let t: String =
                        ao_c.iter().collect::<String>() + &bc[k..].iter().collect::<String>();
                    let _ = &t;
                    let s: String = ai.to_string() + &bc[k..].iter().collect::<String>();
                    out.push(format!(" {s} "));
                }
                // prefix(output_A) == suffix(B)
                if ao_c[..k] == bc[bc.len() - k..] {
                    let s: String = bc[..bc.len() - k].iter().collect::<String>() + ai;
                    out.push(format!(" {s} "));
                }
            }
            if out.len() > 60 {
                break;
            }
        }
    }
    out.sort();
    out.dedup();
    out.truncate(64);
    out
}

fn dict_subset(entries: &[(String, String)]) -> CompiledDict {
    let v: Vec<DictEntry> = entries
        .iter()
        .map(|(p, r)| DictEntry {
            pattern: p.clone(),
            result: r.clone(),
        })
        .collect();
    CompiledDict::new(&v)
}

fn batch_subset(entries: &[(String, String)]) -> CompiledDict {
    let v: Vec<DictEntry> = entries
        .iter()
        .map(|(p, r)| DictEntry {
            pattern: p.clone(),
            result: r.clone(),
        })
        .collect();
    // Two outer vecs so the candidate is a single-PASS head (with an empty
    // sequential tail): a lone `[v]` would make it the sequential tail and
    // compare seq-vs-seq (always equal) — the bug that merged everything.
    CompiledDict::new_batched_entries(&[v, Vec::new()])
}

fn main() {
    let flat: Vec<(String, String)> = FLAT
        .iter()
        .map(|(p, r)| (p.to_string(), r.to_string()))
        .collect();
    println!("flat entries: {}", flat.len());
    for (p, r) in flat.iter() {
        assert!(
            !p.contains('"') && !r.contains('"'),
            "raw-literal-unsafe {p:?}"
        );
    }

    // --- classify ---
    let mut n_lit = 0;
    let mut n_force = 0;
    let mut neg_class = Vec::new();
    for (p, _) in flat.iter() {
        if !has_meta(p) {
            n_lit += 1;
        }
        if force_singleton(p) {
            n_force += 1;
        }
        if p.contains("[^") {
            neg_class.push(p.clone());
        }
    }
    println!("literals: {n_lit}, force-singleton regex: {n_force}");
    println!(
        "negated classes ({}): {:?}",
        neg_class.len(),
        neg_class.iter().take(5).collect::<Vec<_>>()
    );

    // --- precompute examples (full match strings) + bare cores ---
    let mut full_ex: Vec<Vec<String>> = Vec::with_capacity(flat.len());
    let mut cores: Vec<Vec<String>> = Vec::with_capacity(flat.len());
    for (p, _) in flat.iter() {
        if force_singleton(p) {
            full_ex.push(Vec::new());
            cores.push(Vec::new());
            continue;
        }
        let mut ex = if !has_meta(p) {
            vec![p.clone()]
        } else {
            expand_pattern(p)
        };
        ex.sort();
        ex.dedup();
        ex.truncate(MAX_EXAMPLES);
        let c = bare_cores(&ex);
        full_ex.push(ex);
        cores.push(c);
    }

    // --- corpus sample chunks (wordlist-stage inputs would be padded/collapsed;
    // use raw chunks: subset dicts operate on same strings for seq vs batch) ---
    let sample_chunks: Vec<String> = match std::fs::read(CORPUS) {
        Ok(bytes) => {
            let s = String::from_utf8_lossy(&bytes).into_owned();
            let mut chunks = Vec::new();
            let mut i = 0;
            while i < s.len() && chunks.len() < 12 {
                let e = (i + CHUNK).min(s.len());
                // snap to char boundary
                let mut e2 = e;
                while e2 < s.len() && !s.is_char_boundary(e2) {
                    e2 += 1;
                }
                chunks.push(s[i..e2].to_string());
                i = e2;
            }
            chunks
        }
        Err(e) => {
            println!("no corpus ({e}), using empty sample");
            Vec::new()
        }
    };
    println!("sample chunks: {}", sample_chunks.len());

    let tricky = vec![
        " альвеоладвент ".to_string(),
        " брэстам ".to_string(),
        " брэста ".to_string(),
        " ласо ".to_string(),
        " ганконг ".to_string(),
        " ґанконг ".to_string(),
    ];

    // --- per-entry in->out pairs (single-entry sequential) for cascade probes ---
    // Only for batchable entries; forced singletons get empty vecs.
    let mut inout: Vec<Vec<(String, String)>> = Vec::with_capacity(flat.len());
    for (i, (p, r)) in flat.iter().enumerate() {
        if force_singleton(p) {
            inout.push(Vec::new());
            continue;
        }
        let single = CompiledDict::new(&[DictEntry {
            pattern: p.clone(),
            result: r.clone(),
        }]);
        let mut v = Vec::new();
        for ex in full_ex[i].iter().take(6) {
            let o = single.replace_all(ex);
            if o != *ex {
                v.push((ex.clone(), o));
            }
        }
        v.truncate(6);
        inout.push(v);
    }

    // --- greedy batching ---
    // batches hold flat indices; last entry (\ue0ff cleanup) forced to sequential tail.
    let last = flat.len() - 1;
    assert_eq!(
        flat[last].0, "\\ue0ff",
        "last must be cleanup, got {:?}",
        flat[last].0
    );
    let mut batches: Vec<Vec<usize>> = Vec::new();
    let mut cur: Vec<usize> = Vec::new();

    let subset_entries =
        |ids: &[usize]| -> Vec<(String, String)> { ids.iter().map(|&i| flat[i].clone()).collect() };

    for idx in 0..last {
        let p = &flat[idx].0;
        // forced singletons always start a new batch (alone)
        if force_singleton(p) {
            if !cur.is_empty() {
                batches.push(std::mem::take(&mut cur));
            }
            batches.push(vec![idx]);
            continue;
        }
        if cur.is_empty() {
            cur.push(idx);
            continue;
        }
        // candidate = cur + idx; test pairwise synthetics of idx vs each member
        // plus sample chunks + tricky, comparing subset-seq vs subset-batch.
        let mut cand = cur.clone();
        cand.push(idx);
        let seq_d = dict_subset(&subset_entries(&cand));
        let bat_d = batch_subset(&subset_entries(&cand));
        let mut ok = true;
        // pairwise synthetic inputs (overlap/adjacency) + cascade probes
        'outer: for &m in cur.iter() {
            // skip forced-singleton members? they never share (each alone) — cur only
            // holds batchable entries by construction.
            let inputs = pair_inputs(&cores[m], &cores[idx]);
            for s in inputs.iter() {
                if seq_d.replace_all(s) != bat_d.replace_all(s) {
                    ok = false;
                    break 'outer;
                }
            }
            // cascade: m (earlier) -> idx (later)
            let cinputs = cascade_inputs(&inout[m], &full_ex[idx]);
            for s in cinputs.iter() {
                if seq_d.replace_all(s) != bat_d.replace_all(s) {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            for s in sample_chunks.iter().chain(tricky.iter()) {
                if seq_d.replace_all(s) != bat_d.replace_all(s) {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            cur.push(idx);
        } else {
            batches.push(std::mem::take(&mut cur));
            cur.push(idx);
        }
        if idx % 200 == 0 {
            println!(
                "... {idx}/{} batches={} cur={}",
                last,
                batches.len(),
                cur.len()
            );
        }
    }
    if !cur.is_empty() {
        batches.push(cur);
    }
    println!("batches (excl tail): {}, sizes:", batches.len());
    let mut hist = [0usize; 16];
    for b in batches.iter() {
        hist[b.len().min(15)] += 1;
    }
    println!("size-hist[1..=15+]: {:?}", &hist[1..]);

    // --- write wordlist.rs ---
    let mut cuts: Vec<usize> = vec![0];
    let mut acc = 0;
    for b in batches.iter() {
        acc += b.len();
        cuts.push(acc);
    }
    // cuts cover [0..last); tail [last..flat.len()) sequential
    assert_eq!(acc, last, "batches must cover 0..{last}, got {acc}");
    let mut out = String::new();
    out.push_str(
        "//! GENERATED FILE — do not modify directly.\n//! Derivative of `flat_wordlist.rs`: same entries, re-split into single-pass\n//! batches of fully independent patterns (sequential tail last).\n//! To change the dictionary, edit `flat_wordlist.rs` and regenerate with:\n//! `cargo run --release --example resplit`.\n",
    );
    out.push_str("crate::dict_batches! {\n");
    out.push_str("    pub WORD_LIST:\n");
    for w in cuts.windows(2) {
        let (s, e) = (w[0], w[1]);
        let _ = writeln!(out, "    batch [ // [{s}..{e}) ({})", e - s);
        for i in s..e {
            let (p, r) = &flat[i];
            let _ = writeln!(out, "        (r\"{p}\", r\"{r}\"),");
        }
        out.push_str("    ],\n");
    }
    {
        let (p, r) = &flat[last];
        let _ = writeln!(out, "    sequential [ // SEQ [{last}..{})", last + 1);
        let _ = writeln!(out, "        (r\"{p}\", r\"{r}\"),");
        out.push_str("    ],\n");
    }
    out.push_str("}\n");
    std::fs::write(WORDLIST_RS, &out).expect("write wordlist.rs");
    println!("wrote {WORDLIST_RS} ({} batches + tail)", cuts.len() - 1);

    // --- full verification: full-seq vs full-batched on chunks + tricky + singles ---
    let flat_entries: Vec<DictEntry> = flat
        .iter()
        .map(|(p, r)| DictEntry {
            pattern: p.clone(),
            result: r.clone(),
        })
        .collect();
    let full_seq = CompiledDict::new(&flat_entries);
    // rebuild batched from cuts
    let mut bv: Vec<Vec<DictEntry>> = Vec::new();
    for w in cuts.windows(2) {
        bv.push(flat_entries[w[0]..w[1]].to_vec());
    }
    bv.push(vec![flat_entries[last].clone()]);
    let full_bat = CompiledDict::new_batched_entries(&bv);
    let mut bad = 0;
    for (i, c) in sample_chunks.iter().enumerate() {
        let a = full_seq.replace_all(c);
        let b = full_bat.replace_all(c);
        if a != b {
            bad += 1;
            if bad <= 3 {
                println!("chunk {i} DIFFERS (seq {} vs bat {})", a.len(), b.len());
            }
        }
    }
    println!("full-corpus-sample diffs: {bad}/{}", sample_chunks.len());
    for t in tricky.iter() {
        let a = full_seq.replace_all(t);
        let b = full_bat.replace_all(t);
        println!("tricky {t:?} seq={a:?} bat={b:?} equal={}", a == b);
    }
    // nested-paren check on single words (bare cores without interior spaces)
    let mut nest = 0;
    let mut checked = 0;
    for (i, c) in cores.iter().enumerate() {
        for w in c.iter() {
            if w.contains(' ') || w.contains('(') || w.contains('|') {
                continue; // multi-word / already variation-shaped; single words only
            }
            let s = format!(" {w} ");
            let o = full_seq.replace_all(&s);
            checked += 1;
            if o.contains("((") {
                nest += 1;
                if nest <= 10 {
                    println!("NESTED {w:?} via [{i}] {:?} -> {o:?}", flat[i].0);
                }
            }
        }
    }
    println!("nested-paren singles: {nest}/{checked}");
    if bad > 0 {
        println!("RESULT: BATCHED != SEQUENTIAL on corpus sample");
        std::process::exit(1);
    }
}
