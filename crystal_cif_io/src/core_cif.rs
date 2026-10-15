//! The typed block view of one data block (roadmap D12).
//!
//! `CoreCif` is the typed document: the category groups of the D12
//! waves (wave 1 and the T2 through T5 tiers), and the `extra` residue.
//! No typed category consumes the residue (foreign tags, save frames);
//! it keeps their order and bytes. The guarantee is IR round-trip, not
//! byte round-trip.

use crystal_cif_fmt::grammar::DataBlock;
use crystal_cif_fmt::{
    parse_cif, syntax_error_report, CifDialect, CifDocument, Comments, DataBlockHeading,
    DataBlockMember,
};

use crate::data_dict::core_cif::atom_site::AtomSite;
use crate::data_dict::core_cif::atom_sites::AtomSites;
use crate::data_dict::core_cif::atom_type::AtomType;
use crate::data_dict::core_cif::audit::Audit;
use crate::data_dict::core_cif::audit_author::{AuditAuthor, AuditAuthorRow};
use crate::data_dict::core_cif::audit_conform::{AuditConform, AuditConformRow};
use crate::data_dict::core_cif::audit_contact_author::AuditContactAuthor;
use crate::data_dict::core_cif::audit_link::AuditLink;
use crate::data_dict::core_cif::cell::Cell;
use crate::data_dict::core_cif::cell_measurement_refln::CellMeasurementRefln;
use crate::data_dict::core_cif::chemical::Chemical;
use crate::data_dict::core_cif::chemical_conn_atom::ChemicalConnAtomRow;
use crate::data_dict::core_cif::chemical_conn_bond::ChemicalConnBond;
use crate::data_dict::core_cif::chemical_formula::ChemicalFormula;
use crate::data_dict::core_cif::citation::{Citation, CitationRow};
use crate::data_dict::core_cif::citation_author::CitationAuthor;
use crate::data_dict::core_cif::citation_editor::{CitationEditor, CitationEditorRow};
use crate::data_dict::core_cif::computing::{Computing, ComputingRow};
use crate::data_dict::core_cif::database::{Database, DatabaseRow};
use crate::data_dict::core_cif::diffrn::Diffrn;
use crate::data_dict::core_cif::diffrn_attenuator::DiffrnAttenuator;
use crate::data_dict::core_cif::diffrn_detector::DiffrnDetector;
use crate::data_dict::core_cif::diffrn_measurement::DiffrnMeasurement;
use crate::data_dict::core_cif::diffrn_orient_matrix::DiffrnOrientMatrix;
use crate::data_dict::core_cif::diffrn_orient_refln::DiffrnOrientRefln;
use crate::data_dict::core_cif::diffrn_radiation::DiffrnRadiation;
use crate::data_dict::core_cif::diffrn_radiation_wavelength::DiffrnRadiationWavelengthRow;
use crate::data_dict::core_cif::diffrn_refln::DiffrnRefln;
use crate::data_dict::core_cif::diffrn_reflns::{DiffrnReflns, DiffrnReflnsRow};
use crate::data_dict::core_cif::diffrn_reflns_class::DiffrnReflnsClass;
use crate::data_dict::core_cif::diffrn_scale_group::DiffrnScaleGroup;
use crate::data_dict::core_cif::diffrn_source::DiffrnSource;
use crate::data_dict::core_cif::diffrn_standard_refln::{
    DiffrnStandardRefln, DiffrnStandardReflnRow,
};
use crate::data_dict::core_cif::diffrn_standards::DiffrnStandards;
use crate::data_dict::core_cif::exptl::Exptl;
use crate::data_dict::core_cif::exptl_crystal::ExptlCrystal;
use crate::data_dict::core_cif::exptl_crystal_face::ExptlCrystalFace;
use crate::data_dict::core_cif::geom::Geom;
use crate::data_dict::core_cif::geom_angle::GeomAngle;
use crate::data_dict::core_cif::geom_bond::GeomBond;
use crate::data_dict::core_cif::geom_contact::GeomContact;
use crate::data_dict::core_cif::geom_hbond::GeomHbond;
use crate::data_dict::core_cif::geom_torsion::GeomTorsion;
use crate::data_dict::core_cif::journal::{Journal, JournalRow};
use crate::data_dict::core_cif::journal_index::{JournalIndex, JournalIndexRow};
use crate::data_dict::core_cif::publ::{Publ, PublRow};
use crate::data_dict::core_cif::publ_author::{PublAuthor, PublAuthorRow};
use crate::data_dict::core_cif::publ_body::PublBody;
use crate::data_dict::core_cif::publ_manuscript_incl::PublManuscriptIncl;
use crate::data_dict::core_cif::refine::Refine;
use crate::data_dict::core_cif::refine_ls_class::RefineLsClass;
use crate::data_dict::core_cif::refln::Refln;
use crate::data_dict::core_cif::reflns::Reflns;
use crate::data_dict::core_cif::reflns_class::ReflnsClass;
use crate::data_dict::core_cif::reflns_scale::ReflnsScale;
use crate::data_dict::core_cif::reflns_shell::ReflnsShell;
use crate::data_dict::core_cif::space_group::SpaceGroup;
use crate::data_dict::core_cif::space_group_symop::Symop;
use crate::data_dict::core_cif::symmetry::Symmetry;
use crate::data_dict::core_cif::symmetry_equiv::SymmetryEquiv;
use crate::data_dict::core_cif::valence_param::ValenceParam;
use crate::data_dict::core_cif::valence_ref::ValenceRefRow;
use crate::data_dict::core_cif::CifCategory;

