use crate::pipeline::PipelineContext;

/// Replace `г'` with `ґ`, unless followed by one of `еёіюя`.
///
/// Equivalent to `/г'(?![еёіюя])/g`, but without the regex engine:
/// single pass over the input, one allocation (output is never longer
/// than the input: 3 bytes `D0 B3 27` shrink to 2 bytes `D2 91`),
/// SIMD-accelerated scanning via `str::find('\'')`, and a byte-level
/// lookahead (`е`=D0 B5, `ё`=D1 91, `і`=D1 96, `ю`=D1 8E, `я`=D1 8F).
pub(crate) fn replace_g_apostrophe(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut flush_from = 0usize;
    let mut search_from = 0usize;
    // `'` is rare, so jumping between apostrophes with the vectorized
    // `find` keeps the hot loop in SIMD `memchr` instead of a per-byte
    // branch. Slicing is safe: `search_from`/`flush_from` always follow
    // an ASCII `'` (or 0), and `pos - 2` points at a `0xD0` lead byte,
    // which in valid UTF-8 can only start a char.
    while let Some(rel) = text[search_from..].find('\'') {
        let pos = search_from + rel; // byte index of `'`
        search_from = pos + 1;
        // Must be preceded by `г` (D0 B3).
        if pos < 2 || bytes[pos - 2] != 0xD0 || bytes[pos - 1] != 0xB3 {
            continue;
        }
        // Negative lookahead for [еёіюя].
        let after = pos + 1;
        let keep = if after >= bytes.len() {
            false
        } else {
            let b1 = bytes[after];
            if b1 == 0xD0 {
                after + 1 < bytes.len() && bytes[after + 1] == 0xB5
            } else if b1 == 0xD1 {
                after + 1 < bytes.len() && matches!(bytes[after + 1], 0x91 | 0x96 | 0x8E | 0x8F)
            } else {
                false
            }
        };
        if keep {
            continue;
        }
        out.push_str(&text[flush_from..pos - 2]);
        out.push_str("ґ");
        flush_from = pos + 1;
    }
    out.push_str(&text[flush_from..]);
    out
}

/// Replace `'`, `` ` ``, `’` with `ʼ` when followed by a non-whitespace char.
///
/// Equivalent to `/['`’](?=\S)/g`, but without the regex engine: single
/// pass, one allocation, byte-level scanning. The lookahead uses
/// `char::is_whitespace`, which was probed to match `fancy_regex`'s `\S`
/// exactly (blocks on the full Unicode White_Space set, replaces before
/// U+FEFF/U+200B, fails at end-of-string).
pub(crate) fn normalize_apostrophes(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    // Quotes are rare, so skip whole chars to step over Cyrillic text
    // 2 bytes at a time instead of byte-by-byte. `i` always stays on a
    // char boundary (`&str` is valid UTF-8), so all slicing is safe.
    while i < len {
        let b = bytes[i];
        // (quote, byte length): `'` = 27, `` ` `` = 60, `’` = E2 80 99.
        let qlen = if b == 0x27 || b == 0x60 {
            1
        } else if b == 0xE2 && i + 3 <= len && bytes[i + 1] == 0x80 && bytes[i + 2] == 0x99
        {
            3
        } else {
            i += utf8_char_len(b);
            continue;
        };
        // Lookahead `(?=\S)`: end-of-string or whitespace blocks the change.
        let replace = match text[i + qlen..].chars().next() {
            None => false,
            Some(c) => !c.is_whitespace(),
        };
        if replace {
            out.push_str(&text[flush_from..i]);
            out.push_str("ʼ");
            flush_from = i + qlen;
        }
        i += qlen;
    }
    out.push_str(&text[flush_from..]);
    out
}

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

