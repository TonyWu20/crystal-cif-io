use std::fmt::Display;

use winnow::{
    combinator::{preceded, repeat},
    Parser,
};

use crate::grammar::{
    reserved_words::Loop, tags_values::Tag, whitespace_comments::WhiteSpace, SyntacticUnit,
};

#[derive(Debug, Clone)]
pub struct LoopHeader {
    tags: Vec<Tag>,
}

impl LoopHeader {
    pub fn new(tags: Vec<Tag>) -> Self {
        Self { tags }
    }
    pub fn num_of_tags(&self) -> usize {
        self.tags.len()
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }
}

impl SyntacticUnit for LoopHeader {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded(
            Loop::parser,
            repeat(1.., preceded(WhiteSpace::parser, Tag::parser)),
        )
        .map(LoopHeader::new)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        let tags = self
            .tags
            .iter()
            .map(|t| format!("{t}"))
            .collect::<Vec<String>>()
            .join("\n");
        format!("loop_\n{tags}")
    }
}

impl Display for LoopHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.formatted_output())
    }
}
