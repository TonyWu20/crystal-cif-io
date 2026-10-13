use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_source_target`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Target {
    /// `H`
    H,
    /// `Ar`
    Ar,
    /// `As`
    As,
    /// `Cd`
    Cd,
    /// `Eu`
    Eu,
    /// `Pt`
    Pt,
    /// `Np`
    Np,
    /// `Hs`
    Hs,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::H => "H",
            Self::Ar => "Ar",
            Self::As => "As",
            Self::Cd => "Cd",
            Self::Eu => "Eu",
            Self::Pt => "Pt",
            Self::Np => "Np",
            Self::Hs => "Hs",
        };
        f.write_str(code)
    }
}

impl From<Target> for UnquotedString {
    fn from(value: Target) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Target> for CharString {
    fn from(value: Target) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Target> for Value {
    fn from(value: Target) -> Self {
        Value::CharString(CharString::from(value))
    }
}
