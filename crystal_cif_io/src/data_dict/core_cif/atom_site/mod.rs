use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Float, Matrix, Numeric, Tag, Value};

pub use self::adp_type::AdpType;
pub use self::calc_flag::CalcFlag;
pub use self::label_symbol::{TypeSymbol, TypeSymbolCode};
pub use self::refine_flags::{AdpRefineFlag, OccupancyRefineFlag, PosnRefineFlag, RefinementFlag};
pub use self::symmetry_multiplicity::SymMultiplicity;

mod adp_type;
mod calc_flag;
mod label_symbol;
mod refine_flags;
mod symmetry_multiplicity;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum AtomSiteLoopItem {
    Adp_type(Vec<AdpType>),
    Aniso_B_11(Vec<Numeric>),
    Aniso_B_12(Vec<Numeric>),
    Aniso_B_13(Vec<Numeric>),
    Aniso_B_22(Vec<Numeric>),
    Aniso_B_23(Vec<Numeric>),
    Aniso_B_33(Vec<Numeric>),
    Aniso_label(Vec<CharString>),
    Aniso_ratio(Vec<Numeric>),
    Aniso_type_symbol(Vec<TypeSymbol>),
    Aniso_U_11(Vec<Numeric>),
    Aniso_U_12(Vec<Numeric>),
    Aniso_U_13(Vec<Numeric>),
    Aniso_U_22(Vec<Numeric>),
    Aniso_U_23(Vec<Numeric>),
    Aniso_U_33(Vec<Numeric>),
    Attached_hydrogens(Vec<Numeric>),
    B_equiv_geom_mean(Vec<Numeric>),
    B_iso_or_equiv(Vec<Numeric>),
    Calc_attached_atom(Vec<CharString>),
    Calc_flag(Vec<CalcFlag>),
    Cartn_x(Vec<Numeric>),
    Cartn_y(Vec<Numeric>),
    Cartn_z(Vec<Numeric>),
    Chemical_conn_number(Vec<Numeric>),
    Constraints(Vec<CharString>),
    Description(Vec<CharString>),
    Disorder_assembly(Vec<CharString>),
    Disorder_group(Vec<CharString>),
    Fract_x(Vec<Numeric>),
    Fract_y(Vec<Numeric>),
    Fract_z(Vec<Numeric>),
    Label(Vec<CharString>),
    Label_component_0(Vec<CharString>),
    Label_component_1(Vec<CharString>),
    Label_component_2(Vec<CharString>),
    Label_component_3(Vec<CharString>),
    Label_component_4(Vec<CharString>),
    Label_component_5(Vec<CharString>),
    Label_component_6(Vec<CharString>),
    Occupancy(Vec<Float>),
    Refinement_flags(Vec<RefinementFlag>),
    Refinement_flags_adp(Vec<AdpRefineFlag>),
    Refinement_flags_occupancy(Vec<OccupancyRefineFlag>),
    Refinement_flags_posn(Vec<PosnRefineFlag>),
    Restraints(Vec<CharString>),
    Site_symmetry_multiplicity(Vec<SymMultiplicity>),
    Site_symmetry_order(Vec<Numeric>),
    Symmetry_multiplicity(Vec<SymMultiplicity>),
    Thermal_displace_type(Vec<AdpType>),
    Type_symbol(Vec<TypeSymbol>),
    U_equiv_geom_mean(Vec<Numeric>),
    U_iso_or_equiv(Vec<Float>),
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
pub enum AtomSiteTerms {
    /// The shell records a 3-vector (`_type.dimension` `[3]`);
    /// the `Matrix` payload holds the row list.
    Cartn_xyz(Matrix),
    Fract_symmform(CharString),
    /// The shell records a 3-vector (`_type.dimension` `[3]`);
    /// the `Matrix` payload holds the row list.
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
