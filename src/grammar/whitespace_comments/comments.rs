use std::fmt::Display;

use winnow::{
    ascii::line_ending,
    combinator::{delimited, repeat},
    stream::Accumulate,
    Parser,
};

use crate::grammar::{
    character_sets::{AnyPrintChar, LeadingBlank},
    SyntacticUnit,
};

#[derive(Debug, Clone)]
pub struct Comments {
    content: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TokenizedComments {
    leading: String,
    comments: Comments,
}

impl TokenizedComments {
    pub fn new(leading: String, comments: Comments) -> Self {
        Self { leading, comments }
    }
}

impl Comments {
    pub fn new(content: Vec<String>) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for Comments {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        repeat(
            1..,
            delimited(
                '#',
                repeat::<_, _, String, _, _>(0.., AnyPrintChar::parser),
                line_ending,
            ),
        )
        .map(Comments::new)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.content
            .iter()
            .map(|line| format!("#{line}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for Comments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.formatted_output())
    }
}

impl SyntacticUnit for TokenizedComments {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (repeat(1.., LeadingBlank::parser), Comments::parser)
            .map(|(blanks, comments)| TokenizedComments::new(blanks, comments))
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("{}{}", self.leading, self.comments)
    }
}

impl Display for TokenizedComments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl Accumulate<TokenizedComments> for String {
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => String::with_capacity(capacity.clamp(0, 65536)),
            None => String::new(),
        }
    }

    fn accumulate(&mut self, acc: TokenizedComments) {
        *self = [self.clone(), acc.formatted_output()].concat()
    }
}
