use winnow::{ascii::Caseless, Parser};

use crate::grammar::SyntacticUnit;

#[derive(Debug, Clone, Copy)]
pub struct Save;

impl SyntacticUnit for Save {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        Caseless("save_").map(|_| Save).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        "save_".to_string()
    }
}
