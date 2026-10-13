use std::fmt::Display;

use elements_rs::Element;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

#[derive(Debug, Clone)]
pub enum TypeSymbolCode {
    Element(Element),
    Custom(String),
}

impl Display for TypeSymbolCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSymbolCode::Element(element) => write!(f, "{element}"),
            TypeSymbolCode::Custom(code) => write!(f, "{}", code),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeSymbol {
    code: TypeSymbolCode,
    oxidation_state: Option<i32>,
}

impl TypeSymbol {
    pub fn new(code: TypeSymbolCode, oxidation_state: Option<i32>) -> Self {
        Self {
            code,
            oxidation_state,
        }
    }
}

impl Display for TypeSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let oxidation_state = if let Some(i) = self.oxidation_state {
            if i > 0 {
                format!("{i}+")
            } else {
                format!("{i}-")
            }
        } else {
            String::new()
        };
        write!(f, "{}{}", self.code, oxidation_state)
    }
}

impl From<TypeSymbol> for UnquotedString {
    fn from(value: TypeSymbol) -> Self {
        UnquotedString::new(value.to_string())
    }
}

impl From<TypeSymbol> for CharString {
    fn from(value: TypeSymbol) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<TypeSymbol> for Value {
    fn from(value: TypeSymbol) -> Self {
        Value::CharString(CharString::from(value))
    }
}
