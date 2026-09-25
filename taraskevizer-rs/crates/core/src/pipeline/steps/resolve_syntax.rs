use crate::{
    pipeline::{
        helpers::{apply_abc_lower, apply_abc_upper, find_unescaped_gt},
        steps::restore_case::restore_case_words,
        PipelineContext,
    },
    text::is_lu,
};

pub fn step_resolve_special_syntax(ctx: &mut PipelineContext) {
    let do_escape = ctx.cfg.do_escape_capitalized;
    let no_fix_ph = &ctx.cfg.no_fix_placeholder;
    let abc = ctx.cfg.abc;

    let text = &ctx.text;
    let mut result = String::with_capacity(text.len());
    let no_fix = &mut ctx.no_fix_arr;
    let mut i = 0;
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let len = chars.len();

    while i < len {
        let (byte_pos, ch) = chars[i];

        if ch == '<' {
            let rest = &text[byte_pos + ch.len_utf8()..];
            if let Some(end_rel) = find_unescaped_gt(rest) {
                let inner_end_byte = byte_pos + 1 + end_rel;
                let inner = &text[byte_pos + 1..inner_end_byte];
                let mut clean_inner = String::new();
                let mut j = 0;
                let cchars: Vec<(usize, char)> = inner.char_indices().collect();
                while j < cchars.len() {
                    let (_, c) = cchars[j];
                    if c == '\\' && j + 1 < cchars.len() && cchars[j + 1].1 == '>' {
                        clean_inner.push('>');
                        j += 2;
                    } else {
                        clean_inner.push(c);
                        j += 1;
                    }
                }
                let ic = clean_inner.chars().collect::<Vec<_>>();
                if ic.is_empty() {
                    result.push('<');
                    result.push('>');
                    let inner_end_char_idx = text[..inner_end_byte + 1].chars().count();
                    i = inner_end_char_idx;
                    continue;
                }
                let is_abc = ic.first() == Some(&'*');
                let char_offset = if is_abc { 1 } else { 0 };
                let do_remove = ic.get(char_offset) == Some(&'.');
                let do_tarask = ic.get(char_offset) == Some(&',');
                let content_start = is_abc as usize + do_remove as usize + do_tarask as usize;
                if content_start < ic.len() {
                    let real_content: String = ic[content_start..].iter().collect();

                    if do_tarask {
                        result.push('<');
                        result.push_str(&real_content);
                        result.push('>');
                    } else if is_abc {
                        let lowered = apply_abc_lower(&real_content, abc);
                        let converted = apply_abc_upper(&lowered, abc).unwrap_or(lowered);
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
                let inner_end_char_idx = text[..inner_end_byte + 1].chars().count();
                i = inner_end_char_idx;
                continue;
            }
        }

        // Caps-escape, mirroring the JS
        // `/(?!<=\p{Lu} )\p{Lu}{2}[\p{Lu} ]*(?!= \p{Lu})/gu`: the leading
        // guard is vacuous (a match must start with two Lu, never `<=`),
        // so this is a greedy Lu/space run of length ≥ 2, backed off by
        // one char when it ends right before `= Lu`. One match is one
        // stash (no resume-inside-match: `У ХХІ` escapes `ХХІ ` whole,
        // never `Х` + `ХІ `).
        if do_escape && is_lu(ch as u32) && i + 1 < len && is_lu(chars[i + 1].1 as u32) {
            let start = i;
            let mut end = i + 2;
            while end < len && (chars[end].1 == ' ' || is_lu(chars[end].1 as u32)) {
                end += 1;
            }
            // `(?!= \p{Lu})`: `=`, space, uppercase right after the run.
            let cut_off = end < len
                && chars[end].1 == '='
                && end + 1 < len
                && chars[end + 1].1 == ' '
                && end + 2 < len
                && is_lu(chars[end + 2].1 as u32);
            if cut_off {
                if end == start + 2 {
                    // Below the two-letter minimum: no match here.
                    result.push(ch);
                    i += 1;
                    continue;
                }
                end -= 1;
            }
            let word: String = chars[start..end]
                .iter()
                .map(|(_, c)| c.to_string())
                .collect::<Vec<_>>()
                .join("");
            // JS `convertAlphavet`: lower + `restoreCase` against the
            // original (NOT full-upper + `to_uppercase`: restoring copies
            // the original word on lowercase-equality, preserving
            // letters like Turkish `İ`).
            let lowered = word.to_lowercase();
            let conv = apply_abc_lower(&lowered, abc);
            let mut text_words: Vec<String> = conv.split(' ').map(|s| s.to_string()).collect();
            let orig_words: Vec<String> = word.split(' ').map(|s| s.to_string()).collect();
            restore_case_words(&mut text_words, &orig_words);
            no_fix.push(text_words.join(" "));
            result.push_str(no_fix_ph);
            i = end;
            continue;
        }

        result.push(ch);
        i += 1;
    }

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
