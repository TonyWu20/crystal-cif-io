use std::fmt::Display;

use winnow::{
    combinator::{alt, delimited, opt, peek, terminated},
    Parser,
};

use crate::grammar::SyntacticUnit;

pub use self::{float::Float, integer::Integer, unsigned_integer::UnsignedInteger};

use super::{whitespace_comments::WhiteSpace, CIFValue, Value};

mod exponent;
mod float;
mod integer;
mod unsigned_integer;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
    Integer(Integer),
    Float(Float),
}

impl Number {
    pub fn as_integer(&self) -> Option<&Integer> {
        if let Self::Integer(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<&Float> {
        if let Self::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl CIFValue for Number {}

impl SyntacticUnit for Number {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            Float::parser.map(Number::Float),
            Integer::parser.map(Number::Integer),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            Number::Integer(i) => format!("{i}"),
            Number::Float(f) => format!("{:<6.3}", &f),
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Integer(Integer(value))
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Numeric {
    number: Number,
    std_uncertainty: Option<UnsignedInteger>,
}

impl Numeric {
    pub fn new(number: Number, std_uncertainty: Option<UnsignedInteger>) -> Self {
        Self {
            number,
            std_uncertainty,
        }
    }

    pub fn number(&self) -> Number {
        self.number
    }

    pub fn std_uncertainty(&self) -> Option<UnsignedInteger> {
        self.std_uncertainty
    }
}

impl SyntacticUnit for Numeric {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        terminated(
            (
                Number::parser,
                opt(delimited('(', UnsignedInteger::parser, ')')),
            ),
            peek(WhiteSpace::parser),
        )
        .map(|(number, uncer)| Numeric::new(number, uncer))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self.std_uncertainty {
            Some(c) => format!("{}({})", self.number, c),
            None => format!("{}", self.number),
        }
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl From<f64> for Numeric {
    fn from(value: f64) -> Self {
        Numeric::new(Number::Float(Float(value as f32)), None)
    }
}

impl From<f32> for Numeric {
    fn from(value: f32) -> Self {
        Numeric::new(Number::Float(Float(value)), None)
    }
}

impl From<i32> for Numeric {
    fn from(value: i32) -> Self {
        Numeric::new(Number::from(value), None)
    }
}

impl From<Numeric> for Value {
    fn from(value: Numeric) -> Self {
        Value::Numeric(value)
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::SyntacticUnit;

    use super::Numeric;

    #[test]
    fn numeric_test() {
        let mut input = "482.66(9)\n";
        match Numeric::parser(&mut input) {
            Ok(n) => println!("{n:?}"),
            Err(e) => {
                println!("{e}");
                println!("{input}");
            }
        }
    }
}
