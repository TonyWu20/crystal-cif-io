//! The `atom_site` category of the typed document layer (roadmap D12).
//!
//! The `atom_site` loop-row struct. One `Option` field per registry
//! loop column. The closed-set items keep the typed enum. `CifCategory`
//! is on the `Vec<AtomSite>` group: it emits one `MultiValues` loop
//! and fills the rows from the loop columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use {
    crate::data_dict::core_cif::atom_site::AdpRefineFlag,
    crate::data_dict::core_cif::atom_site::AdpType,
    crate::data_dict::core_cif::atom_site::CalcFlag,
    crate::data_dict::core_cif::atom_site::OccupancyRefineFlag,
    crate::data_dict::core_cif::atom_site::PosnRefineFlag,
    crate::data_dict::core_cif::atom_site::RefinementFlag,
    crate::data_dict::core_cif::atom_site::SymMultiplicity,
};

#[derive(Debug, Clone, Default)]
/// The `atom_site` category struct. One field per registry item.
pub struct AtomSite {
    /// The `_atom_site_adp_type` item. The field holds the column values, one per loop row.
    ///
    /// A standard code used to describe the type of atomic displacement
    /// parameters used for the site.
    pub adp_type: Option<AdpType>,
    /// The `_atom_site_aniso_B_11` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-(1/4) sum~i~ [ sum~j~ (B^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row. The IUCr
    /// Commission on Nomenclature recommends against the use of B for
    /// reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub aniso_b_11: Option<Numeric>,
    /// The `_atom_site_aniso_B_12` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-(1/4) sum~i~ [ sum~j~ (B^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row. The IUCr
    /// Commission on Nomenclature recommends against the use of B for
    /// reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub aniso_b_12: Option<Numeric>,
    /// The `_atom_site_aniso_B_13` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-(1/4) sum~i~ [ sum~j~ (B^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row. The IUCr
    /// Commission on Nomenclature recommends against the use of B for
    /// reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub aniso_b_13: Option<Numeric>,
    /// The `_atom_site_aniso_B_22` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-(1/4) sum~i~ [ sum~j~ (B^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row. The IUCr
    /// Commission on Nomenclature recommends against the use of B for
    /// reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub aniso_b_22: Option<Numeric>,
    /// The `_atom_site_aniso_B_23` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-(1/4) sum~i~ [ sum~j~ (B^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row. The IUCr
    /// Commission on Nomenclature recommends against the use of B for
    /// reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub aniso_b_23: Option<Numeric>,
    /// The `_atom_site_aniso_B_33` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-(1/4) sum~i~ [ sum~j~ (B^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row. The IUCr
    /// Commission on Nomenclature recommends against the use of B for
    /// reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub aniso_b_33: Option<Numeric>,
    /// The `_atom_site_aniso_label` item. The field holds the column values, one per loop row.
    ///
    /// Anisotropic atomic displacement parameters are usually looped in a
    /// separate list. If this is the case, this code must match the
    /// _atom_site_label of the associated atom in the atom coordinate list
    /// and conform with the same rules described in _atom_site_label.
    pub aniso_label: Option<CharString>,
    /// The `_atom_site_aniso_ratio` item. The field holds the column values, one per loop row.
    ///
    /// Ratio of the maximum to minimum principal axes of displacement
    /// (thermal) ellipsoids.
    pub aniso_ratio: Option<Numeric>,
    /// The `_atom_site_aniso_type_symbol` item. The field holds the column values, one per loop row.
    ///
    /// This _atom_type_symbol code links the anisotropic atom parameters to
    /// the atom-type data associated with this site and must match one of
    /// the _atom_type_symbol codes in this list.
    pub aniso_type_symbol: Option<CharString>,
    /// The `_atom_site_aniso_U_11` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    pub aniso_u_11: Option<Numeric>,
    /// The `_atom_site_aniso_U_12` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    pub aniso_u_12: Option<Numeric>,
    /// The `_atom_site_aniso_U_13` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    pub aniso_u_13: Option<Numeric>,
    /// The `_atom_site_aniso_U_22` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    pub aniso_u_22: Option<Numeric>,
    /// The `_atom_site_aniso_U_23` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    pub aniso_u_23: Option<Numeric>,
    /// The `_atom_site_aniso_U_33` item. The field holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    pub aniso_u_33: Option<Numeric>,
    /// The `_atom_site_attached_hydrogens` item. The field holds the column values, one per loop row.
    ///
    /// The number of hydrogen atoms attached to the atom at this site
    /// excluding any hydrogen atoms for which coordinates (measured or
    /// calculated) are given.
    pub attached_hydrogens: Option<Numeric>,
    /// The `_atom_site_B_equiv_geom_mean` item. The field holds the column values, one per loop row.
    ///
    /// Equivalent isotropic atomic displacement parameter, B(equiv), in
    /// angstroms squared, calculated as the geometric mean of the
    /// anisotropic atomic displacement parameters. B(equiv) = (B~i~ B~j~
    /// B~k~)^1/3^ B~n~ = the principal components of the orthogonalized
    /// B^ij^ The IUCr Commission on Nomenclature recommends against the use
    /// of B for reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub b_equiv_geom_mean: Option<Numeric>,
    /// The `_atom_site_B_iso_or_equiv` item. The field holds the column values, one per loop row.
    ///
    /// Isotropic atomic displacement parameter, or equivalent isotropic
    /// atomic displacement parameter, B(equiv), in angstroms squared,
    /// calculated from anisotropic displacement components. B(equiv) =
    /// (1/3) sum~i~[sum~j~(B^ij^ a*~i~ a*~j~ a~i~ a~j~)] a = the real-space
    /// cell lengths a* = the reciprocal-space cell lengths B^ij^ = 8 pi^2^
    /// U^ij^ Ref: Fischer, R. X. & Tillmanns, E. (1988). Acta Cryst. C44,
    /// 775-776. The IUCr Commission on Nomenclature recommends against the
    /// use of B for reporting atomic displacement parameters. U, being
    /// directly proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    pub b_iso_or_equiv: Option<Numeric>,
    /// The `_atom_site_calc_attached_atom` item. The field holds the column values, one per loop row.
    ///
    /// The _atom_site_label of the atom site to which the 'geometry-
    /// calculated' atom site is attached.
    pub calc_attached_atom: Option<CharString>,
    /// The `_atom_site_calc_flag` item. The field holds the column values, one per loop row.
    ///
    /// A standard code to signal whether the site coordinates have been
    /// determined from the intensities or calculated from the geometry of
    /// surrounding sites, or have been assigned dummy coordinates. The
    /// abbreviation 'c' may be used in place of 'calc'.
    pub calc_flag: Option<CalcFlag>,
    /// The `_atom_site_Cartn_x` item. The field holds the column values, one per loop row.
    ///
    /// The atom-site coordinates in angstroms specified according to a set
    /// of orthogonal Cartesian axes related to the cell axes as specified
    /// by the _atom_sites_Cartn_transform_axes description.
    /// Units: A (angstrom).
    pub cartn_x: Option<Numeric>,
    /// The `_atom_site_Cartn_y` item. The field holds the column values, one per loop row.
    ///
    /// The atom-site coordinates in angstroms specified according to a set
    /// of orthogonal Cartesian axes related to the cell axes as specified
    /// by the _atom_sites_Cartn_transform_axes description.
    /// Units: A (angstrom).
    pub cartn_y: Option<Numeric>,
    /// The `_atom_site_Cartn_z` item. The field holds the column values, one per loop row.
    ///
    /// The atom-site coordinates in angstroms specified according to a set
    /// of orthogonal Cartesian axes related to the cell axes as specified
    /// by the _atom_sites_Cartn_transform_axes description.
    /// Units: A (angstrom).
    pub cartn_z: Option<Numeric>,
    /// The `_atom_site_chemical_conn_number` item. The field holds the column values, one per loop row.
    ///
    /// This number links an atom site to the chemical connectivity list. It
    /// must match a number specified by _chemical_conn_atom_number.
    pub chemical_conn_number: Option<Numeric>,
    /// The `_atom_site_constraints` item. The field holds the column values, one per loop row.
    ///
    /// A description of the constraints applied to parameters at this site
    /// during refinement. See also _atom_site_refinement_flags and
    /// _refine_ls_number_constraints.
    pub constraints: Option<CharString>,
    /// The `_atom_site_description` item. The field holds the column values, one per loop row.
    ///
    /// A description of special aspects of this site. See also
    /// _atom_site_refinement_flags.
    pub description: Option<CharString>,
    /// The `_atom_site_disorder_assembly` item. The field holds the column values, one per loop row.
    ///
    /// A code which identifies a cluster of atoms that show long-range
    /// positional disorder but are locally ordered. Within each such
    /// cluster of atoms, _atom_site_disorder_group is used to identify the
    /// sites that are simultaneously occupied. This field is only needed if
    /// there is more than one cluster of disordered atoms showing
    /// independent local order.
    pub disorder_assembly: Option<CharString>,
    /// The `_atom_site_disorder_group` item. The field holds the column values, one per loop row.
    ///
    /// A code which identifies a group of positionally disordered atom
    /// sites that are locally simultaneously occupied. Atoms that are
    /// positionally disordered over two or more sites (e.g. the hydrogen
    /// atoms of a methyl group that exists in two orientations) can be
    /// assigned to two or more groups. Sites belonging to the same group
    /// are simultaneously occupied, but those belonging to different groups
    /// are not. A minus prefix (e.g. "-1") is used to indicate sites
    /// disordered about a special position.
    pub disorder_group: Option<CharString>,
    /// The `_atom_site_fract_x` item. The field holds the column values, one per loop row.
    ///
    /// Atom-site coordinates as fractions of the _cell_length_ values.
    pub fract_x: Option<Numeric>,
    /// The `_atom_site_fract_y` item. The field holds the column values, one per loop row.
    ///
    /// Atom-site coordinates as fractions of the _cell_length_ values.
    pub fract_y: Option<Numeric>,
    /// The `_atom_site_fract_z` item. The field holds the column values, one per loop row.
    ///
    /// Atom-site coordinates as fractions of the _cell_length_ values.
    pub fract_z: Option<Numeric>,
    /// The `_atom_site_label` item. The field holds the column values, one per loop row.
    ///
    /// The _atom_site_label is a unique identifier for a particular site in
    /// the crystal. This code is made up of a sequence of up to seven
    /// components, _atom_site_label_component_0 to *_6, which may be
    /// specified as separate data items. Component 0 usually matches one of
    /// the specified _atom_type_symbol codes. This is not mandatory if an
    /// _atom_site_type_symbol item is included in the atom-site list. The
    /// _atom_site_type_symbol always takes precedence over an
    /// _atom_site_label in the identification of the atom type. The label
    /// components 1 to 6 are optional, and normally only components 0 and 1
    /// are used. Note that components 0 and 1 are concatenated, while all
    /// other components, if specified, are separated by an underscore.
    /// Underscores are only used if higher-order components exist. If an
    /// intermediate component is not used, it may be omitted provided the
    /// underscore separators are inserted. For example, the label
    /// 'C233__ggg' is acceptable and represents the components C, 233, ''
    /// and ggg. Different labels may have a different number of components.
    pub label: Option<CharString>,
    /// The `_atom_site_label_component_0` item. The field holds the column values, one per loop row.
    ///
    /// Component 0 is normally a code which matches identically with one of
    /// the _atom_type_symbol codes. If this is the case, then the rules
    /// governing the _atom_type_symbol code apply. If, however, the data
    /// item _atom_site_type_symbol is also specified in the atom-site list,
    /// component 0 need not match this symbol or adhere to any of the
    /// _atom_type_symbol rules. Component 1 is referred to as the "atom
    /// number". When component 0 is the atom-type code, it is used to
    /// number the sites with the same atom type. This component code must
    /// start with at least one digit which is not followed by a + or - sign
    /// (to distinguish it from the component 0 rules). Components 2 to 6
    /// contain the identifier, residue, sequence, asymmetry identifier and
    /// alternate codes, respectively. These codes may be composed of any
    /// characters except an underscore.
    pub label_component_0: Option<CharString>,
    /// The `_atom_site_label_component_1` item. The field holds the column values, one per loop row.
    ///
    /// Component 0 is normally a code which matches identically with one of
    /// the _atom_type_symbol codes. If this is the case, then the rules
    /// governing the _atom_type_symbol code apply. If, however, the data
    /// item _atom_site_type_symbol is also specified in the atom-site list,
    /// component 0 need not match this symbol or adhere to any of the
    /// _atom_type_symbol rules. Component 1 is referred to as the "atom
    /// number". When component 0 is the atom-type code, it is used to
    /// number the sites with the same atom type. This component code must
    /// start with at least one digit which is not followed by a + or - sign
    /// (to distinguish it from the component 0 rules). Components 2 to 6
    /// contain the identifier, residue, sequence, asymmetry identifier and
    /// alternate codes, respectively. These codes may be composed of any
    /// characters except an underscore.
    pub label_component_1: Option<CharString>,
    /// The `_atom_site_label_component_2` item. The field holds the column values, one per loop row.
    ///
    /// Component 0 is normally a code which matches identically with one of
    /// the _atom_type_symbol codes. If this is the case, then the rules
    /// governing the _atom_type_symbol code apply. If, however, the data
    /// item _atom_site_type_symbol is also specified in the atom-site list,
    /// component 0 need not match this symbol or adhere to any of the
    /// _atom_type_symbol rules. Component 1 is referred to as the "atom
    /// number". When component 0 is the atom-type code, it is used to
    /// number the sites with the same atom type. This component code must
    /// start with at least one digit which is not followed by a + or - sign
    /// (to distinguish it from the component 0 rules). Components 2 to 6
    /// contain the identifier, residue, sequence, asymmetry identifier and
    /// alternate codes, respectively. These codes may be composed of any
    /// characters except an underscore.
    pub label_component_2: Option<CharString>,
    /// The `_atom_site_label_component_3` item. The field holds the column values, one per loop row.
    ///
    /// Component 0 is normally a code which matches identically with one of
    /// the _atom_type_symbol codes. If this is the case, then the rules
    /// governing the _atom_type_symbol code apply. If, however, the data
    /// item _atom_site_type_symbol is also specified in the atom-site list,
    /// component 0 need not match this symbol or adhere to any of the
    /// _atom_type_symbol rules. Component 1 is referred to as the "atom
    /// number". When component 0 is the atom-type code, it is used to
    /// number the sites with the same atom type. This component code must
    /// start with at least one digit which is not followed by a + or - sign
    /// (to distinguish it from the component 0 rules). Components 2 to 6
    /// contain the identifier, residue, sequence, asymmetry identifier and
    /// alternate codes, respectively. These codes may be composed of any
    /// characters except an underscore.
    pub label_component_3: Option<CharString>,
    /// The `_atom_site_label_component_4` item. The field holds the column values, one per loop row.
    ///
    /// Component 0 is normally a code which matches identically with one of
    /// the _atom_type_symbol codes. If this is the case, then the rules
    /// governing the _atom_type_symbol code apply. If, however, the data
    /// item _atom_site_type_symbol is also specified in the atom-site list,
    /// component 0 need not match this symbol or adhere to any of the
    /// _atom_type_symbol rules. Component 1 is referred to as the "atom
    /// number". When component 0 is the atom-type code, it is used to
    /// number the sites with the same atom type. This component code must
    /// start with at least one digit which is not followed by a + or - sign
    /// (to distinguish it from the component 0 rules). Components 2 to 6
    /// contain the identifier, residue, sequence, asymmetry identifier and
    /// alternate codes, respectively. These codes may be composed of any
    /// characters except an underscore.
    pub label_component_4: Option<CharString>,
    /// The `_atom_site_label_component_5` item. The field holds the column values, one per loop row.
    ///
    /// Component 0 is normally a code which matches identically with one of
    /// the _atom_type_symbol codes. If this is the case, then the rules
    /// governing the _atom_type_symbol code apply. If, however, the data
    /// item _atom_site_type_symbol is also specified in the atom-site list,
    /// component 0 need not match this symbol or adhere to any of the
    /// _atom_type_symbol rules. Component 1 is referred to as the "atom
    /// number". When component 0 is the atom-type code, it is used to
    /// number the sites with the same atom type. This component code must
    /// start with at least one digit which is not followed by a + or - sign
    /// (to distinguish it from the component 0 rules). Components 2 to 6
    /// contain the identifier, residue, sequence, asymmetry identifier and
    /// alternate codes, respectively. These codes may be composed of any
    /// characters except an underscore.
    pub label_component_5: Option<CharString>,
    /// The `_atom_site_label_component_6` item. The field holds the column values, one per loop row.
    ///
    /// Component 0 is normally a code which matches identically with one of
    /// the _atom_type_symbol codes. If this is the case, then the rules
    /// governing the _atom_type_symbol code apply. If, however, the data
    /// item _atom_site_type_symbol is also specified in the atom-site list,
    /// component 0 need not match this symbol or adhere to any of the
    /// _atom_type_symbol rules. Component 1 is referred to as the "atom
    /// number". When component 0 is the atom-type code, it is used to
    /// number the sites with the same atom type. This component code must
    /// start with at least one digit which is not followed by a + or - sign
    /// (to distinguish it from the component 0 rules). Components 2 to 6
    /// contain the identifier, residue, sequence, asymmetry identifier and
    /// alternate codes, respectively. These codes may be composed of any
    /// characters except an underscore.
    pub label_component_6: Option<CharString>,
    /// The `_atom_site_occupancy` item. The field holds the column values, one per loop row.
    ///
    /// The fraction of the atom type present at this site. The sum of the
    /// occupancies of all the atom types at this site may not significantly
    /// exceed 1.0 unless it is a dummy site. The value must lie in the
    /// 99.97% Gaussian confidence interval -3u =&lt; x =&lt; 1 + 3u. The
    /// _enumeration_range of 0.0:1.0 is thus correctly interpreted as
    /// meaning (0.0 - 3u) =&lt; x =&lt; (1.0 + 3u).
    pub occupancy: Option<Numeric>,
    /// The `_atom_site_refinement_flags` item. The field holds the column values, one per loop row.
    ///
    /// A concatenated series of single-letter codes which indicate the
    /// refinement restraints or constraints applied to this site. This item
    /// should not be used. It has been replaced by
    /// _atom_site_refinement_flags_posn, *_adp and *_occupancy. It is
    /// retained in this dictionary only to provide compatibility with
    /// legacy CIFs.
    pub refinement_flags: Option<RefinementFlag>,
    /// The `_atom_site_refinement_flags_adp` item. The field holds the column values, one per loop row.
    ///
    /// A code which indicates the refinement restraints or constraints
    /// applied to the atomic displacement parameters of this site.
    pub refinement_flags_adp: Option<AdpRefineFlag>,
    /// The `_atom_site_refinement_flags_occupancy` item. The field holds the column values, one per loop row.
    ///
    /// A code which indicates that refinement restraints or constraints
    /// were applied to the occupancy of this site.
    pub refinement_flags_occupancy: Option<OccupancyRefineFlag>,
    /// The `_atom_site_refinement_flags_posn` item. The field holds the column values, one per loop row.
    ///
    /// A code which indicates the refinement restraints or constraints
    /// applied to the positional coordinates of this site.
    pub refinement_flags_posn: Option<PosnRefineFlag>,
    /// The `_atom_site_restraints` item. The field holds the column values, one per loop row.
    ///
    /// A description of restraints applied to specific parameters at this
    /// site during refinement. See also _atom_site_refinement_flags and
    /// _refine_ls_number_restraints.
    pub restraints: Option<CharString>,
    /// The `_atom_site_site_symmetry_multiplicity` item. The field holds the column values, one per loop row.
    ///
    /// The number of different sites that are generated by the application
    /// of the space-group symmetry to the coordinates given for this site.
    /// It is equal to the multiplicity given for this Wyckoff site in
    /// International Tables for Crystallography Vol. A (2002). It is equal
    /// to the multiplicity of the general position divided by the order of
    /// the site symmetry given in _atom_site_site_symmetry_order.
    pub site_symmetry_multiplicity: Option<SymMultiplicity>,
    /// The `_atom_site_site_symmetry_order` item. The field holds the column values, one per loop row.
    ///
    /// The order of the site symmetry of the site identified by
    /// _atom_site_label. This is the number of times application of the
    /// crystallographic symmetry to the coordinates given for this site
    /// generates the same set of coordinates. It is equal to: multiplicity
    /// of the general position ------------------------------------
    /// multiplicity of this site where 'multiplicity of this site' is given
    /// in _atom_site_site_symmetry_multiplicity.
    pub site_symmetry_order: Option<Numeric>,
    /// The `_atom_site_symmetry_multiplicity` item. The field holds the column values, one per loop row.
    ///
    /// The multiplicity of a site due to the space-group symmetry as given
    /// in International Tables for Crystallography Vol. A (2002). Use of
    /// this data name is deprecated because of inconsistencies in practice
    /// among structure refinement software packages. The number of
    /// positions given for this Wyckoff site in International Tables for
    /// Crystallography Vol. A (2002). should now be expressed using the
    /// data name _atom_site_site_symmetry_multiplicity. In the historic
    /// archive some CIFs use this item to give values that belong in
    /// _atom_site_site_symmetry_order.
    pub symmetry_multiplicity: Option<SymMultiplicity>,
    /// The `_atom_site_thermal_displace_type` item. The field holds the column values, one per loop row.
    ///
    /// A standard code used to describe the type of atomic displacement
    /// parameters used for the site.
    pub thermal_displace_type: Option<AdpType>,
    /// The `_atom_site_type_symbol` item. The field holds the column values, one per loop row.
    ///
    /// A code to identify the atom species (singular or plural) occupying
    /// this site. This code must match a corresponding _atom_type_symbol.
    /// The specification of this code is optional if component 0 of the
    /// _atom_site_label is used for this purpose. See _atom_type_symbol.
    pub type_symbol: Option<CharString>,
    /// The `_atom_site_U_equiv_geom_mean` item. The field holds the column values, one per loop row.
    ///
    /// Equivalent isotropic atomic displacement parameter, U(equiv), in
    /// angstroms squared, calculated as the geometric mean of the
    /// anisotropic atomic displacement parameters. U(equiv) = (U~i~ U~j~
    /// U~k~)^1/3^ U~n~ = the principal components of the orthogonalized
    /// U^ij^
    /// Units: A^2^ (angstrom squared).
    pub u_equiv_geom_mean: Option<Numeric>,
    /// The `_atom_site_U_iso_or_equiv` item. The field holds the column values, one per loop row.
    ///
    /// Isotropic atomic displacement parameter, or equivalent isotropic
    /// atomic displacement parameter, U(equiv), in angstroms squared,
    /// calculated from anisotropic atomic displacement parameters. U(equiv)
    /// = (1/3) sum~i~[sum~j~(U^ij^ a*~i~ a*~j~ a~i~ a~j~)] a = the
    /// real-space cell lengths a* = the reciprocal-space cell lengths Ref:
    /// Fischer, R. X. & Tillmanns, E. (1988). Acta Cryst. C44, 775-776.
    /// Units: A^2^ (angstrom squared).
    pub u_iso_or_equiv: Option<Numeric>,
    /// The `_atom_site_Wyckoff_symbol` item. The field holds the column values, one per loop row.
    ///
    /// The Wyckoff symbol (letter) as listed in the space-group tables of
    /// International Tables for Crystallography Vol. A (2002).
    pub wyckoff_symbol: Option<CharString>,
}

