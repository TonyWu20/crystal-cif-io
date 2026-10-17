use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// An unquoted character string: `<OrdinaryChar><NonBlankChar>*`
/// (spec Appendix A).
#[derive(Debug, Clone)]
pub struct UnquotedString {
    content: String,
}

impl AsRef<str> for UnquotedString {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.content)
    }
}

impl UnquotedString {
    /// Build a `UnquotedString` from `the content as written (no delimiters).`
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for UnquotedString {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        self.content.to_owned()
    }
}

impl Display for UnquotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
