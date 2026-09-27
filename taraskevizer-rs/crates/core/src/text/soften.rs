//! Assimilatory palatalization (`soften`) without regexes, as a SINGLE
//! right-to-left pass over the raw UTF-8 bytes.
//!
//! Mirrors `src/dict/softening.ts` + `src/lib/soften.ts`:
//! - `noSoften` runs once up front, inserting `\u{E0FF}` markers
//!   (`мас\u{E0FF}фільм`, …) exactly like JS — later rules can never span
//!   a marker, and protectors emerging mid-pass stay unguarded,
//! - then `softeners` applied once, right-to-left.
//!
//! Why RTL works in one pass: every rule only ever makes the right context
//! *softer* (inserts `ь`, inserts `зь` for `ддз`, or rewrites `ʼ` → `ь`),
//! and every trigger looks only to the right. A left neighbour can only
//! become fireable because its right neighbour just got softer — so
//! processing gaps from right to left sees the already-closed suffix and
//! never needs a second sweep. A left-to-right scan needs an outer
//! `do…while` loop for exactly this reason (`ллле` → `лльле` → `льльле`,
//! ` з смех` → ` з сьмех` → ` зь сьмех`, …); RTL settles each gap once,
//! turning the worst case (long `ллл…е` / `ззз…е` runs, quadratic for any
//! looping scan) linear.
//!
//! Why bytes, not `char`s: every trigger/lookahead anchor is a fixed 1–2
//! byte sequence (ASCII space/`(` or 2-byte Cyrillic/`ʼ`), so the whole
//! pass runs directly on the UTF-8 bytes with no `chars()` decode and no
//! re-encode — original bytes are copied through, only inserted `ь`/`з`
//! are literal bytes. Anything longer than 2 bytes (markers, emoji, …)
//! can never match and is copied opaquely, which also keeps softening
//! from spanning markers for free.
//!
//! Gap model (all rules reframed as insertions at original gaps, plus
//! `ʼ` → `ь` replacement when the char itself is pushed):
//! - `([лнц])\1(?=…)` → insert `ь` between the pair,
//! - `дд(з(?=…))` → insert `зь` between the two `д`s (`ддз` → `дзьдз`);
//!   `дздз` needs no separate case — its gap (`з|д`) coincides with the
//!   `з` + `дз` + soft gap below, and both insert the same single `ь`,
//! - `з(?=(?:[бвзлмнц]|дз)[…])` / `с(?=…)` / `ц(?=[вм]…)` → insert `ь`
//!   right after the trigger,
//! - big ` (без|бяз|праз|цераз)?з |…` → insert `ь` between `з` and the
//!   trailing space,
//! - `([сз])ʼ(?=[яюеё])` → push `ь` instead of `ʼ`.
//!
//! Gap types are disjoint by `(left, right)` (`(л,л)` vs `(д,д)` vs
//! `(з,cons)` vs `(з,space)` vs …), except `(з,д)` where the `ддз` and the
//! `з` rule want the same `ь` — idempotent, emitted once. Same-position
//! dict-order priority therefore needs no special-casing.
//!
//! INTENTIONAL divergence from JS: the `ддз` rule has no `(\S\S)` prefix
//! (plain `дз?дз(?=[еёіюяь])` → `дзьдз`). The аддз-family that prefix was
//! guarding is covered by the `noSoften` protectors instead, so the rule
//! fires anywhere — including string-initially and after a single char
//! (`ддзі` → `дзьдзі`, `аддзі` → `адзьдзі`), where JS leaves the text
//! unchanged. The TS side (`softening.ts`) still has `(\S\S)` and needs
//! the same change for the two implementations to agree. (The JS oracle
//! vectors predate this change for `дз`; the `pinned_cases` test below
//! pins the new behavior directly.)
//!
//! A single `(` after ` з ` is transparent: it can only come from variation
//! markup (`(а|б)`) produced by earlier dicts — a real `(` never reaches
//! softening (`prepare` turns it into `&#40`). This preserves the
//! long-standing `з медзі` → `зь (медзі|сьпіжу)` expectation and matches how
//! `ia_words` treats `(`.

use std::borrow::Cow;

// ---------------------------------------------------------------------------
// Unit ids: `(byte_len << 24) | big-endian bytes`. Comparing one `u32`
// compares length and content at once; every trigger/lookahead anchor is a
// distinct 1–2 byte id, so dispatch and lookahead are exact matches.
// ---------------------------------------------------------------------------

const fn uid1(b0: u8) -> u32 {
    0x0100_0000 | b0 as u32
}
const fn uid2(b0: u8, b1: u8) -> u32 {
    0x0200_0000 | ((b0 as u32) << 8) | (b1 as u32)
}
const fn uid3(b0: u8, b1: u8, b2: u8) -> u32 {
    0x0300_0000 | ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32)
}