/// Code-point ranges with Unicode general category P (punctuation) or S
/// (symbol), generated from Unicode 16.0 — the same tables
/// `regex-automata` 0.4.15 (used by `fancy-regex` 0.14) compiles
/// `\p{P}`/`\p{S}` from, so this matches them exactly.
pub(crate) const PUNCT_SYM_RANGES: &[(u32, u32)] = &[
    (0x0021, 0x002F), (0x003A, 0x0040), (0x005B, 0x0060), (0x007B, 0x007E), (0x00A1, 0x00A9), (0x00AB, 0x00AC),
    (0x00AE, 0x00B1), (0x00B4, 0x00B4), (0x00B6, 0x00B8), (0x00BB, 0x00BB), (0x00BF, 0x00BF), (0x00D7, 0x00D7),
    (0x00F7, 0x00F7), (0x02C2, 0x02C5), (0x02D2, 0x02DF), (0x02E5, 0x02EB), (0x02ED, 0x02ED), (0x02EF, 0x02FF),
    (0x0375, 0x0375), (0x037E, 0x037E), (0x0384, 0x0385), (0x0387, 0x0387), (0x03F6, 0x03F6), (0x0482, 0x0482),
    (0x055A, 0x055F), (0x0589, 0x058A), (0x058D, 0x058F), (0x05BE, 0x05BE), (0x05C0, 0x05C0), (0x05C3, 0x05C3),
    (0x05C6, 0x05C6), (0x05F3, 0x05F4), (0x0606, 0x060F), (0x061B, 0x061B), (0x061D, 0x061F), (0x066A, 0x066D),
    (0x06D4, 0x06D4), (0x06DE, 0x06DE), (0x06E9, 0x06E9), (0x06FD, 0x06FE), (0x0700, 0x070D), (0x07F6, 0x07F9),
    (0x07FE, 0x07FF), (0x0830, 0x083E), (0x085E, 0x085E), (0x0888, 0x0888), (0x0964, 0x0965), (0x0970, 0x0970),
    (0x09F2, 0x09F3), (0x09FA, 0x09FB), (0x09FD, 0x09FD), (0x0A76, 0x0A76), (0x0AF0, 0x0AF1), (0x0B70, 0x0B70),
    (0x0BF3, 0x0BFA), (0x0C77, 0x0C77), (0x0C7F, 0x0C7F), (0x0C84, 0x0C84), (0x0D4F, 0x0D4F), (0x0D79, 0x0D79),
    (0x0DF4, 0x0DF4), (0x0E3F, 0x0E3F), (0x0E4F, 0x0E4F), (0x0E5A, 0x0E5B), (0x0F01, 0x0F17), (0x0F1A, 0x0F1F),
    (0x0F34, 0x0F34), (0x0F36, 0x0F36), (0x0F38, 0x0F38), (0x0F3A, 0x0F3D), (0x0F85, 0x0F85), (0x0FBE, 0x0FC5),
    (0x0FC7, 0x0FCC), (0x0FCE, 0x0FDA), (0x104A, 0x104F), (0x109E, 0x109F), (0x10FB, 0x10FB), (0x1360, 0x1368),
    (0x1390, 0x1399), (0x1400, 0x1400), (0x166D, 0x166E), (0x169B, 0x169C), (0x16EB, 0x16ED), (0x1735, 0x1736),
    (0x17D4, 0x17D6), (0x17D8, 0x17DB), (0x1800, 0x180A), (0x1940, 0x1940), (0x1944, 0x1945), (0x19DE, 0x19FF),
    (0x1A1E, 0x1A1F), (0x1AA0, 0x1AA6), (0x1AA8, 0x1AAD), (0x1B4E, 0x1B4F), (0x1B5A, 0x1B6A), (0x1B74, 0x1B7F),
    (0x1BFC, 0x1BFF), (0x1C3B, 0x1C3F), (0x1C7E, 0x1C7F), (0x1CC0, 0x1CC7), (0x1CD3, 0x1CD3), (0x1FBD, 0x1FBD),
    (0x1FBF, 0x1FC1), (0x1FCD, 0x1FCF), (0x1FDD, 0x1FDF), (0x1FED, 0x1FEF), (0x1FFD, 0x1FFE), (0x2010, 0x2027),
    (0x2030, 0x205E), (0x207A, 0x207E), (0x208A, 0x208E), (0x20A0, 0x20C0), (0x2100, 0x2101), (0x2103, 0x2106),
    (0x2108, 0x2109), (0x2114, 0x2114), (0x2116, 0x2118), (0x211E, 0x2123), (0x2125, 0x2125), (0x2127, 0x2127),
    (0x2129, 0x2129), (0x212E, 0x212E), (0x213A, 0x213B), (0x2140, 0x2144), (0x214A, 0x214D), (0x214F, 0x214F),
    (0x218A, 0x218B), (0x2190, 0x2429), (0x2440, 0x244A), (0x249C, 0x24E9), (0x2500, 0x2775), (0x2794, 0x2B73),
    (0x2B76, 0x2B95), (0x2B97, 0x2BFF), (0x2CE5, 0x2CEA), (0x2CF9, 0x2CFC), (0x2CFE, 0x2CFF), (0x2D70, 0x2D70),
    (0x2E00, 0x2E2E), (0x2E30, 0x2E5D), (0x2E80, 0x2E99), (0x2E9B, 0x2EF3), (0x2F00, 0x2FD5), (0x2FF0, 0x2FFF),
    (0x3001, 0x3004), (0x3008, 0x3020), (0x3030, 0x3030), (0x3036, 0x3037), (0x303D, 0x303F), (0x309B, 0x309C),
    (0x30A0, 0x30A0), (0x30FB, 0x30FB), (0x3190, 0x3191), (0x3196, 0x319F), (0x31C0, 0x31E5), (0x31EF, 0x31EF),
    (0x3200, 0x321E), (0x322A, 0x3247), (0x3250, 0x3250), (0x3260, 0x327F), (0x328A, 0x32B0), (0x32C0, 0x33FF),
    (0x4DC0, 0x4DFF), (0xA490, 0xA4C6), (0xA4FE, 0xA4FF), (0xA60D, 0xA60F), (0xA673, 0xA673), (0xA67E, 0xA67E),
    (0xA6F2, 0xA6F7), (0xA700, 0xA716), (0xA720, 0xA721), (0xA789, 0xA78A), (0xA828, 0xA82B), (0xA836, 0xA839),
    (0xA874, 0xA877), (0xA8CE, 0xA8CF), (0xA8F8, 0xA8FA), (0xA8FC, 0xA8FC), (0xA92E, 0xA92F), (0xA95F, 0xA95F),
    (0xA9C1, 0xA9CD), (0xA9DE, 0xA9DF), (0xAA5C, 0xAA5F), (0xAA77, 0xAA79), (0xAADE, 0xAADF), (0xAAF0, 0xAAF1),
    (0xAB5B, 0xAB5B), (0xAB6A, 0xAB6B), (0xABEB, 0xABEB), (0xFB29, 0xFB29), (0xFBB2, 0xFBC2), (0xFD3E, 0xFD4F),
    (0xFDCF, 0xFDCF), (0xFDFC, 0xFDFF), (0xFE10, 0xFE19), (0xFE30, 0xFE52), (0xFE54, 0xFE66), (0xFE68, 0xFE6B),
    (0xFF01, 0xFF0F), (0xFF1A, 0xFF20), (0xFF3B, 0xFF40), (0xFF5B, 0xFF65), (0xFFE0, 0xFFE6), (0xFFE8, 0xFFEE),
    (0xFFFC, 0xFFFD), (0x10100, 0x10102), (0x10137, 0x1013F), (0x10179, 0x10189), (0x1018C, 0x1018E), (0x10190, 0x1019C),
    (0x101A0, 0x101A0), (0x101D0, 0x101FC), (0x1039F, 0x1039F), (0x103D0, 0x103D0), (0x1056F, 0x1056F), (0x10857, 0x10857),
    (0x10877, 0x10878), (0x1091F, 0x1091F), (0x1093F, 0x1093F), (0x10A50, 0x10A58), (0x10A7F, 0x10A7F), (0x10AC8, 0x10AC8),
    (0x10AF0, 0x10AF6), (0x10B39, 0x10B3F), (0x10B99, 0x10B9C), (0x10D6E, 0x10D6E), (0x10D8E, 0x10D8F), (0x10EAD, 0x10EAD),
    (0x10F55, 0x10F59), (0x10F86, 0x10F89), (0x11047, 0x1104D), (0x110BB, 0x110BC), (0x110BE, 0x110C1), (0x11140, 0x11143),
    (0x11174, 0x11175), (0x111C5, 0x111C8), (0x111CD, 0x111CD), (0x111DB, 0x111DB), (0x111DD, 0x111DF), (0x11238, 0x1123D),
    (0x112A9, 0x112A9), (0x113D4, 0x113D5), (0x113D7, 0x113D8), (0x1144B, 0x1144F), (0x1145A, 0x1145B), (0x1145D, 0x1145D),
    (0x114C6, 0x114C6), (0x115C1, 0x115D7), (0x11641, 0x11643), (0x11660, 0x1166C), (0x116B9, 0x116B9), (0x1173C, 0x1173F),
    (0x1183B, 0x1183B), (0x11944, 0x11946), (0x119E2, 0x119E2), (0x11A3F, 0x11A46), (0x11A9A, 0x11A9C), (0x11A9E, 0x11AA2),
    (0x11B00, 0x11B09), (0x11BE1, 0x11BE1), (0x11C41, 0x11C45), (0x11C70, 0x11C71), (0x11EF7, 0x11EF8), (0x11F43, 0x11F4F),
    (0x11FD5, 0x11FF1), (0x11FFF, 0x11FFF), (0x12470, 0x12474), (0x12FF1, 0x12FF2), (0x16A6E, 0x16A6F), (0x16AF5, 0x16AF5),
    (0x16B37, 0x16B3F), (0x16B44, 0x16B45), (0x16D6D, 0x16D6F), (0x16E97, 0x16E9A), (0x16FE2, 0x16FE2), (0x1BC9C, 0x1BC9C),
    (0x1BC9F, 0x1BC9F), (0x1CC00, 0x1CCEF), (0x1CD00, 0x1CEB3), (0x1CF50, 0x1CFC3), (0x1D000, 0x1D0F5), (0x1D100, 0x1D126),
    (0x1D129, 0x1D164), (0x1D16A, 0x1D16C), (0x1D183, 0x1D184), (0x1D18C, 0x1D1A9), (0x1D1AE, 0x1D1EA), (0x1D200, 0x1D241),
    (0x1D245, 0x1D245), (0x1D300, 0x1D356), (0x1D6C1, 0x1D6C1), (0x1D6DB, 0x1D6DB), (0x1D6FB, 0x1D6FB), (0x1D715, 0x1D715),
    (0x1D735, 0x1D735), (0x1D74F, 0x1D74F), (0x1D76F, 0x1D76F), (0x1D789, 0x1D789), (0x1D7A9, 0x1D7A9), (0x1D7C3, 0x1D7C3),
    (0x1D800, 0x1D9FF), (0x1DA37, 0x1DA3A), (0x1DA6D, 0x1DA74), (0x1DA76, 0x1DA83), (0x1DA85, 0x1DA8B), (0x1E14F, 0x1E14F),
    (0x1E2FF, 0x1E2FF), (0x1E5FF, 0x1E5FF), (0x1E95E, 0x1E95F), (0x1ECAC, 0x1ECAC), (0x1ECB0, 0x1ECB0), (0x1ED2E, 0x1ED2E),
    (0x1EEF0, 0x1EEF1), (0x1F000, 0x1F02B), (0x1F030, 0x1F093), (0x1F0A0, 0x1F0AE), (0x1F0B1, 0x1F0BF), (0x1F0C1, 0x1F0CF),
    (0x1F0D1, 0x1F0F5), (0x1F10D, 0x1F1AD), (0x1F1E6, 0x1F202), (0x1F210, 0x1F23B), (0x1F240, 0x1F248), (0x1F250, 0x1F251),
    (0x1F260, 0x1F265), (0x1F300, 0x1F6D7), (0x1F6DC, 0x1F6EC), (0x1F6F0, 0x1F6FC), (0x1F700, 0x1F776), (0x1F77B, 0x1F7D9),
    (0x1F7E0, 0x1F7EB), (0x1F7F0, 0x1F7F0), (0x1F800, 0x1F80B), (0x1F810, 0x1F847), (0x1F850, 0x1F859), (0x1F860, 0x1F887),
    (0x1F890, 0x1F8AD), (0x1F8B0, 0x1F8BB), (0x1F8C0, 0x1F8C1), (0x1F900, 0x1FA53), (0x1FA60, 0x1FA6D), (0x1FA70, 0x1FA7C),
    (0x1FA80, 0x1FA89), (0x1FA8F, 0x1FAC6), (0x1FACE, 0x1FADC), (0x1FADF, 0x1FAE9), (0x1FAF0, 0x1FAF8), (0x1FB00, 0x1FB92),
    (0x1FB94, 0x1FBEF),
];

