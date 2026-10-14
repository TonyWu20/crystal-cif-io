use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

#[derive(Debug, Clone, Copy)]
pub enum AdpType {
    Uani,
    Uiso,
    Uovl,
    Umpe,
    Bani,
    Biso,
    Bovl,
    Betaani,
}

impl Display for AdpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::Betaani => "betaani".to_string(),
            other => format!("{other:?}"),
        };
        write!(f, "{}", content)
    }
}

impl From<AdpType> for UnquotedString {
    fn from(value: AdpType) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<AdpType> for CharString {
    fn from(value: AdpType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<AdpType> for Value {
    fn from(value: AdpType) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl AdpType {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "Uani" => Some(Self::Uani),
            "Uiso" => Some(Self::Uiso),
            "Uovl" => Some(Self::Uovl),
            "Umpe" => Some(Self::Umpe),
            "Bani" => Some(Self::Bani),
            "Biso" => Some(Self::Biso),
            "Bovl" => Some(Self::Bovl),
            "betaani" => Some(Self::Betaani),
            _ => None,
        }
    }
}
