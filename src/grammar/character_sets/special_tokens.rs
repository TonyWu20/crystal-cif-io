use std::fmt::Display;

use winnow::{
    ascii::{line_ending, tab},
    combinator::{alt, peek},
    error::{AddContext, ContextError, ErrMode, StrContext},
    stream::{Accumulate, AsChar, Stream},
    token::any,
    Parser,
};

use crate::grammar::SyntacticUnit;

#[derive(Debug, Copy, Clone)]
pub struct SP;
#[derive(Debug, Copy, Clone)]
pub struct HT;
#[derive(Debug, Copy, Clone)]
pub struct Eol;
#[derive(Debug, Copy, Clone)]
pub struct NotEol(char);
#[derive(Debug, Copy, Clone)]
pub struct SingleQuote;
#[derive(Debug, Copy, Clone)]
pub struct DoubleQuote;

#[derive(Debug, Copy, Clone)]
pub enum LeadingBlank {
    SP,
    HT,
    Eol,
}

impl SyntacticUnit for LeadingBlank {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            SP::parser.map(|_| LeadingBlank::SP),
            HT::parser.map(|_| LeadingBlank::HT),
            line_ending.map(|_| LeadingBlank::Eol),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            LeadingBlank::SP => ' ',
            LeadingBlank::HT => '\t',
            LeadingBlank::Eol => '\n',
        }
    }
}

impl Display for LeadingBlank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl AsChar for LeadingBlank {
    fn as_char(self) -> char {
        self.formatted_output()
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        matches!(self, LeadingBlank::SP)
    }

    fn is_newline(self) -> bool {
        matches!(self, LeadingBlank::Eol)
    }
}

impl SyntacticUnit for SP {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        ' '.map(|_| SP).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        ' '
    }
}

impl AsChar for SP {
    fn as_char(self) -> char {
        self.formatted_output()
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        true
    }

    fn is_newline(self) -> bool {
        false
    }
}

impl SyntacticUnit for HT {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        tab.map(|_| HT).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        '\t'
    }
}

impl AsChar for HT {
    fn as_char(self) -> char {
        self.formatted_output()
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        false
    }

    fn is_newline(self) -> bool {
        false
    }
}

impl SyntacticUnit for Eol {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        line_ending.map(|_| Eol).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        '\n'
    }
}

impl AsChar for Eol {
    fn as_char(self) -> char {
        self.formatted_output()
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        false
    }

    fn is_newline(self) -> bool {
        true
    }
}

impl SyntacticUnit for SingleQuote {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        '\''.map(|_| SingleQuote).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        '\''
    }
}

impl AsChar for SingleQuote {
    fn as_char(self) -> char {
        self.formatted_output()
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        false
    }

    fn is_newline(self) -> bool {
        false
    }
}

impl SyntacticUnit for DoubleQuote {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        '"'.map(|_| DoubleQuote).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        '"'
    }
}

impl AsChar for DoubleQuote {
    fn as_char(self) -> char {
        self.formatted_output()
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        false
    }

    fn is_newline(self) -> bool {
        false
    }
}

impl NotEol {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

impl SyntacticUnit for NotEol {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        if peek(Eol::parser).parse_peek(input).is_err() {
            any.map(NotEol::new).parse_next(input)
        } else {
            Err(ErrMode::Backtrack(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Label("Not Eol"),
            )))
        }
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.0
    }
}

impl Accumulate<LeadingBlank> for String {
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => String::with_capacity(capacity.clamp(0, 65536)),
            None => String::new(),
        }
    }

    fn accumulate(&mut self, acc: LeadingBlank) {
        self.push(acc.as_char())
    }
}
