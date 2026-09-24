use crate::{
    config::{Alphabet, JMode},
    pipeline::{helpers::regex_replace_all_with, PipelineContext},
};

pub fn step_replace_i_by_j(ctx: &mut PipelineContext) {
    if ctx.cfg.j == JMode::Never || ctx.cfg.abc == Alphabet::LatinJi {
        return;
    }
    let text = std::mem::take(&mut ctx.text);
    if ctx.cfg.j == JMode::Always {
        ctx.text = regex_replace_all_with(&text, r"(?<=[аеёіоуыэюя] )і (ў?)", |caps| {
            let short_u = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            if !short_u.is_empty() {
                "й у".to_string()
            } else {
                "й ".to_string()
            }
        });
    } else {
        ctx.text = regex_replace_all_with(&text, r"(?<=[аеёіоуыэюя] )і (ў?)", |caps| {
            if rand::random::<f64>() >= 0.5 {
                let short_u = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                if !short_u.is_empty() {
                    "й у".to_string()
                } else {
                    "й ".to_string()
                }
            } else {
                caps.get(0)
                    .map_or(String::new(), |m| m.as_str().to_string())
            }
        });
    }
}