/// Whether code point `cp` has Unicode general category P or S.
pub(crate) fn is_punct_or_symbol(cp: u32) -> bool {
    let idx = PUNCT_SYM_RANGES.partition_point(|&(lo, _)| lo <= cp);
    idx > 0 && cp <= PUNCT_SYM_RANGES[idx - 1].1
}

/// Code-point ranges with Unicode general category Nd (decimal number),
/// generated from Unicode 16.0 to match `fancy-regex`'s `\d` exactly
/// (with Unicode enabled, `\d` is `\p{Nd}`, not ASCII-only).
pub(crate) const DECIMAL_NUMBER_RANGES: &[(u32, u32)] = &[
    (0x0030, 0x0039), (0x0660, 0x0669), (0x06F0, 0x06F9), (0x07C0, 0x07C9), (0x0966, 0x096F), (0x09E6, 0x09EF),
    (0x0A66, 0x0A6F), (0x0AE6, 0x0AEF), (0x0B66, 0x0B6F), (0x0BE6, 0x0BEF), (0x0C66, 0x0C6F), (0x0CE6, 0x0CEF),
    (0x0D66, 0x0D6F), (0x0DE6, 0x0DEF), (0x0E50, 0x0E59), (0x0ED0, 0x0ED9), (0x0F20, 0x0F29), (0x1040, 0x1049),
    (0x1090, 0x1099), (0x17E0, 0x17E9), (0x1810, 0x1819), (0x1946, 0x194F), (0x19D0, 0x19D9), (0x1A80, 0x1A89),
    (0x1A90, 0x1A99), (0x1B50, 0x1B59), (0x1BB0, 0x1BB9), (0x1C40, 0x1C49), (0x1C50, 0x1C59), (0xA620, 0xA629),
    (0xA8D0, 0xA8D9), (0xA900, 0xA909), (0xA9D0, 0xA9D9), (0xA9F0, 0xA9F9), (0xAA50, 0xAA59), (0xABF0, 0xABF9),
    (0xFF10, 0xFF19), (0x104A0, 0x104A9), (0x10D30, 0x10D39), (0x10D40, 0x10D49), (0x11066, 0x1106F), (0x110F0, 0x110F9),
    (0x11136, 0x1113F), (0x111D0, 0x111D9), (0x112F0, 0x112F9), (0x11450, 0x11459), (0x114D0, 0x114D9), (0x11650, 0x11659),
    (0x116C0, 0x116C9), (0x116D0, 0x116E3), (0x11730, 0x11739), (0x118E0, 0x118E9), (0x11950, 0x11959), (0x11BF0, 0x11BF9),
    (0x11C50, 0x11C59), (0x11D50, 0x11D59), (0x11DA0, 0x11DA9), (0x11F50, 0x11F59), (0x16130, 0x16139), (0x16A60, 0x16A69),
    (0x16AC0, 0x16AC9), (0x16B50, 0x16B59), (0x16D70, 0x16D79), (0x1CCF0, 0x1CCF9), (0x1D7CE, 0x1D7FF), (0x1E140, 0x1E149),
    (0x1E2F0, 0x1E2F9), (0x1E4F0, 0x1E4F9), (0x1E5F1, 0x1E5FA), (0x1E950, 0x1E959), (0x1FBF0, 0x1FBF9),
];

