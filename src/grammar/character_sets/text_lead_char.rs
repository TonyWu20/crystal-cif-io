use std::{
    error::Error,
    fmt::{Display, Write},
};
use winnow::{stream::Accumulate, Parser};

use winnow::{ascii::tab, combinator::alt, stream::AsChar};

use crate::grammar::SyntacticUnit;

use super::ordinary_char::OrdinaryChar;

#[derive(Debug, Clone, Copy)]
pub enum TextLeadChar {
    OrdinaryChar(OrdinaryChar),
    DoubleQuote,
    Num,
    Dollar,
    SingleQuote,
    Underline,
    SP,
    HT,
    BracketOpen,
    BracketClose,
}

impl From<TextLeadChar> for char {
    fn from(value: TextLeadChar) -> Self {
        match value {
            TextLeadChar::OrdinaryChar(oc) => oc.into(),
            TextLeadChar::DoubleQuote => '"',
            TextLeadChar::Num => '#',
            TextLeadChar::Dollar => '$',
            TextLeadChar::SingleQuote => '\'',
            TextLeadChar::Underline => '_',
            TextLeadChar::SP => ' ',
            TextLeadChar::HT => '\t',
            TextLeadChar::BracketOpen => '[',
            TextLeadChar::BracketClose => ']',
        }
    }
}

impl AsChar for TextLeadChar {
    fn as_char(self) -> char {
        char::from(self)
    }

    fn is_alpha(self) -> bool {
        match self {
            TextLeadChar::OrdinaryChar(oc) => oc.is_alpha(),
            TextLeadChar::DoubleQuote
            | TextLeadChar::Num
            | TextLeadChar::Dollar
            | TextLeadChar::SingleQuote
            | TextLeadChar::Underline
            | TextLeadChar::SP
            | TextLeadChar::HT
            | TextLeadChar::BracketOpen
            | TextLeadChar::BracketClose => false,
        }
    }

    fn is_alphanum(self) -> bool {
        match self {
            TextLeadChar::OrdinaryChar(oc) => oc.is_alphanum(),
            TextLeadChar::DoubleQuote
            | TextLeadChar::Num
            | TextLeadChar::Dollar
            | TextLeadChar::SingleQuote
            | TextLeadChar::Underline
            | TextLeadChar::SP
            | TextLeadChar::HT
            | TextLeadChar::BracketOpen
            | TextLeadChar::BracketClose => false,
        }
    }

    fn is_dec_digit(self) -> bool {
        match self {
            TextLeadChar::OrdinaryChar(oc) => oc.is_dec_digit(),
            TextLeadChar::DoubleQuote
            | TextLeadChar::Num
            | TextLeadChar::Dollar
            | TextLeadChar::SingleQuote
            | TextLeadChar::Underline
            | TextLeadChar::SP
            | TextLeadChar::HT
            | TextLeadChar::BracketOpen
            | TextLeadChar::BracketClose => false,
        }
    }

    fn is_hex_digit(self) -> bool {
        match self {
            TextLeadChar::OrdinaryChar(oc) => oc.is_hex_digit(),
            TextLeadChar::DoubleQuote
            | TextLeadChar::Num
            | TextLeadChar::Dollar
            | TextLeadChar::SingleQuote
            | TextLeadChar::Underline
            | TextLeadChar::SP
            | TextLeadChar::HT
            | TextLeadChar::BracketOpen
            | TextLeadChar::BracketClose => false,
        }
    }

    fn is_oct_digit(self) -> bool {
        match self {
            TextLeadChar::OrdinaryChar(oc) => oc.is_oct_digit(),
            TextLeadChar::DoubleQuote
            | TextLeadChar::Num
            | TextLeadChar::Dollar
            | TextLeadChar::SingleQuote
            | TextLeadChar::Underline
            | TextLeadChar::SP
            | TextLeadChar::HT
            | TextLeadChar::BracketOpen
            | TextLeadChar::BracketClose => false,
        }
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        matches!(self, TextLeadChar::SP)
    }

    fn is_newline(self) -> bool {
        false
    }
}

impl Display for TextLeadChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextLeadChar::OrdinaryChar(c) => write!(f, "{c}"),
            TextLeadChar::DoubleQuote => f.write_char('"'),
            TextLeadChar::Num => f.write_char('#'),
            TextLeadChar::Dollar => f.write_char('$'),
            TextLeadChar::SingleQuote => f.write_char('\''),
            TextLeadChar::Underline => f.write_char('_'),
            TextLeadChar::SP => f.write_char(' '),
            TextLeadChar::HT => f.write_char('\t'),
            TextLeadChar::BracketOpen => f.write_char('['),
            TextLeadChar::BracketClose => f.write_char(']'),
        }
    }
}

impl SyntacticUnit for TextLeadChar {
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
            ' ',
            tab,
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
                    ' ' => Ok(Self::SP),
                    '\t' => Ok(Self::HT),
                    '[' => Ok(Self::BracketOpen),
                    ']' => Ok(Self::BracketClose),
                    _ => Err(TextLeadCharError),
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

#[derive(Debug)]
pub struct TextLeadCharError;

impl Display for TextLeadCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Not included in CIF `<TextLeadChar>`")
    }
}

impl Error for TextLeadCharError {}

impl Accumulate<TextLeadChar> for String {
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => String::with_capacity(capacity.clamp(0, 65536)),
            None => String::new(),
        }
    }

    fn accumulate(&mut self, acc: TextLeadChar) {
        self.push(acc.as_char())
    }
}