impl CifCategory for Vec<AtomSite> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.adp_type.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.adp_type {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_adp_type".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_b_11.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_b_11 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_B_11".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_b_12.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_b_12 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_B_12".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_b_13.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_b_13 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_B_13".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_b_22.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_b_22 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_B_22".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_b_23.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_b_23 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_B_23".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_b_33.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_b_33 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_B_33".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_label.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_label {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_label".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_ratio.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_ratio {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_ratio".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_type_symbol.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_type_symbol {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_type_symbol".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_u_11.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_u_11 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_U_11".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_u_12.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_u_12 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_U_12".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_u_13.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_u_13 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_U_13".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_u_22.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_u_22 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_U_22".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_u_23.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_u_23 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_U_23".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.aniso_u_33.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.aniso_u_33 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_aniso_U_33".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.attached_hydrogens.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.attached_hydrogens {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_attached_hydrogens".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.b_equiv_geom_mean.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.b_equiv_geom_mean {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_b_equiv_geom_mean".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.b_iso_or_equiv.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.b_iso_or_equiv {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_b_iso_or_equiv".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.calc_attached_atom.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.calc_attached_atom {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_calc_attached_atom".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.calc_flag.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.calc_flag {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_calc_flag".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.cartn_x.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.cartn_x {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_cartn_x".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.cartn_y.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.cartn_y {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_cartn_y".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.cartn_z.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.cartn_z {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_cartn_z".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.chemical_conn_number.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.chemical_conn_number {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_chemical_conn_number".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.constraints.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.constraints {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_constraints".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.description.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.description {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_description".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.disorder_assembly.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.disorder_assembly {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_disorder_assembly".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.disorder_group.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.disorder_group {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_disorder_group".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.fract_x.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.fract_x {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_fract_x".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.fract_y.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.fract_y {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_fract_y".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.fract_z.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.fract_z {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_fract_z".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label_component_0.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label_component_0 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label_component_0".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label_component_1.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label_component_1 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label_component_1".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label_component_2.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label_component_2 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label_component_2".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label_component_3.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label_component_3 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label_component_3".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label_component_4.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label_component_4 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label_component_4".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label_component_5.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label_component_5 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label_component_5".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label_component_6.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label_component_6 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_label_component_6".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.occupancy.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.occupancy {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_occupancy".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.refinement_flags.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.refinement_flags {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_refinement_flags".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.refinement_flags_adp.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.refinement_flags_adp {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_refinement_flags_adp".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.refinement_flags_occupancy.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.refinement_flags_occupancy {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_refinement_flags_occupancy".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.refinement_flags_posn.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.refinement_flags_posn {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_refinement_flags_posn".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.restraints.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.restraints {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_restraints".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.site_symmetry_multiplicity.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.site_symmetry_multiplicity {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_site_symmetry_multiplicity".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.site_symmetry_order.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.site_symmetry_order {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_site_symmetry_order".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.symmetry_multiplicity.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.symmetry_multiplicity {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_symmetry_multiplicity".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.thermal_displace_type.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.thermal_displace_type {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_thermal_displace_type".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.type_symbol.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.type_symbol {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_type_symbol".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.u_equiv_geom_mean.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.u_equiv_geom_mean {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_u_equiv_geom_mean".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.u_iso_or_equiv.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.u_iso_or_equiv {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_u_iso_or_equiv".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.wyckoff_symbol.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wyckoff_symbol {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("atom_site_wyckoff_symbol".to_string()),
                values,
            ));
        }
        if columns.is_empty() {
            return Vec::new();
        }
        vec![DataBlockMember::DataItems(DataItems::MultiValues(
            LoopColumns::new(columns),
        ))]
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries a required
                    // identity tag. A sibling loop that carries other items of
                    // this category (for example the atom_site_aniso loop) stays
                    // in the residue instead of overwriting the primary rows.
                    let required = [
                        "atom_site_adp_type",
                        "atom_site_label",
                        "atom_site_type_symbol",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| required.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<AtomSite> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![AtomSite::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_adp_type")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("atom_site_adp_type", v, AdpType::parse, "AdpType") {
                Ok(val) => rows[i].adp_type = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_b_11")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_b_11", v) {
                Ok(val) => rows[i].aniso_b_11 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_b_12")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_b_12", v) {
                Ok(val) => rows[i].aniso_b_12 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_b_13")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_b_13", v) {
                Ok(val) => rows[i].aniso_b_13 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_b_22")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_b_22", v) {
                Ok(val) => rows[i].aniso_b_22 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_b_23")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_b_23", v) {
                Ok(val) => rows[i].aniso_b_23 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_b_33")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_b_33", v) {
                Ok(val) => rows[i].aniso_b_33 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_label")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_aniso_label", v) {
                Ok(val) => rows[i].aniso_label = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_ratio")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_ratio", v) {
                Ok(val) => rows[i].aniso_ratio = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_type_symbol")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_aniso_type_symbol", v) {
                Ok(val) => rows[i].aniso_type_symbol = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_u_11")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_u_11", v) {
                Ok(val) => rows[i].aniso_u_11 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_u_12")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_u_12", v) {
                Ok(val) => rows[i].aniso_u_12 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_u_13")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_u_13", v) {
                Ok(val) => rows[i].aniso_u_13 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_u_22")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_u_22", v) {
                Ok(val) => rows[i].aniso_u_22 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_u_23")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_u_23", v) {
                Ok(val) => rows[i].aniso_u_23 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_aniso_u_33")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_aniso_u_33", v) {
                Ok(val) => rows[i].aniso_u_33 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_attached_hydrogens")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_attached_hydrogens", v) {
                Ok(val) => rows[i].attached_hydrogens = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_b_equiv_geom_mean")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_b_equiv_geom_mean", v) {
                Ok(val) => rows[i].b_equiv_geom_mean = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_b_iso_or_equiv")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_b_iso_or_equiv", v) {
                Ok(val) => rows[i].b_iso_or_equiv = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_calc_attached_atom")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_calc_attached_atom", v) {
                Ok(val) => rows[i].calc_attached_atom = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_calc_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("atom_site_calc_flag", v, CalcFlag::parse, "CalcFlag") {
                Ok(val) => rows[i].calc_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_cartn_x")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_cartn_x", v) {
                Ok(val) => rows[i].cartn_x = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_cartn_y")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_cartn_y", v) {
                Ok(val) => rows[i].cartn_y = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_cartn_z")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_cartn_z", v) {
                Ok(val) => rows[i].cartn_z = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_chemical_conn_number")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_chemical_conn_number", v) {
                Ok(val) => rows[i].chemical_conn_number = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_constraints")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_constraints", v) {
                Ok(val) => rows[i].constraints = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_description")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_description", v) {
                Ok(val) => rows[i].description = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_disorder_assembly")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_disorder_assembly", v) {
                Ok(val) => rows[i].disorder_assembly = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_disorder_group")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_disorder_group", v) {
                Ok(val) => rows[i].disorder_group = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_fract_x")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_fract_x", v) {
                Ok(val) => rows[i].fract_x = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_fract_y")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_fract_y", v) {
                Ok(val) => rows[i].fract_y = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_fract_z")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_fract_z", v) {
                Ok(val) => rows[i].fract_z = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label", v) {
                Ok(val) => rows[i].label = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label_component_0")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label_component_0", v) {
                Ok(val) => rows[i].label_component_0 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label_component_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label_component_1", v) {
                Ok(val) => rows[i].label_component_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label_component_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label_component_2", v) {
                Ok(val) => rows[i].label_component_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label_component_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label_component_3", v) {
                Ok(val) => rows[i].label_component_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label_component_4")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label_component_4", v) {
                Ok(val) => rows[i].label_component_4 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label_component_5")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label_component_5", v) {
                Ok(val) => rows[i].label_component_5 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_label_component_6")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_label_component_6", v) {
                Ok(val) => rows[i].label_component_6 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_occupancy")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_occupancy", v) {
                Ok(val) => rows[i].occupancy = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_refinement_flags")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "atom_site_refinement_flags",
                v,
                RefinementFlag::parse,
                "RefinementFlag",
            ) {
                Ok(val) => rows[i].refinement_flags = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_refinement_flags_adp")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "atom_site_refinement_flags_adp",
                v,
                AdpRefineFlag::parse,
                "AdpRefineFlag",
            ) {
                Ok(val) => rows[i].refinement_flags_adp = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_refinement_flags_occupancy")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "atom_site_refinement_flags_occupancy",
                v,
                OccupancyRefineFlag::parse,
                "OccupancyRefineFlag",
            ) {
                Ok(val) => rows[i].refinement_flags_occupancy = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_refinement_flags_posn")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "atom_site_refinement_flags_posn",
                v,
                PosnRefineFlag::parse,
                "PosnRefineFlag",
            ) {
                Ok(val) => rows[i].refinement_flags_posn = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_restraints")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_restraints", v) {
                Ok(val) => rows[i].restraints = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_site_symmetry_multiplicity")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_sym_multiplicity("atom_site_site_symmetry_multiplicity", v) {
                Ok(val) => rows[i].site_symmetry_multiplicity = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_site_symmetry_order")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_site_symmetry_order", v) {
                Ok(val) => rows[i].site_symmetry_order = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_symmetry_multiplicity")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_sym_multiplicity("atom_site_symmetry_multiplicity", v) {
                Ok(val) => rows[i].symmetry_multiplicity = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_thermal_displace_type")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "atom_site_thermal_displace_type",
                v,
                AdpType::parse,
                "AdpType",
            ) {
                Ok(val) => rows[i].thermal_displace_type = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_type_symbol")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_type_symbol", v) {
                Ok(val) => rows[i].type_symbol = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_u_equiv_geom_mean")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_u_equiv_geom_mean", v) {
                Ok(val) => rows[i].u_equiv_geom_mean = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_u_iso_or_equiv")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_site_u_iso_or_equiv", v) {
                Ok(val) => rows[i].u_iso_or_equiv = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_site_wyckoff_symbol")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_site_wyckoff_symbol", v) {
                Ok(val) => rows[i].wyckoff_symbol = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
