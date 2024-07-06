use std::fmt::Display;

use winnow::{combinator::alt, Parser};

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
            LoopUnit::parser.map(DataItems::MultiValues),
            SingleLineData::parser.map(DataItems::SingleValue),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            DataItems::SingleValue(v) => format!("{v}"),
            DataItems::MultiValues(v) => format!("{v}"),
        }
    }
}

impl Display for DataItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