/// A read-path conformance error of the typed layer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CifError {
    /// A typed item carries a value class the dictionary denies.
    #[error("{tag}: expected the {expected} value class, found {found}")]
    ValueClass {
        tag: String,
        expected: &'static str,
        found: &'static str,
    },
    /// A matrix item carries the wrong shape.
    #[error("{tag}: expected a {expected:?} shape, found {found:?}")]
    Shape {
        tag: String,
        expected: (usize, usize),
        found: (usize, usize),
    },
    /// A required item is absent (from [`CoreCif::validate`]).
    #[error("{category}: the required item {item} is missing")]
    Missing {
        category: &'static str,
        item: &'static str,
    },
}

/// The typed view of one data block (roadmap D12g).
#[derive(Debug, Clone, Default)]
pub struct CoreCif {
    /// The heading of the data block this view was read from. `to_block`
    /// writes it back, so the round trip needs no separate heading.
    pub heading: DataBlockHeading,
    pub cell: Cell,
    pub space_group: SpaceGroup,
    pub space_group_symop: Vec<Symop>,
    pub atom_site: Vec<AtomSite>,
    pub chemical_formula: ChemicalFormula,
    // T3 wave: the tier T3 category groups (roadmap D12j).
    pub atom_sites: AtomSites,
    pub atom_type: AtomType,
    pub chemical: Chemical,
    pub chemical_conn_atom: Vec<ChemicalConnAtomRow>,
    pub chemical_conn_bond: ChemicalConnBond,
    pub geom: Geom,
    pub geom_angle: GeomAngle,
    pub geom_bond: GeomBond,
    pub geom_contact: GeomContact,
    pub geom_hbond: GeomHbond,
    pub geom_torsion: GeomTorsion,
    pub valence_param: ValenceParam,
    pub valence_ref: Vec<ValenceRefRow>,
    // T5 wave: the 16 raw-diffraction categories of tier T5.
    pub cell_measurement_refln: Vec<CellMeasurementRefln>,
    pub diffrn: Diffrn,
    pub diffrn_attenuator: Vec<DiffrnAttenuator>,
    pub diffrn_detector: DiffrnDetector,
    pub diffrn_measurement: DiffrnMeasurement,
    pub diffrn_orient_matrix: DiffrnOrientMatrix,
    pub diffrn_orient_refln: Vec<DiffrnOrientRefln>,
    pub diffrn_radiation: DiffrnRadiation,
    pub diffrn_radiation_wavelength: Vec<DiffrnRadiationWavelengthRow>,
    pub diffrn_refln: Vec<DiffrnRefln>,
    pub diffrn_reflns: DiffrnReflns,
    pub diffrn_reflns_rows: Vec<DiffrnReflnsRow>,
    pub diffrn_reflns_class: Vec<DiffrnReflnsClass>,
    pub diffrn_scale_group: Vec<DiffrnScaleGroup>,
    pub diffrn_source: DiffrnSource,
    pub diffrn_standard_refln: DiffrnStandardRefln,
    pub diffrn_standard_refln_rows: Vec<DiffrnStandardReflnRow>,
    pub diffrn_standards: DiffrnStandards,
    // T4 wave: the tier T4 (metadata and publication) category fields.
    pub audit: Audit,
    pub audit_author: AuditAuthor,
    pub audit_author_rows: Vec<AuditAuthorRow>,
    pub audit_conform: AuditConform,
    pub audit_conform_rows: Vec<AuditConformRow>,
    pub audit_contact_author: AuditContactAuthor,
    pub audit_link: Vec<AuditLink>,
    pub citation: Citation,
    pub citation_rows: Vec<CitationRow>,
    pub citation_author: Vec<CitationAuthor>,
    pub citation_editor: CitationEditor,
    pub citation_editor_rows: Vec<CitationEditorRow>,
    pub computing: Computing,
    pub computing_rows: Vec<ComputingRow>,
    pub database: Database,
    pub database_rows: Vec<DatabaseRow>,
    pub journal: Journal,
    pub journal_rows: Vec<JournalRow>,
    pub journal_index: JournalIndex,
    pub journal_index_rows: Vec<JournalIndexRow>,
    pub publ: Publ,
    pub publ_rows: Vec<PublRow>,
    pub publ_author: PublAuthor,
    pub publ_author_rows: Vec<PublAuthorRow>,
    pub publ_body: Vec<PublBody>,
    pub publ_manuscript_incl: Vec<PublManuscriptIncl>,
    // T2 wave: the twelve categories of the T2 tier (roadmap D12j).
    pub exptl: Exptl, // T2 wave: single items and the T loop family.
    pub exptl_crystal: Vec<ExptlCrystal>, // T2 wave:
    pub exptl_crystal_face: Vec<ExptlCrystalFace>, // T2 wave:
    pub refln: Vec<Refln>, // T2 wave:
    pub reflns: Reflns, // T2 wave: single items and the two loop families.
    pub reflns_class: Vec<ReflnsClass>, // T2 wave:
    pub reflns_scale: Vec<ReflnsScale>, // T2 wave:
    pub reflns_shell: Vec<ReflnsShell>, // T2 wave:
    pub refine: Refine, // T2 wave: single items and the loop family.
    pub refine_ls_class: Vec<RefineLsClass>, // T2 wave:
    pub symmetry: Symmetry, // T2 wave:
    pub symmetry_equiv: Vec<SymmetryEquiv>, // T2 wave:
    /// Members no typed category consumed, plus every save frame.
    pub extra: Vec<DataBlockMember>,
}

