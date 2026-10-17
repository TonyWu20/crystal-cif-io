use std::fmt::Display;

use crate::{
    grammar::{
        tags_values::{Tag, Value},
        SyntacticUnit,
    },
    term::{CifTerm, SingleValueTerm},
};

/// One `tag value` line of a data block: the tag and its
/// single value.
#[derive(Debug, Clone, Default)]
pub struct SingleLineData {
    tag: Tag,
    value: Value,
}

impl SingleLineData {
    /// Build the line from its tag and value.
    pub fn from_tag_value(tag_value: (Tag, Value)) -> Self {
        let (tag, value) = tag_value;
        Self { tag, value }
    }

    /// The line tag.
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// The line value.
    pub fn value(&self) -> &Value {
        &self.value
    }
    /// The empty line: the null tag and the `unknown` value.
    pub const fn null() -> Self {
        SingleLineData {
            tag: Tag::null(),
            value: Value::Unknown,
        }
    }
}

impl SyntacticUnit for SingleLineData {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("{:<33} {}", self.tag.to_string(), self.value)
    }
}

impl Display for SingleLineData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl CifTerm for SingleLineData {
    fn tag(&self) -> Tag {
        self.tag.clone()
    }
}

impl SingleValueTerm for SingleLineData {
    fn value(&self) -> Value {
        self.value().clone()
    }
}
