//! WASM bindings for `taraskevizer`.
//!
//! Exposes the core pipelines ([`tarask`], [`alphabetic`], [`phonetic`]).
//! Conversion options are owned by TypeScript (`src/config.ts`): the
//! `TaraskConfig` class and `htmlConfigOptions()` live there as plain JS
//! values, and the pipeline functions below only parse the resulting shape
//! (a `TaraskConfig` instance or a plain options object) via `Reflect`.
//!
//! # JS usage
//!
//! ```js
//! import init, { tarask, alphabetic, phonetic } from 'taraskevizer';
//! import { TaraskConfig, htmlConfigOptions } from 'taraskevizer';
//!
//! await init();
//!
//! tarask('планета'); // default config → 'плянэта'
//! tarask('планета', new TaraskConfig({ abc: 'latin' })); // → 'planeta'
//! tarask('планета', { abc: 'latin' }); // plain object also works
//!
//! const htmlCfg = htmlConfigOptions();
//! tarask('энергія', htmlCfg); // → 'эн<tarF>э</tarF>р<tarH>г</tarH>ія'
//! ```

use js_sys::Reflect;
use taraskevizer_core::{
    config::{Alphabet, JMode, TaraskConfig as CoreConfig, VariationMode},
    wrappers::{ANSI_COLOR_WRAPPERS, HTML_WRAPPERS},
};
use wasm_bindgen::prelude::*;

/// Which predefined wrapper set is active.
///
/// Custom JS-function wrappers cannot cross the WASM boundary, so only the
/// predefined sets from `taraskevizer_core::wrappers` are exposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum WrappersKind {
    /// No highlighting (`wrappers: null` in JS).
    #[default]
    None,
    /// `<tarF>` / `<tarL>` / `<tarH>` tags (web output).
    Html,
    /// ANSI colors (terminal output).
    Ansi,
}

impl WrappersKind {
    fn apply_to(self, cfg: &mut CoreConfig) {
        cfg.wrappers = match self {
            Self::None => None,
            Self::Html => Some(HTML_WRAPPERS),
            Self::Ansi => Some(ANSI_COLOR_WRAPPERS),
        };
    }

    #[cfg(test)]
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Html => "html",
            Self::Ansi => "ansi",
        }
    }
}

/// Validation failure for a config value.
///
/// Pure-Rust error type so all validation logic stays unit-testable on native
/// targets; converted to a JS exception (`JsError`, which can only be built
/// on wasm32) at the `#[wasm_bindgen]` boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConfigError(&'static str);

impl core::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

impl From<ConfigError> for JsError {
    fn from(err: ConfigError) -> Self {
        Self::new(err.0)
    }
}

fn parse_abc(s: &str) -> Result<Alphabet, ConfigError> {
    // Accept `latin-ji` / `latin_ji` spellings alongside the canonical `latinJi`.
    let normalized: String = s
        .chars()
        .filter(|c| *c != '-' && *c != '_')
        .flat_map(|c| c.to_lowercase())
        .collect();
    match normalized.as_str() {
        "cyrillic" => Ok(Alphabet::Cyrillic),
        "latin" => Ok(Alphabet::Latin),
        "latinji" => Ok(Alphabet::LatinJi),
        "arabic" => Ok(Alphabet::Arabic),
        _ => Err(ConfigError(
            "invalid abc: expected one of \"cyrillic\", \"latin\", \"latinJi\", \"arabic\"",
        )),
    }
}

fn parse_j(s: &str) -> Result<JMode, ConfigError> {
    match s.to_lowercase().as_str() {
        "never" => Ok(JMode::Never),
        "random" => Ok(JMode::Random),
        "always" => Ok(JMode::Always),
        _ => Err(ConfigError(
            "invalid j: expected one of \"never\", \"random\", \"always\"",
        )),
    }
}

fn parse_variations(s: &str) -> Result<VariationMode, ConfigError> {
    match s.to_lowercase().as_str() {
        "all" => Ok(VariationMode::All),
        "no" => Ok(VariationMode::No),
        "first" => Ok(VariationMode::First),
        _ => Err(ConfigError(
            "invalid variations: expected one of \"all\", \"no\", \"first\"",
        )),
    }
}

fn parse_wrappers(s: &str) -> Result<WrappersKind, ConfigError> {
    match s.to_lowercase().as_str() {
        "none" | "null" => Ok(WrappersKind::None),
        "html" => Ok(WrappersKind::Html),
        "ansi" | "ansicolor" | "ansi-color" => Ok(WrappersKind::Ansi),
        _ => Err(ConfigError(
            "invalid wrappers: expected one of \"none\", \"html\", \"ansi\"",
        )),
    }
}