/// Whether code point `cp` is a Unicode decimal number (`\d`).
pub(crate) fn is_decimal_number(cp: u32) -> bool {
    let idx = DECIMAL_NUMBER_RANGES.partition_point(|&(lo, _)| lo <= cp);
    idx > 0 && cp <= DECIMAL_NUMBER_RANGES[idx - 1].1
}

/// Whether an ASCII byte is punctuation/symbol: mirrors the table rows
/// below 0x80, so the hot ASCII path avoids the binary search.
pub(crate) fn is_ascii_punct_sym(b: u8) -> bool {
    matches!(b, 0x21..=0x2F | 0x3A..=0x40 | 0x5B..=0x60 | 0x7B..=0x7E)
}

/// Whether `c` belongs to a spaced cluster: punctuation/symbol,
/// decimal digit, or BOM. Shared by the spacing step (`prepare`) and its
/// inverse (`finalize`).
pub(crate) fn is_spaced_cluster_char(c: char) -> bool {
    c == '\u{FEFF}' || is_punct_or_symbol(c as u32) || is_decimal_number(c as u32)
}

/// Wrap runs of Unicode punctuation/symbols, decimal digits, and U+FEFF in spaces.
///
/// Like `/\p{P}|\p{S}|\d+/gu` with `" $0 "`, except consecutive matches form
/// a single cluster wrapped only at its edges (`"!?"` → `" !? "` instead of
/// `" !  ? "`). Single pass, one allocation, byte-level scanning with
/// table-driven classification (`\d` is `\p{Nd}`; U+FEFF is Cf and handled
/// alongside). `i`/`j` always stay on char boundaries, so slicing is safe.
pub(crate) fn space_out_punct_sym_digits(text: &str) -> String {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len);
    let mut flush_from = 0usize;
    let mut i = 0usize;
    while i < len {
        let b = bytes[i];
        let first_len = if b < 0x80 && !b.is_ascii_digit() {
            if !is_ascii_punct_sym(b) {
                i += 1;
                continue;
            }
            1
        } else {
            // `i` is a boundary of valid UTF-8, so this always yields a char.
            let c = text[i..].chars().next().unwrap_or('\0');
            if is_spaced_cluster_char(c) {
                c.len_utf8()
            } else {
                i += c.len_utf8();
                continue;
            }
        };
        // Extend the cluster over following P/S/digit/FEFF chars.
        let mut j = i + first_len;
        while j < len {
            let nb = bytes[j];
            if nb < 0x80 {
                if nb.is_ascii_digit() || is_ascii_punct_sym(nb) {
                    j += 1;
                } else {
                    break;
                }
            } else {
                    let nc = text[j..].chars().next().unwrap_or('\0');
                    if is_spaced_cluster_char(nc) {
                    j += nc.len_utf8();
                } else {
                    break;
                }
            }
        }
        out.push_str(&text[flush_from..i]);
        out.push(' ');
        out.push_str(&text[i..j]);
        out.push(' ');
        flush_from = j;
        i = j;
    }
    out.push_str(&text[flush_from..]);
    out
}

