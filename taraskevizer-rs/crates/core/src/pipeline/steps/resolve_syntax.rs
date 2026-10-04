use crate::{
    pipeline::{
        helpers::{apply_abc_lower, apply_abc_upper, find_unescaped_gt},
        steps::restore_case::restore_case_words,
        PipelineContext, {join_text_words, split_text_words},
    },
    text::is_lu,
};

/// Whether `ch` is uppercase (`\p{Lu}`).
///
/// ASCII resolves to `is_ascii_uppercase` (exactly `\p{Lu}` below U+0080);
/// only non-ASCII pays for `is_lu`.
#[inline]
fn is_upper(ch: char) -> bool {
    if ch.is_ascii() {
        ch.is_ascii_uppercase()
    } else {
        is_lu(ch as u32)
    }
}

/// Whether `step_resolve_special_syntax` can change `text` at all.
///
/// Without `<` there are no tags to resolve; without two consecutive
/// uppercase chars caps-escape (`do_escape`) has nothing to stash —
/// otherwise the output is byte-identical input and the whole step
/// (incl. the `result` allocation) can be skipped.
fn needs_resolve(text: &str, do_escape: bool) -> bool {
    // Single-ASCII-char `contains` compiles to a memchr scan.
    if text.contains('<') {
        return true;
    }
    if !do_escape {
        return false;
    }
    let mut prev_lu = false;
    let mut i = 0;
    while i < text.len() {
        // `i` always sits on a char boundary (see main loop below).
        let ch = text[i..].chars().next().unwrap();
        let lu = is_upper(ch);
        if lu && prev_lu {
            return true;
        }
        prev_lu = lu;
        i += ch.len_utf8();
    }
    false
}