const ID_SPACE: u32 = uid1(0x20);
const ID_LP: u32 = uid1(0x28);

const ID_LL: u32 = uid2(0xD0, 0xBB); // л
const ID_NN: u32 = uid2(0xD0, 0xBD); // н
const ID_TS: u32 = uid2(0xD1, 0x86); // ц
const ID_DD: u32 = uid2(0xD0, 0xB4); // д
const ID_ZZ: u32 = uid2(0xD0, 0xB7); // з
const ID_SS: u32 = uid2(0xD1, 0x81); // с

const ID_YE: u32 = uid2(0xD0, 0xB5); // е
const ID_YO: u32 = uid2(0xD1, 0x91); // ё
const ID_II: u32 = uid2(0xD1, 0x96); // і
const ID_YU: u32 = uid2(0xD1, 0x8E); // ю
const ID_YA: u32 = uid2(0xD1, 0x8F); // я
const ID_SF: u32 = uid2(0xD1, 0x8C); // ь

const ID_BB: u32 = uid2(0xD0, 0xB1); // б
const ID_VE: u32 = uid2(0xD0, 0xB2); // в
const ID_EM: u32 = uid2(0xD0, 0xBC); // м
const ID_PE: u32 = uid2(0xD0, 0xBF); // п
const ID_EF: u32 = uid2(0xD1, 0x84); // ф
const ID_ER: u32 = uid2(0xD1, 0x80); // р
const ID_AA: u32 = uid2(0xD0, 0xB0); // а
const ID_TE: u32 = uid2(0xD1, 0x82); // т
const ID_YT: u32 = uid2(0xD0, 0xB9); // й
const ID_HA: u32 = uid2(0xD1, 0x85); // х

const ID_AP: u32 = uid2(0xCA, 0xBC); // ʼ (U+02BC)
const ID_MK: u32 = uid3(0xEE, 0x83, 0xBF); // \u{E0FF} protector marker

/// Byte blobs pushed for insertions / replacements (literals, no re-encode).
const SOFT_YAT: &[u8] = "ь".as_bytes();
const ZHE: &[u8] = "з".as_bytes();
const APOS: &[u8] = "ʼ".as_bytes();
const MARK: &[u8] = "".as_bytes();

// ---------------------------------------------------------------------------
// Classes: coarse dispatch for the gap's left unit.
// ---------------------------------------------------------------------------

const CL_OTHER: u8 = 0;
const CL_LNC: u8 = 1; // л н ц (identity via id)
const CL_D: u8 = 2;
const CL_Z: u8 = 3;
const CL_S: u8 = 4;
const CL_SPACE: u8 = 5;
const CL_LP: u8 = 6;
const CL_APOS: u8 = 7;
const CL_MARK: u8 = 8;

fn class_of(id: u32) -> u8 {
    match id {
        ID_LL | ID_NN | ID_TS => CL_LNC,
        ID_DD => CL_D,
        ID_ZZ => CL_Z,
        ID_SS => CL_S,
        ID_SPACE => CL_SPACE,
        ID_LP => CL_LP,
        ID_AP => CL_APOS,
        ID_MK => CL_MARK,
        _ => CL_OTHER,
    }
}

/// `[еёіюяь]`
fn is_soft_id(id: u32) -> bool {
    matches!(id, ID_YE | ID_YO | ID_II | ID_YU | ID_YA | ID_SF)
}

/// `[еёюяь]` (big rule first branch — no `і`)
fn is_big_single_id(id: u32) -> bool {
    matches!(id, ID_YE | ID_YO | ID_YU | ID_YA | ID_SF)
}

/// `[яюеё]` (after `сʼ`/`зʼ`)
fn is_apos_id(id: u32) -> bool {
    matches!(id, ID_YA | ID_YU | ID_YE | ID_YO)
}

/// `[бвзлмнц]` (after `з`)
fn is_z_cons_id(id: u32) -> bool {
    matches!(id, ID_BB | ID_VE | ID_ZZ | ID_LL | ID_EM | ID_NN | ID_TS)
}

/// `[бвлмнпсфц]` (after `с`)
fn is_s_cons_id(id: u32) -> bool {
    matches!(
        id,
        ID_BB | ID_VE | ID_LL | ID_EM | ID_NN | ID_PE | ID_SS | ID_EF | ID_TS
    )
}

/// `[вм]` (after `ц`)
fn is_c_cons_id(id: u32) -> bool {
    matches!(id, ID_VE | ID_EM)
}

/// `[бвзйлнпстфц]` (big ` з` rule)
fn is_big_cons_id(id: u32) -> bool {
    matches!(
        id,
        ID_BB
            | ID_VE
            | ID_ZZ
            | ID_YT
            | ID_LL
            | ID_EM
            | ID_NN
            | ID_PE
            | ID_SS
            | ID_TE
            | ID_EF
            | ID_TS
    )
}

