use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A double-quoted character string. The content is stored between
/// the delimiters, exactly as written.
#[derive(Debug, Clone)]
pub struct DoubleQuotedString {
    content: String,
}

impl AsRef<str> for DoubleQuotedString {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.content)
    }
}

impl Display for DoubleQuotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl DoubleQuotedString {
    /// Build a `DoubleQuotedString` from `the content between the double quotes.`
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for DoubleQuotedString {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!(r#""{}""#, self.content)
    }
}
