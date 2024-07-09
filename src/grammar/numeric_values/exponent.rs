use std::{fmt::Display, ops::Deref};

use winnow::{ascii::Caseless, combinator::alt, Parser};

use crate::grammar::SyntacticUnit;

use super::unsigned_integer::UnsignedInteger;

#[derive(Debug, Clone, Copy)]
pub struct Exponent(pub i32);

impl std::ops::Deref for Exponent {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SyntacticUnit for Exponent {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (
            alt((Caseless("e+"), Caseless("e-"), Caseless("e"))),
            UnsignedInteger::parser,
        )
            .map(|(base, power)| {
                if base.contains('-') {
                    let power = -(*power as i32);
                    Exponent(power)
                } else {
                    Exponent(*power as i32)
                }
            })
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("e{}", self.deref())
    }
}

impl Display for Exponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::{numeric_values::exponent::Exponent, SyntacticUnit};

    #[test]
    fn exponent_test() {
        let mut input = "e-3";
        dbg!(Exponent::parser(&mut input).unwrap());
    }
}
