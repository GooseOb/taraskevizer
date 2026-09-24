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