/// Plain-object options accepted by the [`TaraskConfig`] constructor and the
/// pipeline functions.
///
/// Mirrors the JS `Partial<TaraskConfig>` shape with `camelCase` names.
/// Read field-by-field via `Reflect`, so both plain objects and
/// `TaraskConfig` instances (through their prototype getters) are accepted.
/// Unknown fields are ignored, like the JS constructor does.
#[derive(Debug, Default)]
struct PartialOptions {
    abc: Option<String>,
    j: Option<String>,
    do_escape_capitalized: Option<bool>,
    wrappers: Option<String>,
    g: Option<bool>,
    variations: Option<String>,
    new_line: Option<String>,
    left_angle_bracket: Option<String>,
    no_fix_placeholder: Option<String>,
}

/// Read an optional string field via `Reflect::get`.
///
/// `undefined`/`null` mean "not provided". Any other non-string value is a
/// type error.
///
/// # Errors
///
/// Returns an error when the property cannot be read or holds a non-string,
/// non-nullish value.
fn read_string_field(obj: &JsValue, key: &str, ty: &str) -> Result<Option<String>, JsError> {
    let value = Reflect::get(obj, &JsValue::from_str(key))
        .map_err(|_| JsError::new(&format!("failed to read config field {key:?}")))?;
    if value.is_undefined() || value.is_null() {
        return Ok(None);
    }
    value
        .as_string()
        .map(Some)
        .ok_or_else(|| JsError::new(&format!("invalid config field {key:?}: expected {ty}")))
}

/// Read an optional boolean field via `Reflect::get`.
///
/// # Errors
///
/// Returns an error when the property cannot be read or holds a non-boolean,
/// non-nullish value.
fn read_bool_field(obj: &JsValue, key: &str) -> Result<Option<bool>, JsError> {
    let value = Reflect::get(obj, &JsValue::from_str(key))
        .map_err(|_| JsError::new(&format!("failed to read config field {key:?}")))?;
    if value.is_undefined() || value.is_null() {
        return Ok(None);
    }
    value
        .as_bool()
        .map(Some)
        .ok_or_else(|| JsError::new(&format!("invalid config field {key:?}: expected boolean")))
}

/// Read [`PartialOptions`] out of a `JsValue`.
///
/// Accepts `undefined`/`null` (all defaults), plain objects and
/// `TaraskConfig` instances. Anything else is rejected so typos like
/// `tarask(text, "latin")` fail loudly instead of silently using defaults.
///
/// # Errors
///
/// Returns an error for non-object configs or invalid field values.
/// Never touches JS for `undefined`/`null`/non-objects, so those paths are
/// unit-testable on native targets.
fn read_options(value: &JsValue) -> Result<PartialOptions, JsError> {
    if value.is_undefined() || value.is_null() {
        return Ok(PartialOptions::default());
    }
    if !value.is_object() {
        return Err(JsError::new(
            "invalid config: expected an options object, a TaraskConfig instance, or undefined",
        ));
    }
    Ok(PartialOptions {
        abc: read_string_field(value, "abc", "TaraskAlphabet")?,
        j: read_string_field(value, "j", "TaraskJ")?,
        do_escape_capitalized: read_bool_field(value, "doEscapeCapitalized")?,
        wrappers: read_string_field(value, "wrappers", "TaraskWrappers")?,
        g: read_bool_field(value, "g")?,
        variations: read_string_field(value, "variations", "TaraskVariations")?,
        new_line: read_string_field(value, "newLine", "string")?,
        left_angle_bracket: read_string_field(value, "leftAngleBracket", "string")?,
        no_fix_placeholder: read_string_field(value, "noFixPlaceholder", "string")?,
    })
}

/// Parsed conversion options for the pipelines.
///
/// Pure-Rust helper: the `TaraskConfig` class itself is owned by TypeScript
/// (`src/config.ts`). Instances of that class — like plain options objects —
/// are plain JS objects, so [`read_options`] picks their fields up via
/// `Reflect` and this struct only applies the validated values.
#[derive(Debug)]
struct WasmConfig {
    inner: CoreConfig,
    wrappers_kind: WrappersKind,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            inner: CoreConfig::default(),
            wrappers_kind: WrappersKind::None,
        }
    }
}

