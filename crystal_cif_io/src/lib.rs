//! The data layer of the IUCr crystallography `cif` format.
//!
//! This crate holds the data-dictionary layer: the per-category
//! terms, the closed-set enums, and the `bon` builders
//! (`src/builder`). The `crystal-cif-fmt` IR value and block types
//! it names, and the `crystallographic-group` builder types, are
//! re-exported at the crate root (and under `group`). The read,
//! edit, and write workflows stay inside this crate.
//!
//! ```no_run
//! use crystal_cif_io::parse_core_cif;
//!
//! let cifs = parse_core_cif("data_1\n_cell_length_a 7.0\n").expect("the file types");
//! ```
//!
//! Add `crystal-cif-fmt` only for direct IR control: loop columns,
//! save-frame trees, 2.0 containers, canonical JSON, dialect
//! detection. Add `crystallographic-group` only for group machinery
//! beyond `group::SpaceGroup`.
//!
//! See `docs/typed-structs.md` (D13) for the split.

pub mod builder;
pub mod core_cif;
pub mod data_dict;

/// The typed document layer (roadmap D12). The block view and its error.
pub use core_cif::{
    parse_core_cif, write_core_cif, BlockTypeFailure, CifError, CoreCif, CoreCifParseError,
};

/// The wave 1 category structs (roadmap D12).
pub use data_dict::core_cif::{
    atom_site::AtomSite, cell::Cell, chemical_formula::ChemicalFormula, space_group::SpaceGroup,
    space_group_symop::Symop,
};

/// The T3 wave category structs (roadmap D12).
pub use data_dict::core_cif::{
    atom_sites::{AtomSites, AtomSitesRow},
    atom_type::{AtomType, AtomTypeRow},
    chemical::{Chemical, ChemicalRow},
    chemical_conn_atom::ChemicalConnAtomRow,
    chemical_conn_bond::{ChemicalConnBond, ChemicalConnBondRow},
    geom::Geom,
    geom_angle::{GeomAngle, GeomAngleRow},
    geom_bond::{GeomBond, GeomBondRow},
    geom_contact::{GeomContact, GeomContactRow},
    geom_hbond::{GeomHbond, GeomHbondRow},
    geom_torsion::{GeomTorsion, GeomTorsionRow},
    valence_param::{ValenceParam, ValenceParamRow},
    valence_ref::ValenceRefRow,
};

/// The T5 wave category structs of tier T5 (roadmap D12).
pub use data_dict::core_cif::{
    cell_measurement_refln::CellMeasurementRefln,
    diffrn::{Diffrn, DiffrnRow},
    diffrn_attenuator::DiffrnAttenuator,
    diffrn_detector::DiffrnDetector,
    diffrn_measurement::DiffrnMeasurement,
    diffrn_orient_matrix::{DiffrnOrientMatrix, DiffrnOrientMatrixRow},
    diffrn_orient_refln::DiffrnOrientRefln,
    diffrn_radiation::DiffrnRadiation,
    diffrn_radiation_wavelength::DiffrnRadiationWavelengthRow,
    diffrn_refln::DiffrnRefln,
    diffrn_reflns::{DiffrnReflns, DiffrnReflnsRow},
    diffrn_reflns_class::DiffrnReflnsClass,
    diffrn_scale_group::DiffrnScaleGroup,
    diffrn_source::DiffrnSource,
    diffrn_standard_refln::{DiffrnStandardRefln, DiffrnStandardReflnRow},
    diffrn_standards::DiffrnStandards,
};

/// The T4 tier category structs (roadmap D12).
pub use data_dict::core_cif::{
    audit::Audit, audit_author::AuditAuthor, audit_author::AuditAuthorRow,
    audit_conform::AuditConform, audit_conform::AuditConformRow,
    audit_contact_author::AuditContactAuthor, audit_link::AuditLink, citation::Citation,
    citation::CitationRow, citation_author::CitationAuthor, citation_editor::CitationEditor,
    citation_editor::CitationEditorRow, computing::Computing, computing::ComputingRow,
    database::Database, database::DatabaseRow, journal::Journal, journal::JournalRow,
    journal_index::JournalIndex, journal_index::JournalIndexRow, publ::Publ, publ::PublRow,
    publ_author::PublAuthor, publ_author::PublAuthorRow, publ_body::PublBody,
    publ_manuscript_incl::PublManuscriptIncl,
};

// T2 wave: the tier T2 category structs (roadmap D12j).
pub use data_dict::core_cif::{
    exptl::{Exptl, ExptlAbsorptCorrectionT},
    exptl_crystal::ExptlCrystal,
    exptl_crystal_face::ExptlCrystalFace,
    refine::{Refine, RefineDiffDensity},
    refine_ls_class::RefineLsClass,
    refln::Refln,
    reflns::{Reflns, ReflnsDResolution, ReflnsLimit},
    reflns_class::ReflnsClass,
    reflns_scale::ReflnsScale,
    reflns_shell::ReflnsShell,
    symmetry::Symmetry,
    symmetry_equiv::SymmetryEquiv,
};

// ---------------------------------------------------------------------------
// Re-exports: the IR value and block types of `crystal-cif-fmt`, and the
// group types of `crystallographic-group`. The read, edit, write, and
// builder workflows of this crate name these types. Re-exporting them
// keeps those workflows inside this crate.
//
// Add `crystal-cif-fmt` to your own `Cargo.toml` only for direct IR
// control: loop columns, save-frame trees, 2.0 containers, canonical
// JSON, dialect detection. Add `crystallographic-group` only for group
// machinery beyond `group::SpaceGroup`.
// ---------------------------------------------------------------------------

/// The IR value and block types of `crystal-cif-fmt` the read, edit,
/// and write workflows name. Re-exported so those workflows stay
/// inside this crate.
pub use crystal_cif_fmt::{
    parse_cif, CharString, CifDocument, DataBlock, DataBlockHeading, DataBlockMember, Matrix,
    Number, Numeric, SaveFrame, SaveFrameHeading, UnquotedString, UnsignedInteger, Value,
};

/// The `crystallographic-group` type the builders name. Re-exported
/// so the builder workflow stays inside this crate. Resolve a group
/// with `SpaceGroup::try_from_str` over a Hall symbol, for example
/// `SpaceGroup::try_from_str("-P 2ybc")`.
pub mod group {
    pub use crystallographic_group::SpaceGroup;
}
