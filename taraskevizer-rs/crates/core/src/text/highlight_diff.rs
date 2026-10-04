//! Word-level diff highlighting for the `fix` wrapper.
//!
//! Spec:
//! - part of the word added → highlight the added part
//!   (`снег → сьнег` gives `с[ь]нег`),
//! - part changed → highlight the changed part
//!   (`план → плян` gives `пл[я]н`),
//! - part removed → highlight the letters surrounding the gap in the new
//!   word (`казахскі → казаскі` gives `каз[ас]кі`).
//!
//! Multiple disjoint edits produce multiple highlights
//! (`жыццясцвярджальны → жыцьцясьцьвярджальны` gives
//! `жыц[ь]цяс[ь]ц[ь]вярджальны`, not one spanning block).
//! Pure deletions have no new chars to show, hence the surrounding-letter
//! rule. `g`-only changes and `(a|b)` variation lists are left alone —
//! the `g` and variations steps wrap those themselves.

use super::replace_g_str;

pub fn apply_highlight_diff(
    word: &str,
    o_word: &str,
    is_cyrillic: bool,
    fix: &dyn Fn(&str) -> String,
) -> String {
    if is_cyrillic {
        let word_h = replace_g_str(word);
        highlight_diff_word(word, o_word, &word_h, fix)
    } else {
        highlight_diff_word(word, o_word, word, fix)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DiffOp {
    Eq,
    Sub,
    Del,
    Ins,
}

/// Per-position runs for equal-length middles (optimal, no DP needed).
fn equal_len_ranges(
    cmp: &[char],
    old: &[char],
    h_start: usize,
    o_start: usize,
    mid_len: usize,
) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut pos = 0;
    while pos < mid_len {
        while pos < mid_len && cmp[h_start + pos] == old[o_start + pos] {
            pos += 1;
        }
        if pos == mid_len {
            break;
        }
        let first = pos;
        while pos < mid_len && cmp[h_start + pos] != old[o_start + pos] {
            pos += 1;
        }
        ranges.push((h_start + first, h_start + pos));
    }
    ranges
}

/// Levenshtein diff on the trimmed middle; returns highlight ranges in
/// new-word coordinates (base `h_start`, total length `wlen`).
fn levenshtein_ranges(
    old_mid: &[char],
    new_mid: &[char],
    h_start: usize,
    wlen: usize,
) -> Vec<(usize, usize)> {
    let old_len = old_mid.len();
    let new_len = new_mid.len();
    let stride = new_len + 1;
    let mut dp = vec![0u32; (old_len + 1) * stride];
    for row in 0..=old_len {
        dp[row * stride] = row as u32;
    }
    for col in 0..=new_len {
        dp[col] = col as u32;
    }
    for row in 1..=old_len {
        for col in 1..=new_len {
            let idx = row * stride + col;
            dp[idx] = if old_mid[row - 1] == new_mid[col - 1] {
                dp[(row - 1) * stride + (col - 1)]
            } else {
                1 + dp[(row - 1) * stride + (col - 1)]
                    .min(dp[(row - 1) * stride + col])
                    .min(dp[row * stride + (col - 1)])
            };
        }
    }
    // Backtrack. Tie-break `del > sub > ins` so a deletion next to a
    // substitution merges into one hunk instead of two (e.g. мекка->мэка
    // highlights only `э`, not `э` + surrounding `ка`).
    let mut row = old_len;
    let mut col = new_len;
    let mut ops: Vec<DiffOp> = Vec::with_capacity(old_len + new_len);
    while row > 0 || col > 0 {
        if row > 0 && col > 0 && old_mid[row - 1] == new_mid[col - 1] {
            debug_assert_eq!(dp[row * stride + col], dp[(row - 1) * stride + (col - 1)]);
            ops.push(DiffOp::Eq);
            row -= 1;
            col -= 1;
        } else {
            let sub_c = if row > 0 && col > 0 {
                dp[(row - 1) * stride + (col - 1)]
            } else {
                u32::MAX
            };
            let del_c = if row > 0 {
                dp[(row - 1) * stride + col]
            } else {
                u32::MAX
            };
            let ins_c = if col > 0 {
                dp[row * stride + (col - 1)]
            } else {
                u32::MAX
            };
            let best = sub_c.min(del_c).min(ins_c);
            if del_c == best {
                ops.push(DiffOp::Del);
                row -= 1;
            } else if sub_c == best {
                ops.push(DiffOp::Sub);
                row -= 1;
                col -= 1;
            } else {
                ops.push(DiffOp::Ins);
                col -= 1;
            }
        }
    }
    ops.reverse();
    // A hunk with new chars highlights those chars; a pure-deletion hunk
    // highlights its surroundings in the new word.
    let mut ranges = Vec::new();
    let mut new_pos: usize = 0;
    let mut hunk_start: Option<usize> = None;
    let mut hunk_end: usize = 0;
    let mut has_new = false;
    let mut flush =
        |hunk_start: &mut Option<usize>, hunk_end: usize, has_new: &mut bool, at_pos: usize| {
            if let Some(start) = *hunk_start {
                if *has_new {
                    ranges.push((h_start + start, h_start + hunk_end));
                } else {
                    let gap = h_start + start;
                    debug_assert_eq!(start, at_pos);
                    let rs = gap.saturating_sub(1);
                    let re = (gap + 1).min(wlen);
                    ranges.push((rs, re));
                }
                *hunk_start = None;
                *has_new = false;
            }
        };
    for op in ops {
        match op {
            DiffOp::Eq => {
                flush(&mut hunk_start, hunk_end, &mut has_new, new_pos);
                new_pos += 1;
            }
            DiffOp::Sub | DiffOp::Ins => {
                if hunk_start.is_some() {
                    hunk_end = new_pos + 1;
                } else {
                    hunk_start = Some(new_pos);
                    hunk_end = new_pos + 1;
                }
                has_new = true;
                new_pos += 1;
            }
            DiffOp::Del => {
                if hunk_start.is_none() {
                    hunk_start = Some(new_pos);
                    hunk_end = new_pos;
                }
            }
        }
    }
    flush(&mut hunk_start, hunk_end, &mut has_new, new_pos);
    ranges
}

pub(crate) fn highlight_diff_word(
    word: &str,
    o_word: &str,
    word_h: &str,
    highlight: &dyn Fn(&str) -> String,
) -> String {
    // `g`-only changes are wrapped by the `g` step, and `(a|b)` variation
    // lists are wrapped by the variations step, so `fix` stays quiet here.
    if o_word == word || o_word == word_h || word.contains('(') {
        return word.to_string();
    }
    let wchars: Vec<char> = word.chars().collect();
    let ochars: Vec<char> = o_word.chars().collect();
    let hchars: Vec<char> = word_h.chars().collect();
    let wlen = wchars.len();
    let olen = ochars.len();
    if wlen == 0 {
        return String::new();
    }
    // The `g` mapping is 1–1, so `hchars` aligns with `wchars`.
    let cmp: &[char] = if hchars.len() == wlen {
        &hchars
    } else {
        &wchars
    };

    // Common prefix / suffix trim; the real diff lives in the middle.
    let mut prefix = 0;
    while prefix < wlen && prefix < olen && cmp[prefix] == ochars[prefix] {
        prefix += 1;
    }
    let mut suffix = 0;
    while suffix < wlen - prefix
        && suffix < olen - prefix
        && cmp[wlen - 1 - suffix] == ochars[olen - 1 - suffix]
    {
        suffix += 1;
    }
    let h_start = prefix;
    let h_end = wlen - suffix;
    let o_start = prefix;
    let o_end = olen - suffix;
    let mid_new = h_end - h_start;
    let mid_old = o_end - o_start;
    if mid_new == 0 && mid_old == 0 {
        return word.to_string();
    }

    // Highlight ranges in new-word (`wchars`) coordinates.
    let ranges: Vec<(usize, usize)> = if mid_new == 0 {
        // Pure deletion: no new chars to show, highlight surrounding letters.
        let gap = h_start;
        vec![(gap.saturating_sub(1), (gap + 1).min(wlen))]
    } else if mid_old == 0 {
        // Pure insertion: highlight the inserted part.
        vec![(h_start, h_end)]
    } else if mid_new == mid_old {
        equal_len_ranges(cmp, &ochars, h_start, o_start, mid_new)
    } else {
        levenshtein_ranges(&ochars[o_start..o_end], &cmp[h_start..h_end], h_start, wlen)
    };

    if ranges.is_empty() {
        return word.to_string();
    }
    // Merge overlapping / touching ranges (deletion surroundings can touch).
    let mut merged: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());
    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.0 <= last.1 {
                if r.1 > last.1 {
                    last.1 = r.1;
                }
            } else {
                merged.push(r);
            }
        } else {
            merged.push(r);
        }
    }

    let mut result = String::with_capacity(word.len() + merged.len() * 8);
    let mut pos = 0;
    for (rs, re) in merged {
        if rs > pos {
            result.extend(wchars[pos..rs].iter());
        }
        let diff: String = wchars[rs..re].iter().collect();
        result.push_str(&highlight(&diff));
        pos = re;
    }
    if pos < wlen {
        result.extend(wchars[pos..].iter());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::apply_highlight_diff;

    fn check(word: &str, orig: &str, expected: &str) {
        let fix = |s: &str| format!("[{s}]");
        assert_eq!(
            &apply_highlight_diff(word, orig, true, &fix),
            expected,
            "word: {word:?} vs orig: {orig:?}"
        );
    }

    #[test]
    fn spec_added_changed_removed() {
        // Part added → highlight added.
        check("сьнег", "снег", "с[ь]нег");
        check("балькон", "балкон", "бал[ь]кон");
        check("сьмех", "смех", "с[ь]мех");
        // Part changed → highlight changed.
        check("плян", "план", "пл[я]н");
        check("плянэта", "планета", "пл[я]н[э]та");
        check("ббб", "ааа", "[ббб]");
        // Part removed → highlight surrounding letters.
        check("казаскі", "казахскі", "каз[ас]кі");
        check("абба", "абвба", "а[бб]а");
        check("ббаа", "бвбаа", "[бб]аа");
        check("аабб", "аабвб", "аа[бб]");
        check("баторы", "баторый", "батор[ы]");
        check("сцэнар", "сцэнарый", "сцэна[р]");
    }

    #[test]
    fn disjoint_edits_stay_disjoint() {
        check(
            "жыцьцясьцьвярджальны",
            "жыццясцвярджальны",
            "жыц[ь]цяс[ь]ц[ь]вярджальны",
        );
        check("балёньня", "балонья", "бал[ё]нь[н]я");
        check("бэрнардынцы", "бернардзінцы", "б[э]рнард[ы]нцы");
        // Substitution next to a deletion merges into one hunk.
        check("мэка", "мекка", "м[э]ка");
        check("Persanalny", "Piersanalny", "[Pe]rsanalny");
    }

    #[test]
    fn non_cyrillic() {
        let fix = |s: &str| format!("[{s}]");
        assert_eq!(
            &apply_highlight_diff("planeta", "płanieta", false, &fix),
            "p[l]a[ne]ta"
        );
    }

    #[test]
    fn skips_variations_g_only_and_equal() {
        // Variation lists are wrapped by the variations step.
        check("(брэст|берасьце)", "брэст", "(брэст|берасьце)");
        check("санкц(ый|ыяў)", "санкцый", "санкц(ый|ыяў)");
        // `g`-only changes are wrapped by the `g` step.
        check("ґазета", "газета", "ґазета");
        // Equal words pass through.
        check("план", "план", "план");
    }

    #[test]
    fn overlapping_repeats() {
        check("aaa", "aa", "aa[a]");
        check("aaaa", "aa", "aa[aa]");
        check("aaa", "a", "a[aa]");
        check("aa", "aaa", "a[a]");
    }
}