pub fn step_prepare(ctx: &mut PipelineContext) {
    let text = std::mem::take(&mut ctx.text);
    let mut t = replace_g_apostrophe(&text);
    t = t.replace(" - ", " — ").replace(
        &ctx.cfg.left_angle_bracket,
        &format!(" {} ", &ctx.cfg.left_angle_bracket),
    );
    t = normalize_apostrophes(&t);
    t = space_out_punct_sym_digits(&t);
    t = t.replace('(', "&#40");
    ctx.text = t;
}

#[cfg(test)]
mod tests {
    use super::replace_g_apostrophe;

    fn check(input: &str, expected: &str) {
        assert_eq!(replace_g_apostrophe(input), expected, "input: {input:?}");
    }

    #[test]
    fn replaces_plain() {
        check("г'", "ґ");
        check("аг'б", "аґб");
        check("г' г'", "ґ ґ");
        check("г'а г'о", "ґа ґо");
    }

    #[test]
    fn lookahead_blocks() {
        for ch in ['е', 'ё', 'і', 'ю', 'я'] {
            let input = format!("г'{ch}");
            check(&input, &input);
        }
        // End of string still replaces (negative lookahead succeeds).
        check("г'", "ґ");
    }

    #[test]
    fn no_false_positives() {
        check("", "");
        check("hello", "hello");
        check("'", "'");
        check("г", "г");
        check("Г'", "Г'"); // uppercase untouched, like the original regex
        check("а'е", "а'е");
        check("гʼе", "гʼе"); // U+02BC modifier, not ASCII apostrophe
                             // Uppercase lookahead letters don't block (regex class is lowercase-only).
        check("г'Е", "ґЕ");
        check("г'Ё", "ґЁ");
    }

