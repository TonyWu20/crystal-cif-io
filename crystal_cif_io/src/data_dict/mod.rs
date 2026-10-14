pub mod core_cif;
pub mod ineq;
pub mod matrix;
pub mod tags;

pub use ineq::Ineq;

/// The term materialization traits live in the `crystal-cif-fmt`
/// crate (`crystal_cif_fmt::term`). The parser types implement
/// them there, so the data layer re-exports them. The category
/// modules import them from here.
pub use crystal_cif_fmt::{CifTerm, LoopValueTerm, SingleValueTerm};
