//! First-letter capitalization, including `(a|b)` variation-list forms.

use super::regex_replace_all_with;

pub(crate) fn initcap(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

pub(crate) fn initcap_var(word: &str) -> String {
    regex_replace_all_with(word, r"[^(|)]*[|)]", |caps, out| {
        let m = caps.get(0).map_or("", |m| m.as_str());
        out.push_str(&initcap(m));
    })
}

#[cfg(test)]
mod tests {
    use super::{initcap, initcap_var};

    #[test]
    fn capitalizes_first() {
        assert_eq!(initcap("планета"), "Планета");
        assert_eq!(initcap(""), "");
        assert_eq!(initcap("Планета"), "Планета");
    }

    #[test]
    fn capitalizes_each_variation() {
        assert_eq!(initcap_var("(брэст|берасьце)"), "(Брэст|Берасьце)");
    }

    #[test]
    fn plain_word_passthrough() {
        assert_eq!(initcap_var("планета"), "планета");
    }
}
