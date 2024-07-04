use std::{
    error::Error,
    fmt::{Display, Write},
};

use winnow::{
    combinator::alt,
    stream::{Accumulate, AsChar},
    Parser,
};

use crate::grammar::SyntacticUnit;

use super::ordinary_char::OrdinaryChar;

#[derive(Debug, Clone, Copy)]
pub enum NonBlankChar {
    OrdinaryChar(OrdinaryChar),
    DoubleQuote,
    Num,
    Dollar,
    SingleQuote,
    Underline,
    Semicolon,
    BracketOpen,
    BracketClose,
}

impl AsChar for NonBlankChar {
    fn as_char(self) -> char {
        char::from(self)
    }

    fn is_alpha(self) -> bool {
        match self {
            NonBlankChar::OrdinaryChar(oc) => oc.is_alpha(),
            NonBlankChar::DoubleQuote
            | NonBlankChar::Num
            | NonBlankChar::Dollar
            | NonBlankChar::SingleQuote
            | NonBlankChar::Underline
            | NonBlankChar::Semicolon
            | NonBlankChar::BracketOpen
            | NonBlankChar::BracketClose => false,
        }
    }

    fn is_alphanum(self) -> bool {
        match self {
            NonBlankChar::OrdinaryChar(oc) => oc.is_alphanum(),
            NonBlankChar::DoubleQuote
            | NonBlankChar::Num
            | NonBlankChar::Dollar
            | NonBlankChar::SingleQuote
            | NonBlankChar::Underline
            | NonBlankChar::Semicolon
            | NonBlankChar::BracketOpen
            | NonBlankChar::BracketClose => false,
        }
    }

    fn is_dec_digit(self) -> bool {
        match self {
            NonBlankChar::OrdinaryChar(oc) => oc.is_dec_digit(),
            NonBlankChar::DoubleQuote => false,
            NonBlankChar::Num => false,
            NonBlankChar::Dollar => false,
            NonBlankChar::SingleQuote => false,
            NonBlankChar::Underline => false,
            NonBlankChar::Semicolon => false,
            NonBlankChar::BracketOpen => false,
            NonBlankChar::BracketClose => false,
        }
    }

    fn is_hex_digit(self) -> bool {
        match self {
            NonBlankChar::OrdinaryChar(oc) => oc.is_hex_digit(),
            NonBlankChar::DoubleQuote => false,
            NonBlankChar::Num => false,
            NonBlankChar::Dollar => false,
            NonBlankChar::SingleQuote => false,
            NonBlankChar::Underline => false,
            NonBlankChar::Semicolon => false,
            NonBlankChar::BracketOpen => false,
            NonBlankChar::BracketClose => false,
        }
    }

    fn is_oct_digit(self) -> bool {
        match self {
            NonBlankChar::OrdinaryChar(oc) => oc.is_oct_digit(),
            NonBlankChar::DoubleQuote => false,
            NonBlankChar::Num => false,
            NonBlankChar::Dollar => false,
            NonBlankChar::SingleQuote => false,
            NonBlankChar::Underline => false,
            NonBlankChar::Semicolon => false,
            NonBlankChar::BracketOpen => false,
            NonBlankChar::BracketClose => false,
        }
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

impl Display for NonBlankChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonBlankChar::OrdinaryChar(c) => write!(f, "{c}"),
            NonBlankChar::DoubleQuote => f.write_char('"'),
            NonBlankChar::Num => f.write_char('#'),
            NonBlankChar::Dollar => f.write_char('$'),
            NonBlankChar::SingleQuote => f.write_char('\''),
            NonBlankChar::Underline => f.write_char('_'),
            NonBlankChar::Semicolon => f.write_char(';'),
            NonBlankChar::BracketOpen => f.write_char('['),
            NonBlankChar::BracketClose => f.write_char(']'),
        }
    }
}

impl SyntacticUnit for NonBlankChar {
    type ParseResult = Self;

    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            OrdinaryChar::parser.map(char::from),
            '"',
            '#',
            '$',
            '\'',
            '_',
            ';',
            '[',
            ']',
        ))
        .map(|c| {
            if let Ok(oc) = OrdinaryChar::try_from(c) {
                Ok(Self::OrdinaryChar(oc))
            } else {
                match c {
                    '"' => Ok(Self::DoubleQuote),
                    '#' => Ok(Self::Num),
                    '$' => Ok(Self::Dollar),
                    '\'' => Ok(Self::SingleQuote),
                    '_' => Ok(Self::Underline),
                    ';' => Ok(Self::Semicolon),
                    '[' => Ok(Self::BracketOpen),
                    ']' => Ok(Self::BracketClose),
                    _ => Err(NonBlankCharError),
                }
            }
        })
        .map(|res| res.unwrap())
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        char::from(*self)
    }
}

impl From<NonBlankChar> for char {
    fn from(value: NonBlankChar) -> Self {
        match value {
            NonBlankChar::OrdinaryChar(oc) => oc.into(),
            NonBlankChar::DoubleQuote => '"',
            NonBlankChar::Num => '#',
            NonBlankChar::Dollar => '$',
            NonBlankChar::SingleQuote => '\'',
            NonBlankChar::Underline => '_',
            NonBlankChar::Semicolon => ';',
            NonBlankChar::BracketOpen => '[',
            NonBlankChar::BracketClose => ']',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NonBlankCharError;

impl Display for NonBlankCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Not included in CIF `<NonBlankChar>`")
    }
}

impl Error for NonBlankCharError {}

impl Accumulate<NonBlankChar> for String {
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => String::with_capacity(capacity.clamp(0, 65536)),
            None => String::new(),
        }
    }

    fn accumulate(&mut self, acc: NonBlankChar) {
        self.push(acc.as_char())
    }
}