pub fn step_resolve_special_syntax(ctx: &mut PipelineContext) {
    let do_escape = ctx.cfg.do_escape_capitalized;
    let no_fix_ph = &ctx.cfg.no_fix_placeholder;
    let abc = ctx.cfg.abc;

    let text = &ctx.text;
    if !needs_resolve(text, do_escape) {
        return;
    }
    let mut result = String::with_capacity(text.len());
    let no_fix = &mut ctx.no_fix_arr;
    // `flush`/`p` are byte offsets that always sit on char boundaries:
    // every advance is `len_utf8` (or past ASCII, which is 1 byte).
    // Plain runs between events are copied with one `push_str` (memcpy)
    // instead of per-char `push` + branch + uppercase check.
    let mut flush = 0usize;
    let mut p = 0usize;

    while p < text.len() {
        let ch = text[p..].chars().next().unwrap();

        if ch == '<' {
            // `<` is 1 byte, so `p + 1` is a boundary.
            let rest = &text[p + 1..];
            if let Some(end_rel) = find_unescaped_gt(rest) {
                let inner_end_byte = p + 1 + end_rel;
                let inner = &text[p + 1..inner_end_byte];
                result.push_str(&text[flush..p]);
                // Single pass: unescape `\>` straight into chars (tags are
                // short, but avoid the old double-collect).
                let mut ic = Vec::with_capacity(inner.len());
                let mut inner_chars = inner.chars().peekable();
                while let Some(c) = inner_chars.next() {
                    if c == '\\' && inner_chars.peek() == Some(&'>') {
                        inner_chars.next();
                        ic.push('>');
                    } else {
                        ic.push(c);
                    }
                }
                if ic.is_empty() {
                    result.push('<');
                    result.push('>');
                    // Skip past `>` (ASCII, 1 byte).
                    p = inner_end_byte + 1;
                    flush = p;
                    continue;
                }
                let is_abc = ic.first() == Some(&'*');
                let char_offset = usize::from(is_abc);
                let do_remove = ic.get(char_offset) == Some(&'.');
                let do_tarask = ic.get(char_offset) == Some(&',');
                let content_start =
                    usize::from(is_abc) + usize::from(do_remove) + usize::from(do_tarask);
                if content_start < ic.len() {
                    let real_content: String = ic[content_start..].iter().collect();

                    if do_tarask {
                        result.push('<');
                        result.push_str(&real_content);
                        result.push('>');
                    } else if is_abc {
                        let lowered = apply_abc_lower(&real_content, abc);
                        let converted = apply_abc_upper(&lowered, abc).into_owned();
                        no_fix.push(converted);
                        if do_remove {
                            result.push_str(no_fix_ph);
                        } else {
                            result.push('<');
                            result.push_str(no_fix_ph);
                            result.push('>');
                        }
                    } else {
                        no_fix.push(real_content);
                        if do_remove {
                            result.push_str(no_fix_ph);
                        } else {
                            result.push('<');
                            result.push_str(no_fix_ph);
                            result.push('>');
                        }
                    }
                }
                p = inner_end_byte + 1;
                flush = p;
                continue;
            }
            // No closing `>`: literal `<`, stays part of the plain run.
            p += 1;
            continue;
        }

        if !do_escape || !is_upper(ch) {
            p += ch.len_utf8();
            continue;
        }
        let np = p + ch.len_utf8();
        let pair = np < text.len() && is_upper(text[np..].chars().next().unwrap());
        if !pair {
            p += ch.len_utf8();
            continue;
        }

        // Caps-escape, mirroring the JS
        // `/(?!<=\p{Lu} )\p{Lu}{2}[\p{Lu} ]*(?!= \p{Lu})/gu`: the leading
        // guard is vacuous (a match must start with two Lu, never `<=`),
        // so this is a greedy Lu/space run of length ≥ 2, backed off by
        // one char when it ends right before `= Lu`. One match is one
        // stash (no resume-inside-match: `У ХХІ` escapes `ХХІ ` whole,
        // never `Х` + `ХІ `).
        result.push_str(&text[flush..p]);
        let start = p;
        let second = text[np..].chars().next().unwrap();
        let mut end = np + second.len_utf8();
        let mut run_chars = 2usize;
        let mut last_len = second.len_utf8();
        while end < text.len() {
            let ech = text[end..].chars().next().unwrap();
            if ech == ' ' {
                end += 1;
                last_len = 1;
                run_chars += 1;
                continue;
            }
            if !is_upper(ech) {
                break;
            }
            end += ech.len_utf8();
            last_len = ech.len_utf8();
            run_chars += 1;
        }
        // `(?!= \p{Lu})`: `=`, space, uppercase right after the run.
        // `=` and ` ` are ASCII, so the offsets below are boundaries.
        let mut cut_off = false;
        if end < text.len()
            && text[end..].starts_with('=')
            && end + 1 < text.len()
            && text[end + 1..].starts_with(' ')
            && end + 2 < text.len()
        {
            cut_off = is_upper(text[end + 2..].chars().next().unwrap());
        }
        if cut_off {
            if run_chars == 2 {
                // Below the two-letter minimum: no match here.
                result.push_str(&text[p..p + ch.len_utf8()]);
                p += ch.len_utf8();
                flush = p;
                continue;
            }
            // Back off one *char* (may be multibyte — hence `last_len`).
            end -= last_len;
        }
        let word: String = text[start..end].to_string();
        // JS `convertAlphavet`: lower + `restoreCase` against the
        // original (NOT full-upper + `to_uppercase`: restoring copies
        // the original word on lowercase-equality, preserving
        // letters like Turkish `İ`).
        let lowered = word.to_lowercase();
        // `Cow`: no clone for the default cyrillic alphabet.
        let conv = apply_abc_lower(&lowered, abc);
        // Copy-on-write spans over `conv` (no per-word alloc); `word` splits
        // lazily inside `restore_case_words`.
        let mut text_words = split_text_words(&conv);
        restore_case_words(&mut text_words, &conv, &word);
        no_fix.push(join_text_words(&text_words, &conv));
        result.push_str(no_fix_ph);
        p = end;
        flush = end;
    }

    result.push_str(&text[flush..]);
    ctx.text = result;
}

pub fn step_apply_no_fix(ctx: &mut PipelineContext) {
    if ctx.no_fix_arr.is_empty() {
        return;
    }
    let ph = &ctx.cfg.no_fix_placeholder;
    let text = &ctx.text;
    let mut parts = text.split(ph);
    let mut result = String::new();
    if let Some(first) = parts.next() {
        result.push_str(first);
    }
    for orig in ctx.no_fix_arr.drain(..) {
        result.push_str(&orig);
        if let Some(part) = parts.next() {
            result.push_str(part);
        }
    }
    ctx.text = result;
}
