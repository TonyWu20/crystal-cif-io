use crate::data_dict::{CifTerm, LoopValueTerm};
use crate::grammar::{CharString, Float, Numeric, Tag, Value};

use self::adp_type::AdpType;
use self::label_symbol::TypeSymbol;
use self::symmetry_multiplicity::SymMultiplicity;

mod adp_type;
#[cfg(feature = "chemrust-core")]
pub mod chemrust_impl;
mod label_symbol;
mod symmetry_multiplicity;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum AtomSiteLoopItem {
    Adp_type(Vec<AdpType>),
    Aniso_B_11,
    Aniso_B_12,
    Aniso_B_13,
    Aniso_B_22,
    Aniso_B_23,
    Aniso_B_33,
    Aniso_label,
    Aniso_ratio,
    Aniso_type_symbol(Vec<TypeSymbol>),
    Aniso_U_11,
    Aniso_U_12,
    Aniso_U_13,
    Aniso_U_22,
    Aniso_U_23,
    Aniso_U_33,
    Attached_hydrogens,
    B_equiv_geom_mean,
    B_iso_or_equiv,
    Calc_attached_atom,
    Calc_flag,
    Cartn_x(Vec<Numeric>),
    Cartn_y(Vec<Numeric>),
    Cartn_z(Vec<Numeric>),
    Chemical_conn_number,
    Constraints,
    Description,
    Disorder_assembly,
    Disorder_group,
    Fract_x(Vec<Numeric>),
    Fract_y(Vec<Numeric>),
    Fract_z(Vec<Numeric>),
    Label(Vec<CharString>),
    Label_component_0,
    Label_component_1,
    Label_component_2,
    Label_component_3,
    Label_component_4,
    Label_component_5,
    Label_component_6,
    Occupancy(Vec<Float>),
    Refinement_flags,
    Refinement_flags_adp,
    Refinement_flags_occupancy,
    Refinement_flags_posn,
    Restraints,
    Site_symmetry_multiplicity(Vec<SymMultiplicity>),
    Site_symmetry_order,
    Symmetry_multiplicity(Vec<SymMultiplicity>),
    Thermal_displace_type,
    Type_symbol(Vec<TypeSymbol>),
    U_equiv_geom_mean,
    U_iso_or_equiv(Vec<Float>),
    Wyckoff_symbol,
}

impl CifTerm for AtomSiteLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomSiteLoopItem::Adp_type(_) => "adp_type",
            AtomSiteLoopItem::Aniso_B_11 => "aniso_B_11",
            AtomSiteLoopItem::Aniso_B_12 => "aniso_B_12",
            AtomSiteLoopItem::Aniso_B_13 => "aniso_B_13",
            AtomSiteLoopItem::Aniso_B_22 => "aniso_B_22",
            AtomSiteLoopItem::Aniso_B_23 => "aniso_B_23",
            AtomSiteLoopItem::Aniso_B_33 => "aniso_B_33",
            AtomSiteLoopItem::Aniso_label => "aniso_label",
            AtomSiteLoopItem::Aniso_ratio => "aniso_ratio",
            AtomSiteLoopItem::Aniso_type_symbol(_) => "aniso_type_symbol",
            AtomSiteLoopItem::Aniso_U_11 => "aniso_U_11",
            AtomSiteLoopItem::Aniso_U_12 => "aniso_U_12",
            AtomSiteLoopItem::Aniso_U_13 => "aniso_U_13",
            AtomSiteLoopItem::Aniso_U_22 => "aniso_U_22",
            AtomSiteLoopItem::Aniso_U_23 => "aniso_U_23",
            AtomSiteLoopItem::Aniso_U_33 => "aniso_U_33",
            AtomSiteLoopItem::Attached_hydrogens => "attached_hydrogens",
            AtomSiteLoopItem::B_equiv_geom_mean => "b_equiv_geom_mean",
            AtomSiteLoopItem::B_iso_or_equiv => "b_iso_or_equiv",
            AtomSiteLoopItem::Calc_attached_atom => "calc_attached_atom",
            AtomSiteLoopItem::Calc_flag => "calc_flag",
            AtomSiteLoopItem::Cartn_x(_) => "cartn_x",
            AtomSiteLoopItem::Cartn_y(_) => "cartn_y",
            AtomSiteLoopItem::Cartn_z(_) => "cartn_z",
            AtomSiteLoopItem::Chemical_conn_number => "chemical_conn_number",
            AtomSiteLoopItem::Constraints => "constraints",
            AtomSiteLoopItem::Description => "description",
            AtomSiteLoopItem::Disorder_assembly => "disorder_assembly",
            AtomSiteLoopItem::Disorder_group => "disorder_group",
            AtomSiteLoopItem::Fract_x(_) => "fract_x",
            AtomSiteLoopItem::Fract_y(_) => "fract_y",
            AtomSiteLoopItem::Fract_z(_) => "fract_z",
            AtomSiteLoopItem::Label(_) => "label",
            AtomSiteLoopItem::Label_component_0 => "label_component_0",
            AtomSiteLoopItem::Label_component_1 => "label_component_1",
            AtomSiteLoopItem::Label_component_2 => "label_component_2",
            AtomSiteLoopItem::Label_component_3 => "label_component_3",
            AtomSiteLoopItem::Label_component_4 => "label_component_4",
            AtomSiteLoopItem::Label_component_5 => "label_component_5",
            AtomSiteLoopItem::Label_component_6 => "label_component_6",
            AtomSiteLoopItem::Occupancy(_) => "occupancy",
            AtomSiteLoopItem::Refinement_flags => "refinement_flags",
            AtomSiteLoopItem::Refinement_flags_adp => "refinement_flags_adp",
            AtomSiteLoopItem::Refinement_flags_occupancy => "refinement_flags_occupancy",
            AtomSiteLoopItem::Refinement_flags_posn => "refinement_flags_posn",
            AtomSiteLoopItem::Restraints => "restraints",
            AtomSiteLoopItem::Site_symmetry_multiplicity(_) => "site_symmetry_multiplicity",
            AtomSiteLoopItem::Site_symmetry_order => "site_symmetry_order",
            AtomSiteLoopItem::Symmetry_multiplicity(_) => "symmetry_multiplicity",
            AtomSiteLoopItem::Thermal_displace_type => "thermal_displace_type",
            AtomSiteLoopItem::Type_symbol(_) => "type_symbol",
            AtomSiteLoopItem::U_equiv_geom_mean => "u_equiv_geom_mean",
            AtomSiteLoopItem::U_iso_or_equiv(_) => "u_iso_or_equiv",
            AtomSiteLoopItem::Wyckoff_symbol => "wyckoff_symbol",
        };
        Tag::new(format!("atom_site_{suffix}"))
    }
}