/// Test-only accessors mirroring the TS `TaraskConfig` getters/setters, so
/// the same validation and pipeline behavior stays covered natively.
#[cfg(test)]
impl WasmConfig {
    /// Alphabet: `"cyrillic"` (default), `"latin"`, `"latinJi"` or `"arabic"`.
    fn abc(&self) -> String {
        match self.inner.abc {
            Alphabet::Cyrillic => "cyrillic",
            Alphabet::Latin => "latin",
            Alphabet::LatinJi => "latinJi",
            Alphabet::Arabic => "arabic",
        }
        .to_owned()
    }

    /// Set the alphabet.
    ///
    /// # Errors
    ///
    /// Returns an error for unknown alphabet names.
    fn set_abc(&mut self, value: String) -> Result<(), ConfigError> {
        self.inner.abc = parse_abc(&value)?;
        Ok(())
    }

    /// When to replace `і` by `й` after vowels: `"never"` (default),
    /// `"random"` or `"always"`.
    fn j(&self) -> String {
        match self.inner.j {
            JMode::Never => "never",
            JMode::Random => "random",
            JMode::Always => "always",
        }
        .to_owned()
    }

    /// Set the `і→й` mode.
    ///
    /// # Errors
    ///
    /// Returns an error for unknown mode names.
    fn set_j(&mut self, value: String) -> Result<(), ConfigError> {
        self.inner.j = parse_j(&value)?;
        Ok(())
    }

    /// Whether capitalized words are protected from changes (default `true`).
    fn do_escape_capitalized(&self) -> bool {
        self.inner.do_escape_capitalized
    }

    /// Active wrapper set: `"none"` (default), `"html"` or `"ansi"`.
    fn wrappers(&self) -> String {
        self.wrappers_kind.as_str().to_owned()
    }

    /// Set the wrapper set.
    ///
    /// # Errors
    ///
    /// Returns an error for unknown wrapper names.
    fn set_wrappers(&mut self, value: String) -> Result<(), ConfigError> {
        let kind = parse_wrappers(&value)?;
        self.wrappers_kind = kind;
        kind.apply_to(&mut self.inner);
        Ok(())
    }

    /// Whether to convert `ґ→г`-style `г` into `ґ` where appropriate
    /// (default `true`; `false` in the TS `htmlConfigOptions()`).
    fn g(&self) -> bool {
        self.inner.g
    }

    /// Which word variation to keep: `"all"` (default), `"no"` or `"first"`.
    fn variations(&self) -> String {
        match self.inner.variations {
            VariationMode::All => "all",
            VariationMode::No => "no",
            VariationMode::First => "first",
        }
        .to_owned()
    }

    /// Set `variations`.
    ///
    /// # Errors
    ///
    /// Returns an error for unknown variation names.
    fn set_variations(&mut self, value: String) -> Result<(), ConfigError> {
        self.inner.variations = parse_variations(&value)?;
        Ok(())
    }

    /// Replacement for `"\n"` (default `"\n"`, `"<br>"` in HTML mode).
    fn new_line(&self) -> String {
        self.inner.new_line.clone()
    }

    /// Replacement for `"<"` (default `"<"`, `"&lt"` in HTML mode).
    fn left_angle_bracket(&self) -> String {
        self.inner.left_angle_bracket.clone()
    }

    /// Placeholder for `<…>`-protected spans (default `" \u{e0fe} "`).
    fn no_fix_placeholder(&self) -> String {
        self.inner.no_fix_placeholder.clone()
    }
}