// ---------------------------------------------------------------------------
// Byte cursor helpers. All inputs derive from valid UTF-8, so every stepped
// position is a char boundary; classifiers never match across units.
// ---------------------------------------------------------------------------

/// Classify `buf[start..end]` (one whole unit). Returns `(byte_len, class, id)`.
fn classify_at(buf: &[u8], start: usize, end: usize) -> (u8, u8, u32) {
    let len = end - start;
    debug_assert!((1..=4).contains(&len));
    let id = match len {
        1 => uid1(buf[start]),
        2 => uid2(buf[start], buf[start + 1]),
        3 => uid3(buf[start], buf[start + 1], buf[start + 2]),
        _ => 0x0400_0000,
    };
    (len as u8, class_of(id), id)
}

/// Starts of candidate units (2-byte л/н/ц/д/з/с), ascending. Found by
/// forward SIMD search for their second bytes; anything else can never
/// start a firing gap, so the sweep bulk-copies it unexamined.
fn collect_candidates(s: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity(s.len() / 32);
    let mut from = 0usize;
    while from < s.len() {
        let rest = &s[from..];
        // Second bytes of {л,н,ц} / {д,з,с}.
        let a = memchr::memchr3(0xBB, 0xBD, 0x86, rest);
        let b = memchr::memchr3(0xB4, 0xB7, 0x81, rest);
        let h = match (a, b) {
            (Some(x), Some(y)) => x.min(y),
            (Some(x), None) => x,
            (None, Some(y)) => y,
            (None, None) => break,
        };
        let hp = from + h;
        // A hit is only real if preceded by its D0/D1 lead — which, in
        // valid UTF-8, is then necessarily a char boundary, so no extra
        // check is needed.
        if hp >= 1 {
            let id = uid2(s[hp - 1], s[hp]);
            if matches!(id, ID_LL | ID_NN | ID_TS | ID_DD | ID_ZZ | ID_SS) {
                out.push((hp - 1) as u32);
            }
        }
        // No unit can start at `hp` (hit bytes are continuations), so
        // continuing past it is safe; tiling prevents overlaps.
        from = hp + 1;
    }
    out
}

/// Unit ending at byte `end` (`end` is a boundary, `end > 0`).
/// Returns `(start, byte_len, class, id)`.
fn prev_unit(s: &[u8], end: usize) -> Option<(usize, u8, u8, u32)> {
    if end == 0 || end > s.len() {
        return None;
    }
    let mut st = end - 1;
    while st > 0 && s[st] & 0xC0 == 0x80 {
        st -= 1;
    }
    Some({
        let (len, class, id) = classify_at(s, st, end);
        (st, len, class, id)
    })
}

/// Byte length of the unit starting at `i` (`i` is a boundary).
fn fwd_len(s: &[u8], i: usize) -> usize {
    let b0 = s[i];
    if b0 < 0x80 {
        1
    } else if b0 < 0xE0 {
        2
    } else if b0 < 0xF0 {
        3
    } else {
        4
    }
}

/// Unit starting at `i` (`i` is a boundary). Returns `(byte_len, class, id)`.
fn fwd_unit(s: &[u8], i: usize) -> (usize, u8, u32) {
    let len = fwd_len(s, i);
    let (l, class, id) = classify_at(s, i, i + len);
    (l as usize, class, id)
}

// ---------------------------------------------------------------------------
// `noSoften` protectors on bytes. Literals: `масфільм`, `пэндзлік`, and
// ` (п|н|пр|пер)?аддз`. Returns borrowed input when no protector literal
// is present (the common case — zero copy).
// ---------------------------------------------------------------------------

