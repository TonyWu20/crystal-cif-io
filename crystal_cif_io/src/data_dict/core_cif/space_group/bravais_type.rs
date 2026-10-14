use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed set of Bravais type codes (the shell dictionary records fourteen).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BravaisType {
    AP,
    MP,
    MS,
    OP,
    OS,
    OI,
    OF,
    TP,
    TI,
    HP,
    HR,
    CP,
    CI,
    CF,
}

impl Display for BravaisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::AP => "aP",
            Self::MP => "mP",
            Self::MS => "mS",
            Self::OP => "oP",
            Self::OS => "oS",
            Self::OI => "oI",
            Self::OF => "oF",
            Self::TP => "tP",
            Self::TI => "tI",
            Self::HP => "hP",
            Self::HR => "hR",
            Self::CP => "cP",
            Self::CI => "cI",
            Self::CF => "cF",
        };
        write!(f, "{}", code)
    }
}

impl From<BravaisType> for UnquotedString {
    fn from(value: BravaisType) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<BravaisType> for CharString {
    fn from(value: BravaisType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<BravaisType> for Value {
    fn from(value: BravaisType) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl BravaisType {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "aP" => Some(Self::AP),
            "mP" => Some(Self::MP),
            "mS" => Some(Self::MS),
            "oP" => Some(Self::OP),
            "oS" => Some(Self::OS),
            "oI" => Some(Self::OI),
            "oF" => Some(Self::OF),
            "tP" => Some(Self::TP),
            "tI" => Some(Self::TI),
            "hP" => Some(Self::HP),
            "hR" => Some(Self::HR),
            "cP" => Some(Self::CP),
            "cI" => Some(Self::CI),
            "cF" => Some(Self::CF),
            _ => None,
        }
    }
}
