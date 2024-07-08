use std::fmt::Display;

use winnow::{combinator::alt, error::StrContext, Parser};

use crate::grammar::SyntacticUnit;

use super::{loop_struct::LoopUnit, tag_value_line::SingleLineData};

#[derive(Debug, Clone)]
pub enum DataItems {
    SingleValue(SingleLineData),
    MultiValues(LoopUnit),
}

impl SyntacticUnit for DataItems {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            SingleLineData::parser
                .map(DataItems::SingleValue)
                .context(StrContext::Label("Single line data")),
            LoopUnit::parser
                .map(DataItems::MultiValues)
                .context(StrContext::Label("Loop")),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            DataItems::SingleValue(v) => format!("{v}"),
            DataItems::MultiValues(v) => format!("\n{v}\n"),
        }
    }
}

impl Display for DataItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
