use std::fmt::Display;

use winnow::{ascii::digit1, error::StrContext, Parser};

use crate::grammar::SyntacticUnit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsignedInteger(u32);

impl std::ops::Deref for UnsignedInteger {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SyntacticUnit for UnsignedInteger {
    type ParseResult = Self;

    type FormatOutput = u32;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        digit1
            .map(|n: &str| {
                n.parse::<u32>()
                    .map(UnsignedInteger)
                    .expect("Failed to parse as u32")
            })
            .context(StrContext::Label("<UnsignedInteger> as u32"))
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.0
    }
}

impl Display for UnsignedInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
