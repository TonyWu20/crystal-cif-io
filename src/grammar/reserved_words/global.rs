use winnow::{ascii::Caseless, Parser};

use crate::grammar::SyntacticUnit;

#[derive(Debug, Clone, Copy)]
pub struct Global;

impl SyntacticUnit for Global {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        Caseless("global_").map(|_| Global).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        "global_".to_string()
    }
}
