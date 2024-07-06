use std::fmt::Display;

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

impl Display for Save {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
