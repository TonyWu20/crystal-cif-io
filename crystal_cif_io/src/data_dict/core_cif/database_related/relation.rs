use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_database_related_relation`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Relation {
    /// `Identical`
    Identical,
    /// `Subset`
    Subset,
    /// `Superset`
    Superset,
    /// `Derived`
    Derived,
    /// `Common`
    Common,
    /// `Other`
    Other,
}

impl Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Identical => "Identical",
            Self::Subset => "Subset",
            Self::Superset => "Superset",
            Self::Derived => "Derived",
            Self::Common => "Common",
            Self::Other => "Other",
        };
        f.write_str(code)
    }
}

impl From<Relation> for UnquotedString {
    fn from(value: Relation) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Relation> for CharString {
    fn from(value: Relation) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Relation> for Value {
    fn from(value: Relation) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl Relation {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "Identical" => Some(Self::Identical),
            "Subset" => Some(Self::Subset),
            "Superset" => Some(Self::Superset),
            "Derived" => Some(Self::Derived),
            "Common" => Some(Self::Common),
            "Other" => Some(Self::Other),
            _ => None,
        }
    }
}