impl CoreCif {
    /// Read one data block into the typed view.
    ///
    /// Each typed category consumes its own tags in turn. Save-frame
    /// members and unknown tags land in `extra`, order kept. Every
    /// value-class and shape mismatch is collected; the call fails with
    /// the full list, never the first.
    pub fn from_block(block: &DataBlock) -> Result<CoreCif, Vec<CifError>> {
        let mut doc = CoreCif::default();
        doc.heading = DataBlockHeading::new(block.heading().to_string());
        let members: Vec<DataBlockMember> = block.members().to_vec();
        let mut errors: Vec<CifError> = Vec::new();
        // Each category consumes the residue of the last. The residue is
        // threaded, so the order of the `extra` members is kept.
        let (residue, cell_errors) = doc.cell.from_members(&members);
        errors.extend(cell_errors);
        let (residue, space_group_errors) = doc.space_group.from_members(&residue);
        errors.extend(space_group_errors);
        let (residue, symop_errors) = doc.space_group_symop.from_members(&residue);
        errors.extend(symop_errors);
        let (residue, atom_site_errors) = doc.atom_site.from_members(&residue);
        errors.extend(atom_site_errors);
        let (residue, formula_errors) = doc.chemical_formula.from_members(&residue);
        errors.extend(formula_errors);
        // T3 wave: the tier T3 categories (roadmap D12j).
        let (residue, atom_sites_errors) = doc.atom_sites.from_members(&residue);
        errors.extend(atom_sites_errors);
        let (residue, atom_type_errors) = doc.atom_type.from_members(&residue);
        errors.extend(atom_type_errors);
        let (residue, chemical_errors) = doc.chemical.from_members(&residue);
        errors.extend(chemical_errors);
        let (residue, conn_atom_errors) = doc.chemical_conn_atom.from_members(&residue);
        errors.extend(conn_atom_errors);
        let (residue, conn_bond_errors) = doc.chemical_conn_bond.from_members(&residue);
        errors.extend(conn_bond_errors);
        let (residue, geom_errors) = doc.geom.from_members(&residue);
        errors.extend(geom_errors);
        let (residue, geom_angle_errors) = doc.geom_angle.from_members(&residue);
        errors.extend(geom_angle_errors);
        let (residue, geom_bond_errors) = doc.geom_bond.from_members(&residue);
        errors.extend(geom_bond_errors);
        let (residue, geom_contact_errors) = doc.geom_contact.from_members(&residue);
        errors.extend(geom_contact_errors);
        let (residue, geom_hbond_errors) = doc.geom_hbond.from_members(&residue);
        errors.extend(geom_hbond_errors);
        let (residue, geom_torsion_errors) = doc.geom_torsion.from_members(&residue);
        errors.extend(geom_torsion_errors);
        let (residue, valence_param_errors) = doc.valence_param.from_members(&residue);
        errors.extend(valence_param_errors);
        let (residue, valence_ref_errors) = doc.valence_ref.from_members(&residue);
        errors.extend(valence_ref_errors);
        // T5 wave: the 16 raw-diffraction categories of tier T5.
        let (residue, cell_measurement_refln_errors) =
            doc.cell_measurement_refln.from_members(&residue);
        errors.extend(cell_measurement_refln_errors);
        let (residue, diffrn_errors) = doc.diffrn.from_members(&residue);
        errors.extend(diffrn_errors);
        let (residue, diffrn_attenuator_errors) = doc.diffrn_attenuator.from_members(&residue);
        errors.extend(diffrn_attenuator_errors);
        let (residue, diffrn_detector_errors) = doc.diffrn_detector.from_members(&residue);
        errors.extend(diffrn_detector_errors);
        let (residue, diffrn_measurement_errors) = doc.diffrn_measurement.from_members(&residue);
        errors.extend(diffrn_measurement_errors);
        let (residue, diffrn_orient_matrix_errors) =
            doc.diffrn_orient_matrix.from_members(&residue);
        errors.extend(diffrn_orient_matrix_errors);
        let (residue, diffrn_orient_refln_errors) = doc.diffrn_orient_refln.from_members(&residue);
        errors.extend(diffrn_orient_refln_errors);
        let (residue, diffrn_radiation_errors) = doc.diffrn_radiation.from_members(&residue);
        errors.extend(diffrn_radiation_errors);
        let (residue, diffrn_radiation_wavelength_errors) =
            doc.diffrn_radiation_wavelength.from_members(&residue);
        errors.extend(diffrn_radiation_wavelength_errors);
        let (residue, diffrn_refln_errors) = doc.diffrn_refln.from_members(&residue);
        errors.extend(diffrn_refln_errors);
        let (residue, diffrn_reflns_errors) = doc.diffrn_reflns.from_members(&residue);
        errors.extend(diffrn_reflns_errors);
        let (residue, diffrn_reflns_rows_errors) = doc.diffrn_reflns_rows.from_members(&residue);
        errors.extend(diffrn_reflns_rows_errors);
        let (residue, diffrn_reflns_class_errors) = doc.diffrn_reflns_class.from_members(&residue);
        errors.extend(diffrn_reflns_class_errors);
        let (residue, diffrn_scale_group_errors) = doc.diffrn_scale_group.from_members(&residue);
        errors.extend(diffrn_scale_group_errors);
        let (residue, diffrn_source_errors) = doc.diffrn_source.from_members(&residue);
        errors.extend(diffrn_source_errors);
        let (residue, diffrn_standard_refln_errors) =
            doc.diffrn_standard_refln.from_members(&residue);
        errors.extend(diffrn_standard_refln_errors);
        let (residue, diffrn_standard_refln_rows_errors) =
            doc.diffrn_standard_refln_rows.from_members(&residue);
        errors.extend(diffrn_standard_refln_rows_errors);
        let (residue, diffrn_standards_errors) = doc.diffrn_standards.from_members(&residue);
        errors.extend(diffrn_standards_errors);
        // T4 wave: the tier T4 categories consume the residue in turn.
        let (residue, audit_errors) = doc.audit.from_members(&residue);
        errors.extend(audit_errors);
        let (residue, audit_author_errors) = doc.audit_author.from_members(&residue);
        errors.extend(audit_author_errors);
        let (residue, audit_author_rows_errors) = doc.audit_author_rows.from_members(&residue);
        errors.extend(audit_author_rows_errors);
        let (residue, audit_conform_errors) = doc.audit_conform.from_members(&residue);
        errors.extend(audit_conform_errors);
        let (residue, audit_conform_rows_errors) = doc.audit_conform_rows.from_members(&residue);
        errors.extend(audit_conform_rows_errors);
        let (residue, audit_contact_author_errors) =
            doc.audit_contact_author.from_members(&residue);
        errors.extend(audit_contact_author_errors);
        let (residue, audit_link_errors) = doc.audit_link.from_members(&residue);
        errors.extend(audit_link_errors);
        let (residue, citation_errors) = doc.citation.from_members(&residue);
        errors.extend(citation_errors);
        let (residue, citation_rows_errors) = doc.citation_rows.from_members(&residue);
        errors.extend(citation_rows_errors);
        let (residue, citation_author_errors) = doc.citation_author.from_members(&residue);
        errors.extend(citation_author_errors);
        let (residue, citation_editor_errors) = doc.citation_editor.from_members(&residue);
        errors.extend(citation_editor_errors);
        let (residue, citation_editor_rows_errors) =
            doc.citation_editor_rows.from_members(&residue);
        errors.extend(citation_editor_rows_errors);
        let (residue, computing_errors) = doc.computing.from_members(&residue);
        errors.extend(computing_errors);
        let (residue, computing_rows_errors) = doc.computing_rows.from_members(&residue);
        errors.extend(computing_rows_errors);
        let (residue, database_errors) = doc.database.from_members(&residue);
        errors.extend(database_errors);
        let (residue, database_rows_errors) = doc.database_rows.from_members(&residue);
        errors.extend(database_rows_errors);
        let (residue, journal_errors) = doc.journal.from_members(&residue);
        errors.extend(journal_errors);
        let (residue, journal_rows_errors) = doc.journal_rows.from_members(&residue);
        errors.extend(journal_rows_errors);
        let (residue, journal_index_errors) = doc.journal_index.from_members(&residue);
        errors.extend(journal_index_errors);
        let (residue, journal_index_rows_errors) = doc.journal_index_rows.from_members(&residue);
        errors.extend(journal_index_rows_errors);
        let (residue, publ_errors) = doc.publ.from_members(&residue);
        errors.extend(publ_errors);
        let (residue, publ_rows_errors) = doc.publ_rows.from_members(&residue);
        errors.extend(publ_rows_errors);
        let (residue, publ_author_errors) = doc.publ_author.from_members(&residue);
        errors.extend(publ_author_errors);
        let (residue, publ_author_rows_errors) = doc.publ_author_rows.from_members(&residue);
        errors.extend(publ_author_rows_errors);
        let (residue, publ_body_errors) = doc.publ_body.from_members(&residue);
        errors.extend(publ_body_errors);
        let (residue, publ_manuscript_incl_errors) =
            doc.publ_manuscript_incl.from_members(&residue);
        errors.extend(publ_manuscript_incl_errors);
        // T2 wave: the twelve categories of the T2 tier.
        let (residue, exptl_errors) = doc.exptl.from_members(&residue);
        errors.extend(exptl_errors);
        let (residue, exptl_crystal_errors) = doc.exptl_crystal.from_members(&residue);
        errors.extend(exptl_crystal_errors);
        let (residue, exptl_crystal_face_errors) = doc.exptl_crystal_face.from_members(&residue);
        errors.extend(exptl_crystal_face_errors);
        let (residue, refln_errors) = doc.refln.from_members(&residue);
        errors.extend(refln_errors);
        let (residue, reflns_errors) = doc.reflns.from_members(&residue);
        errors.extend(reflns_errors);
        let (residue, reflns_class_errors) = doc.reflns_class.from_members(&residue);
        errors.extend(reflns_class_errors);
        let (residue, reflns_scale_errors) = doc.reflns_scale.from_members(&residue);
        errors.extend(reflns_scale_errors);
        let (residue, reflns_shell_errors) = doc.reflns_shell.from_members(&residue);
        errors.extend(reflns_shell_errors);
        let (residue, refine_errors) = doc.refine.from_members(&residue);
        errors.extend(refine_errors);
        let (residue, refine_ls_class_errors) = doc.refine_ls_class.from_members(&residue);
        errors.extend(refine_ls_class_errors);
        let (residue, symmetry_errors) = doc.symmetry.from_members(&residue);
        errors.extend(symmetry_errors);
        let (residue, symmetry_equiv_errors) = doc.symmetry_equiv.from_members(&residue);
        errors.extend(symmetry_equiv_errors);
        doc.extra = residue;
        if errors.is_empty() {
            Ok(doc)
        } else {
            Err(errors)
        }
    }