fn inject_protectors_bytes(text: &str) -> (Cow<'_, [u8]>, bool) {
    // Fast path: none of the protector literals present (every protector
    // contains one of these substrings, so this is exact, not heuristic).
    if !text.contains("масфільм") && !text.contains("пэндзлік") && !text.contains("аддз")
    {
        return (Cow::Borrowed(text.as_bytes()), false);
    }
    // Occurrences are rare: locate each with `str::find` (SIMD) and splice
    // markers around them, bulk-copying everything between. At one `pos`
    // the longest match wins; an earlier match shadows later overlaps —
    // exactly the old left-to-right walk's behavior.
    // `аддз`-family: ONE `match_indices` loop over `аддз`; each occurrence
    // is classified by its preceding bytes (longest variant wins). `аддз`
    // starts with two 2-byte leads, so every match starts at a boundary;
    // all compared preceding bytes are leads/spaces (never continuations),
    // so those comparisons imply boundaries too.
    // (head, total): marker goes after `head` bytes of the match span —
    // bytes after the break soften normally, exactly like post-marker text.
    // At one `pos` the longest match wins; an earlier match shadows later
    // overlaps — exactly the old left-to-right walk's behavior.
    let mut hits: Vec<(usize, usize, usize)> = Vec::new();
    if text.contains("масфільм") {
        let mut from = 0;
        while let Some(rel) = text[from..].find("масфільм") {
            hits.push((from + rel, 6, 16)); // `мас`, free `фільм`
            from += rel + 16;
        }
    }
    if text.contains("пэндзлік") {
        let mut from = 0;
        while let Some(rel) = text[from..].find("пэндзлік") {
            hits.push((from + rel, 10, 16)); // `пэндз`, free `лік`
            from += rel + 16;
        }
    }
    for (pa, _) in text.match_indices("аддз") {
        let b = text.as_bytes();
        // Longest first. Guards end at `pa+4` (through the first д);
        // totals add the free `дз` (4B).
        let hit: Option<(usize, usize)> = if pa >= 7
            && b[pa - 7] == 0x20
            && b[pa - 6] == 0xD0
            && b[pa - 5] == 0xBF
            && b[pa - 4] == 0xD0
            && b[pa - 3] == 0xB5
            && b[pa - 2] == 0xD1
            && b[pa - 1] == 0x80
        {
            Some((pa - 7, 11)) // ` перад`, free `дз`
        } else if pa >= 5
            && b[pa - 5] == 0x20
            && b[pa - 4] == 0xD0
            && b[pa - 3] == 0xBF
            && b[pa - 2] == 0xD1
            && b[pa - 1] == 0x80
        {
            Some((pa - 5, 9)) // ` прад`, free `дз`
        } else if pa >= 3
            && b[pa - 3] == 0x20
            && b[pa - 2] == 0xD0
            && (b[pa - 1] == 0xBF || b[pa - 1] == 0xBD)
        {
            // ` паддз` / ` наддз` (2nd byte: п=D0 BF, н=D0 BD)
            Some((pa - 3, 7))
        } else if pa >= 1 && b[pa - 1] == 0x20 {
            Some((pa - 1, 5)) // ` ад`, free `дз`
        } else {
            None // bare `аддз` (`аддзі`, …) — no protector
        };
        if let Some((pos, head)) = hit {
            hits.push((pos, head, head + 4));
        }
    }
    // The contains-gate above is loose (e.g. `аддзі` contains `аддз`
    // with no protector matching); empty hits simply mean borrowed.
    hits.sort_by(|a, b| a.0.cmp(&b.0).then(b.2.cmp(&a.2)));
    if hits.is_empty() {
        return (Cow::Borrowed(text.as_bytes()), false);
    }
    let s = text.as_bytes();
    let mut out = Vec::with_capacity(s.len() + 8);
    let mut p = 0usize;
    for (pos, head, total) in hits {
        if pos < p {
            continue; // shadowed by an earlier (longer-or-equal) match
        }
        out.extend_from_slice(&s[p..pos]);
        out.extend_from_slice(&s[pos..pos + head]);
        out.extend_from_slice(MARK);
        out.extend_from_slice(&s[pos + head..pos + total]);
        p = pos + total;
    }
    out.extend_from_slice(&s[p..]);
    (Cow::Owned(out), true)
}

// ---------------------------------------------------------------------------
// Single right-to-left pass over char-boundary gaps `n … 0`.
// `out` always holds `reverse(closed suffix starting at pos)`. Lookaheads
// peek into it by walking units backwards — no history buffer needed: the
// last byte of each reversed unit is its original lead byte, which
// determines the unit length. Gaps `> pos` are closed, `<= pos` original.
//
// Only gaps whose left unit names л/н/ц/д/з/с are ever evaluated (found by
// reverse memchr for their second bytes); everything between is bulk-copied
// reversed — no firing gap can hide there, since insertions need a trigger
// left unit (and `ʼ` replacement needs one too).
// ---------------------------------------------------------------------------

fn push_unit(out: &mut Vec<u8>, unit: &[u8]) {
    for &b in unit.iter().rev() {
        out.push(b);
    }
}

/// Reversed-copy `chunk` (forward bytes) to the reversed `out` buffer,
/// 8 bytes at a time where possible (bswap whole words, scalar tail).
fn push_reversed(out: &mut Vec<u8>, mut chunk: &[u8]) {
    while chunk.len() >= 8 {
        let (head, tail) = chunk.split_at(chunk.len() - 8);
        let v = u64::from_le_bytes(tail.try_into().unwrap()).swap_bytes();
        out.extend_from_slice(&v.to_le_bytes());
        chunk = head;
    }
    out.extend(chunk.iter().rev());
}

