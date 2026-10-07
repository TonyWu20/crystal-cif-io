use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A single-quoted character string. The content is stored between
/// the delimiters, exactly as written (embedded quotes are kept
/// verbatim, spec para. 15, 16).
#[derive(Debug, Clone)]
pub struct SingleQuotedString {
    content: String,
}

impl AsRef<str> for SingleQuotedString {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.content)
    }
}

impl Display for SingleQuotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl SingleQuotedString {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for SingleQuotedString {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("'{}'", self.content)
    }
}

impl From<SingleQuotedString> for super::CharString {
    fn from(value: SingleQuotedString) -> Self {
        Self::SingleQuoted(value)
    }
}
