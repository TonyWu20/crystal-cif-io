use std::fmt::{Display, Write};

use winnow::{
    combinator::{alt, opt, peek, preceded, terminated},
    PResult, Parser,
};

use crate::grammar::{
    character_sets::Eol,
    numeric_values::Numeric,
    strings_textfields::{CharString, TextField},
    whitespace_comments::WhiteSpace,
    SyntacticUnit,
};

#[derive(Debug, Clone, Default)]
pub enum Value {
    /// '.'
    Inapplicable,
    #[default]
    /// '?'
    Unknown,
    Numeric(Numeric),
    CharString(CharString),
    TextField(TextField),
}

impl Value {
    pub fn as_numeric(&self) -> Option<&Numeric> {
        if let Self::Numeric(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_char_string(&self) -> Option<&CharString> {
        if let Self::CharString(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_text_field(&self) -> Option<&TextField> {
        if let Self::TextField(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

pub trait CIFValue {}

impl CIFValue for Numeric {}
impl CIFValue for CharString {}
impl CIFValue for TextField {}

/// Parse value '?' when the it does not have trailing chars.
fn unknown_parser(input: &mut &str) -> PResult<Value> {
    terminated('?', peek(Eol::parser))
        .map(|_| Value::Unknown)
        .parse_next(input)
}

/// Parse value '.' when the it does not have trailing chars.
fn inapplicable_parser(input: &mut &str) -> PResult<Value> {
    terminated('.', peek(Eol::parser))
        .map(|_| Value::Inapplicable)
        .parse_next(input)
}

impl SyntacticUnit for Value {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            preceded(opt(WhiteSpace::parser), unknown_parser),
            preceded(opt(WhiteSpace::parser), inapplicable_parser),
            preceded(opt(WhiteSpace::parser), Numeric::parser).map(Self::Numeric),
            TextField::parser.map(Self::TextField),
            CharString::parser.map(Self::CharString),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            Value::Inapplicable => ".".to_string(),
            Value::Unknown => "?".to_string(),
            Value::Numeric(numeric) => numeric.to_string(),
            Value::CharString(char_string) => char_string.to_string(),
            Value::TextField(text_field) => text_field.to_string(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Inapplicable => f.write_char('.'),
            Value::Unknown => f.write_char('?'),
            Value::Numeric(n) => write!(f, "{n}"),
            Value::CharString(cs) => write!(f, "{cs}"),
            Value::TextField(tf) => write!(f, "{tf}"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::SyntacticUnit;

    use super::Value;

    #[test]
    fn value_test() {
        let mut inputs = [
            " ;
PROBLEM: _A ADDSYM Suggests Possible Pseudo/New Spacegroup       P-1          
RESPONSE: .See above
;
",
            " 'C16 H38 N4 2+, C4 H4 O5 2-, 2C H4 O'
",
            "   482.66
",
            " rm
",
            " ?ii
",
            "   ?
",
            "?
",
            " .
",
            "     .
",
            "'P 1'
",
            " 662.31(12) ",
            "8456
",
            "  MoK\\a
",
            "MoK\\a
",
            "1_445",
        ];
        inputs.iter_mut().map(Value::parser).for_each(|res| {
            res.map(|v| println!("{v:?}")).unwrap();
        })
    }
}
