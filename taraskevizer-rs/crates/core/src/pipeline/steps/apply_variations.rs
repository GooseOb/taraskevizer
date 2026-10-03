use crate::{
    config::{VariationMode, VariationWrappers},
    pipeline::PipelineContext,
};

/// Push the replacement for one `(…)` variation match into `out` (borrowed
/// slices, no per-match allocation except the wrapper round-trip when
/// wrappers are configured).
fn push_variation(
    matched: &str,
    mode: VariationMode,
    wrap: Option<&VariationWrappers>,
    out: &mut String,
) {
    match mode {
        VariationMode::No => {
            if let Some(w) = wrap {
                out.push_str(&(w.no)(matched));
            } else {
                // `^\(([^|]*)`: text after `(` up to the first `|` (or to
                // the end when there is none — the regex has no trailing
                // anchor, so `(a)` yields `a)`).
                match matched.find('|') {
                    Some(pipe) => out.push_str(&matched[1..pipe]),
                    None => out.push_str(&matched[1..]),
                }
            }
        }
        VariationMode::First => {
            if let Some(w) = wrap {
                out.push_str(&(w.first)(matched));
            } else {
                // `^[^|]*?\|([^|)]*)`: after the first `|`, up to the next
                // `|`/`)`. No `|` → no match → unchanged.
                match matched.find('|') {
                    Some(pipe) => {
                        let rest = &matched[pipe + 1..];
                        let end = rest.find(['|', ')']).unwrap_or(rest.len());
                        out.push_str(&rest[..end]);
                    }
                    None => out.push_str(matched),
                }
            }
        }
        VariationMode::All => {
            if let Some(w) = wrap {
                out.push_str(&(w.all)(matched));
            } else {
                out.push_str(matched);
            }
        }
    }
}

pub fn step_apply_variations(ctx: &mut PipelineContext) {
    // Manual scan for `\([^)]*?\)` (no regex engine): at each `(` the match
    // runs to the first `)` after it; a `(` with no later `)` never matches.
    // Leftmost, non-overlapping — exactly the regex semantics — with zero
    // per-match engine allocations (the old path spent ~1 alloc per hit
    // inside `captures_iter` on top of the callback temporaries).
    let text = &ctx.text;
    let Some(first_open) = text.find('(') else {
        return;
    };
    if !text[first_open..].contains(')') {
        return;
    }
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut flush = 0usize;
    let mut i = first_open;
    while i < bytes.len() {
        if bytes[i] != b'(' {
            // Jump between candidates with SIMD instead of stepping bytes.
            let rest = &text[i..];
            match rest.find('(') {
                Some(rel) => i += rel,
                None => break,
            }
        }
        // `i` is at `(` (1 byte, so `i + 1` is a boundary).
        let after = &text[i + 1..];
        match after.find(')') {
            Some(rel) => {
                let end = i + 1 + rel + 1; // one past `)`.
                out.push_str(&text[flush..i]);
                push_variation(
                    &text[i..end],
                    ctx.cfg.variations,
                    ctx.cfg.wrappers.as_ref().map(|w| &w.variable),
                    &mut out,
                );
                flush = end;
                i = end;
            }
            // No closing `)`: literal `(`, like the regex engine (which would
            // fail the match here and resume past it).
            None => i += 1,
        }
    }
    out.push_str(&text[flush..]);
    ctx.text = out;
}
