use std::fmt::Display;

use winnow::{
    combinator::{alt, opt, preceded},
    Parser,
};

use self::unquoted_string::pure_unquoted;

use super::{reserved_words::ReservedWords, whitespace_comments::WhiteSpace, SyntacticUnit, Value};

mod double_quoted_string;
mod single_quoted_string;
mod text_field;
mod unquoted_string;

pub use text_field::TextField;

pub use double_quoted_string::DoubleQuotedString;
pub use single_quoted_string::SingleQuotedString;
pub use unquoted_string::UnquotedString;

#[derive(Debug, Clone)]
pub enum CharString {
    Unquoted(UnquotedString),
    SingleQuoted(SingleQuotedString),
    DoubleQuoted(DoubleQuotedString),
}

impl AsRef<str> for CharString {
    fn as_ref(&self) -> &str {
        match self {
            CharString::Unquoted(u) => u.as_ref(),
            CharString::SingleQuoted(s) => s.as_ref(),
            CharString::DoubleQuoted(d) => d.as_ref(),
        }
    }
}

impl SyntacticUnit for CharString {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            preceded(opt(WhiteSpace::parser), SingleQuotedString::parser)
                .map(CharString::SingleQuoted),
            preceded(opt(WhiteSpace::parser), DoubleQuotedString::parser)
                .map(CharString::DoubleQuoted),
            alt((
                preceded(opt(WhiteSpace::parser), pure_unquoted),
                UnquotedString::parser,
            ))
            .verify(|u| {
                let mut input = u.as_ref();
                ReservedWords::not_reserved_words(&mut input)
            })
            .map(CharString::Unquoted),
        ))
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            CharString::Unquoted(s) => format!("{s}"),
            CharString::SingleQuoted(s) => format!("{s}"),
            CharString::DoubleQuoted(s) => format!("{s}"),
        }
    }
}

impl Display for CharString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl From<CharString> for Value {
    fn from(value: CharString) -> Self {
        Value::CharString(value)
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::{strings_textfields::CharString, SyntacticUnit};

    #[test]
    fn char_string() {
        let mut input = "'C16 H38 N4 2+, C4 H4 O5 2-, 2C H4 O'
";
        let mut input_2 = "    rm
";
        let mut input_3 = "_symmetry_cell_setting
";
        dbg!(CharString::parser(&mut input).unwrap());
        dbg!(CharString::parser(&mut input_2).unwrap());
        dbg!(CharString::parser(&mut input_3).unwrap());
    }
}