impl LoopValueTerm for AtomSiteLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AtomSiteLoopItem::Adp_type(v) => v.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Aniso_B_11 => todo!(),
            AtomSiteLoopItem::Aniso_B_12 => todo!(),
            AtomSiteLoopItem::Aniso_B_13 => todo!(),
            AtomSiteLoopItem::Aniso_B_22 => todo!(),
            AtomSiteLoopItem::Aniso_B_23 => todo!(),
            AtomSiteLoopItem::Aniso_B_33 => todo!(),
            AtomSiteLoopItem::Aniso_label => todo!(),
            AtomSiteLoopItem::Aniso_ratio => todo!(),
            AtomSiteLoopItem::Type_symbol(symbol) | AtomSiteLoopItem::Aniso_type_symbol(symbol) => {
                symbol.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Aniso_U_11 => todo!(),
            AtomSiteLoopItem::Aniso_U_12 => todo!(),
            AtomSiteLoopItem::Aniso_U_13 => todo!(),
            AtomSiteLoopItem::Aniso_U_22 => todo!(),
            AtomSiteLoopItem::Aniso_U_23 => todo!(),
            AtomSiteLoopItem::Aniso_U_33 => todo!(),
            AtomSiteLoopItem::Attached_hydrogens => todo!(),
            AtomSiteLoopItem::B_equiv_geom_mean => todo!(),
            AtomSiteLoopItem::B_iso_or_equiv => todo!(),
            AtomSiteLoopItem::Calc_attached_atom => todo!(),
            AtomSiteLoopItem::Calc_flag => todo!(),
            AtomSiteLoopItem::Cartn_x(v)
            | AtomSiteLoopItem::Cartn_y(v)
            | AtomSiteLoopItem::Cartn_z(v)
            | AtomSiteLoopItem::Fract_x(v)
            | AtomSiteLoopItem::Fract_y(v)
            | AtomSiteLoopItem::Fract_z(v) => v.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Chemical_conn_number => todo!(),
            AtomSiteLoopItem::Constraints => todo!(),
            AtomSiteLoopItem::Description => todo!(),
            AtomSiteLoopItem::Disorder_assembly => todo!(),
            AtomSiteLoopItem::Disorder_group => todo!(),
            AtomSiteLoopItem::Label(s) => s.iter().cloned().map(Value::from).collect(),
            AtomSiteLoopItem::Label_component_0 => todo!(),
            AtomSiteLoopItem::Label_component_1 => todo!(),
            AtomSiteLoopItem::Label_component_2 => todo!(),
            AtomSiteLoopItem::Label_component_3 => todo!(),
            AtomSiteLoopItem::Label_component_4 => todo!(),
            AtomSiteLoopItem::Label_component_5 => todo!(),
            AtomSiteLoopItem::Label_component_6 => todo!(),
            AtomSiteLoopItem::U_iso_or_equiv(f) | AtomSiteLoopItem::Occupancy(f) => {
                f.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Refinement_flags => todo!(),
            AtomSiteLoopItem::Refinement_flags_adp => todo!(),
            AtomSiteLoopItem::Refinement_flags_occupancy => todo!(),
            AtomSiteLoopItem::Refinement_flags_posn => todo!(),
            AtomSiteLoopItem::Restraints => todo!(),
            AtomSiteLoopItem::Site_symmetry_multiplicity(m)
            | AtomSiteLoopItem::Symmetry_multiplicity(m) => {
                m.iter().cloned().map(Value::from).collect()
            }
            AtomSiteLoopItem::Site_symmetry_order => todo!(),
            AtomSiteLoopItem::Thermal_displace_type => todo!(),
            AtomSiteLoopItem::U_equiv_geom_mean => todo!(),
            AtomSiteLoopItem::Wyckoff_symbol => todo!(),
        }
    }
}
