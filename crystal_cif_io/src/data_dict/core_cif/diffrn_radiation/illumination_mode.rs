use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_radiation_illumination_mode`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum IlluminationMode {
    /// `convergent`
    Convergent,
    /// `parallel`
    Parallel,
}

impl Display for IlluminationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Convergent => "convergent",
            Self::Parallel => "parallel",
        };
        f.write_str(code)
    }
}

impl From<IlluminationMode> for UnquotedString {
    fn from(value: IlluminationMode) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<IlluminationMode> for CharString {
    fn from(value: IlluminationMode) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<IlluminationMode> for Value {
    fn from(value: IlluminationMode) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl IlluminationMode {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "convergent" => Some(Self::Convergent),
            "parallel" => Some(Self::Parallel),
            _ => None,
        }
    }
}
