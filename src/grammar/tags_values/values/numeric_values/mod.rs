use std::fmt::Display;

use winnow::{
    combinator::{alt, delimited, opt},
    Parser,
};

use crate::grammar::SyntacticUnit;

use self::{float::Float, integer::Integer, unsigned_integer::UnsignedInteger};

mod exponent;
mod float;
mod integer;
mod unsigned_integer;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
    Integer(Integer),
    Float(Float),
}

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
            Number::Float(f) => format!("{f}"),
        }
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
}

impl SyntacticUnit for Numeric {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (
            Number::parser,
            opt(delimited('(', UnsignedInteger::parser, ')')),
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