impl WasmConfig {
    /// Apply validated [`PartialOptions`] onto this config.
    ///
    /// Pure Rust (no JS interop), so this is directly unit-testable on
    /// native targets; the `JsValue` reading lives in [`read_options`].
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] for invalid string option values.
    fn apply_options(&mut self, opts: &PartialOptions) -> Result<(), ConfigError> {
        if let Some(v) = &opts.abc {
            self.inner.abc = parse_abc(v)?;
        }
        if let Some(v) = &opts.j {
            self.inner.j = parse_j(v)?;
        }
        if let Some(v) = opts.do_escape_capitalized {
            self.inner.do_escape_capitalized = v;
        }
        if let Some(v) = &opts.wrappers {
            let kind = parse_wrappers(v)?;
            self.wrappers_kind = kind;
            kind.apply_to(&mut self.inner);
        }
        if let Some(v) = opts.g {
            self.inner.g = v;
        }
        if let Some(v) = &opts.variations {
            self.inner.variations = parse_variations(v)?;
        }
        if let Some(v) = &opts.new_line {
            self.inner.new_line.clone_from(v);
        }
        if let Some(v) = &opts.left_angle_bracket {
            self.inner.left_angle_bracket.clone_from(v);
        }
        if let Some(v) = &opts.no_fix_placeholder {
            self.inner.no_fix_placeholder.clone_from(v);
        }
        Ok(())
    }

    // Plain-Rust entry points (borrowed config, reusable) behind the
    // `JsValue`-accepting WASM wrappers below. These are what unit tests
    // exercise; they never touch JS values.
    #[cfg(test)]
    fn core_config(config: Option<&Self>) -> CoreConfig {
        config.map_or_else(CoreConfig::default, |c| c.inner.clone())
    }

    #[cfg(test)]
    fn tarask_inner(text: &str, config: Option<&Self>) -> String {
        let cfg = Self::core_config(config);
        taraskevizer_core::tarask(text, &cfg)
    }

    #[cfg(test)]
    fn alphabetic_inner(text: &str, config: Option<&Self>) -> String {
        let cfg = Self::core_config(config);
        taraskevizer_core::alphabetic(text, &cfg)
    }

    #[cfg(test)]
    fn phonetic_inner(text: &str, config: Option<&Self>) -> String {
        let cfg = Self::core_config(config);
        taraskevizer_core::phonetic(text, &cfg)
    }
}

/// Resolve the `config` argument accepted by the pipeline wrappers.
///
/// Accepts `undefined`/`null` (defaults), a TS `TaraskConfig` instance (read
/// through its getters, reusable across calls) or a plain options object:
///
/// ```js
/// tarask('планета');
/// tarask('планета', cfg);
/// tarask('планета', { abc: 'latin', g: false });
/// ```
///
/// # Errors
///
/// Returns an error when a non-object is given or one of the fields is
/// invalid.
fn resolve_config(config: &JsValue) -> Result<CoreConfig, JsError> {
    let opts = read_options(config)?;
    let mut cfg = WasmConfig::default();
    cfg.apply_options(&opts)?;
    Ok(cfg.inner)
}

/// Convert academic orthography to classical (Taraskevica).
///
/// ```js
/// tarask('планета'); // → 'плянэта'
/// tarask('планета', new TaraskConfig({ abc: 'latin' })); // → 'planeta'
/// tarask('планета', { abc: 'latin' }); // plain object also works
/// ```
///
/// # Errors
///
/// Returns an error (throws in JS) when `config` is a plain object with
/// invalid option values.
#[wasm_bindgen(js_name = tarask)]
pub fn tarask_js(text: &str, config: JsValue) -> Result<String, JsError> {
    let cfg = resolve_config(&config)?;
    Ok(taraskevizer_core::tarask(text, &cfg))
}

/// Alphabet-only conversion (no Taraskevization).
///
/// Accepts the same `config` shapes as [`tarask_js`].
///
/// # Errors
///
/// Returns an error (throws in JS) when `config` is a plain object with
/// invalid option values.
#[wasm_bindgen(js_name = alphabetic)]
pub fn alphabetic_js(text: &str, config: JsValue) -> Result<String, JsError> {
    let cfg = resolve_config(&config)?;
    Ok(taraskevizer_core::alphabetic(text, &cfg))
}