    /// Write the view back: the typed categories in a fixed order, then
    /// the `extra` residue. The block heading is the one recorded at the
    /// read, so no argument is needed.
    pub fn to_block(&self) -> DataBlock {
        let mut members = Vec::new();
        members.extend(self.cell.to_members());
        members.extend(self.space_group.to_members());
        members.extend(self.space_group_symop.to_members());
        members.extend(self.atom_site.to_members());
        members.extend(self.chemical_formula.to_members());
        // T3 wave: the tier T3 category groups (roadmap D12j).
        members.extend(self.atom_sites.to_members());
        members.extend(self.atom_type.to_members());
        members.extend(self.chemical.to_members());
        members.extend(self.chemical_conn_atom.to_members());
        members.extend(self.chemical_conn_bond.to_members());
        members.extend(self.geom.to_members());
        members.extend(self.geom_angle.to_members());
        members.extend(self.geom_bond.to_members());
        members.extend(self.geom_contact.to_members());
        members.extend(self.geom_hbond.to_members());
        members.extend(self.geom_torsion.to_members());
        members.extend(self.valence_param.to_members());
        members.extend(self.valence_ref.to_members());
        // T5 wave: the 16 raw-diffraction categories of tier T5.
        members.extend(self.cell_measurement_refln.to_members());
        members.extend(self.diffrn.to_members());
        members.extend(self.diffrn_attenuator.to_members());
        members.extend(self.diffrn_detector.to_members());
        members.extend(self.diffrn_measurement.to_members());
        members.extend(self.diffrn_orient_matrix.to_members());
        members.extend(self.diffrn_orient_refln.to_members());
        members.extend(self.diffrn_radiation.to_members());
        members.extend(self.diffrn_radiation_wavelength.to_members());
        members.extend(self.diffrn_refln.to_members());
        members.extend(self.diffrn_reflns.to_members());
        members.extend(self.diffrn_reflns_rows.to_members());
        members.extend(self.diffrn_reflns_class.to_members());
        members.extend(self.diffrn_scale_group.to_members());
        members.extend(self.diffrn_source.to_members());
        members.extend(self.diffrn_standard_refln.to_members());
        members.extend(self.diffrn_standard_refln_rows.to_members());
        members.extend(self.diffrn_standards.to_members());
        // T4 wave: the tier T4 field blocks, then the extra residue.
        members.extend(self.audit.to_members());
        members.extend(self.audit_author.to_members());
        members.extend(self.audit_author_rows.to_members());
        members.extend(self.audit_conform.to_members());
        members.extend(self.audit_conform_rows.to_members());
        members.extend(self.audit_contact_author.to_members());
        members.extend(self.audit_link.to_members());
        members.extend(self.citation.to_members());
        members.extend(self.citation_rows.to_members());
        members.extend(self.citation_author.to_members());
        members.extend(self.citation_editor.to_members());
        members.extend(self.citation_editor_rows.to_members());
        members.extend(self.computing.to_members());
        members.extend(self.computing_rows.to_members());
        members.extend(self.database.to_members());
        members.extend(self.database_rows.to_members());
        members.extend(self.journal.to_members());
        members.extend(self.journal_rows.to_members());
        members.extend(self.journal_index.to_members());
        members.extend(self.journal_index_rows.to_members());
        members.extend(self.publ.to_members());
        members.extend(self.publ_rows.to_members());
        members.extend(self.publ_author.to_members());
        members.extend(self.publ_author_rows.to_members());
        members.extend(self.publ_body.to_members());
        members.extend(self.publ_manuscript_incl.to_members());
        // T2 wave: the twelve categories of the T2 tier.
        members.extend(self.exptl.to_members());
        members.extend(self.exptl_crystal.to_members());
        members.extend(self.exptl_crystal_face.to_members());
        members.extend(self.refln.to_members());
        members.extend(self.reflns.to_members());
        members.extend(self.reflns_class.to_members());
        members.extend(self.reflns_scale.to_members());
        members.extend(self.reflns_shell.to_members());
        members.extend(self.refine.to_members());
        members.extend(self.refine_ls_class.to_members());
        members.extend(self.symmetry.to_members());
        members.extend(self.symmetry_equiv.to_members());
        members.extend(self.extra.iter().cloned());
        DataBlock::from_heading_members((self.heading.clone(), members))
    }

