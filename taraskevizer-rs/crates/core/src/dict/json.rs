//! Serialization of dictionary entries back to JSON as compact
//! `["pattern", "result"]` pairs.

use std::fmt::Write as _;

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
            '\u{0}'..='\u{1f}' => {
                let _ = write!(out, "\\u{:04x}", ch as u32);
            }
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// Render batched entries as compact JSON: `[[["p", "r"], ...], ...]`.
///
/// Outer array = batches in execution order; the LAST inner array holds the
/// sequential tail, all preceding arrays are single-pass batches. No
/// whitespace is emitted: entries are `["pattern", "result"]` pairs.
pub fn to_json_batches(batches: &[&[(&str, &str)]]) -> String {
    let mut out = String::from("[");
    for (bi, batch) in batches.iter().enumerate() {
        if bi > 0 {
            out.push(',');
        }
        out.push('[');
        for (i, (pattern, result)) in batch.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push('[');
            out.push_str(&escape(pattern));
            out.push(',');
            out.push_str(&escape(result));
            out.push(']');
        }
        out.push(']');
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batches_empty() {
        assert_eq!(to_json_batches(&[]), "[]");
        assert_eq!(to_json_batches(&[&[]]), "[[]]");
    }

    #[test]
    fn batches_nested_shape() {
        // Last inner array is the sequential tail.
        let batches: &[&[(&str, &str)]] = &[&[("a", "b")], &[("c", "d"), ("e", "f")]];
        assert_eq!(
            to_json_batches(batches),
            r#"[[["a","b"]],[["c","d"],["e","f"]]]"#
        );
    }

    #[test]
    fn batches_escapes() {
        let batches: &[&[(&str, &str)]] = &[&[(r"(\S\S[дт])р ", " $1р "), ("a\nb", "q\"q")]];
        assert_eq!(
            to_json_batches(batches),
            r#"[[["(\\S\\S[дт])р "," $1р "],["a\nb","q\"q"]]]"#
        );
    }
}
