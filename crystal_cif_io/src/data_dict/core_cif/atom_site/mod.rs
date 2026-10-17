use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Float, Matrix, Numeric, Tag, Value};

pub use self::adp_type::AdpType;
pub use self::calc_flag::CalcFlag;
pub use self::label_symbol::{TypeSymbol, TypeSymbolCode};
pub use self::refine_flags::{AdpRefineFlag, OccupancyRefineFlag, PosnRefineFlag, RefinementFlag};
pub use self::symmetry_multiplicity::SymMultiplicity;
pub use atom_site::AtomSite;

mod adp_type;
mod atom_site;
mod calc_flag;
mod label_symbol;
mod refine_flags;
mod symmetry_multiplicity;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
/// The loop-value terms of the `atom_site` category: one variant per

/// registry loop-value item, in item order.
pub enum AtomSiteLoopItem {
    /// The `_atom_site_adp_type` item. The variant holds the column values, one per loop row.
    ///
    /// A standard code used to describe the type of atomic displacement
    /// parameters used for the site.
    Adp_type(Vec<AdpType>),
    /// The `_atom_site_aniso_B_11` item. The variant holds the column values, one per loop row.
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
    Aniso_B_11(Vec<Numeric>),
    /// The `_atom_site_aniso_B_12` item. The variant holds the column values, one per loop row.
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
    Aniso_B_12(Vec<Numeric>),
    /// The `_atom_site_aniso_B_13` item. The variant holds the column values, one per loop row.
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
    Aniso_B_13(Vec<Numeric>),
    /// The `_atom_site_aniso_B_22` item. The variant holds the column values, one per loop row.
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
    Aniso_B_22(Vec<Numeric>),
    /// The `_atom_site_aniso_B_23` item. The variant holds the column values, one per loop row.
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
    Aniso_B_23(Vec<Numeric>),
    /// The `_atom_site_aniso_B_33` item. The variant holds the column values, one per loop row.
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
    Aniso_B_33(Vec<Numeric>),
    /// The `_atom_site_aniso_label` item. The variant holds the column values, one per loop row.
    ///
    /// Anisotropic atomic displacement parameters are usually looped in a
    /// separate list. If this is the case, this code must match the
    /// _atom_site_label of the associated atom in the atom coordinate list
    /// and conform with the same rules described in _atom_site_label.
    Aniso_label(Vec<CharString>),
    /// The `_atom_site_aniso_ratio` item. The variant holds the column values, one per loop row.
    ///
    /// Ratio of the maximum to minimum principal axes of displacement
    /// (thermal) ellipsoids.
    Aniso_ratio(Vec<Numeric>),
    /// The `_atom_site_aniso_type_symbol` item. The variant holds the column values, one per loop row.
    ///
    /// This _atom_type_symbol code links the anisotropic atom parameters to
    /// the atom-type data associated with this site and must match one of
    /// the _atom_type_symbol codes in this list.
    Aniso_type_symbol(Vec<TypeSymbol>),
    /// The `_atom_site_aniso_U_11` item. The variant holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    Aniso_U_11(Vec<Numeric>),
    /// The `_atom_site_aniso_U_12` item. The variant holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    Aniso_U_12(Vec<Numeric>),
    /// The `_atom_site_aniso_U_13` item. The variant holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    Aniso_U_13(Vec<Numeric>),
    /// The `_atom_site_aniso_U_22` item. The variant holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    Aniso_U_22(Vec<Numeric>),
    /// The `_atom_site_aniso_U_23` item. The variant holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    Aniso_U_23(Vec<Numeric>),
    /// The `_atom_site_aniso_U_33` item. The variant holds the column values, one per loop row.
    ///
    /// These are the standard anisotropic atomic displacement components in
    /// angstroms squared which appear in the structure-factor term T =
    /// exp{-2pi^2^ sum~i~ [sum~j~ (U^ij^ h~i~ h~j~ a*~i~ a*~j~) ] } h = the
    /// Miller indices a* = the reciprocal-space cell lengths The unique
    /// elements of the real symmetric matrix are entered by row.
    /// Units: A^2^ (angstrom squared).
    Aniso_U_33(Vec<Numeric>),
    /// The `_atom_site_attached_hydrogens` item. The variant holds the column values, one per loop row.
    ///
    /// The number of hydrogen atoms attached to the atom at this site
    /// excluding any hydrogen atoms for which coordinates (measured or
    /// calculated) are given.
    Attached_hydrogens(Vec<Numeric>),
    /// The `_atom_site_B_equiv_geom_mean` item. The variant holds the column values, one per loop row.
    ///
    /// Equivalent isotropic atomic displacement parameter, B(equiv), in
    /// angstroms squared, calculated as the geometric mean of the
    /// anisotropic atomic displacement parameters. B(equiv) = (B~i~ B~j~
    /// B~k~)^1/3^ B~n~ = the principal components of the orthogonalized
    /// B^ij^ The IUCr Commission on Nomenclature recommends against the use
    /// of B for reporting atomic displacement parameters. U, being directly
    /// proportional to B, is preferred.
    /// Units: A^2^ (angstrom squared).
    B_equiv_geom_mean(Vec<Numeric>),
    /// The `_atom_site_B_iso_or_equiv` item. The variant holds the column values, one per loop row.
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
    B_iso_or_equiv(Vec<Numeric>),
    /// The `_atom_site_calc_attached_atom` item. The variant holds the column values, one per loop row.
    ///
    /// The _atom_site_label of the atom site to which the 'geometry-
    /// calculated' atom site is attached.
    Calc_attached_atom(Vec<CharString>),
    /// The `_atom_site_calc_flag` item. The variant holds the column values, one per loop row.
    ///
    /// A standard code to signal whether the site coordinates have been
    /// determined from the intensities or calculated from the geometry of
    /// surrounding sites, or have been assigned dummy coordinates. The
    /// abbreviation 'c' may be used in place of 'calc'.
    Calc_flag(Vec<CalcFlag>),
    /// The `_atom_site_Cartn_x` item. The variant holds the column values, one per loop row.
    ///
    /// The atom-site coordinates in angstroms specified according to a set
    /// of orthogonal Cartesian axes related to the cell axes as specified
    /// by the _atom_sites_Cartn_transform_axes description.
    /// Units: A (angstrom).
    Cartn_x(Vec<Numeric>),
    /// The `_atom_site_Cartn_y` item. The variant holds the column values, one per loop row.
    ///
    /// The atom-site coordinates in angstroms specified according to a set
    /// of orthogonal Cartesian axes related to the cell axes as specified
    /// by the _atom_sites_Cartn_transform_axes description.
    /// Units: A (angstrom).
    Cartn_y(Vec<Numeric>),
    /// The `_atom_site_Cartn_z` item. The variant holds the column values, one per loop row.
    ///
    /// The atom-site coordinates in angstroms specified according to a set
    /// of orthogonal Cartesian axes related to the cell axes as specified
    /// by the _atom_sites_Cartn_transform_axes description.
    /// Units: A (angstrom).
    Cartn_z(Vec<Numeric>),
    /// The `_atom_site_chemical_conn_number` item. The variant holds the column values, one per loop row.
    ///
    /// This number links an atom site to the chemical connectivity list. It
    /// must match a number specified by _chemical_conn_atom_number.
    Chemical_conn_number(Vec<Numeric>),
    /// The `_atom_site_constraints` item. The variant holds the column values, one per loop row.
    ///
    /// A description of the constraints applied to parameters at this site
    /// during refinement. See also _atom_site_refinement_flags and
    /// _refine_ls_number_constraints.
    Constraints(Vec<CharString>),
    /// The `_atom_site_description` item. The variant holds the column values, one per loop row.
    ///
    /// A description of special aspects of this site. See also
    /// _atom_site_refinement_flags.
    Description(Vec<CharString>),
    /// The `_atom_site_disorder_assembly` item. The variant holds the column values, one per loop row.
    ///
    /// A code which identifies a cluster of atoms that show long-range
    /// positional disorder but are locally ordered. Within each such
    /// cluster of atoms, _atom_site_disorder_group is used to identify the
    /// sites that are simultaneously occupied. This field is only needed if
    /// there is more than one cluster of disordered atoms showing
    /// independent local order.
    Disorder_assembly(Vec<CharString>),
    /// The `_atom_site_disorder_group` item. The variant holds the column values, one per loop row.
    ///
    /// A code which identifies a group of positionally disordered atom
    /// sites that are locally simultaneously occupied. Atoms that are
    /// positionally disordered over two or more sites (e.g. the hydrogen
    /// atoms of a methyl group that exists in two orientations) can be
    /// assigned to two or more groups. Sites belonging to the same group
    /// are simultaneously occupied, but those belonging to different groups
    /// are not. A minus prefix (e.g. "-1") is used to indicate sites
    /// disordered about a special position.
    Disorder_group(Vec<CharString>),
    /// The `_atom_site_fract_x` item. The variant holds the column values, one per loop row.
    ///
    /// Atom-site coordinates as fractions of the _cell_length_ values.
    Fract_x(Vec<Numeric>),
    /// The `_atom_site_fract_y` item. The variant holds the column values, one per loop row.
    ///
    /// Atom-site coordinates as fractions of the _cell_length_ values.
    Fract_y(Vec<Numeric>),
    /// The `_atom_site_fract_z` item. The variant holds the column values, one per loop row.
    ///
    /// Atom-site coordinates as fractions of the _cell_length_ values.
    Fract_z(Vec<Numeric>),
    /// The `_atom_site_label` item. The variant holds the column values, one per loop row.
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
    Label(Vec<CharString>),
    /// The `_atom_site_label_component_0` item. The variant holds the column values, one per loop row.
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
    Label_component_0(Vec<CharString>),
    /// The `_atom_site_label_component_1` item. The variant holds the column values, one per loop row.
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
    Label_component_1(Vec<CharString>),
    /// The `_atom_site_label_component_2` item. The variant holds the column values, one per loop row.
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
    Label_component_2(Vec<CharString>),
    /// The `_atom_site_label_component_3` item. The variant holds the column values, one per loop row.
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
    Label_component_3(Vec<CharString>),
    /// The `_atom_site_label_component_4` item. The variant holds the column values, one per loop row.
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
    Label_component_4(Vec<CharString>),
    /// The `_atom_site_label_component_5` item. The variant holds the column values, one per loop row.
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
    Label_component_5(Vec<CharString>),
    /// The `_atom_site_label_component_6` item. The variant holds the column values, one per loop row.
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
    Label_component_6(Vec<CharString>),
    /// The `_atom_site_occupancy` item. The variant holds the column values, one per loop row.
    ///
    /// The fraction of the atom type present at this site. The sum of the
    /// occupancies of all the atom types at this site may not significantly
    /// exceed 1.0 unless it is a dummy site. The value must lie in the
    /// 99.97% Gaussian confidence interval -3u =&lt; x =&lt; 1 + 3u. The
    /// _enumeration_range of 0.0:1.0 is thus correctly interpreted as
    /// meaning (0.0 - 3u) =&lt; x =&lt; (1.0 + 3u).
    Occupancy(Vec<Float>),
    /// The `_atom_site_refinement_flags` item. The variant holds the column values, one per loop row.
    ///
    /// A concatenated series of single-letter codes which indicate the
    /// refinement restraints or constraints applied to this site. This item
    /// should not be used. It has been replaced by
    /// _atom_site_refinement_flags_posn, *_adp and *_occupancy. It is
    /// retained in this dictionary only to provide compatibility with
    /// legacy CIFs.
    Refinement_flags(Vec<RefinementFlag>),
    /// The `_atom_site_refinement_flags_adp` item. The variant holds the column values, one per loop row.
    ///
    /// A code which indicates the refinement restraints or constraints
    /// applied to the atomic displacement parameters of this site.
    Refinement_flags_adp(Vec<AdpRefineFlag>),
    /// The `_atom_site_refinement_flags_occupancy` item. The variant holds the column values, one per loop row.
    ///
    /// A code which indicates that refinement restraints or constraints
    /// were applied to the occupancy of this site.
    Refinement_flags_occupancy(Vec<OccupancyRefineFlag>),
    /// The `_atom_site_refinement_flags_posn` item. The variant holds the column values, one per loop row.
    ///
    /// A code which indicates the refinement restraints or constraints
    /// applied to the positional coordinates of this site.
    Refinement_flags_posn(Vec<PosnRefineFlag>),
    /// The `_atom_site_restraints` item. The variant holds the column values, one per loop row.
    ///
    /// A description of restraints applied to specific parameters at this
    /// site during refinement. See also _atom_site_refinement_flags and
    /// _refine_ls_number_restraints.
    Restraints(Vec<CharString>),
    /// The `_atom_site_site_symmetry_multiplicity` item. The variant holds the column values, one per loop row.
    ///
    /// The number of different sites that are generated by the application
    /// of the space-group symmetry to the coordinates given for this site.
    /// It is equal to the multiplicity given for this Wyckoff site in
    /// International Tables for Crystallography Vol. A (2002). It is equal
    /// to the multiplicity of the general position divided by the order of
    /// the site symmetry given in _atom_site_site_symmetry_order.
    Site_symmetry_multiplicity(Vec<SymMultiplicity>),
    /// The `_atom_site_site_symmetry_order` item. The variant holds the column values, one per loop row.
    ///
    /// The order of the site symmetry of the site identified by
    /// _atom_site_label. This is the number of times application of the
    /// crystallographic symmetry to the coordinates given for this site
    /// generates the same set of coordinates. It is equal to: multiplicity
    /// of the general position ------------------------------------
    /// multiplicity of this site where 'multiplicity of this site' is given
    /// in _atom_site_site_symmetry_multiplicity.
    Site_symmetry_order(Vec<Numeric>),
    /// The `_atom_site_symmetry_multiplicity` item. The variant holds the column values, one per loop row.
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
    Symmetry_multiplicity(Vec<SymMultiplicity>),
    /// The `_atom_site_thermal_displace_type` item. The variant holds the column values, one per loop row.
    ///
    /// A standard code used to describe the type of atomic displacement
    /// parameters used for the site.
    Thermal_displace_type(Vec<AdpType>),
    /// The `_atom_site_type_symbol` item. The variant holds the column values, one per loop row.
    ///
    /// A code to identify the atom species (singular or plural) occupying
    /// this site. This code must match a corresponding _atom_type_symbol.
    /// The specification of this code is optional if component 0 of the
    /// _atom_site_label is used for this purpose. See _atom_type_symbol.
    Type_symbol(Vec<TypeSymbol>),
    /// The `_atom_site_U_equiv_geom_mean` item. The variant holds the column values, one per loop row.
    ///
    /// Equivalent isotropic atomic displacement parameter, U(equiv), in
    /// angstroms squared, calculated as the geometric mean of the
    /// anisotropic atomic displacement parameters. U(equiv) = (U~i~ U~j~
    /// U~k~)^1/3^ U~n~ = the principal components of the orthogonalized
    /// U^ij^
    /// Units: A^2^ (angstrom squared).
    U_equiv_geom_mean(Vec<Numeric>),
    /// The `_atom_site_U_iso_or_equiv` item. The variant holds the column values, one per loop row.
    ///
    /// Isotropic atomic displacement parameter, or equivalent isotropic
    /// atomic displacement parameter, U(equiv), in angstroms squared,
    /// calculated from anisotropic atomic displacement parameters. U(equiv)
    /// = (1/3) sum~i~[sum~j~(U^ij^ a*~i~ a*~j~ a~i~ a~j~)] a = the
    /// real-space cell lengths a* = the reciprocal-space cell lengths Ref:
    /// Fischer, R. X. & Tillmanns, E. (1988). Acta Cryst. C44, 775-776.
    /// Units: A^2^ (angstrom squared).
    U_iso_or_equiv(Vec<Float>),
    /// The `_atom_site_Wyckoff_symbol` item. The variant holds the column values, one per loop row.
    ///
    /// The Wyckoff symbol (letter) as listed in the space-group tables of
    /// International Tables for Crystallography Vol. A (2002).
    Wyckoff_symbol(Vec<CharString>),
}

