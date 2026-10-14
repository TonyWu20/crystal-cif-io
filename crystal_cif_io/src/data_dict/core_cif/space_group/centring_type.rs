use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed set of centring codes (the shell dictionary records nine).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CentringType {
    P,
    A,
    B,
    C,
    F,
    I,
    R,
    Rrev,
    H,
}

impl Display for CentringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = format!("{:?}", self);
        write!(f, "{}", content)
    }
}

impl From<CentringType> for UnquotedString {
    fn from(value: CentringType) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<CentringType> for CharString {
    fn from(value: CentringType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<CentringType> for Value {
    fn from(value: CentringType) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl CentringType {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "P" => Some(Self::P),
            "A" => Some(Self::A),
            "B" => Some(Self::B),
            "C" => Some(Self::C),
            "F" => Some(Self::F),
            "I" => Some(Self::I),
            "R" => Some(Self::R),
            "Rrev" => Some(Self::Rrev),
            "H" => Some(Self::H),
            _ => None,
        }
    }
}
