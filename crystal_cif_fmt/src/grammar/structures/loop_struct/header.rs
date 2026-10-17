use std::fmt::Display;

use crate::grammar::{tags_values::Tag, SyntacticUnit};

/// The loop header: the loop tags, one per column, in loop
/// order.
#[derive(Debug, Clone)]
pub struct LoopHeader {
    tags: Vec<Tag>,
}

impl LoopHeader {
    /// Build the header from the loop tags.
    pub fn new(tags: Vec<Tag>) -> Self {
        Self { tags }
    }
    /// The number of loop tags.
    pub fn num_of_tags(&self) -> usize {
        self.tags.len()
    }

    /// The loop tags, in loop order.
    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    /// The index of the column that carries the tag.
    /// `None` on a tag miss.
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
