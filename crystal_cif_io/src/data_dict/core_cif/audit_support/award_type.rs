use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_audit_support_award_type`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AwardType {
    /// `gift`
    Gift,
    /// `grant`
    Grant,
    /// `contract`
    Contract,
    /// `studentship`
    Studentship,
    /// `other`
    Other,
}

impl Display for AwardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Gift => "gift",
            Self::Grant => "grant",
            Self::Contract => "contract",
            Self::Studentship => "studentship",
            Self::Other => "other",
        };
        f.write_str(code)
    }
}

impl From<AwardType> for UnquotedString {
    fn from(value: AwardType) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<AwardType> for CharString {
    fn from(value: AwardType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<AwardType> for Value {
    fn from(value: AwardType) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl AwardType {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "gift" => Some(Self::Gift),
            "grant" => Some(Self::Grant),
            "contract" => Some(Self::Contract),
            "studentship" => Some(Self::Studentship),
            "other" => Some(Self::Other),
            _ => None,
        }
    }
}
