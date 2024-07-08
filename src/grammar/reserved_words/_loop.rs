use std::fmt::Display;

use winnow::{ascii::Caseless, Parser};

use crate::grammar::SyntacticUnit;

#[derive(Debug, Clone, Copy)]
pub struct Loop;
impl SyntacticUnit for Loop {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        Caseless("loop_").map(|_| Loop).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        "loop_".to_string()
    }
}

impl Display for Loop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
