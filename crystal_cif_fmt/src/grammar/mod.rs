//! Intermediate representation (IR) types for CIF files.
//!
//! The value grammar is the CIF 1.1 form, plus the CIF 2.0 list
//! value (`Value::List`) for files in the 2.0 dialect. The types
//! are built by the chumsky parser in [`crate::parser`] and encoded
//! to the groundtruth JSON by [`crate::canonical`]. The old winnow
//! parser is gone; see `docs/refactor-chumsky-ariadne.md`.

use std::fmt::Display;

mod index;
mod matrix;
mod numeric_values;
mod strings_textfields;
mod structures;
mod tags_values;
mod whitespace_comments;

pub use matrix::{Matrix, MatrixError};
pub use numeric_values::{Float, Integer, Number, Numeric, UnsignedInteger};
pub use strings_textfields::{
    CharString, DoubleQuotedString, SemiColonTextField, SingleQuotedString, TextField,
    TripleDoubleQuotedString, TripleSingleQuotedString, UnquotedString,
};
pub use structures::{
    CIFDataType, DataBlock, DataBlockHeading, DataBlockMember, DataItems, LoopColumn, LoopColumns,
    LoopHeader, LoopUnit, SaveFrame, SaveFrameHeading, SingleLineData,
};
pub use tags_values::{CIFValue, Tag, Value};
pub use whitespace_comments::Comments;

/// Uniform writer hook over the IR types: each syntactic unit knows
/// how to render itself back to CIF text (the `Display` impls of the
/// types delegate to `formatted_output`).
pub trait SyntacticUnit {
    type FormatOutput: Display;
    fn formatted_output(&self) -> Self::FormatOutput;
}

#[derive(Debug, Clone)]
pub struct CifDocument {
    comments: Option<Comments>,
    data_blocks: Option<Vec<DataBlock>>,
}

impl CifDocument {
    pub fn new(comments: Option<Comments>, data_blocks: Option<Vec<DataBlock>>) -> Self {
        Self {
            comments,
            data_blocks,
        }
    }

    pub fn data_blocks(&self) -> Option<&Vec<DataBlock>> {
        self.data_blocks.as_ref()
    }

    /// The leading comment lines, when the document carries them.
    pub fn comments(&self) -> &Option<Comments> {
        &self.comments
    }

    pub fn get_data_block_by_name(&self, data_block_name: &str) -> Option<&DataBlock> {
        self.data_blocks().map(|blocks| {
            blocks
                .iter()
                .find(|block| block.heading() == data_block_name)
        })?
    }

    pub fn data_blocks_mut(&mut self) -> &mut Option<Vec<DataBlock>> {
        &mut self.data_blocks
    }
}

impl CifDocument {
    /// Render the document back to CIF text.
    pub fn formatted_output(&self) -> String {
        let comment = match &self.comments {
            Some(c) => c.to_string(),
            None => "#\\#CIF_1.1".to_string(),
        };
        let data_blocks = match &self.data_blocks {
            Some(blocks) => blocks
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            None => String::new(),
        };
        [comment, data_blocks].join("\n")
    }
}

impl Display for CifDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
