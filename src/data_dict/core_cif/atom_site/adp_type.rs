use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

#[derive(Debug, Clone, Copy)]
pub enum AdpType {
    Uani,
    Uiso,
    Uovl,
    Umpe,
    Bani,
    Biso,
    Bovl,
}

impl Display for AdpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = format!("{:?}", self);
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
