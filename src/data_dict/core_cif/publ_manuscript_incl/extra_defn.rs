use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_publ_manuscript_incl_extra_defn`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ExtraDefn {
    /// `no`
    No,
    /// `n`
    N,
    /// `yes`
    Yes,
    /// `y`
    Y,
}

impl Display for ExtraDefn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::No => "no",
            Self::N => "n",
            Self::Yes => "yes",
            Self::Y => "y",
        };
        f.write_str(code)
    }
}

impl From<ExtraDefn> for UnquotedString {
    fn from(value: ExtraDefn) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<ExtraDefn> for CharString {
    fn from(value: ExtraDefn) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<ExtraDefn> for Value {
    fn from(value: ExtraDefn) -> Self {
        Value::CharString(CharString::from(value))
    }
}
