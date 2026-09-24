use crate::pipeline::PipelineContext;

pub fn step_whitespaces_to_spaces(ctx: &mut PipelineContext) {
    let text = std::mem::take(&mut ctx.text);
    ctx.spaces.clear();
    let mut result = String::with_capacity(text.len());
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (_, ch) = chars[i];
        if ch.is_whitespace() {
            let start = chars[i].0;
            let mut end = start;
            while i < chars.len() && chars[i].1.is_whitespace() {
                end = chars[i].0 + chars[i].1.len_utf8();
                i += 1;
            }
            ctx.spaces.push(text[start..end].to_string());
            result.push(' ');
        } else {
            result.push(ch);
            i += 1;
        }
    }
    ctx.text = result;
}

pub fn step_restore_whitespaces(ctx: &mut PipelineContext) {
    let text = std::mem::take(&mut ctx.text);
    ctx.spaces.reverse();
    let mut result = String::with_capacity(text.len());
    for ch in text.chars() {
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
