use std::{
    error::Error,
    fmt::{Display, Write},
};

use winnow::{
    ascii::tab,
    combinator::alt,
    stream::{Accumulate, AsChar},
    Parser,
};

use crate::grammar::SyntacticUnit;

use super::ordinary_char::OrdinaryChar;

#[derive(Debug, Clone, Copy)]
pub enum AnyPrintChar {
    OrdinaryChar(OrdinaryChar),
    DoubleQuote,
    Num,
    Dollar,
    SingleQuote,
    Underline,
    SP,
    HT,
    Semicolon,
    BracketOpen,
    BracketClose,
}

impl From<AnyPrintChar> for char {
    fn from(value: AnyPrintChar) -> Self {
        match value {
            AnyPrintChar::OrdinaryChar(oc) => oc.into(),
            AnyPrintChar::DoubleQuote => '"',
            AnyPrintChar::Num => '#',
            AnyPrintChar::Dollar => '$',
            AnyPrintChar::SingleQuote => '\'',
            AnyPrintChar::Underline => '_',
            AnyPrintChar::SP => ' ',
            AnyPrintChar::HT => '\t',
            AnyPrintChar::Semicolon => ';',
            AnyPrintChar::BracketOpen => '[',
            AnyPrintChar::BracketClose => ']',
        }
    }
}

impl Display for AnyPrintChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyPrintChar::OrdinaryChar(c) => write!(f, "{c}"),
            AnyPrintChar::DoubleQuote => f.write_char('"'),
            AnyPrintChar::Num => f.write_char('#'),
            AnyPrintChar::Dollar => f.write_char('$'),
            AnyPrintChar::SingleQuote => f.write_char('\''),
            AnyPrintChar::Underline => f.write_char('_'),
            AnyPrintChar::SP => f.write_char(' '),
            AnyPrintChar::HT => f.write_char('\t'),
            AnyPrintChar::Semicolon => f.write_char(';'),
            AnyPrintChar::BracketOpen => f.write_char('['),
            AnyPrintChar::BracketClose => f.write_char(']'),
        }
    }
}

impl SyntacticUnit for AnyPrintChar {
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
                    ' ' => Ok(Self::SP),
                    '\t' => Ok(Self::HT),
                    ';' => Ok(Self::Semicolon),
                    '[' => Ok(Self::BracketOpen),
                    ']' => Ok(Self::BracketClose),
                    _ => Err(AnyPrintCharError),
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
pub struct AnyPrintCharError;

impl Display for AnyPrintCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Not included in CIF `<AnyPrintChar>`")
    }
}

impl Error for AnyPrintCharError {}

impl AsChar for AnyPrintChar {
    fn as_char(self) -> char {
        self.into()
    }

    fn is_alpha(self) -> bool {
        match self {
            AnyPrintChar::OrdinaryChar(oc) => oc.is_alpha(),
            _ => false,
        }
    }

    fn is_alphanum(self) -> bool {
        match self {
            AnyPrintChar::OrdinaryChar(oc) => oc.is_alphanum(),
            _ => false,
        }
    }

    fn is_dec_digit(self) -> bool {
        match self {
            AnyPrintChar::OrdinaryChar(oc) => oc.is_dec_digit(),
            _ => false,
        }
    }

    fn is_hex_digit(self) -> bool {
        match self {
            AnyPrintChar::OrdinaryChar(oc) => oc.is_hex_digit(),
            _ => false,
        }
    }

    fn is_oct_digit(self) -> bool {
        match self {
            AnyPrintChar::OrdinaryChar(oc) => oc.is_oct_digit(),
            _ => false,
        }
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        matches!(self, AnyPrintChar::SP)
    }

    fn is_newline(self) -> bool {
        false
    }
}

impl Accumulate<AnyPrintChar> for String {
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => String::with_capacity(capacity.clamp(0, 65536)),
            None => String::new(),
        }
    }

    fn accumulate(&mut self, acc: AnyPrintChar) {
        self.push(acc.as_char())
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::{character_sets::AnyPrintChar, SyntacticUnit};

    #[test]
    fn any_print_char() {
        let mut input = "'";
        dbg!(AnyPrintChar::parser(&mut input).unwrap());
    }
}
