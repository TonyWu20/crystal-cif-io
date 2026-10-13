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

pub mod builder;
pub mod data_dict;
