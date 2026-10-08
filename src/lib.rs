pub mod data_dict;

mod grammar;

pub use grammar::{
    CIFDataType, CIFValue, CharString, CifDocument, Comments, DataBlock, DataBlockHeading,
    DataBlockMember, DataItems, DoubleQuotedString, Float, Integer, LoopColumn, LoopColumns,
    LoopHeader, LoopUnit, Matrix, MatrixError, Number, Numeric, SaveFrame, SaveFrameHeading,
    SemiColonTextField, SingleLineData, SingleQuotedString, Tag, TextField, UnquotedString,
    UnsignedInteger, Value,
};


/// The chumsky + ariadne CIF 1.1 parser. `parse_cif` is the strict
/// entry point (`Ok` means the whole input was consumed);
/// `rich_error` renders a `Rich` error as an ariadne report with the
/// source text labeled. See `docs/refactor-chumsky-ariadne.md`.
pub use parser::{parse_cif, rich_error};

/// Canonical JSON encoding of the parsed IR, plus spec-level
/// validation (duplicate block names / tags). Shared by the
/// groundtruth harness (`tests/ground_truth.rs`) and the generator
/// (`examples/gt_generate.rs`). See `docs/ground-truth.md`.
pub mod canonical;

mod parser;
