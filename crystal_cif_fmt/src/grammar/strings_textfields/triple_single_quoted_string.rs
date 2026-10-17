use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A triple-apostrophe-quoted character string (CIF 2.0 only). The
/// content is stored between the delimiters, exactly as written; a
/// line terminator is content. The closing delimiter is the first
/// `'''` run after the open. (Roadmap section 14.)
#[derive(Debug, Clone)]
pub struct TripleSingleQuotedString {
    content: String,
}

impl AsRef<str> for TripleSingleQuotedString {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.content)
    }
}

impl Display for TripleSingleQuotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl TripleSingleQuotedString {
    /// Build a `TripleSingleQuotedString` from `the content between the triple apostrophes.`
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for TripleSingleQuotedString {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("'''{}'''", self.content)
    }
}

impl From<TripleSingleQuotedString> for super::CharString {
    fn from(value: TripleSingleQuotedString) -> Self {
        Self::TripleSingleQuoted(value)
    }
}
