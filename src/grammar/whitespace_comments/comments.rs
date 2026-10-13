use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A whitespace/comment region, held verbatim. In the chumsky parser
/// the region is skipped and not captured; this type is kept for the
/// document-level writer hook.
#[derive(Debug, Clone)]
pub struct Comments {
    pub content: String,
}

impl Comments {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for Comments {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        self.content.to_string()
    }
}

impl Display for Comments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
