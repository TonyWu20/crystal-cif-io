use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_component_pid_framework`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Framework {
    /// `DataCite`
    DataCite,
    /// `ePIC`
    EPIC,
}

impl Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::DataCite => "DataCite",
            Self::EPIC => "ePIC",
        };
        f.write_str(code)
    }
}

impl From<Framework> for UnquotedString {
    fn from(value: Framework) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Framework> for CharString {
    fn from(value: Framework) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Framework> for Value {
    fn from(value: Framework) -> Self {
        Value::CharString(CharString::from(value))
    }
}
