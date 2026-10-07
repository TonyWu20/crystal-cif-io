use std::fmt::Display;

use crate::grammar::{CharString, Numeric, TextField};

#[derive(Debug, Clone, Default)]
pub enum Value {
    /// '.'
    Inapplicable,
    #[default]
    /// '?'
    Unknown,
    Numeric(Numeric),
    CharString(CharString),
    TextField(TextField),
}

impl Value {
    pub fn as_numeric(&self) -> Option<&Numeric> {
        if let Self::Numeric(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_char_string(&self) -> Option<&CharString> {
        if let Self::CharString(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_text_field(&self) -> Option<&TextField> {
        if let Self::TextField(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

pub trait CIFValue {}

impl CIFValue for Numeric {}
impl CIFValue for TextField {}

impl crate::grammar::SyntacticUnit for Value {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            Value::Inapplicable => ".".to_string(),
            Value::Unknown => "?".to_string(),
            Value::Numeric(numeric) => numeric.to_string(),
            Value::CharString(char_string) => char_string.to_string(),
            Value::TextField(text_field) => text_field.to_string(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Inapplicable => write!(f, "."),
            Value::Unknown => write!(f, "?"),
            Value::Numeric(n) => write!(f, "{n}"),
            Value::CharString(cs) => write!(f, "{cs}"),
            Value::TextField(tf) => write!(f, "{tf}"),
        }
    }
}
