//! Alphabet conversion dispatch, borrowing when the alphabet needs no
//! conversion so hot call sites pay no allocation.

use std::borrow::Cow;

use crate::config::Alphabet;

use super::{convert_arabic, convert_latin_ji_lower, convert_latin_ji_upper, convert_latin_lower, convert_latin_upper};

/// Lower-case alphabet conversion, borrowing the input when the alphabet
/// needs no conversion (cyrillic) so hot call sites pay no allocation.
/// Converters for other alphabets must build a new string (`Cow::Owned`).
pub(crate) fn apply_abc_lower(text: &str, abc: Alphabet) -> Cow<'_, str> {
    match abc {
        Alphabet::Cyrillic => Cow::Borrowed(text),
        Alphabet::Latin => Cow::Owned(convert_latin_lower(text)),
        Alphabet::LatinJi => Cow::Owned(convert_latin_ji_lower(text)),
        Alphabet::Arabic => Cow::Owned(convert_arabic(text)),
    }
}

/// Upper-case alphabet conversion, borrowing the input when the alphabet has
/// no upper table (cyrillic, arabic) so call sites can treat "no table" and
/// "converted" uniformly via `Cow` instead of branching on `Option`.
pub(crate) fn apply_abc_upper(text: &str, abc: Alphabet) -> Cow<'_, str> {
    match abc {
        Alphabet::Latin => Cow::Owned(convert_latin_upper(text)),
        Alphabet::LatinJi => Cow::Owned(convert_latin_ji_upper(text)),
        Alphabet::Cyrillic | Alphabet::Arabic => Cow::Borrowed(text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn passthrough_borrows() {
        assert!(matches!(
            apply_abc_lower("план", Alphabet::Cyrillic),
            Cow::Borrowed(_)
        ));
        assert!(matches!(
            apply_abc_upper("план", Alphabet::Cyrillic),
            Cow::Borrowed(_)
        ));
        assert!(matches!(
            apply_abc_upper("план", Alphabet::Arabic),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn converted_matches_direct_converters() {
        assert_eq!(
            apply_abc_lower("planeta", Alphabet::Latin),
            convert_latin_lower("planeta")
        );
        assert_eq!(
            apply_abc_upper("planeta", Alphabet::LatinJi),
            convert_latin_ji_upper("planeta")
        );
    }
}
