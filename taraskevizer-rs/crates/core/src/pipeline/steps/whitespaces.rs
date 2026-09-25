use crate::pipeline::PipelineContext;

pub fn step_whitespaces_to_spaces(ctx: &mut PipelineContext) {
    ctx.spaces.clear();
    let mut result = String::with_capacity(ctx.text.len());
    let mut chars = ctx.text.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        if ch.is_whitespace() {
            let mut end = start + ch.len_utf8();
            while let Some(&(next_pos, next_ch)) = chars.peek() {
                if !next_ch.is_whitespace() {
                    break;
                }
                end = next_pos + next_ch.len_utf8();
                chars.next();
            }
            ctx.spaces.push(ctx.text[start..end].to_string());
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    ctx.text = result;
}

pub fn step_restore_whitespaces(ctx: &mut PipelineContext) {
    ctx.spaces.reverse();
    let mut result = String::with_capacity(ctx.text.len());
    for ch in ctx.text.chars() {
        if ch == ' ' {
            if let Some(s) = ctx.spaces.pop() {
                result.push_str(&s);
            } else {
                result.push(' ');
            }
        } else {
            result.push(ch);
        }
    }
    ctx.text = result;
}
