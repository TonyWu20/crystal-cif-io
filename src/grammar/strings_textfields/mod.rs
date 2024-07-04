use std::fmt::Display;

use winnow::{combinator::alt, Parser};

use self::{
    double_quoted_string::DoubleQuotedString, single_quoted_string::SingleQuotedString,
    unquoted_string::UnquotedString,
};

use super::SyntacticUnit;

mod double_quoted_string;
mod single_quoted_string;
mod text_field;
mod unquoted_string;

#[derive(Debug, Clone)]
pub enum CharString {
    Unquoted(UnquotedString),
    SingleQuoted(SingleQuotedString),
    DoubleQuoted(DoubleQuotedString),
}

impl SyntacticUnit for CharString {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            UnquotedString::parser.map(CharString::Unquoted),
            SingleQuotedString::parser.map(CharString::SingleQuoted),
            DoubleQuotedString::parser.map(CharString::DoubleQuoted),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            CharString::Unquoted(s) => format!("{s}"),
            CharString::SingleQuoted(s) => format!("{s}"),
            CharString::DoubleQuoted(s) => format!("{s}"),
        }
    }
}

impl Display for CharString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