/// `(start, end)` of the unit ending at byte `end` in reversed `out`:
/// its last byte is the original lead byte, giving the unit length.
/// `None` past the buffer head (or on corrupt lengths — unreachable for
/// units we pushed ourselves).
fn unit_before(out: &[u8], end: usize) -> Option<(usize, usize)> {
    if end == 0 || end > out.len() {
        return None;
    }
    let b = out[end - 1];
    let l = if b < 0x80 {
        1
    } else if b < 0xE0 {
        2
    } else if b < 0xF0 {
        3
    } else {
        4
    };
    if l > end {
        return None;
    }
    Some((end - l, end))
}

/// Byte range in `out` of the `k`-th unit back from the suffix head
/// (`k = 0` is the first unit). `end` is the suffix length in bytes.
/// `None` when fewer than `k + 1` units exist.
fn peek_range(out: &[u8], end: usize, k: usize) -> Option<(usize, usize)> {
    let mut e = end;
    for _ in 0..k {
        e = unit_before(out, e)?.0;
    }
    unit_before(out, e)
}

/// Id of the `k`-th unit back from the suffix head.
fn peek_id(out: &[u8], k: usize) -> Option<u32> {
    let (st, en) = peek_range(out, out.len(), k)?;
    let mut tmp = [0u8; 4];
    for (i, &b) in out[st..en].iter().rev().enumerate() {
        tmp[i] = b;
    }
    Some(classify_at(&tmp, 0, en - st).2)
}

/// `і`-words check against the *transformed* tail (up to 32 units forward
/// from `start`; insertions included — a `ь` inside breaks literal matches,
/// exactly like matching against re-scanned text).
fn match_iwords_at(out: &[u8], start: usize) -> bool {
    let mut buf = [0u8; 160];
    let mut ranges = [(0usize, 0usize); 32];
    let mut cnt = 0usize;
    for k in 0..32 {
        match peek_range(out, out.len(), start + k) {
            Some(r) => {
                ranges[cnt] = r;
                cnt += 1;
            }
            None => break,
        }
    }
    if cnt == 0 {
        return false;
    }
    let mut blen = 0usize;
    // `ranges` is already head-forward; emit in order, un-reversing each
    // unit's bytes.
    for &(a, b) in ranges.iter().take(cnt) {
        for &byte in out[a..b].iter().rev() {
            buf[blen] = byte;
            blen += 1;
        }
    }
    match std::str::from_utf8(&buf[..blen]) {
        Ok(tail) => crate::text::matches_iwords(tail),
        Err(_) => false,
    }
}

/// Big ` з` lookahead against the transformed suffix. `start` is the unit
/// index (in the suffix) of the first lookahead unit — just after the
/// trailing space and an optional transparent `(`.
fn big_lookahead_at(out: &[u8], start: usize) -> bool {
    let c0 = match peek_id(out, start) {
        Some(c) => c,
        None => return false,
    };
    if is_big_single_id(c0) {
        return true;
    }
    if is_big_cons_id(c0) {
        return peek_id(out, start + 1).is_some_and(is_soft_id);
    }
    if c0 == ID_DD {
        return peek_id(out, start + 1) == Some(ID_ZZ)
            && peek_id(out, start + 2).is_some_and(is_soft_id);
    }
    if c0 == ID_II {
        // `імі? `.
        if peek_id(out, start + 1) == Some(ID_EM) {
            match peek_id(out, start + 2) {
                Some(id) if id == ID_SPACE => return true,
                Some(id) if id == ID_II && peek_id(out, start + 3) == Some(ID_SPACE) => {
                    return true
                }
                _ => {}
            }
        }
        // `іх(?:ні)?` needs only the `іх` prefix for a lookahead hit.
        if peek_id(out, start + 1) == Some(ID_HA) {
            return true;
        }
        // `і` + iwords.
        if match_iwords_at(out, start + 1) {
            return true;
        }
    }
    false
}

/// Big ` (без|бяз|праз|цераз)?з ` prefix ending at the gap: the left unit
/// (ending at `pos`) is `з` starting at `lstart`. All variants insert the
/// same `ь`, so any match fires.
fn is_big_prefix_at(s: &[u8], lstart: usize) -> bool {
    let mut ids = [0u32; 5];
    let mut cnt = 0usize;
    let mut e = lstart;
    while cnt < 5 {
        match prev_unit(s, e) {
            Some((st, _, _, id)) => {
                ids[cnt] = id;
                cnt += 1;
                e = st;
            }
            None => break,
        }
    }
    // `ids[0]` is `s[g-2]`, `ids[1]` is `s[g-3]`, …
    if cnt >= 1 && ids[0] == ID_SPACE {
        return true; // ` з `
    }
    if cnt >= 3 && (ids[0] == ID_YE || ids[0] == ID_YA) && ids[1] == ID_BB && ids[2] == ID_SPACE {
        return true; // ` без ` / ` бяз `
    }
    if cnt >= 4 && ids[0] == ID_AA && ids[1] == ID_ER && ids[2] == ID_PE && ids[3] == ID_SPACE {
        return true; // ` праз `
    }
    if cnt >= 5
        && ids[0] == ID_AA
        && ids[1] == ID_ER
        && ids[2] == ID_YE
        && ids[3] == ID_TS
        && ids[4] == ID_SPACE
    {
        return true; // ` цераз `
    }
    false
}