/// Phonetic conversion (experimental in the reference implementation).
///
/// Accepts the same `config` shapes as [`tarask_js`].
///
/// # Errors
///
/// Returns an error (throws in JS) when `config` is a plain object with
/// invalid option values.
#[wasm_bindgen(js_name = phonetic)]
pub fn phonetic_js(text: &str, config: JsValue) -> Result<String, JsError> {
    let cfg = resolve_config(&config)?;
    Ok(taraskevizer_core::phonetic(text, &cfg))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    fn cfg_with(abc: &str) -> WasmConfig {
        let mut cfg = WasmConfig::default();
        cfg.set_abc(abc.to_owned()).expect("valid test alphabet");
        cfg
    }

    #[test]
    fn default_config_matches_core_defaults() {
        let cfg = WasmConfig::default();
        assert_eq!(cfg.abc(), "cyrillic");
        assert_eq!(cfg.j(), "never");
        assert!(cfg.do_escape_capitalized());
        assert_eq!(cfg.wrappers(), "none");
        assert!(cfg.g());
        assert_eq!(cfg.variations(), "all");
        assert_eq!(cfg.new_line(), "\n");
        assert_eq!(cfg.left_angle_bracket(), "<");
    }

    #[test]
    fn invalid_option_strings_rejected() {
        // Pure validation: no JS interop, runs natively.
        assert_eq!(
            parse_abc("klingon").unwrap_err(),
            ConfigError(
                "invalid abc: expected one of \"cyrillic\", \"latin\", \"latinJi\", \"arabic\""
            )
        );
        assert!(parse_j("sometimes").is_err());
        assert!(parse_variations("many").is_err());
        assert!(parse_wrappers("markdown").is_err());
        // Case-insensitive spellings are accepted.
        assert!(parse_abc("Latin").is_ok());
        assert!(parse_j("Always").is_ok());
        assert!(parse_variations("NO").is_ok());
        assert!(parse_wrappers("HTML").is_ok());
    }

    #[test]
    fn apply_options_covers_all_fields() {
        // Same path the pipelines take for a fully-specified object, but
        // with a hand-built struct so it runs natively.
        let mut cfg = WasmConfig::default();
        cfg.apply_options(&PartialOptions {
            abc: Some("latin".to_owned()),
            j: Some("always".to_owned()),
            do_escape_capitalized: Some(false),
            wrappers: Some("ansi".to_owned()),
            g: Some(false),
            variations: Some("no".to_owned()),
            new_line: Some("<br>".to_owned()),
            left_angle_bracket: Some("&lt".to_owned()),
            no_fix_placeholder: Some("|".to_owned()),
        })
        .expect("valid options");
        assert_eq!(cfg.abc(), "latin");
        assert_eq!(cfg.j(), "always");
        assert!(!cfg.do_escape_capitalized());
        assert_eq!(cfg.wrappers(), "ansi");
        assert!(!cfg.g());
        assert_eq!(cfg.variations(), "no");
        assert_eq!(cfg.new_line(), "<br>");
        assert_eq!(cfg.left_angle_bracket(), "&lt");
        assert_eq!(cfg.no_fix_placeholder(), "|");
    }

    #[test]
    fn apply_options_rejects_bad_values() {
        let mut cfg = WasmConfig::default();
        let opts = PartialOptions {
            abc: Some("klingon".to_owned()),
            ..PartialOptions::default()
        };
        assert!(cfg.apply_options(&opts).is_err());
    }

    #[test]
    fn wrappers_setter_syncs_pipeline_output() {
        let mut cfg = WasmConfig::default();
        cfg.set_wrappers("html".to_owned()).expect("valid wrappers");
        assert_eq!(cfg.wrappers(), "html");
        assert_eq!(
            WasmConfig::tarask_inner("энергія", Some(&cfg)),
            "эн<tarF>э</tarF>р<tarH>ґ</tarH>ія"
        );
    }

    #[test]
    fn tarask_default_pipeline() {
        assert_eq!(WasmConfig::tarask_inner("планета", None), "плянэта");
        assert_eq!(WasmConfig::tarask_inner("гродна", None), "(гродна|горадня)");
    }

    #[test]
    fn tarask_with_latin_config() {
        let cfg = cfg_with("latin");
        assert_eq!(WasmConfig::tarask_inner("планета", Some(&cfg)), "planeta");
    }

    #[test]
    fn tarask_latin_ji_aliases() {
        for alias in ["latinJi", "latin-ji", "latin_ji"] {
            let cfg = cfg_with(alias);
            assert_eq!(cfg.abc(), "latinJi", "alias {alias}");
        }
    }

    #[test]
    fn tarask_itoj_modes() {
        let mut cfg = WasmConfig::default();
        cfg.set_j("always".to_owned()).expect("valid j");
        assert_eq!(WasmConfig::tarask_inner("яна і ён", Some(&cfg)), "яна й ён");
        assert_eq!(WasmConfig::tarask_inner("яна і ён", None), "яна і ён");
    }

    #[test]
    fn tarask_variation_modes() {
        let mut cfg = WasmConfig::default();
        cfg.set_variations("no".to_owned())
            .expect("valid variations");
        assert_eq!(WasmConfig::tarask_inner("гродна", Some(&cfg)), "гродна");
        cfg.set_variations("first".to_owned())
            .expect("valid variations");
        assert_eq!(WasmConfig::tarask_inner("гродна", Some(&cfg)), "горадня");
    }

    #[test]
    fn alphabetic_and_phonetic_pipelines() {
        let latin = cfg_with("latin");
        assert_eq!(
            WasmConfig::alphabetic_inner("планета", Some(&latin)),
            "płanieta"
        );
        assert_eq!(
            WasmConfig::phonetic_inner("не маю часу", None),
            "ня маю часу"
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn js_wrappers_default_to_defaults() {
        assert_eq!(
            tarask_js("планета", JsValue::UNDEFINED).expect("default call"),
            "плянэта"
        );
        assert_eq!(
            tarask_js("планета", JsValue::NULL).expect("null call"),
            "плянэта"
        );
    }

    // `JsValue` payloads can only be built on wasm32 (constructors like
    // `JsValue::from_str` abort natively), so this runs under
    // `wasm-pack test`, not `cargo test`.
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn non_object_config_rejected() {
        // Guard against typos like `tarask(text, "latin")`: primitives must
        // fail loudly instead of silently using defaults.
        assert!(resolve_config(&JsValue::from_str("latin")).is_err());
        assert!(resolve_config(&JsValue::from_f64(42.0)).is_err());
        assert!(resolve_config(&JsValue::TRUE).is_err());
    }

    // Reads every field out of a real JS object, including type errors and
    // the `null`-means-default convention. Runs under `wasm-pack test`.
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn plain_object_options_parsed() {
        use js_sys::{Object, Reflect};

        fn set(obj: &JsValue, key: &str, value: &JsValue) {
            Reflect::set(obj, &JsValue::from_str(key), value)
                .expect("Reflect.set on a fresh object");
        }

        let obj: JsValue = Object::new().into();
        set(&obj, "abc", &JsValue::from_str("latin"));
        set(&obj, "j", &JsValue::from_str("always"));
        set(&obj, "doEscapeCapitalized", &JsValue::from(false));
        set(&obj, "wrappers", &JsValue::from_str("html"));
        set(&obj, "g", &JsValue::from(false));
        set(&obj, "variations", &JsValue::from_str("no"));
        set(&obj, "newLine", &JsValue::from_str("<br>"));
        set(&obj, "leftAngleBracket", &JsValue::NULL);
        let cfg = resolve_config(&obj).expect("valid plain object");
        assert!(matches!(cfg.abc, Alphabet::Latin));
        assert!(matches!(cfg.j, JMode::Always));
        assert!(!cfg.do_escape_capitalized);
        assert!(cfg.wrappers.is_some());
        assert!(!cfg.g);
        assert!(matches!(cfg.variations, VariationMode::No));
        assert_eq!(cfg.new_line, "<br>");
        // `null` falls back to the default.
        assert_eq!(cfg.left_angle_bracket, "<");

        // Wrong field types are rejected.
        let bad: JsValue = Object::new().into();
        set(&bad, "g", &JsValue::from_str("yes"));
        assert!(resolve_config(&bad).is_err());

        let bad_enum: JsValue = Object::new().into();
        set(&bad_enum, "abc", &JsValue::from_str("klingon"));
        assert!(resolve_config(&bad_enum).is_err());
    }

    // `j: "random"` draws from `getrandom` (the `js` backend on wasm32):
    // smoke-test that it produces one of the two valid forms.
    #[test]
    fn jrandom_produces_valid_forms() {
        let mut cfg = WasmConfig::default();
        cfg.set_j("random".to_owned()).expect("valid j");
        for _ in 0..10 {
            let out = WasmConfig::tarask_inner("яна і ён", Some(&cfg));
            assert!(out == "яна і ён" || out == "яна й ён", "got {out:?}");
        }
    }

    #[test]
    fn html_config_options_mirror_reference() {
        // Mirrors the TS `htmlConfigOptions()`: HTML wrappers, no `g`,
        // `<br>` newlines and `"&lt"` for `<`.
        let cfg = WasmConfig {
            inner: taraskevizer_core::html_config_options(),
            wrappers_kind: WrappersKind::Html,
        };
        assert_eq!(cfg.wrappers(), "html");
        assert!(!cfg.g());
        assert_eq!(cfg.new_line(), "<br>");
        assert_eq!(cfg.left_angle_bracket(), "&lt");
        assert_eq!(
            WasmConfig::tarask_inner("жыццясцвярджальны план", Some(&cfg)),
            "жыц<tarF>ь</tarF>цяс<tarF>ь</tarF>ц<tarF>ь</tarF>вярджальны пл<tarF>я</tarF>н",
        );
    }
}
