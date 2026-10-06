pub mod data_dict;

mod grammar;

pub use grammar::{
    CIFDataType, CIFValue, CharString, CifDocument, DataBlock, DataBlockHeading, DataBlockMember,
    DataItems, DoubleQuotedString, Float, Integer, LoopColumn, LoopColumns, LoopUnit, Number,
    Numeric, SingleLineData, SingleQuotedString, Tag, TextField, UnquotedString, UnsignedInteger,
    Value,
};

#[cfg(feature = "chemrust-core")]
pub use grammar::chemrust_impl::{from_data_block_members, to_cif_document, to_data_block};

/// Canonical JSON encoding of the parsed IR, plus spec-level validation
/// (duplicate block names / tags). Shared by the groundtruth harness
/// (`tests/ground_truth.rs`) and the generator (`examples/gt_generate.rs`).
/// See `docs/ground-truth.md`.
pub mod canonical;

mod parser;
