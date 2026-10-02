/// Whether an ASCII byte is punctuation/symbol: mirrors the table rows
/// below 0x80, so the hot ASCII path avoids the binary search.
pub(crate) fn is_ascii_punct_sym(b: u8) -> bool {
    matches!(
        b,
        b'!'..=b'/' | b':'..=b'@' | b'['..=b'`' | b'{'..=b'~'
    )
}
