use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_audit_author_role_role`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Role {
    /// `design`
    Design,
    /// `synthesis`
    Synthesis,
    /// `preparation`
    Preparation,
    /// `characterisation`
    Characterisation,
    /// `measurement`
    Measurement,
    /// `analysis`
    Analysis,
    /// `software`
    Software,
    /// `submission`
    Submission,
    /// `other`
    Other,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Design => "design",
            Self::Synthesis => "synthesis",
            Self::Preparation => "preparation",
            Self::Characterisation => "characterisation",
            Self::Measurement => "measurement",
            Self::Analysis => "analysis",
            Self::Software => "software",
            Self::Submission => "submission",
            Self::Other => "other",
        };
        f.write_str(code)
    }
}

impl From<Role> for UnquotedString {
    fn from(value: Role) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Role> for CharString {
    fn from(value: Role) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Role> for Value {
    fn from(value: Role) -> Self {
        Value::CharString(CharString::from(value))
    }
}
