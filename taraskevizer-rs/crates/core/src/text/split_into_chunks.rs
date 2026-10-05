use super::is_spaced_cluster_char::is_spaced_cluster_char;

/// Chunk delimiter predicate: any spacing char EXCEPT apostrophe-likes.
/// `'`, `` ` ``, `’` are word-internal in Belarusian (`аб'яднанне`) even
/// though punctuation — cutting after one splits a word and changes
/// apostrophe normalization downstream. (`ʼ` U+02BC is a letter (Lm), not
/// punctuation, so it never qualifies anyway.)
fn is_chunk_delimiter(c: char) -> bool {
    !matches!(c, '\'' | '`' | '\u{2019}') && is_spaced_cluster_char(c)
}

/// Split text into ~`target`-byte chunks (`target = len / n`) for parallel
/// processing, in a single linear pass.
///
/// A chunk ends after the first [`is_chunk_delimiter`] at/after
/// `start + target` — any spacing char is a good delimiter, not just `\n` —
/// except inside `<...>` angle-bracket syntax (tags/special syntax must never
/// be split): a `<` suppresses cuts until the next `>` re-enables them.
/// Since `<`/`>` are themselves spacing chars, processing brackets *before*
/// the delimiter check cuts right after a tag closes for free. Nesting is
/// not tracked: any `>` closes, so `<a><b>` may split between the tags (each
/// tag itself stays whole); stray `>` is a no-op. A `>` preceded by an odd
/// run of backslashes (`\>`, as in TeX spacing or escaped quotes) is literal
/// text, not a tag closer, and does not re-enable cuts.
///
/// There is deliberately no upper bound on chunk size. Cutting mid-word
/// corrupts conversion (word endings feed case, dictionary and variation
/// rules), so a delimiter desert yields one big but correct chunk instead of
/// corrupt small ones. The big chunk still converts exactly like the whole
/// text — it just gets no parallelism for that span. Keep `target` small
/// (the CLI uses 16 KiB) so deserts stay rare. Chunks are always non-empty,
/// so every iteration advances and the whole split is O(n) time / O(#chunks)
/// memory.
pub fn split_into_chunks(text: &str, n: usize) -> Vec<(usize, usize)> {
    if n <= 1 || text.is_empty() {
        return vec![(0, text.len())];
    }
    let target = text.len().div_ceil(n.max(1));
    let len = text.len();
    let mut chunks = Vec::new();
    let mut start = 0usize;
    let mut in_tag = false;
    let mut escaped = false;
    for (pos, c) in text.char_indices() {
        // Brackets first (see doc comment): `<` suppresses the cut, `>`
        // re-enables it right after the tag — unless escaped (`\>`).
        // Raw `<` in text (shouldn't occur in valid XML) only coarsens
        // chunking until the next `>`; stray `>` is a no-op.
        match c {
            '<' => in_tag = true,
            '>' if !escaped => in_tag = false,
            _ => {}
        }
        // Parity toggle: `\\>` is an escaped backslash plus a real closer.
        escaped = c == '\\' && !escaped;
        let after = pos + c.len_utf8();
        if pos >= start + target && !in_tag && is_chunk_delimiter(c) {
            chunks.push((start, after));
            start = after;
        }
    }
    if start < len {
        chunks.push((start, len));
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::split_into_chunks;

    /// Whether cutting at byte offset `o` would split a `<...>` token: a
    /// `<` before `o` is still open (no unescaped `>` closed it), and an
    /// unescaped `>` closes it at/after `o`. Mirrors the splitter exactly,
    /// including the `\>` escape rule. An unclosed `<` (no closer ever)
    /// never counts as split — there is no complete token to break.
    fn boundary_splits_token(text: &str, o: usize) -> bool {
        let mut open = false;
        let mut escaped = false;
        for (pos, c) in text.char_indices() {
            if pos >= o {
                break;
            }
            match c {
                '<' => open = true,
                '>' if !escaped => open = false,
                _ => {}
            }
            escaped = c == '\\' && !escaped;
        }
        if !open {
            return false;
        }
        for c in text[o..].chars() {
            match c {
                '>' if !escaped => return true,
                _ => {}
            }
            escaped = c == '\\' && !escaped;
        }
        false
    }

    fn assert_chunks_valid(text: &str, chunks: &[(usize, usize)]) {
        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].0, 0);
        assert_eq!(chunks.last().unwrap().1, text.len());
        for w in chunks.windows(2) {
            assert_eq!(w[0].1, w[1].0, "contiguous");
        }
        for &(s, e) in chunks {
            assert!(e > s, "non-empty");
            assert!(text.is_char_boundary(s) && text.is_char_boundary(e));
            // Never split inside a `<...>` token (between-tag cuts are fine,
            // each tag stays whole — see `nested_tags_split_only_between_tokens`).
            assert!(!boundary_splits_token(text, s), "cut inside a tag (start)");
            assert!(!boundary_splits_token(text, e), "cut inside a tag (end)");
        }
    }

    #[test]
    fn tags_are_never_split() {
        let text = "<revision><text xml:space=\"preserve\">".to_string()
            + &"word ".repeat(5000)
            + "</text></revision>"
            + &"after ".repeat(5000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_chunks_valid(&text, &chunks);
        // Reassembly is exact.
        let reassembled: String = chunks.iter().map(|&(s, e)| &text[s..e]).collect();
        assert_eq!(reassembled, text);
    }

    #[test]
    fn nested_tags_split_only_between_tokens() {
        // Nesting is not tracked: cuts may fall between sibling tags (after
        // an inner `>`), but never inside a single `<...>` token.
        let text = "<a><b><c>deep nesting here</c></b></a>".to_string()
            + &"filler text here ".repeat(3000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_chunks_valid(&text, &chunks);
        let reassembled: String = chunks.iter().map(|&(s, e)| &text[s..e]).collect();
        assert_eq!(reassembled, text);
    }

    #[test]
    fn large_target_cuts_at_delimiters() {
        // Regression test for the 30M-slice corruption: a target above the
        // old 1 MiB cap must still cut at delimiters, never mid-word (where
        // whole-word dictionary hits like `(краін|краінаў)` were lost).
        use super::is_chunk_delimiter;
        let text = "слова, ".repeat(300_000);
        assert!(text.len() > 1 << 20);
        let chunks = split_into_chunks(&text, 2);
        assert!(chunks.len() > 1);
        assert_chunks_valid(&text, &chunks);
        for &(s, e) in &chunks[..chunks.len() - 1] {
            let last = text[..e].chars().next_back().unwrap();
            assert!(is_chunk_delimiter(last), "chunk [{s}..{e}] cut mid-word");
        }
        let reassembled: String = chunks.iter().map(|&(s, e)| &text[s..e]).collect();
        assert_eq!(reassembled, text);
    }

    #[test]
    fn escaped_gt_does_not_close_tag() {
        // `\>` is literal text (TeX spacing, escaped quotes), not a tag
        // closer: with `<note ... \> ...>` unclosed until the final `>`,
        // no cut may fall inside the span.
        let text =
            "<note ".to_string() + &"слова, ".repeat(2000) + "\\>" + &"слова, ".repeat(2000) + ">";
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_chunks_valid(&text, &chunks);
        assert_eq!(chunks, vec![(0, text.len())]);
        let reassembled: String = chunks.iter().map(|&(s, e)| &text[s..e]).collect();
        assert_eq!(reassembled, text);
    }

    #[test]
    fn desert_yields_single_chunk() {
        // No delimiters at all: one big but correct chunk instead of corrupt
        // mid-word cuts. Must still terminate in linear time.
        let text = "x".repeat(3_000_000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_eq!(chunks, vec![(0, text.len())]);
    }

    #[test]
    fn unclosed_bracket_still_terminates() {
        // Stray `<` with no `>`: everything past it is one chunk (no cut is
        // safe inside a tag), the delimited prefix still splits normally.
        let text = "слова, ".repeat(1000) + "<unclosed" + &"y".repeat(2_000_000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_chunks_valid(&text, &chunks);
        let reassembled: String = chunks.iter().map(|&(s, e)| &text[s..e]).collect();
        assert_eq!(reassembled, text);
        // No cut is safe past the `<`, so the unclosed tag and everything
        // after it sit in a single final chunk.
        let tail = chunks.last().unwrap();
        assert!(text[tail.0..tail.1].contains("<unclosed"));
    }

    #[test]
    fn apostrophes_never_terminate_chunks() {
        // Apostrophe-likes are word-internal: no chunk (except the last) may
        // end right after one, even though they are punctuation. Commas give
        // the chunks somewhere legal to end.
        let text = "аб'яднанне, слова, don`t, x, словы ’так’, y, ".repeat(3000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(4_000));
        assert!(chunks.len() > 3);
        for &(s, e) in &chunks[..chunks.len() - 1] {
            let last = text[..e].chars().next_back().unwrap();
            assert!(
                !matches!(last, '\'' | '`' | '\u{2019}'),
                "chunk [{s}..{e}] ends after apostrophe-like {last:?}"
            );
            assert!(e > s);
        }
        let reassembled: String = chunks.iter().map(|&(s, e)| &text[s..e]).collect();
        assert_eq!(reassembled, text);
    }

    #[test]
    fn edges() {
        assert_eq!(split_into_chunks("", 4), vec![(0, 0)]);
        assert_eq!(split_into_chunks("abc", 0), vec![(0, 3)]);
        assert_eq!(split_into_chunks("abc", 1), vec![(0, 3)]);
        let c = split_into_chunks("a b c", 100);
        assert_eq!(c, vec![(0, 5)]);
    }
}
