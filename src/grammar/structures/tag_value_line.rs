use std::fmt::Display;

use winnow::{
    combinator::{peek, separated_pair},
    error::StrContext,
    Parser,
};

use crate::grammar::{
    tags_values::{Tag, Value},
    whitespace_comments::WhiteSpace,
    SyntacticUnit,
};

#[derive(Debug, Clone)]
pub struct SingleLineData {
    tag: Tag,
    value: Value,
}

impl SingleLineData {
    pub fn from_tag_value(tag_value: (Tag, Value)) -> Self {
        let (tag, value) = tag_value;
        Self { tag, value }
    }
}

impl SyntacticUnit for SingleLineData {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        separated_pair(
            Tag::parser.context(StrContext::Label("Tag")),
            peek(WhiteSpace::parser.context(StrContext::Label("Whitespace"))),
            Value::parser.context(StrContext::Label("Value")),
        )
        .map(SingleLineData::from_tag_value)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("{} {}", self.tag, self.value)
    }
}

impl Display for SingleLineData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