fn pass_rtl_bytes(s: &[u8]) -> Vec<u8> {
    let n = s.len();
    let mut out: Vec<u8> = Vec::with_capacity(n + n / 16 + 8);
    // Candidates ascend; the sweep consumes them right-to-left, bulk-
    // copying everything between (no firing gap can hide there: every
    // insertion site has a trigger left unit, and `ʼ` replacement does too).
    let cands = collect_candidates(s);
    let mut pos = n;
    // Right unit of the current gap: (class, id). `None` while the suffix
    // (from `pos`) is empty.
    let mut right: Option<(u8, u32)> = None;
    for &u in cands.iter().rev() {
        let us = u as usize;
        debug_assert!(us + 2 <= pos);
        let ue = us + 2; // candidates are always 2 bytes
        let lid = uid2(s[us], s[us + 1]);
        let lclass = class_of(lid);
        // Optional ʼ head of the bulk (only meaningful after с/з).
        let apos_head = ue + 2 <= pos
            && s[ue] == 0xCA
            && s[ue + 1] == 0xBC
            && (lclass == CL_Z || lclass == CL_S);
        let bs = if apos_head { ue + 2 } else { ue };
        // 1. bulk body (may be empty): inert, copy reversed.
        push_reversed(&mut out, &s[bs..pos]);
        let mut first: Option<(u8, u32)> = if bs < pos {
            let (_, rc, rid) = fwd_unit(s, bs);
            Some((rc, rid))
        } else {
            None
        };
        // 2. ʼ head replacement (sees the full suffix via peeks).
        if apos_head {
            let guarded = peek_id(&out, 0).is_some_and(is_apos_id);
            push_unit(&mut out, if guarded { SOFT_YAT } else { APOS });
            if first.is_none() {
                first = Some(if guarded {
                    (CL_OTHER, ID_SF)
                } else {
                    (CL_APOS, ID_AP)
                });
            }
        }
        // 3. right unit for gap ue (carried when adjacent to the previous
        // candidate: the suffix head is unchanged then).
        if first.is_some() {
            right = first;
        }
        // 4. evaluate gap ue: left is the candidate, lookahead is closed.
        if let Some((rclass, rid)) = right {
            // Gap at `pos`: left is `(lclass, lid)`, suffix head is `rid`.
            let mut soft = false; // insert `ь`
            let mut zhe = false; // insert `зь` (ддз)
            if lclass == CL_LNC && rclass == CL_LNC && lid == rid {
                // `([лнц])\1(?=…)` → `ь` between the pair.
                if peek_id(&out, 1).is_some_and(is_soft_id) {
                    soft = true;
                }
            } else if lclass == CL_D && rclass == CL_D {
                // `ддз(?=…)` → `зь` between the two `д`s; consecutive in
                // the *closed* suffix, so a just-inserted `зь` is visible.
                if peek_id(&out, 1) == Some(ID_ZZ) && peek_id(&out, 2).is_some_and(is_soft_id) {
                    zhe = true;
                }
            } else if lclass == CL_Z && rclass == CL_SPACE {
                // Big ` з`: `ь` between `з` and the trailing space.
                if is_big_prefix_at(s, us) {
                    let mut start = 1;
                    if peek_id(&out, start) == Some(ID_LP) {
                        start += 1;
                    }
                    if big_lookahead_at(&out, start) {
                        soft = true;
                    }
                }
            } else if lclass == CL_Z {
                if is_z_cons_id(rid) {
                    // `з(?=[бвзлмнц]…)` — next-in-final is index 1.
                    if peek_id(&out, 1).is_some_and(is_soft_id) {
                        soft = true;
                    }
                } else if rid == ID_DD
                    && peek_id(&out, 1) == Some(ID_ZZ)
                    && peek_id(&out, 2).is_some_and(is_soft_id)
                {
                    // `з(?=дз…)` — subsumes the `дздз` gap (same `ь`).
                    soft = true;
                }
            } else if lclass == CL_S {
                if is_s_cons_id(rid) && peek_id(&out, 1).is_some_and(is_soft_id) {
                    soft = true;
                }
            } else if lclass == CL_LNC
                && lid == ID_TS
                && is_c_cons_id(rid)
                && peek_id(&out, 1).is_some_and(is_soft_id)
            {
                soft = true;
            }
            if zhe {
                push_unit(&mut out, SOFT_YAT);
                push_unit(&mut out, ZHE);
            } else if soft {
                push_unit(&mut out, SOFT_YAT);
            }
        }
        // 5. push the candidate itself; it heads the suffix now.
        push_unit(&mut out, &s[us..ue]);
        right = Some((lclass, lid));
        pos = us;
    }
    // Head bulk (no candidates left inside).
    push_reversed(&mut out, &s[..pos]);
    out
}

