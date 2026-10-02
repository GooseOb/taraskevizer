/// Byte length of the UTF-8 char starting with lead byte `b`.
/// `i` only ever points at a char boundary of valid UTF-8, so `b` is
/// always a real lead byte; the fallback still guarantees progress.
pub(crate) fn utf8_char_len(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b < 0xE0 {
        2
    } else if b < 0xF0 {
        3
    } else {
        4
    }
}

/// The two UTF-8 bytes of a 2-byte `&str` literal, for byte-level matching
/// without per-position decoding: write `byte_pair("б")` instead of raw
/// `(0xD0, 0xB1)` — identical codegen, readable source.
pub(crate) const fn byte_pair(s: &str) -> (u8, u8) {
    let b = s.as_bytes();
    // All call sites pass 2-byte literals; out-of-bounds would fail to
    // compile, so the indexing is safe.
    (b[0], b[1])
}
