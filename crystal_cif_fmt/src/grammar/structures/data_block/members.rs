use std::fmt::Display;

use crate::grammar::SyntacticUnit;

use crate::grammar::structures::{data_items::DataItems, save_frame::SaveFrame};

/// One member of a data block: a data item line or a save
/// frame, in the order the block holds them.
#[derive(Debug, Clone)]
pub enum DataBlockMember {
    /// A run of tag lines, one or more loop units, or both.
    DataItems(DataItems),
    /// A `save_` frame with its own members.
    SaveFrame(SaveFrame),
}

impl crate::grammar::SyntacticUnit for DataBlockMember {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            DataBlockMember::DataItems(d) => d.to_string(),
            DataBlockMember::SaveFrame(s) => s.to_string(),
        }
    }
}

impl Display for DataBlockMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
