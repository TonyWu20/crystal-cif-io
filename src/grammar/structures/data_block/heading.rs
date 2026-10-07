use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// The heading of a data block: `data_` plus the block code. The
/// stored name is the block code without the `data_` prefix, exactly
/// as written (reserved words are case-insensitive, spec para. 26).
#[derive(Debug, Clone)]
pub struct DataBlockHeading {
    name: String,
}

impl AsRef<str> for DataBlockHeading {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.name)
    }
}

impl DataBlockHeading {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl crate::grammar::SyntacticUnit for DataBlockHeading {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("\ndata_{}\n", &self.name)
    }
}

impl Display for DataBlockHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
