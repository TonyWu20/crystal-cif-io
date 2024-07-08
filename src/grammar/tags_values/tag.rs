use std::fmt::Display;

use winnow::{
    combinator::{preceded, repeat},
    Parser,
};

use crate::grammar::{character_sets::NonBlankChar, SyntacticUnit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    name: String,
}

impl AsRef<str> for Tag {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.name)
    }
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl SyntacticUnit for Tag {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded('_', repeat(1.., NonBlankChar::parser))
            .map(Tag::new)
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("_{}", self.name)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
