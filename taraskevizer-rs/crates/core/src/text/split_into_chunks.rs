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
/// be split): `<`/`>` nesting depth is tracked and only depth 0 cuts. Since
/// `<`/`>` are themselves spacing chars, processing brackets *before* the
/// delimiter check cuts right after a tag closes for free.
///
/// Fallbacks (rare, never on sane inputs): a mid-text char-boundary cut at
/// `MAX_CHUNK` when no delimiter appears (newline deserts), even mid-tag —
/// completion beats tag integrity there. Chunks are always non-empty, so
/// every iteration advances and the whole split is O(n) time / O(#chunks)
/// memory (the old forward/backward `\n` search re-scanned deserts per
/// chunk, which was quadratic there).
pub fn split_into_chunks(text: &str, n: usize) -> Vec<(usize, usize)> {
    if n <= 1 || text.is_empty() {
        return vec![(0, text.len())];
    }
    const MAX_CHUNK: usize = 1 << 20;
    let target = text.len().div_ceil(n.max(1));
    let len = text.len();
    let mut chunks = Vec::new();
    let mut start = 0usize;
    let mut depth = 0u32;
    for (pos, c) in text.char_indices() {
        // Brackets first (see doc comment): `<` suppresses the cut, `>`
        // re-enables it right after the tag. Raw `<` in text (shouldn't
        // occur in valid XML) only coarsens chunking until the next `>`;
        // stray `>` is a no-op via saturation.
        match c {
            '<' => depth = depth.saturating_add(1),
            '>' => depth = depth.saturating_sub(1),
            _ => {}
        }
        let after = pos + c.len_utf8();
        if after - start >= MAX_CHUNK {
            chunks.push((start, after));
            start = after;
        } else if pos >= start + target && depth == 0 && is_chunk_delimiter(c) {
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

    /// Bracket depth at each byte (pure `<`+1 / `>`-1, floor 0).
    fn depth_at(text: &str, upto: usize) -> u32 {
        let mut d = 0u32;
        for c in text[..upto].chars() {
            match c {
                '<' => d = d.saturating_add(1),
                '>' => d = d.saturating_sub(1),
                _ => {}
            }
        }
        d
    }

    fn assert_chunks_valid(text: &str, chunks: &[(usize, usize)], max: usize) {
        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].0, 0);
        assert_eq!(chunks.last().unwrap().1, text.len());
        for w in chunks.windows(2) {
            assert_eq!(w[0].1, w[1].0, "contiguous");
        }
        for &(s, e) in chunks {
            assert!(e > s, "non-empty");
            assert!(text.is_char_boundary(s) && text.is_char_boundary(e));
            assert!(e - s <= max, "bounded");
            // Never split inside angle brackets...
            assert_eq!(depth_at(text, s), 0, "cut outside tags (start)");
            assert_eq!(depth_at(text, e), 0, "cut outside tags (end)");
            // ...except the pathological cap fallback tested separately.
        }
    }

    #[test]
    fn tags_are_never_split() {
        let text = "<revision><text xml:space=\"preserve\">".to_string()
            + &"word ".repeat(5000)
            + "</text></revision>"
            + &"after ".repeat(5000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_chunks_valid(&text, &chunks, (1 << 20) + 4);
        // Reassembly is exact.
        let reassembled: String = chunks.iter().map(|&(s, e)| &text[s..e]).collect();
        assert_eq!(reassembled, text);
    }

    #[test]
    fn nested_tags_are_never_split() {
        let text = "<a><b><c>deep nesting here</c></b></a>".to_string()
            + &"filler text here ".repeat(3000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_chunks_valid(&text, &chunks, (1 << 20) + 4);
    }

    #[test]
    fn desert_is_capped_and_linear() {
        // No delimiters at all: must terminate with bounded chunks.
        let text = "x".repeat(3_000_000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_chunks_valid(&text, &chunks, (1 << 20) + 4);
        assert!(chunks.len() >= 3, "desert is split, not swallowed");
    }

    #[test]
    fn unclosed_bracket_still_terminates() {
        // Stray `<` with no `>` for megabytes: cap wins over tag integrity.
        let text = "ok ".repeat(1000) + "<unclosed" + &"y".repeat(2_000_000);
        let chunks = split_into_chunks(&text, text.len().div_ceil(16_000));
        assert_eq!(chunks[0].0, 0);
        assert_eq!(chunks.last().unwrap().1, text.len());
        for w in chunks.windows(2) {
            assert_eq!(w[0].1, w[1].0);
        }
        for &(s, e) in &chunks {
            assert!(e > s && e - s <= (1 << 20) + 4);
        }
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
