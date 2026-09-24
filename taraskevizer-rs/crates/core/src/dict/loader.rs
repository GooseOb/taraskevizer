use super::types::{CompiledDict, DictEntry};

/// Load a CompiledDict from JSON embedded at compile time.
/// The JSON should be an array of DictEntry objects `[{"p":..., "x":..., "r":...}, ...]`.
pub fn load_dict_from_json(json_str: &str) -> CompiledDict {
    let entries: Vec<DictEntry> =
        serde_json::from_str(json_str).expect("failed to parse dictionary JSON");
    CompiledDict::new(&entries)
}

/// Convenience: load with `include_str!`.
///
/// ```ignore
/// let dict = load_include_dict!(include_str!("data/wordlist.json"));
/// ```
#[macro_export]
macro_rules! load_include_dict {
    ($json:expr) => {
        $crate::dict::loader::load_dict_from_json($json)
    };
}
