use super::{is_decimal_number, is_punct_or_symbol};

/// Whether `c` belongs to a spaced cluster: punctuation/symbol,
/// decimal digit, or BOM. Shared by the spacing step (`prepare`) and its
/// inverse (`finalize`).
pub(crate) fn is_spaced_cluster_char(c: char) -> bool {
    c == '\u{FEFF}' || is_punct_or_symbol(c as u32) || is_decimal_number(c as u32)
}
