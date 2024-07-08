#![allow(dead_code, unused_imports)]
mod _loop;
mod data;
mod global;
mod save;
mod stop;

pub use _loop::Loop;
pub use data::Data;
pub use global::Global;
pub use save::Save;
pub use stop::Stop;
use winnow::{
    combinator::{alt, peek},
    Parser,
};

use super::SyntacticUnit;

#[derive(Debug, Clone)]
pub enum ReservedWords {
    Loop,
    Data,
    Global,
    Save,
    Stop,
}

impl SyntacticUnit for ReservedWords {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            Loop::parser.map(|_| Self::Loop),
            Data::parser.map(|_| Self::Data),
            Global::parser.map(|_| Self::Global),
            Save::parser.map(|_| Self::Save),
            Stop::parser.map(|_| Self::Stop),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        todo!()
    }
}

impl ReservedWords {
    pub fn not_reserved_words(input: &mut &str) -> bool {
        peek(ReservedWords::parser).parse_peek(input).is_err()
    }
}
