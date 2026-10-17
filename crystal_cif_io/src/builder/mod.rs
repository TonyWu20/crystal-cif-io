//! The per-category builder layer (roadmap decision D11).
//!
//! Each builder takes the author-supplied items of one dictionary
//! category as input and materializes `DataItems` through the
//! `data_dict` traits (`CifTerm`, `SingleValueTerm`,
//! `LoopValueTerm`). The required fields of the builder input spec
//! (`docs/dict-completeness.md` section 5) are needed at the
//! `build()` call: a missing needed field fails the build. The
//! derived items are not builder input. The builder computes them
//! and cross-checks a supplied value within tolerance. A mismatch
//! rejects with a `Result`, never a panic.
//!
//! The builders are on the `bon` crate. Equivalent minimum entry
//! sets own their own builders (decision D11): the `cell` category
//! ships two builders (the direct six, the three lattice vectors),
//! the `space_group` category ships one builder with three entry
//! points, resolved through the `crystallographic-group` crate.
//!
//! The `_su` uncertainty items are skipped per decision D2: the
//! uncertainty stays inside `Numeric` (the `5.231(7)` form), and
//! the dictionary layer records no `_su` items.
//!
//! - `cell`: the direct-six builder and the three-vector builder.
//! - `space_group`: the builder with the `IT_number`, `name_Hall`,
//!   and `name_H-M` entry points.
//! - `symop`: the `space_group_symop` loop builder. The row text
//!   cross-checks against the general-position Seitz list.
//! - `atom_site`: the row builder. One coordinate mode per row, a
//!   needed displacement-type enum.
//! - `chemical_formula`: the builder over `structural`, the other
//!   spellings optional. The `weight` helper takes the density
//!   argument. It is not a builder field.
//! - `formula_units`: the structure-level `Z` computation and
//!   cross-check.

pub mod atom_site;
pub mod cell;
pub mod chemical_formula;
pub mod formula_units;
pub mod space_group;
pub mod symop;
