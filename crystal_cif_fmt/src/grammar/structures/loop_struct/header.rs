use std::fmt::Display;

use crate::grammar::{tags_values::Tag, SyntacticUnit};

#[derive(Debug, Clone)]
pub struct LoopHeader {
    tags: Vec<Tag>,
}

impl LoopHeader {
    pub fn new(tags: Vec<Tag>) -> Self {
        Self { tags }
    }
    pub fn num_of_tags(&self) -> usize {
        self.tags.len()
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn get_tag_index<T: AsRef<str>>(&self, tag: T) -> Option<usize> {
        self.tags().iter().position(|t| t.as_ref() == tag.as_ref())
    }
}

impl SyntacticUnit for LoopHeader {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        let tags = self
            .tags
            .iter()
            .map(|t| format!("{t}"))
            .collect::<Vec<String>>()
            .join("\n");
        format!("loop_\n{tags}")
    }
}

impl Display for LoopHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.formatted_output())
    }
}
