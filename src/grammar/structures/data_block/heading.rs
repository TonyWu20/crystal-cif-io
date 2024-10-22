use std::fmt::Display;

use winnow::{
    combinator::{preceded, repeat},
    Parser,
};

use crate::grammar::{character_sets::NonBlankChar, reserved_words::Data, SyntacticUnit};

#[derive(Debug, Clone)]
pub struct DataBlockHeading {
    name: String,
}

impl AsRef<str> for DataBlockHeading {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.name)
    }
}

impl DataBlockHeading {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl SyntacticUnit for DataBlockHeading {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded(Data::parser, repeat(1.., NonBlankChar::parser))
            .map(DataBlockHeading::new)
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("\ndata_{}\n", &self.name)
    }
}

impl Display for DataBlockHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
