use winnow::{ascii::Caseless, Parser};

use crate::grammar::SyntacticUnit;

#[derive(Debug, Clone, Copy)]
pub struct Data;

impl SyntacticUnit for Data {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        Caseless("data_").map(|_| Data).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        "data_".to_string()
    }
}
