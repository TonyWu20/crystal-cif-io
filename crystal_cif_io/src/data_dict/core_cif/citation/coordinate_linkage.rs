use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_citation_coordinate_linkage`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CoordinateLinkage {
    /// `no`
    No,
    /// `n`
    N,
    /// `yes`
    Yes,
    /// `y`
    Y,
}

impl Display for CoordinateLinkage {
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

impl From<CoordinateLinkage> for UnquotedString {
    fn from(value: CoordinateLinkage) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<CoordinateLinkage> for CharString {
    fn from(value: CoordinateLinkage) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<CoordinateLinkage> for Value {
    fn from(value: CoordinateLinkage) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl CoordinateLinkage {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "no" => Some(Self::No),
            "n" => Some(Self::N),
            "yes" => Some(Self::Yes),
            "y" => Some(Self::Y),
            _ => None,
        }
    }
}
