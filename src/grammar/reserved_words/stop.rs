use std::fmt::Display;

use winnow::{ascii::Caseless, Parser};

use crate::grammar::SyntacticUnit;

#[derive(Debug, Clone, Copy)]
pub struct Stop;

impl SyntacticUnit for Stop {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        Caseless("stop_").map(|_| Stop).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        "Stop_".to_string()
    }
}

impl Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
