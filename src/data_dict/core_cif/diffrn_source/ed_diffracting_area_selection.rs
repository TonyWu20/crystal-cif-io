use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_source_ed_diffracting_area_selection`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum EdDiffractingAreaSelection {
    /// `probe`
    Probe,
    /// `SAED`
    SAED,
}

impl Display for EdDiffractingAreaSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Probe => "probe",
            Self::SAED => "SAED",
        };
        f.write_str(code)
    }
}

impl From<EdDiffractingAreaSelection> for UnquotedString {
    fn from(value: EdDiffractingAreaSelection) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<EdDiffractingAreaSelection> for CharString {
    fn from(value: EdDiffractingAreaSelection) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<EdDiffractingAreaSelection> for Value {
    fn from(value: EdDiffractingAreaSelection) -> Self {
        Value::CharString(CharString::from(value))
    }
}