    #[test]
    fn mixed_runs() {
        check("г'е г'а г'ю г'б", "г'е ґа г'ю ґб");
        check("г''", "ґ'");
        check("'г", "'г");
        check("г' г''", "ґ ґ'");
    }

    #[test]
    fn matches_fancy_regex() {
        let re = fancy_regex::Regex::new(r"г'(?![еёіюя])").unwrap();
        let inputs = [
            "",
            "'",
            "г",
            "г'",
            "г'е",
            "г'ё",
            "г'і",
            "г'ю",
            "г'я",
            "г'а",
            "г'Е",
            "г' ",
            "г'.",
            "prefix г' suffix",
            "г'г'",
            "г''г'",
            "аб'г'в",
            "г'ег'а",
            "Г' г'",
            "г'\u{301}е",
            "emoji 😀 г' end",
        ];
        for input in inputs {
            let expected = re.replace_all(input, "ґ").into_owned();
            assert_eq!(replace_g_apostrophe(input), expected, "input: {input:?}");
        }
    }

    use super::normalize_apostrophes;

    fn check_apos(input: &str, expected: &str) {
        assert_eq!(normalize_apostrophes(input), expected, "input: {input:?}");
    }

    #[test]
    fn apos_replaces_before_non_space() {
        check_apos("a'b", "aʼb");
        check_apos("a`b", "aʼb");
        check_apos("a’b", "aʼb");
        check_apos("'a", "ʼa");
        check_apos("''a", "ʼʼa");
        check_apos("’’a", "ʼʼa");
        check_apos("a''b", "aʼʼb");
        check_apos("don't", "donʼt");
        check_apos("a'.", "aʼ.");
        check_apos("a'😀", "aʼ😀");
    }

