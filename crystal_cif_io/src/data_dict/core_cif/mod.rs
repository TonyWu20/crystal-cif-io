pub mod atom_analytical;
pub mod atom_analytical_mass_loss;
pub mod atom_analytical_source;
pub mod atom_scat_versus_stol;
pub mod atom_site;
pub mod atom_site_aniso;
pub mod atom_sites;
pub mod atom_sites_cartn_transform;
pub mod atom_sites_fract_transform;
pub mod atom_type;
pub mod atom_type_scat;
pub mod audit;
pub mod audit_author;
pub mod audit_author_role;
pub mod audit_conform;
pub mod audit_contact_author;
pub mod audit_link;
pub mod audit_support;
pub mod cell;
pub mod cell_measurement_refln;
pub mod chemical;
pub mod chemical_conn_atom;
pub mod chemical_conn_bond;
pub mod chemical_formula;
pub mod citation;
pub mod citation_author;
pub mod citation_editor;
pub mod computing;
pub mod database;
pub mod database_code;
pub mod database_related;
pub mod diffrn;
pub mod diffrn_attenuator;
pub mod diffrn_component_pid;
pub mod diffrn_detector;
pub mod diffrn_measurement;
pub mod diffrn_orient_matrix;
pub mod diffrn_orient_refln;
pub mod diffrn_radiation;
pub mod diffrn_radiation_wavelength;
pub mod diffrn_refln;
pub mod diffrn_reflns;
pub mod diffrn_reflns_class;
pub mod diffrn_reflns_transf_matrix;
pub mod diffrn_scale_group;
pub mod diffrn_source;
pub mod diffrn_standard_refln;
pub mod diffrn_standards;
pub mod display_colour;
pub mod exptl;
pub mod exptl_absorpt;
pub mod exptl_crystal;
pub mod exptl_crystal_face;
pub mod function;
pub mod geom;
pub mod geom_angle;
pub mod geom_bond;
pub mod geom_contact;
pub mod geom_hbond;
pub mod geom_torsion;
pub mod journal;
pub mod journal_index;
pub mod model_site;
pub mod publ;
pub mod publ_author;
pub mod publ_body;
pub mod publ_contact_author;
pub mod publ_manuscript_incl;
pub mod range_check;
pub mod refine;
pub mod refine_diff;
pub mod refine_ls;
pub mod refine_ls_class;
pub mod refln;
pub mod reflns;
pub mod reflns_class;
pub mod reflns_scale;
pub mod reflns_shell;
pub mod space_group;
pub mod space_group_generator;
pub mod space_group_symop;
pub mod space_group_wyckoff;
pub mod symmetry;
pub mod symmetry_equiv;
pub mod valence_param;
pub mod valence_ref;

// ---------------------------------------------------------------------------
// The typed document layer (roadmap D12).
//
// The `CifCategory` trait is the read/write contract of one dictionary
// category. A category struct fills its `Option` fields from a slice of
// block members and emits the set members back. `from_members` consumes
// only the tags of its own category and returns the unconsumed residue
// (foreign tags, save frames) together with every value-class or shape
// mismatch it finds. It never stops at the first mismatch.
// ---------------------------------------------------------------------------

pub mod convert;

use crystal_cif_fmt::DataBlockMember;
use crate::core_cif::CifError;

/// One dictionary category of the typed document layer.
///
/// Implemented on the single-value category structs (`Cell`,
/// `SpaceGroup`, `ChemicalFormula`) and on the loop-row groups
/// (`Vec<Symop>`, `Vec<AtomSite>`). A category fills its fields from
/// the members it owns and emits the set members.
pub trait CifCategory: Default + Sized {
    /// Emit the set members. A `None` field emits nothing.
    fn to_members(&self) -> Vec<DataBlockMember>;

    /// Fill the fields from the members of this category.
    ///
    /// Consumes only the category's own tags. Returns the unconsumed
    /// residue (foreign tags, save frames) and every value-class or
    /// shape mismatch found. The residue keeps the input order.
    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>);
}
