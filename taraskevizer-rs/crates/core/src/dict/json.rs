//! Serialization of dictionary entries back to the `{p, r}` JSON format.
//!
//! The layout matches the JSON emitted by the TypeScript build
//! (`generate-dict-json.mjs`): a tab-indented array, one entry per line,
//! with entries wider than the 80-column budget expanded over several lines.

/// Width of one indentation level, in columns.
const INDENT_WIDTH: usize = 2;
/// Maximum width of a line before an object gets expanded.
const LINE_WIDTH: usize = 80;

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            // Other control characters only; everything else (including the
            // non-ASCII letters and the U+E0FF soft-sign marker) stays literal.
            '\u{0}'..='\u{1f}' => out.push_str(&format!("\\u{:04x}", ch as u32)),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// Render entries as JSON: `[{"p": ..., "r": ...}, ...]`.
pub fn to_json(entries: &[(&str, &str)]) -> String {
    let mut out = String::from("[\n");
    for (i, (pattern, result)) in entries.iter().enumerate() {
        let one_line = format!(
            "\t{{ \"p\": {}, \"r\": {} }}",
            escape(pattern),
            escape(result)
        );
        // The leading tab is worth INDENT_WIDTH columns, like the TS formatter.
        if one_line.chars().count() + INDENT_WIDTH - 1 <= LINE_WIDTH {
            out.push_str(&one_line);
        } else {
            out.push_str(&format!(
                "\t{{\n\t\t\"p\": {},\n\t\t\"r\": {}\n\t}}",
                escape(pattern),
                escape(result)
            ));
        }
        if i + 1 < entries.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("]\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(to_json(&[]), "[\n]\n");
    }

    #[test]
    fn single_entry() {
        assert_eq!(
            to_json(&[("сш", "шш")]),
            "[\n\t{ \"p\": \"сш\", \"r\": \"шш\" }\n]\n"
        );
    }

    #[test]
    fn escapes() {
        assert_eq!(
            to_json(&[(r"(\S\S[дт])р ", " $1\u{e0ff}р ")]),
            "[\n\t{ \"p\": \"(\\\\S\\\\S[дт])р \", \"r\": \" $1р \" }\n]\n"
        );
        assert_eq!(
            to_json(&[("a\nb", "q\"q")]),
            "[\n\t{ \"p\": \"a\\nb\", \"r\": \"q\\\"q\" }\n]\n"
        );
    }

    #[test]
    fn wraps_long_entries() {
        let long = "x".repeat(100);
        assert_eq!(
            to_json(&[(&long, "y")]),
            format!("[\n\t{{\n\t\t\"p\": \"{long}\",\n\t\t\"r\": \"y\"\n\t}}\n]\n")
        );
    }

    #[test]
    fn fits_within_width() {
        // 79 columns wide, so it stays on one line.
        let (pattern, result) = ("p".repeat(30), "r".repeat(28));
        assert_eq!(
            to_json(&[(&pattern, &result)]),
            format!("[\n\t{{ \"p\": \"{pattern}\", \"r\": \"{result}\" }}\n]\n")
        );
        // One column wider and it gets expanded.
        let pattern = "p".repeat(31);
        assert_eq!(
            to_json(&[(&pattern, &result)]),
            format!("[\n\t{{\n\t\t\"p\": \"{pattern}\",\n\t\t\"r\": \"{result}\"\n\t}}\n]\n")
        );
    }
}
