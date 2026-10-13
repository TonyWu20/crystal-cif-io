use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A triple-quote-quoted character string (CIF 2.0 only). The
/// content is stored between the delimiters, exactly as written; a
/// line terminator is content. The closing delimiter is the first
/// `"""` run after the open. (Roadmap section 14.)
#[derive(Debug, Clone)]
pub struct TripleDoubleQuotedString {
    content: String,
}

impl AsRef<str> for TripleDoubleQuotedString {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.content)
    }
}

impl Display for TripleDoubleQuotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl TripleDoubleQuotedString {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for TripleDoubleQuotedString {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("\"\"\"{}\"\"\"", self.content)
    }
}

impl From<TripleDoubleQuotedString> for super::CharString {
    fn from(value: TripleDoubleQuotedString) -> Self {
        Self::TripleDoubleQuoted(value)
    }
}
