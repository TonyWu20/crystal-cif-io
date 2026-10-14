use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_journal_index_type`. The codes are recorded in
/// the shell dictionary `_enumeration` (see `codegen/depends.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Type {
    /// `O`
    O,
    /// `I`
    I,
    /// `M`
    M,
    /// `S`
    S,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::O => "O",
            Self::I => "I",
            Self::M => "M",
            Self::S => "S",
        };
        f.write_str(code)
    }
}

impl From<Type> for UnquotedString {
    fn from(value: Type) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Type> for CharString {
    fn from(value: Type) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Type> for Value {
    fn from(value: Type) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl Type {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "O" => Some(Self::O),
            "I" => Some(Self::I),
            "M" => Some(Self::M),
            "S" => Some(Self::S),
            _ => None,
        }
    }
}
