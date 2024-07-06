use std::fmt::Display;

use winnow::{
    combinator::{preceded, repeat, terminated},
    Parser,
};

use crate::grammar::{
    character_sets::NonBlankChar, reserved_words::Save, whitespace_comments::WhiteSpace,
    SyntacticUnit,
};

use super::data_items::DataItems;

#[derive(Debug, Clone)]
pub struct SaveFrameHeading {
    heading: String,
}

impl SaveFrameHeading {
    pub fn new(heading: String) -> Self {
        Self { heading }
    }
}

impl SyntacticUnit for SaveFrameHeading {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded(Save::parser, repeat(1.., NonBlankChar::parser))
            .map(SaveFrameHeading::new)
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("save_{}", self.heading)
    }
}

impl Display for SaveFrameHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[derive(Debug, Clone)]
pub struct SaveFrame {
    heading: SaveFrameHeading,
    data_items: Vec<DataItems>,
}

impl SaveFrame {
    // pub fn new(heading: SaveFrameHeading, data_items: Vec<DataItems>) -> Self {
    //     Self {
    //         heading,
    //         data_items,
    //     }
    // }
    pub fn from_heading_items(input: (SaveFrameHeading, Vec<DataItems>)) -> Self {
        let (heading, data_items) = input;
        Self {
            heading,
            data_items,
        }
    }
}

impl SyntacticUnit for SaveFrame {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        terminated(
            (
                SaveFrameHeading::parser,
                repeat(1.., preceded(WhiteSpace::parser, DataItems::parser)),
            ),
            (WhiteSpace::parser, Save::parser),
        )
        .map(SaveFrame::from_heading_items)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!(
            "{}\n{}",
            self.heading,
            self.data_items
                .iter()
                .map(|i| format!("{i}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for SaveFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
