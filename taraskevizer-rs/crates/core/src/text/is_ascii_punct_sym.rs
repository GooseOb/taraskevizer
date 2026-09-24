/// Whether an ASCII byte is punctuation/symbol: mirrors the table rows
/// below 0x80, so the hot ASCII path avoids the binary search.
pub(crate) fn is_ascii_punct_sym(b: u8) -> bool {
    matches!(b, 0x21..=0x2F | 0x3A..=0x40 | 0x5B..=0x60 | 0x7B..=0x7E)
}