impl CifTerm for AtomSiteLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomSiteLoopItem::Adp_type(_) => "adp_type",
            AtomSiteLoopItem::Aniso_B_11(_) => "aniso_B_11",
            AtomSiteLoopItem::Aniso_B_12(_) => "aniso_B_12",
            AtomSiteLoopItem::Aniso_B_13(_) => "aniso_B_13",
            AtomSiteLoopItem::Aniso_B_22(_) => "aniso_B_22",
            AtomSiteLoopItem::Aniso_B_23(_) => "aniso_B_23",
            AtomSiteLoopItem::Aniso_B_33(_) => "aniso_B_33",
            AtomSiteLoopItem::Aniso_label(_) => "aniso_label",
            AtomSiteLoopItem::Aniso_ratio(_) => "aniso_ratio",
            AtomSiteLoopItem::Aniso_type_symbol(_) => "aniso_type_symbol",
            AtomSiteLoopItem::Aniso_U_11(_) => "aniso_U_11",
            AtomSiteLoopItem::Aniso_U_12(_) => "aniso_U_12",
            AtomSiteLoopItem::Aniso_U_13(_) => "aniso_U_13",
            AtomSiteLoopItem::Aniso_U_22(_) => "aniso_U_22",
            AtomSiteLoopItem::Aniso_U_23(_) => "aniso_U_23",
            AtomSiteLoopItem::Aniso_U_33(_) => "aniso_U_33",
            AtomSiteLoopItem::Attached_hydrogens(_) => "attached_hydrogens",
            AtomSiteLoopItem::B_equiv_geom_mean(_) => "b_equiv_geom_mean",
            AtomSiteLoopItem::B_iso_or_equiv(_) => "b_iso_or_equiv",
            AtomSiteLoopItem::Calc_attached_atom(_) => "calc_attached_atom",
            AtomSiteLoopItem::Calc_flag(_) => "calc_flag",
            AtomSiteLoopItem::Cartn_x(_) => "cartn_x",
            AtomSiteLoopItem::Cartn_y(_) => "cartn_y",
            AtomSiteLoopItem::Cartn_z(_) => "cartn_z",
            AtomSiteLoopItem::Chemical_conn_number(_) => "chemical_conn_number",
            AtomSiteLoopItem::Constraints(_) => "constraints",
            AtomSiteLoopItem::Description(_) => "description",
            AtomSiteLoopItem::Disorder_assembly(_) => "disorder_assembly",
            AtomSiteLoopItem::Disorder_group(_) => "disorder_group",
            AtomSiteLoopItem::Fract_x(_) => "fract_x",
            AtomSiteLoopItem::Fract_y(_) => "fract_y",
            AtomSiteLoopItem::Fract_z(_) => "fract_z",
            AtomSiteLoopItem::Label(_) => "label",
            AtomSiteLoopItem::Label_component_0(_) => "label_component_0",
            AtomSiteLoopItem::Label_component_1(_) => "label_component_1",
            AtomSiteLoopItem::Label_component_2(_) => "label_component_2",
            AtomSiteLoopItem::Label_component_3(_) => "label_component_3",
            AtomSiteLoopItem::Label_component_4(_) => "label_component_4",
            AtomSiteLoopItem::Label_component_5(_) => "label_component_5",
            AtomSiteLoopItem::Label_component_6(_) => "label_component_6",
            AtomSiteLoopItem::Occupancy(_) => "occupancy",
            AtomSiteLoopItem::Refinement_flags(_) => "refinement_flags",
            AtomSiteLoopItem::Refinement_flags_adp(_) => "refinement_flags_adp",
            AtomSiteLoopItem::Refinement_flags_occupancy(_) => "refinement_flags_occupancy",
            AtomSiteLoopItem::Refinement_flags_posn(_) => "refinement_flags_posn",
            AtomSiteLoopItem::Restraints(_) => "restraints",
            AtomSiteLoopItem::Site_symmetry_multiplicity(_) => "site_symmetry_multiplicity",
            AtomSiteLoopItem::Site_symmetry_order(_) => "site_symmetry_order",
            AtomSiteLoopItem::Symmetry_multiplicity(_) => "symmetry_multiplicity",
            AtomSiteLoopItem::Thermal_displace_type(_) => "thermal_displace_type",
            AtomSiteLoopItem::Type_symbol(_) => "type_symbol",
            AtomSiteLoopItem::U_equiv_geom_mean(_) => "u_equiv_geom_mean",
            AtomSiteLoopItem::U_iso_or_equiv(_) => "u_iso_or_equiv",
            AtomSiteLoopItem::Wyckoff_symbol(_) => "wyckoff_symbol",
        };
        Tag::new(format!("atom_site_{suffix}"))
    }
}

