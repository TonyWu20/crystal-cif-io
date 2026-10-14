use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_radiation_wavelength_determination`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Determination {
    /// `fundamental`
    Fundamental,
}

impl Display for Determination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Fundamental => "fundamental",
        };
        f.write_str(code)
    }
}

impl From<Determination> for UnquotedString {
    fn from(value: Determination) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Determination> for CharString {
    fn from(value: Determination) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Determination> for Value {
    fn from(value: Determination) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl Determination {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "fundamental" => Some(Self::Fundamental),
            _ => None,
        }
    }
}
