use super::types::{CompiledDict, DictEntry};

/// Load a flat sequential `CompiledDict` from JSON embedded at compile time.
/// The JSON should be an array of `DictEntry` objects `[{"p":..., "r":...}, ...]`.
/// Kept for tests; production dicts use the batched variant below.
pub fn load_dict_from_json(json_str: &str) -> CompiledDict {
    let entries: Vec<DictEntry> =
        serde_json::from_str(json_str).expect("failed to parse dictionary JSON");
    CompiledDict::new(&entries)
}

/// Load a batched `CompiledDict` from JSON `[[{p, r}, ...], ...]` where the
/// LAST inner array is the sequential tail and all preceding arrays are
/// single-pass batches.
pub fn load_batched_dict_from_json(json_str: &str) -> CompiledDict {
    let batches: Vec<Vec<DictEntry>> =
        serde_json::from_str(json_str).expect("failed to parse batched dictionary JSON");
    CompiledDict::new_batched_entries(&batches)
}

/// Convenience: load with `include_str!`.
///
/// ```ignore
/// let dict = load_include_dict!(include_str!("data/phonetic.json"));
/// let dict = load_include_batched_dict!(include_str!("data/wordlist.json"));
/// ```
#[macro_export]
macro_rules! load_include_dict {
    ($json:expr) => {
        $crate::dict::loader::load_dict_from_json($json)
    };
}

/// Convenience for batched `[[{p, r}]]` JSON (wordlist).
#[macro_export]
macro_rules! load_include_batched_dict {
    ($json:expr) => {
        $crate::dict::loader::load_batched_dict_from_json($json)
    };
}
