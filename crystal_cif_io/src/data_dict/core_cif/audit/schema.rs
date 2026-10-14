use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_audit_schema`. The codes are recorded in
/// the shell dictionary `_enumeration` (see `codegen/depends.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Schema {
    /// `Base`
    Base,
    /// `'Space`
    Space,
    /// `Entry`
    Entry,
    /// `Custom`
    Custom,
    /// `Local`
    Local,
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Base => "Base",
            Self::Space => "'Space",
            Self::Entry => "Entry",
            Self::Custom => "Custom",
            Self::Local => "Local",
        };
        f.write_str(code)
    }
}

impl From<Schema> for UnquotedString {
    fn from(value: Schema) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Schema> for CharString {
    fn from(value: Schema) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Schema> for Value {
    fn from(value: Schema) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl Schema {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "Base" => Some(Self::Base),
            "'Space" => Some(Self::Space),
            "Entry" => Some(Self::Entry),
            "Custom" => Some(Self::Custom),
            "Local" => Some(Self::Local),
            _ => None,
        }
    }
}
