use std::fmt::Display;

use winnow::{combinator::alt, error::StrContext, Parser};

use crate::grammar::{
    structures::{data_items::DataItems, save_frame::SaveFrame},
    SyntacticUnit,
};

#[derive(Debug, Clone)]
pub enum DataBlockMember {
    DataItems(DataItems),
    SaveFrame(SaveFrame),
}

impl SyntacticUnit for DataBlockMember {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            DataItems::parser
                .map(Self::DataItems)
                .context(StrContext::Label("DataItems")),
            SaveFrame::parser
                .map(Self::SaveFrame)
                .context(StrContext::Label("SaveFrame")),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            DataBlockMember::DataItems(d) => d.to_string(),
            DataBlockMember::SaveFrame(s) => s.to_string(),
        }
    }
}

impl Display for DataBlockMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
