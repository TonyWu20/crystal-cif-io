use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;

use crate::grammar::{CharString, SyntacticUnit, UnquotedString, Value};

#[derive(Debug, Clone)]
pub enum TypeSymbolCode {
    ElementSymbol(ElementSymbol),
    Custom(String),
}

impl Display for TypeSymbolCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSymbolCode::ElementSymbol(symbol) => write!(f, "{symbol}"),
            TypeSymbolCode::Custom(code) => write!(f, "{}", code),
        }
    }
}

impl SyntacticUnit for TypeSymbol {
    type ParseResult;

    type FormatOutput;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        todo!()
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        todo!()
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
