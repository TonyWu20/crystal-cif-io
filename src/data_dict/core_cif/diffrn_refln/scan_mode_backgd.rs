use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_refln_scan_mode_backgd`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ScanModeBackgd {
    /// `st`
    St,
    /// `mo`
    Mo,
}

impl Display for ScanModeBackgd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::St => "st",
            Self::Mo => "mo",
        };
        f.write_str(code)
    }
}

impl From<ScanModeBackgd> for UnquotedString {
    fn from(value: ScanModeBackgd) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<ScanModeBackgd> for CharString {
    fn from(value: ScanModeBackgd) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<ScanModeBackgd> for Value {
    fn from(value: ScanModeBackgd) -> Self {
        Value::CharString(CharString::from(value))
    }
}
