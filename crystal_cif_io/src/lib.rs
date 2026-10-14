//! The data layer of the IUCr crystallography `cif` format.
//!
//! This crate holds the data-dictionary layer: the per-category
//! terms, the closed-set enums, and the `bon` builders
//! (`src/builder`). It builds on the `crystal-cif-fmt` crate,
//! which owns the parser, the IR types (`Tag`, `Value`,
//! `DataBlock`), and the canonical JSON. Import the two crates
//! side by side:
//!
//! ```no_run
//! use crystal_cif_fmt::parse_cif;
//! use crystal_cif_io::builder::cell::CellBuilder;
//! ```
//!
//! See `docs/typed-structs.md` (D13) for the split.

pub mod core_cif;
pub mod builder;
pub mod data_dict;

/// The typed document layer (roadmap D12). The block view and its error.
pub use core_cif::{CifError, CoreCif};

/// The wave 1 category structs (roadmap D12).
pub use data_dict::core_cif::{
    atom_site::AtomSite, cell::Cell, chemical_formula::ChemicalFormula,
    space_group::SpaceGroup, space_group_symop::Symop,
};

// T2 wave: the tier T2 category structs (roadmap D12j).
pub use data_dict::core_cif::{
    exptl::{Exptl, ExptlAbsorptCorrectionT}, exptl_crystal::ExptlCrystal,
    exptl_crystal_face::ExptlCrystalFace, refine::{Refine, RefineDiffDensity},
    refine_ls_class::RefineLsClass, refln::Refln,
    reflns::{Reflns, ReflnsDResolution, ReflnsLimit}, reflns_class::ReflnsClass,
    reflns_scale::ReflnsScale, reflns_shell::ReflnsShell, symmetry::Symmetry,
    symmetry_equiv::SymmetryEquiv,
};
