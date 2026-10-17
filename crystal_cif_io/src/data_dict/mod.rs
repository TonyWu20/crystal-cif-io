//! The data-dictionary layer of the typed document (roadmap D12).
//!
//! The `core_cif` tree holds one module per CIF category: the term
//! enums, the closed-set enums, and the category structs. The tag
//! registry (`tags`) and the inequality and matrix helpers serve
//! them.

/// The CIF category tree: one module per category of the typed layer.
pub mod core_cif;
/// The inequality-value helpers of the dictionary layer.
pub mod ineq;
/// The matrix-value helpers of the dictionary layer.
pub mod matrix;
/// The tag registry of the implemented categories.
pub mod tags;

pub use ineq::Ineq;

/// The term materialization traits live in the `crystal-cif-fmt`
/// crate (`crystal_cif_fmt::term`). The parser types implement
/// them there, so the data layer re-exports them. The category
/// modules import them from here.
pub use crystal_cif_fmt::{CifTerm, LoopValueTerm, SingleValueTerm};
