#![warn(missing_docs)]
//! Parser and IR for the IUCr crystallography `cif` format.
//!
//! This crate holds the CIF syntax layer: the IR types
//! (`grammar`), the chumsky + ariadne parser (`parse_cif`),
//! and the canonical JSON encoding (`canonical`). The typed
//! data-dictionary layer of `crystal-cif-io` builds on this
//! crate.
//!
//! ```
//! use crystal_cif_fmt::parse_cif;
//!
//! let doc = parse_cif("data_1\n_cell_length_a 7.0\n").expect("a valid file");
//! let blocks = doc.data_blocks().expect("the blocks");
//! assert_eq!(blocks.len(), 1);
//! ```
//!
//! The value types carry the CIF 2.0 list value as `Value::List`.
//! It parses only in the 2.0 dialect. See the `Value` docs.
//!
//! The parser design note lives in `docs/refactor-chumsky-ariadne.md`.

pub mod grammar;

/// The value exports carry the CIF 2.0 list value as `Value::List`
/// (nested lists are legal; it parses only in the 2.0 dialect).
///
/// ```
/// use crystal_cif_fmt::Value;
///
/// let value = Value::List(Vec::new());
/// assert_eq!(value.to_string(), "[]");
/// ```
/// The `Matrix` constructor reads a list of numeric lists:
///
/// ```
/// use crystal_cif_fmt::{Matrix, Numeric, Value};
///
/// let value = Value::List(vec![Value::List(vec![
///     Value::Numeric(Numeric::from(1i32)),
///     Value::Numeric(Numeric::from(2i32)),
/// ])]);
/// let matrix = Matrix::from_list(value.as_list().expect("a list"), Some((1, 2)))
///     .expect("a 1 x 2 matrix");
/// assert_eq!(matrix.shape(), (1, 2));
/// ```
pub use grammar::{
    CIFDataType, CIFValue, CharString, CifDocument, Comments, DataBlock, DataBlockHeading,
    DataBlockMember, DataItems, DoubleQuotedString, Float, Integer, LoopColumn, LoopColumns,
    LoopHeader, LoopUnit, Matrix, MatrixError, Number, Numeric, SaveFrame, SaveFrameHeading,
    SemiColonTextField, SingleLineData, SingleQuotedString, Tag, TextField,
    TripleDoubleQuotedString, TripleSingleQuotedString, UnquotedString, UnsignedInteger, Value,
};

/// The chumsky + ariadne CIF parser. `parse_cif` is the strict entry
/// point (`Ok` means the whole input was consumed); `rich_error`
/// renders a `Rich` error as an ariadne report with the source text
/// labeled. `CifDialect` holds the value-rule dialect (CIF 1.1 or
/// the 2.0 list-value rules) detected from the file start. See
/// `docs/refactor-chumsky-ariadne.md`.
pub use parser::{parse_cif, rich_error, syntax_error_report, CifDialect};

/// The term materialization traits. A term knows its tag and
/// materializes into the IR value types. The data-dictionary
/// layer of `crystal-cif-io` implements the traits; the parser
/// types (`SingleLineData`, `LoopColumn`) implement them here.
pub use term::{CifTerm, LoopValueTerm, SingleValueTerm};

/// Canonical JSON encoding of the parsed IR, plus spec-level
/// validation (duplicate block names / tags). Shared by the
/// groundtruth harness (`tests/ground_truth.rs`) and the generator
/// (`examples/gt_generate.rs`). See `docs/ground-truth.md`.
pub mod canonical;

pub mod parser;
pub mod term;
