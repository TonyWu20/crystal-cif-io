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
    diffrn::{Diffrn, DiffrnRow}, diffrn_attenuator::DiffrnAttenuator,
    diffrn_detector::DiffrnDetector, diffrn_measurement::DiffrnMeasurement,
    diffrn_orient_matrix::{DiffrnOrientMatrix, DiffrnOrientMatrixRow},
    diffrn_orient_refln::DiffrnOrientRefln, diffrn_radiation::DiffrnRadiation,
    diffrn_radiation_wavelength::DiffrnRadiationWavelengthRow,
    diffrn_refln::DiffrnRefln, diffrn_reflns::{DiffrnReflns, DiffrnReflnsRow},
    diffrn_reflns_class::DiffrnReflnsClass, diffrn_scale_group::DiffrnScaleGroup,
    diffrn_source::DiffrnSource,
    diffrn_standard_refln::{DiffrnStandardRefln, DiffrnStandardReflnRow},
    diffrn_standards::DiffrnStandards,
};

/// The T4 tier category structs (roadmap D12).
pub use data_dict::core_cif::{
    audit::Audit, audit_author::AuditAuthor, audit_author::AuditAuthorRow,
    audit_conform::AuditConform, audit_conform::AuditConformRow,
    audit_contact_author::AuditContactAuthor, audit_link::AuditLink,
    citation::Citation, citation::CitationRow, citation_author::CitationAuthor,
    citation_editor::CitationEditor, citation_editor::CitationEditorRow,
    computing::Computing, computing::ComputingRow, database::Database,
    database::DatabaseRow, journal::Journal, journal::JournalRow,
    journal_index::JournalIndex, journal_index::JournalIndexRow,
    publ::Publ, publ::PublRow, publ_author::PublAuthor,
    publ_author::PublAuthorRow, publ_body::PublBody,
    publ_manuscript_incl::PublManuscriptIncl,
// T2 wave: the tier T2 category structs (roadmap D12j).
pub use data_dict::core_cif::{
    exptl::{Exptl, ExptlAbsorptCorrectionT}, exptl_crystal::ExptlCrystal,
    exptl_crystal_face::ExptlCrystalFace, refine::{Refine, RefineDiffDensity},
    refine_ls_class::RefineLsClass, refln::Refln,
    reflns::{Reflns, ReflnsDResolution, ReflnsLimit}, reflns_class::ReflnsClass,
    reflns_scale::ReflnsScale, reflns_shell::ReflnsShell, symmetry::Symmetry,
    symmetry_equiv::SymmetryEquiv,
};
