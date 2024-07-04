use std::fmt::Display;

use winnow::PResult;

pub trait SyntacticUnit {
    type ParseResult;
    type FormatOutput: Display;
    fn parser(input: &mut &str) -> PResult<Self::ParseResult>;
    fn formatted_output(&self) -> Self::FormatOutput;
}

mod character_sets;
mod numeric_values;
mod reserved_words;
mod strings_textfields;
mod structures;
mod tags_values;
mod whitespace_comments;
