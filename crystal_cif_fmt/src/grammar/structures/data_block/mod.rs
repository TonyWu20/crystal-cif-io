use std::fmt::Display;

use crate::grammar::SyntacticUnit;

use crate::LoopColumn;

pub use self::{heading::DataBlockHeading, members::DataBlockMember};

use super::SingleLineData;

mod heading;
mod members;

/// One `data_` block: the heading and the ordered members
/// (data item lines and save frames).
#[derive(Debug, Clone)]
pub struct DataBlock {
    heading: DataBlockHeading,
    members: Vec<DataBlockMember>,
}

impl DataBlock {
    /// Build a block from its heading and members.
    pub fn from_heading_members(heading_member: (DataBlockHeading, Vec<DataBlockMember>)) -> Self {
        let (heading, members) = heading_member;
        Self { heading, members }
    }

    /// The loop column that carries the tag, when one of the
    /// block's data items holds it. The column values are a
    /// copy.
    pub fn find_loop_column_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<LoopColumn> {
        self.members.iter().find_map(|member| {
            if let DataBlockMember::DataItems(data_item) = member {
                data_item.get_loop_column_values_by_tag(&tag)
            } else {
                None
            }
        })
    }

    /// The single-value line that carries the tag, when one of
    /// the block's data items holds it.
    pub fn find_single_value_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<&SingleLineData> {
        self.members.iter().find_map(|member| {
            if let DataBlockMember::DataItems(data_item) = member {
                data_item.get_single_value_by_tag(&tag)
            } else {
                None
            }
        })
    }

    /// The block name without the `data_` prefix, as written.
    pub fn heading(&self) -> &str {
        self.heading.as_ref()
    }

    /// The members, in block order.
    pub fn members(&self) -> &[DataBlockMember] {
        &self.members
    }

    /// The members, mutable.
    pub fn members_mut(&mut self) -> &mut Vec<DataBlockMember> {
        &mut self.members
    }
}

impl crate::grammar::SyntacticUnit for DataBlock {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        let members = self
            .members
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        [self.heading.to_string(), members].join("\n")
    }
}

impl Display for DataBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