pub(crate) fn soften(text: &str) -> String {
    // Protectors injected once up front (like JS `noSoften`); borrowed when
    // absent (zero copy). Markers shift with the text, so the pass needs no
    // guard tracking. One RTL pass closes all cascades — no outer loop.
    let (buf, injected) = inject_protectors_bytes(text);
    let s: &[u8] = &buf;
    // Always run the single RTL pass (no firing pre-scan — pipeline text
    // virtually always triggers, so the scan was pure overhead). `injected`
    // tells whether markers exist, skipping the strip scan otherwise.
    let mut out = pass_rtl_bytes(s);
    out.reverse();
    let result = String::from_utf8(out).expect("soften output is valid UTF-8 by construction");

    if injected {
        result.replace('\u{E0FF}', "")
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::soften;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    /// Byte-id consts must match the chars they name (single source of
    /// truth guard for the hand-written hex).
    #[test]
    fn byte_consts_agree() {
        use super::*;
        let probe = |c: char| {
            let mut buf = [0u8; 4];
            let b = c.encode_utf8(&mut buf);
            let (len, class, id) = classify_at(b.as_bytes(), 0, b.len());
            (len, class, id)
        };
        assert_eq!(probe(' '), (1, CL_SPACE, ID_SPACE));
        assert_eq!(probe('('), (1, CL_LP, ID_LP));
        assert_eq!(probe('л'), (2, CL_LNC, ID_LL));
        assert_eq!(probe('н'), (2, CL_LNC, ID_NN));
        assert_eq!(probe('ц'), (2, CL_LNC, ID_TS));
        assert_eq!(probe('д'), (2, CL_D, ID_DD));
        assert_eq!(probe('з'), (2, CL_Z, ID_ZZ));
        assert_eq!(probe('с'), (2, CL_S, ID_SS));
        assert_eq!(probe('е'), (2, CL_OTHER, ID_YE));
        assert_eq!(probe('ё'), (2, CL_OTHER, ID_YO));
        assert_eq!(probe('і'), (2, CL_OTHER, ID_II));
        assert_eq!(probe('ю'), (2, CL_OTHER, ID_YU));
        assert_eq!(probe('я'), (2, CL_OTHER, ID_YA));
        assert_eq!(probe('ь'), (2, CL_OTHER, ID_SF));
        assert_eq!(probe('б'), (2, CL_OTHER, ID_BB));
        assert_eq!(probe('в'), (2, CL_OTHER, ID_VE));
        assert_eq!(probe('м'), (2, CL_OTHER, ID_EM));
        assert_eq!(probe('п'), (2, CL_OTHER, ID_PE));
        assert_eq!(probe('ф'), (2, CL_OTHER, ID_EF));
        assert_eq!(probe('р'), (2, CL_OTHER, ID_ER));
        assert_eq!(probe('а'), (2, CL_OTHER, ID_AA));
        assert_eq!(probe('т'), (2, CL_OTHER, ID_TE));
        assert_eq!(probe('й'), (2, CL_OTHER, ID_YT));
        assert_eq!(probe('х'), (2, CL_OTHER, ID_HA));
        assert_eq!(probe('ʼ'), (2, CL_APOS, ID_AP));
        assert_eq!(probe(''), (3, CL_MARK, ID_MK));
        assert!(is_soft_id(ID_YE) && is_soft_id(ID_SF) && !is_soft_id(ID_EM));
        assert_eq!(SOFT_YAT, "ь".as_bytes());
        assert_eq!(ZHE, "з".as_bytes());
        assert_eq!(APOS, "ʼ".as_bytes());
        assert_eq!(MARK, "".as_bytes());
    }

    /// Pinned regression cases (fast, file-free; the oracle tests below
    /// cover breadth against JS). Expectations are derived from the rules
    /// documented above: the `(*, *)` pairs below were cross-checked
    /// against the JS oracle vectors where covered (`лле`, ` знік`,
    /// protectors, `ддз`-family, cascades) and hand-derived otherwise
    /// (big-`з` variants, аддз-family guards, `ʼ`, long runs).
    #[test]
    fn pinned_cases() {
        let cases: &[(&str, &str)] = &[
            ("лле", "льле"),
            (" сміх", " сьміх"),
            (" знік", " зьнік"),
            ("масфільм", "масфільм"),
            ("пэндзлік", "пэндзлік"),
            (" паддзень", " паддзень"),
            (" наддзіраць", " наддзіраць"),
            (" прадзед", " прадзед"),
            (" перадзіраць", " перадзіраць"),
            (" з е", " зь е"),
            (" без е", " безь е"),
            (" бяз імя", " бяз імя"),
            (" праз акно", " праз акно"),
            (" цераз лес", " церазь лес"),
            (" з (е", " зь (е"),
            (" з (медзі", " зь (медзі"),
            ("ддзі", "дзьдзі"),
            ("аддзі", "адзьдзі"),
            (" ддзі", " дзьдзі"),
            ("дздзі", "дзьдзі"),
            ("паддзі", "падзьдзі"),
            ("ллле", "льльле"),
            ("зззе", "зьзьзе"),
            (" з смех", " зь сьмех"),
            ("ХХдздздзе", "ХХдзьдзьдзе"),
            ("наддзіманне", "надзьдзіманьне"),
            ("сізоцеразддзіуёхцянпд", "сізоцеразьдзьдзіуёхцянпд"),
            (
                "коньнікі з бяздоннай цішыні",
                "коньнікі зь бяздоннай цішыні",
            ),
            ("сʼяў", "сьяў"),
            // `concat!` to make the `ь`-insertion boundary explicit.
            ("зʼявіцца", concat!("зь", "явіцца")),
            ("hello world", "hello world"),
            ("", ""),
        ];
        for (input, expected) in cases {
            assert_eq!(&soften(input), expected, "input: {input:?}");
        }
        // Long cascading runs settle fully: every inner pair fires.
        assert_eq!(soften(&("л".repeat(200) + "е")), "ль".repeat(199) + "ле");
        assert_eq!(soften(&("з".repeat(200) + "е")), "зь".repeat(199) + "зе");
    }

    fn oracle_present() -> bool {
        // `/tmp` vectors generated from the JS oracle (see task notes).
        PathBuf::from("/tmp/soften_edge.json").exists()
            && PathBuf::from("/tmp/soften_fuzz.json").exists()
    }

    #[test]
    fn oracle_edge_cases() {
        if !oracle_present() {
            return;
        }
        let raw = fs::read_to_string("/tmp/soften_edge.json").unwrap();
        let map: HashMap<String, String> = serde_json::from_str(&raw).unwrap();
        for (input, expected) in &map {
            // `(` is transparent variation markup here on purpose (see
            // module docs); raw JS blocks on it, so those oracle entries
            // are covered by `basic_softening` instead.
            if input.contains('(') {
                continue;
            }
            assert_eq!(&soften(input), expected, "input: {input:?}");
        }
    }

    #[test]
    fn oracle_fuzz() {
        if !oracle_present() {
            return;
        }
        let raw = fs::read_to_string("/tmp/soften_fuzz.json").unwrap();
        let vecs: Vec<(String, String)> = serde_json::from_str(&raw).unwrap();
        for (input, expected) in &vecs {
            if input.contains('(') {
                continue;
            }
            assert_eq!(&soften(input), expected, "input: {input:?}");
        }
    }

    #[test]
    fn basic_softening() {
        assert_eq!(soften("лле"), "льле");
        assert_eq!(soften(" сміх"), " сьміх");
        assert_eq!(soften(" знік"), " зьнік");
        assert_eq!(soften("масфільм"), "масфільм");
        assert_eq!(soften("пэндзлік"), "пэндзлік");
        assert_eq!(soften(" паддзень"), " паддзень");
        assert_eq!(soften(" з е"), " зь е");
        // `(` is transparent variation markup (see module docs).
        assert_eq!(soften(" з (е"), " зь (е");
        assert_eq!(soften(" з (медзі"), " зь (медзі");
        // No `(\S\S)` guard: `ддз` softens anywhere (unlike JS).
        assert_eq!(soften("ддзі"), "дзьдзі");
        assert_eq!(soften("аддзі"), "адзьдзі");
        assert_eq!(soften(" ддзі"), " дзьдзі");
        assert_eq!(soften("дздзі"), "дзьдзі");
        assert_eq!(soften("паддзі"), "падзьдзі");
        // Cascades settle in the single RTL sweep.
        assert_eq!(soften("ллле"), "льльле");
        assert_eq!(soften("зззе"), "зьзьзе");
        assert_eq!(soften(" з смех"), " зь сьмех");
        assert_eq!(soften("ХХдздздзе"), "ХХдзьдзьдзе");
        assert_eq!(soften("наддзіманне"), "надзьдзіманьне");
    }
}
