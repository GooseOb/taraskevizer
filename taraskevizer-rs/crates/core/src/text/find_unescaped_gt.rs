//! Byte index of the first unescaped `>` (a `\` escapes the next char).

pub(crate) fn find_unescaped_gt(s: &str) -> Option<usize> {
    let mut chars = s.char_indices();
    while let Some((byte_idx, c)) = chars.next() {
        if c == '\\' {
            chars.next();
        } else if c == '>' {
            return Some(byte_idx);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::find_unescaped_gt;

    #[test]
    fn finds_plain() {
        assert_eq!(find_unescaped_gt("a>b"), Some(1));
        assert_eq!(find_unescaped_gt(">"), Some(0));
        assert_eq!(find_unescaped_gt("аб>в"), Some(4));
    }

    #[test]
    fn skips_escaped() {
        assert_eq!(find_unescaped_gt("a\\>b"), None);
        assert_eq!(find_unescaped_gt("\\>x>"), Some(3));
        assert_eq!(find_unescaped_gt("a\\>b>c"), Some(4));
    }

    #[test]
    fn no_match() {
        assert_eq!(find_unescaped_gt("abc"), None);
        assert_eq!(find_unescaped_gt(""), None);
        assert_eq!(find_unescaped_gt("\\"), None);
        assert_eq!(find_unescaped_gt("a\\"), None);
    }
}
