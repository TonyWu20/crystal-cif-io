use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A CIF text field held between two `;` lines: one line per
/// field line. The first line is the remainder of the line
/// holding the opening `;` (it may be empty).
#[derive(Debug, Clone)]
pub struct SemiColonTextField {
    lines: Vec<String>,
}

impl AsRef<Vec<String>> for SemiColonTextField {
    fn as_ref(&self) -> &Vec<String> {
        <Vec<String> as AsRef<Vec<String>>>::as_ref(&self.lines)
    }
}

impl From<&SemiColonTextField> for String {
    fn from(value: &SemiColonTextField) -> Self {
        value.lines.join("\n")
    }
}

/// In the present revision the production for `<TextField>` is a
/// trivial equivalence to `<SemiColonTextField>`. The redundancy is
/// retained to permit possible future extensions to text fields, in
/// particular the possible introduction of a bracket-delimited text
/// value.
#[derive(Debug, Clone)]
pub enum TextField {
    /// The `;`-delimited text field.
    SemiColonTextField(SemiColonTextField),
}

impl TextField {
    /// The text field content: the lines joined with a newline. The
    /// first line is the remainder of the line holding the opening
    /// `;` (it may be empty).
    pub fn raw_string(&self) -> String {
        match self {
            TextField::SemiColonTextField(sm) => sm.lines.join("\n"),
        }
    }
}

impl From<&TextField> for String {
    fn from(value: &TextField) -> Self {
        match value {
            TextField::SemiColonTextField(s) => s.into(),
        }
    }
}

impl Display for TextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextField::SemiColonTextField(t) => write!(f, "{t}"),
        }
    }
}

impl SemiColonTextField {
    /// Build the field from its lines.
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

impl SyntacticUnit for SemiColonTextField {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("\n;{}\n;", self.lines.join("\n"))
    }
}

impl Display for SemiColonTextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl SyntacticUnit for TextField {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            TextField::SemiColonTextField(f) => f.to_string(),
        }
    }
}