    /// Write one view back to CIF text. This is the write step of the
    /// read-edit-write workflow for a single-block file.
    pub fn to_cif(&self) -> String {
        let document = CifDocument::new(None, Some(vec![self.to_block()]));
        document.to_string()
    }

    /// Check the wave-1 required sets. Reports every missing required
    /// item, not just the first.
    pub fn validate(&self) -> Result<(), Vec<CifError>> {
        let mut errors: Vec<CifError> = Vec::new();
        // `cell`: the direct six.
        for (item, present) in [
            ("length_a", self.cell.length_a.is_some()),
            ("length_b", self.cell.length_b.is_some()),
            ("length_c", self.cell.length_c.is_some()),
            ("angle_alpha", self.cell.angle_alpha.is_some()),
            ("angle_beta", self.cell.angle_beta.is_some()),
            ("angle_gamma", self.cell.angle_gamma.is_some()),
        ] {
            if !present {
                errors.push(CifError::Missing {
                    category: "cell",
                    item,
                });
            }
        }
        // `space_group`: one identifier (any of the three names).
        let identified = self.space_group.it_number.is_some()
            || self.space_group.name_hall.is_some()
            || self.space_group.symmetry_name_h_m.is_some();
        if !identified {
            errors.push(CifError::Missing {
                category: "space_group",
                item: "it_number, name_hall, or symmetry_name_h_m",
            });
        }
        // `chemical_formula`: the required spelling.
        if self.chemical_formula.structural.is_none() {
            errors.push(CifError::Missing {
                category: "chemical_formula",
                item: "structural",
            });
        }
        // `space_group_symop`: the required column per row.
        for row in &self.space_group_symop {
            if row.operation_xyz.is_none() {
                errors.push(CifError::Missing {
                    category: "space_group_symop",
                    item: "operation_xyz",
                });
            }
        }
        // `atom_site`: the required columns per row.
        for row in &self.atom_site {
            for (item, present) in [
                ("label", row.label.is_some()),
                ("type_symbol", row.type_symbol.is_some()),
                ("adp_type", row.adp_type.is_some()),
            ] {
                if !present {
                    errors.push(CifError::Missing {
                        category: "atom_site",
                        item,
                    });
                }
            }
            // Exactly one coordinate mode per row.
            let fract = row.fract_x.is_some() && row.fract_y.is_some() && row.fract_z.is_some();
            let cartn = row.cartn_x.is_some() && row.cartn_y.is_some() && row.cartn_z.is_some();
            if !fract && !cartn {
                errors.push(CifError::Missing {
                    category: "atom_site",
                    item: "fract_x/y/z or cartn_x/y/z",
                });
            }
            // The aniso columns, when the displacement type is aniso.
            let need_u = matches!(
                row.adp_type,
                Some(crate::data_dict::core_cif::atom_site::AdpType::Uani)
                    | Some(crate::data_dict::core_cif::atom_site::AdpType::Uovl)
            );
            let need_b = matches!(
                row.adp_type,
                Some(crate::data_dict::core_cif::atom_site::AdpType::Bani)
                    | Some(crate::data_dict::core_cif::atom_site::AdpType::Bovl)
            );
            if need_u
                && !(row.aniso_u_11.is_some()
                    && row.aniso_u_12.is_some()
                    && row.aniso_u_13.is_some()
                    && row.aniso_u_22.is_some()
                    && row.aniso_u_23.is_some()
                    && row.aniso_u_33.is_some())
            {
                errors.push(CifError::Missing {
                    category: "atom_site",
                    item: "aniso_U_11..33",
                });
            }
            if need_b
                && !(row.aniso_b_11.is_some()
                    && row.aniso_b_12.is_some()
                    && row.aniso_b_13.is_some()
                    && row.aniso_b_22.is_some()
                    && row.aniso_b_23.is_some()
                    && row.aniso_b_33.is_some())
            {
                errors.push(CifError::Missing {
                    category: "atom_site",
                    item: "aniso_B_11..33",
                });
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// ---------------------------------------------------------------------------
// The typed parse entry point (the read workflow). Call it with the file
// text; it returns one `CoreCif` per data block. The IR document and the
// per-block `from_block` step stay inside this function.
// ---------------------------------------------------------------------------

/// A data block that failed to type, with every conformance error of it.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("block {block}: {n} conformance error(s)", n = errors.len())]
pub struct BlockTypeFailure {
    /// The name of the data block that failed to type.
    pub block: String,
    /// Every conformance error collected from that block.
    pub errors: Vec<CifError>,
}

/// A failure of the typed parse.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CoreCifParseError {
    /// The file does not parse. Each string is one rendered syntax report.
    #[error("the file does not parse: {n} syntax report(s)", n = reports.len())]
    Syntax { reports: Vec<String> },
    /// One or more data blocks failed to type. Every failure is carried.
    #[error("{n} data block(s) failed to type", n = failures.len())]
    Typed { failures: Vec<BlockTypeFailure> },
}

/// The typed document: one `CoreCif` per data block, plus the
/// document-level metadata. The read entry point returns this, not a
/// bare `Vec`.
///
/// The `dialect` and the leading `comments` record what the file said
/// about itself. The `blocks` carry the typed categories in file
/// order.
#[derive(Debug, Clone)]
pub struct CoreCifDocument {
    /// The value-rule dialect detected from the file head.
    pub dialect: CifDialect,
    /// The leading comment lines, when the file carries them.
    pub comments: Option<Comments>,
    /// The typed data blocks, in file order.
    pub blocks: Vec<CoreCif>,
}

impl CoreCifDocument {
    /// The number of typed data blocks.
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Whether the document carries no data blocks.
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Look up a block by its heading name.
    pub fn block(&self, name: &str) -> Option<&CoreCif> {
        self.blocks.iter().find(|block| block.heading.name() == name)
    }

    /// Look up a block mutably by its heading name. The borrow of the
    /// document lives as long as the returned reference.
    pub fn block_mut(&mut self, name: &str) -> Option<&mut CoreCif> {
        self.blocks.iter_mut().find(|block| block.heading.name() == name)
    }

    /// Write the document back to CIF text. The blocks keep their
    /// recorded headings and the document keeps its comments. This is
    /// the write step of the read-edit-write workflow.
    pub fn to_cif(&self) -> String {
        let document = CifDocument::new(
            self.comments.clone(),
            Some(self.blocks.iter().map(|block| block.to_block()).collect()),
        );
        document.to_string()
    }

    /// The IR document. The direct-control escape hatch.
    pub fn to_document(&self) -> CifDocument {
        CifDocument::new(
            self.comments.clone(),
            Some(self.blocks.iter().map(|block| block.to_block()).collect()),
        )
    }
}

impl std::fmt::Display for CoreCifDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_cif())
    }
}

