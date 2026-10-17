use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A CIF tag: the data name without the leading underscore.
/// The tag renders back to CIF text with the underscore.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tag {
    name: String,
}

impl AsRef<str> for Tag {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.name)
    }
}

impl Tag {
    /// Create a tag from its data name, without the leading
    /// underscore.
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// The data name, without the leading underscore.
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
    /// The null tag: the empty name.
    pub const fn null() -> Self {
        Self {
            name: String::new(),
        }
    }

    /// The data name, without the leading underscore.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl SyntacticUnit for Tag {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("_{}", self.name)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
