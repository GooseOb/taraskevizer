/// Collapse every maximal run of `char::is_whitespace` into a single `' '`.
///
/// Returns the collapsed text plus one borrowed slice per run, in order.
/// The slices borrow `src`, so the caller must keep it alive until
/// [`restore_whitespaces`] has run.
pub fn collapse_whitespaces(src: &str) -> (String, Vec<&str>) {
    let mut spaces = Vec::new();
    let mut result = String::with_capacity(src.len());
    let mut chars = src.char_indices().peekable();
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
            spaces.push(&src[start..end]);
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    (result, spaces)
}

/// Inverse of [`collapse_whitespaces`]: replace each `' '` in `collapsed`
/// with the next borrowed run, in order.
///
/// A `' '` with no remaining run (possible only if an intermediate step
/// introduced spaces) is kept as-is.
pub fn restore_whitespaces(collapsed: &str, spaces: &[&str]) -> String {
    let extra: usize = spaces.iter().map(|s| s.len().saturating_sub(1)).sum();
    let mut result = String::with_capacity(collapsed.len() + extra);
    let mut runs = spaces.iter();
    for ch in collapsed.chars() {
        if ch == ' ' {
            if let Some(run) = runs.next() {
                result.push_str(run);
            } else {
                result.push(' ');
            }
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_all_whitespace() {
        let ws = [
            '\u{9}', '\u{A}', '\u{B}', '\u{C}', '\u{D}', '\u{20}', '\u{85}', '\u{A0}', '\u{1680}',
            '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
            '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}',
            '\u{205F}', '\u{3000}',
        ];
        for &w in &ws {
            assert!(w.is_whitespace(), "U+{:04X} not ws", w as u32);
        }
        let mut cases = vec!["".to_string(), "abc".to_string(), "  abc  ".to_string()];
        for &w in &ws {
            cases.push(format!("а{w}б"));
            cases.push(format!("а{w}{w}б"));
            cases.push(format!("{w}аб{w}"));
        }
        cases.push("а\u{85}б\u{A0}в\u{2009}г".to_string());
        cases.push("a\u{200B}b⁠c￾x".to_string());

        for text in cases {
            let (collapsed, spaces) = collapse_whitespaces(&text);
            assert_eq!(restore_whitespaces(&collapsed, &spaces), text);
        }
    }
}
