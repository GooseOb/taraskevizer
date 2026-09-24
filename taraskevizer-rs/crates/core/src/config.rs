use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alphabet {
    Cyrillic,
    Latin,
    LatinJi,
    Arabic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JMode {
    Never,
    Random,
    Always,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariationMode {
    No,
    First,
    All,
}

pub struct Wrappers {
    pub fix: Option<fn(&str) -> String>,
    pub variable: VariationWrappers,
    pub letter_h: Option<fn(char) -> String>,
}

impl fmt::Debug for Wrappers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wrappers")
            .field("fix", &self.fix.as_ref().map(|_| "<fn>"))
            .field("variable", &self.variable)
            .field("letter_h", &self.letter_h.as_ref().map(|_| "<fn>"))
            .finish()
    }
}

impl Clone for Wrappers {
    fn clone(&self) -> Self {
        Self {
            fix: self.fix,
            variable: self.variable.clone(),
            letter_h: self.letter_h,
        }
    }
}

pub struct VariationWrappers {
    pub all: fn(&str) -> String,
    pub first: fn(&str) -> String,
    pub no: fn(&str) -> String,
}

impl fmt::Debug for VariationWrappers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariationWrappers")
            .field("all", &"<fn>")
            .field("first", &"<fn>")
            .field("no", &"<fn>")
            .finish()
    }
}

impl Clone for VariationWrappers {
    fn clone(&self) -> Self {
        Self {
            all: self.all,
            first: self.first,
            no: self.no,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaraskConfig {
    pub abc: Alphabet,
    pub j: JMode,
    pub do_escape_capitalized: bool,
    pub wrappers: Option<Wrappers>,
    pub g: bool,
    pub variations: VariationMode,
    pub new_line: String,
    pub left_angle_bracket: String,
    pub no_fix_placeholder: String,
}

impl Default for TaraskConfig {
    fn default() -> Self {
        Self {
            abc: Alphabet::Cyrillic,
            j: JMode::Never,
            do_escape_capitalized: true,
            wrappers: None,
            g: true,
            variations: VariationMode::All,
            new_line: "\n".into(),
            left_angle_bracket: "<".into(),
            no_fix_placeholder: " \u{e0fe} ".into(),
        }
    }
}
