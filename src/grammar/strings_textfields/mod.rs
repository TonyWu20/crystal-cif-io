use std::fmt::Display;

use crate::grammar::SyntacticUnit;

use super::CIFValue;

mod double_quoted_string;
mod single_quoted_string;
mod text_field;
mod unquoted_string;

pub use double_quoted_string::DoubleQuotedString;
pub use single_quoted_string::SingleQuotedString;
pub use text_field::{SemiColonTextField, TextField};
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

impl CharString {
    /// The character string content, as written between the
    /// delimiters.
    pub fn content(&self) -> &str {
        self.as_ref()
    }
}

impl super::SyntacticUnit for CharString {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            CharString::Unquoted(s) => format!("{s}"),
            CharString::SingleQuoted(s) => format!("{s}"),
            CharString::DoubleQuoted(d) => format!("{d}"),
        }
    }
}

impl Display for CharString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl From<CharString> for crate::Value {
    fn from(value: CharString) -> Self {
        crate::Value::CharString(value)
    }
}

impl CIFValue for CharString {}
