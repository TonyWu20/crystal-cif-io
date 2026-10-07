use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{
        tags_values::{Tag, Value},
        SyntacticUnit,
    },
};

#[derive(Debug, Clone, Default)]
pub struct SingleLineData {
    tag: Tag,
    value: Value,
}

impl SingleLineData {
    pub fn from_tag_value(tag_value: (Tag, Value)) -> Self {
        let (tag, value) = tag_value;
        Self { tag, value }
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
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
