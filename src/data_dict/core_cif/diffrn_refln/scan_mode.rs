use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_refln_scan_mode`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ScanMode {
    /// `om`
    Om,
    /// `ot`
    Ot,
    /// `q`
    Q,
}

impl Display for ScanMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Om => "om",
            Self::Ot => "ot",
            Self::Q => "q",
        };
        f.write_str(code)
    }
}

impl From<ScanMode> for UnquotedString {
    fn from(value: ScanMode) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<ScanMode> for CharString {
    fn from(value: ScanMode) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<ScanMode> for Value {
    fn from(value: ScanMode) -> Self {
        Value::CharString(CharString::from(value))
    }
}
