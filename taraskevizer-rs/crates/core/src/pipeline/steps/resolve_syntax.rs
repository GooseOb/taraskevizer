use crate::pipeline::{
    helpers::{alphabet_dict, find_unescaped_gt},
    PipelineContext,
};

pub fn step_resolve_special_syntax(ctx: &mut PipelineContext) {
    let do_escape = ctx.cfg.do_escape_capitalized;
    let no_fix_ph = &ctx.cfg.no_fix_placeholder;
    let abc = ctx.cfg.abc;
    let abc_lower = alphabet_dict(abc, "lower");
    let abc_upper = alphabet_dict(abc, "upper");

    let text = std::mem::take(&mut ctx.text);
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
                        let converted = abc_lower.replace_all(&real_content);
                        let converted = if abc_upper.has_entries() {
                            abc_upper.replace_all(&converted)
                        } else {
                            converted
                        };
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

        if do_escape && ch.is_uppercase() {
            let start = i;
            let mut upper_count = 0u32;
            while i < len {
                let c = chars[i].1;
                if c.is_uppercase() {
                    upper_count += 1;
                    i += 1;
                } else if c == ' ' && upper_count >= 2 {
                    i += 1;
                } else {
                    break;
                }
            }
            if upper_count >= 2 {
                let preceded_by_upper_space =
                    start > 1 && chars[start - 1].1 == ' ' && chars[start - 2].1.is_uppercase();
                let after = i;
                let followed_by_space_upper = after < len
                    && chars[after].1 == ' '
                    && after + 1 < len
                    && chars[after + 1].1.is_uppercase();
                if !preceded_by_upper_space && !followed_by_space_upper {
                    let word: String = chars[start..i]
                        .iter()
                        .map(|(_, c)| c.to_uppercase().to_string())
                        .collect::<Vec<_>>()
                        .join("");
                    let lowered = word.to_lowercase();
                    let conv = abc_lower.replace_all(&lowered);
                    let conv = if abc_upper.has_entries() {
                        abc_upper.replace_all(&conv)
                    } else {
                        conv
                    };
                    no_fix.push(conv.to_uppercase());
                    result.push_str(no_fix_ph);
                    continue;
                }
                i = start;
            } else {
                i = start;
            }
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
    let text = std::mem::take(&mut ctx.text);
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
