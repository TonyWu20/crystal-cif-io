use std::fmt::Display;

use winnow::{
    combinator::{alt, repeat},
    Parser,
};

use crate::grammar::SyntacticUnit;

use super::unsigned_integer::UnsignedInteger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer(pub i32);

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        <i32 as Display>::fmt(&self.0, f)
    }
}

impl std::ops::Deref for Integer {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SyntacticUnit for Integer {
    type ParseResult = Self;

    type FormatOutput = i32;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (repeat(0..=1, alt(('+', '-'))), UnsignedInteger::parser)
            .map(|(sign, u): (String, UnsignedInteger)| {
                if sign.contains('-') {
                    Integer(-(*u as i32))
                } else {
                    Integer(*u as i32)
                }
            })
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.0
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer(value as i32)
    }
}