// The T6 single-value items of the `atom_site` category.

/// The single-value items of the `atom_site` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `atom_site` category: one variant per

/// registry single-value item, in item order.
pub enum AtomSiteTerms {
    /// The shell records a 3-vector (`_type.dimension` `[3]`);
    /// the `Matrix` payload holds the row list.
    /// The `_atom_site_Cartn_xyz` item. The variant holds the column values, one per loop row.
    Cartn_xyz(Matrix),
    /// The `_atom_site_fract_symmform` item. The variant holds the column values, one per loop row.
    Fract_symmform(CharString),
    /// The shell records a 3-vector (`_type.dimension` `[3]`);
    /// the `Matrix` payload holds the row list.
    /// The `_atom_site_fract_xyz` item. The variant holds the column values, one per loop row.
    Fract_xyz(Matrix),
}

impl CifTerm for AtomSiteTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomSiteTerms::Cartn_xyz(_) => "cartn_xyz",
            AtomSiteTerms::Fract_symmform(_) => "fract_symmform",
            AtomSiteTerms::Fract_xyz(_) => "fract_xyz",
        };
        Tag::new(format!("atom_site_{suffix}"))
    }
}

impl SingleValueTerm for AtomSiteTerms {
    fn value(&self) -> Value {
        match self {
            AtomSiteTerms::Cartn_xyz(m) | AtomSiteTerms::Fract_xyz(m) => matrix::to_value(m),
            AtomSiteTerms::Fract_symmform(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for AtomSiteTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl LoopValueTerm for AtomSiteLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AtomSiteLoopItem::Adp_type(v) => v.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Aniso_B_11(v)
            | AtomSiteLoopItem::Aniso_B_12(v)
            | AtomSiteLoopItem::Aniso_B_13(v)
            | AtomSiteLoopItem::Aniso_B_22(v)
            | AtomSiteLoopItem::Aniso_B_23(v)
            | AtomSiteLoopItem::Aniso_B_33(v)
            | AtomSiteLoopItem::Aniso_ratio(v)
            | AtomSiteLoopItem::Aniso_U_11(v)
            | AtomSiteLoopItem::Aniso_U_12(v)
            | AtomSiteLoopItem::Aniso_U_13(v)
            | AtomSiteLoopItem::Aniso_U_22(v)
            | AtomSiteLoopItem::Aniso_U_23(v)
            | AtomSiteLoopItem::Aniso_U_33(v)
            | AtomSiteLoopItem::Attached_hydrogens(v)
            | AtomSiteLoopItem::B_equiv_geom_mean(v)
            | AtomSiteLoopItem::B_iso_or_equiv(v)
            | AtomSiteLoopItem::Chemical_conn_number(v)
            | AtomSiteLoopItem::Site_symmetry_order(v)
            | AtomSiteLoopItem::U_equiv_geom_mean(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Aniso_label(v)
            | AtomSiteLoopItem::Calc_attached_atom(v)
            | AtomSiteLoopItem::Constraints(v)
            | AtomSiteLoopItem::Description(v)
            | AtomSiteLoopItem::Disorder_assembly(v)
            | AtomSiteLoopItem::Disorder_group(v)
            | AtomSiteLoopItem::Label_component_0(v)
            | AtomSiteLoopItem::Label_component_1(v)
            | AtomSiteLoopItem::Label_component_2(v)
            | AtomSiteLoopItem::Label_component_3(v)
            | AtomSiteLoopItem::Label_component_4(v)
            | AtomSiteLoopItem::Label_component_5(v)
            | AtomSiteLoopItem::Label_component_6(v)
            | AtomSiteLoopItem::Restraints(v)
            | AtomSiteLoopItem::Wyckoff_symbol(v) => v.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Type_symbol(symbol) | AtomSiteLoopItem::Aniso_type_symbol(symbol) => {
                symbol.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Calc_flag(v) => v.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Cartn_x(v)
            | AtomSiteLoopItem::Cartn_y(v)
            | AtomSiteLoopItem::Cartn_z(v)
            | AtomSiteLoopItem::Fract_x(v)
            | AtomSiteLoopItem::Fract_y(v)
            | AtomSiteLoopItem::Fract_z(v) => v.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Label(s) => s.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::U_iso_or_equiv(f) | AtomSiteLoopItem::Occupancy(f) => {
                f.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Refinement_flags(v) => v.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Refinement_flags_adp(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Refinement_flags_occupancy(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Refinement_flags_posn(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Site_symmetry_multiplicity(m)
            | AtomSiteLoopItem::Symmetry_multiplicity(m) => {
                m.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Thermal_displace_type(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