/// The typed parse entry point. Parses the file text and returns the
/// typed document: one `CoreCif` per data block, plus the dialect and
/// the leading comments. The IR document and the per-block typing stay
/// inside this call.
///
/// # Errors
///
/// `CoreCifParseError::Syntax` when the file does not parse. The reports
/// render each syntax error with the source snippet. `CoreCifParseError::Typed`
/// when one or more data blocks fail to type. Every failure is carried,
/// never the first.
pub fn parse_core_cif(source: &str) -> Result<CoreCifDocument, CoreCifParseError> {
    let dialect = CifDialect::detect(source);
    let document = match parse_cif(source) {
        Ok(document) => document,
        Err(errors) => {
            let reports = errors
                .iter()
                .map(|error| syntax_error_report(error, "<cif>", source))
                .collect();
            return Err(CoreCifParseError::Syntax { reports });
        }
    };
    let mut blocks: Vec<CoreCif> = Vec::new();
    let mut failures: Vec<BlockTypeFailure> = Vec::new();
    if let Some(raw_blocks) = document.data_blocks() {
        for block in raw_blocks {
            match CoreCif::from_block(block) {
                Ok(block_view) => blocks.push(block_view),
                Err(errors) => failures.push(BlockTypeFailure {
                    block: block.heading().to_string(),
                    errors,
                }),
            }
        }
    }
    if failures.is_empty() {
        Ok(CoreCifDocument {
            dialect,
            comments: document.comments().clone(),
            blocks,
        })
    } else {
        Err(CoreCifParseError::Typed { failures })
    }
}

/// Write one or more typed views back to CIF text. The blocks keep their
/// recorded headings. This is the write step of the read-edit-write
/// workflow.
pub fn write_core_cif(blocks: &[CoreCif]) -> String {
    let document = CifDocument::new(
        None,
        Some(blocks.iter().map(|block| block.to_block()).collect()),
    );
    document.to_string()
}