    #[test]
    fn apos_keeps_before_whitespace_or_eos() {
        check_apos("", "");
        check_apos("'", "'");
        check_apos("`", "`");
        check_apos("’", "’");
        check_apos("a'", "a'");
        check_apos("a' ", "a' ");
        check_apos("a'  b", "a'  b");
        check_apos("a'\tb", "a'\tb");
        check_apos("a'\nb", "a'\nb");
        check_apos("a'\u{00A0}b", "a'\u{00A0}b");
        check_apos("a'\u{2028}b", "a'\u{2028}b");
        // Already-normalized modifier stays untouched.
        check_apos("aʼb", "aʼb");
        // U+FEFF is not whitespace for `\S`: still replaces.
        check_apos("a'\u{FEFF}b", "aʼ\u{FEFF}b");
    }

    #[test]
    fn apos_matches_fancy_regex() {
        let re = fancy_regex::Regex::new(r"['`’](?=\S)").unwrap();
        let spaces = [
            " ", "\t", "\n", "\r", "\u{00A0}", "\u{0085}", "\u{1680}", "\u{2000}",
            "\u{2009}", "\u{2028}", "\u{2029}", "\u{202F}", "\u{205F}", "\u{3000}",
            "\u{FEFF}", "\u{200B}",
        ];
        let mut inputs = vec![
            String::new(),
            "'".into(),
            "`".into(),
            "’".into(),
            "ʼ".into(),
            "a'b".into(),
            "''a".into(),
            "a''".into(),
            "’'’".into(),
            "пад'ём аб'яднанне".into(),
            "— ’quote’ —".into(),
            "emoji 😀'x".into(),
            "г'е".into(),
        ];
        for w in spaces {
            for q in ["'", "`", "’"] {
                inputs.push(format!("a{q}{w}b"));
            }
        }
        for input in &inputs {
            let expected = re.replace_all(input, "ʼ").into_owned();
            assert_eq!(normalize_apostrophes(input), expected, "input: {input:?}");
        }
    }

    use super::space_out_punct_sym_digits;

    fn check_space(input: &str, expected: &str) {
        assert_eq!(
            space_out_punct_sym_digits(input),
            expected,
            "input: {input:?}"
        );
    }

