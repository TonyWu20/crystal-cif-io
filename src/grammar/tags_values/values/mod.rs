use std::fmt::{Display, Write};

use winnow::{combinator::alt, Parser};

use crate::grammar::{
    strings_textfields::{CharString, TextField},
    SyntacticUnit,
};

use self::numeric_values::Numeric;

mod numeric_values;

#[derive(Debug, Clone)]
pub enum Value {
    /// '.'
    Inapplicable,
    /// '?'
    Unknown,
    Numeric(Numeric),
    CharString(CharString),
    TextField(TextField),
}

impl SyntacticUnit for Value {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            Numeric::parser.map(Self::Numeric),
            CharString::parser.map(Self::CharString),
            TextField::parser.map(Self::TextField),
            '.'.map(|_| Self::Inapplicable),
            '?'.map(|_| Self::Unknown),
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
            "'C16 H38 N4 2+, C4 H4 O5 2-, 2C H4 O'
",
            "482.66
",
            " rm
",
            "?
",
            ".
",
            "'P 1'
",
            "662.31(12)
",
            "8456
",
        ];
        inputs.iter_mut().map(Value::parser).for_each(|res| {
            res.map(|v| println!("{v}")).unwrap();
        })
    }
}
