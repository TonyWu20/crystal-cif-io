pub mod core_cif;
pub mod matrix;
pub mod tags;

/// The term materialization traits live in the `crystal-cif-fmt`
/// crate (`crystal_cif_fmt::term`). The parser types implement
/// them there, so the data layer re-exports them. The category
/// modules import them from here.
pub use crystal_cif_fmt::{CifTerm, LoopValueTerm, SingleValueTerm};
