use crate::{
    config::VariationMode,
    pipeline::{helpers::regex_replace_all_with, PipelineContext},
};

pub fn step_apply_variations(ctx: &mut PipelineContext) {
    let mode = ctx.cfg.variations;
    let wrap = ctx.cfg.wrappers.as_ref().map(|w| &w.variable);
    let text = std::mem::take(&mut ctx.text);
    ctx.text = regex_replace_all_with(&text, r"\([^)]*?\)", |caps| {
        let matched = caps.get(0).map_or("", |m| m.as_str());
        match mode {
            VariationMode::No => {
                if let Some(w) = wrap {
                    (w.no)(matched)
                } else {
                    fancy_regex::Regex::new(r"^\(([^|]*)")
                        .ok()
                        .and_then(|re| re.captures(matched).ok())
                        .flatten()
                        .and_then(|c| c.get(1))
                        .map_or(matched.to_string(), |m| m.as_str().to_string())
                }
            }
            VariationMode::First => {
                if let Some(w) = wrap {
                    (w.first)(matched)
                } else {
                    fancy_regex::Regex::new(r"^[^|]*?\|([^|)]*)")
                        .ok()
                        .and_then(|re| re.captures(matched).ok())
                        .flatten()
                        .and_then(|c| c.get(1))
                        .map_or(matched.to_string(), |m| m.as_str().to_string())
                }
            }
            VariationMode::All => wrap
                .map(|w| (w.all)(matched))
                .unwrap_or_else(|| matched.to_string()),
        }
    });
}