    #[test]
    fn space_basic() {
        check_space("", "");
        check_space("hello", "hello");
        check_space("плянэта", "плянэта");
        check_space("a,b", "a , b");
        check_space("a.b!c", "a . b ! c");
        check_space("(a)", " ( a ) ");
        // Consecutive matches merge into one cluster wrapped at the edges.
        check_space("!?", " !? ");
        check_space("«плянэта»", " « плянэта » ");
        check_space("a—b", "a — b");
        // ASCII `'` is Po: wrapped, not normalized, by this step.
        check_space("don't", "don ' t");
        // Already-normalized modifier (Lm) stays untouched.
        check_space("aʼb", "aʼb");
    }

    #[test]
    fn space_digits_and_feff() {
        // Digit runs (possibly mixed-script) merge with touching punct.
        check_space("abc123def", "abc 123 def");
        check_space("12", " 12 ");
        check_space("1,2", " 1,2 ");
        check_space("100%", " 100% ");
        check_space("3.14", " 3.14 ");
        check_space("+375 29 123-45-67", " +375   29   123-45-67 ");
        // `\d` is `\p{Nd}`: non-ASCII decimal digits wrap too.
        check_space("٣٤٥", " ٣٤٥ ");
        check_space("1٣2", " 1٣2 ");
        // U+FEFF (Cf) is wrapped by the merged-in second operation.
        check_space("\u{FEFF}", " \u{FEFF} ");
        check_space("a\u{FEFF}b", "a \u{FEFF} b");
        check_space("\u{FEFF}\u{FEFF}", " \u{FEFF}\u{FEFF} ");
    }

    #[test]
    fn space_clusters() {
        // Adjacent matches merge into one cluster wrapped at the edges.
        check_space("!?", " !? ");
        check_space("1,2", " 1,2 ");
        check_space("12%", " 12% ");
        check_space("100%!", " 100%! ");
        check_space("3.14", " 3.14 ");
        check_space("(...)", " (...) ");
        check_space("\u{FEFF}\u{FEFF}", " \u{FEFF}\u{FEFF} ");
        check_space("1٣2", " 1٣2 ");
        check_space("a,b.c!d?e:f;g", "a , b . c ! d ? e : f ; g");
        check_space("abc123def", "abc 123 def");
        check_space("+375 29 123-45-67", " +375   29   123-45-67 ");
        check_space("Я 77-ы", "Я  77- ы");
        check_space("emoji 😀🎉!", "emoji  😀🎉! ");
        check_space("0x123", " 0 x 123 ");
        check_space("12 34", " 12   34 ");
        check_space("100\u{00A0}000", " 100 \u{00A0} 000 ");
        check_space(" \u{FEFF} ", "  \u{FEFF}  ");
    }

    #[test]
    fn space_exhaustive_matches_fancy_regex() {
        let re = fancy_regex::Regex::new(r"\p{P}|\p{S}|\d+").unwrap();
        // Every BMP char isolated by '\n' (Cc: never matches, breaks digit
        // runs), so one batched call checks each char independently.
        // U+FEFF is skipped: Cf is outside P/S and covered by its own tests.
        let mut batch = String::new();
        for cp in 0u32..0x10000 {
            if (0xD800..0xE000).contains(&cp) || cp == 0xFEFF {
                continue;
            }
            batch.push(char::from_u32(cp).unwrap());
            batch.push('\n');
        }
        // Astral range edges (where table errors would live) plus strided
        // samples across the supplementary planes.
        let mut extra = Vec::new();
        for &(lo, hi) in super::PUNCT_SYM_RANGES.iter().filter(|&&(_, hi)| hi >= 0x10000) {
            for cp in [lo.saturating_sub(1), lo, hi, (hi + 1).min(0x10FFFF)] {
                if char::from_u32(cp).is_some() {
                    extra.push(cp);
                }
            }
        }
        let mut cp = 0x10000u32;
        while cp <= 0x10FFFF {
            extra.push(cp);
            cp += 997;
        }
        for cp in extra {
            batch.push(char::from_u32(cp).unwrap());
            batch.push('\n');
        }
        let expected = re.replace_all(&batch, " $0 ").into_owned();
        assert_eq!(space_out_punct_sym_digits(&batch), expected);
    }
}
