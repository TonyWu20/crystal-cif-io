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
pub struct AtomSite {
    pub adp_type: Option<AdpType>,
    pub aniso_b_11: Option<Numeric>,
    pub aniso_b_12: Option<Numeric>,
    pub aniso_b_13: Option<Numeric>,
    pub aniso_b_22: Option<Numeric>,
    pub aniso_b_23: Option<Numeric>,
    pub aniso_b_33: Option<Numeric>,
    pub aniso_label: Option<CharString>,
    pub aniso_ratio: Option<Numeric>,
    pub aniso_type_symbol: Option<CharString>,
    pub aniso_u_11: Option<Numeric>,
    pub aniso_u_12: Option<Numeric>,
    pub aniso_u_13: Option<Numeric>,
    pub aniso_u_22: Option<Numeric>,
    pub aniso_u_23: Option<Numeric>,
    pub aniso_u_33: Option<Numeric>,
    pub attached_hydrogens: Option<Numeric>,
    pub b_equiv_geom_mean: Option<Numeric>,
    pub b_iso_or_equiv: Option<Numeric>,
    pub calc_attached_atom: Option<CharString>,
    pub calc_flag: Option<CalcFlag>,
    pub cartn_x: Option<Numeric>,
    pub cartn_y: Option<Numeric>,
    pub cartn_z: Option<Numeric>,
    pub chemical_conn_number: Option<Numeric>,
    pub constraints: Option<CharString>,
    pub description: Option<CharString>,
    pub disorder_assembly: Option<CharString>,
    pub disorder_group: Option<CharString>,
    pub fract_x: Option<Numeric>,
    pub fract_y: Option<Numeric>,
    pub fract_z: Option<Numeric>,
    pub label: Option<CharString>,
    pub label_component_0: Option<CharString>,
    pub label_component_1: Option<CharString>,
    pub label_component_2: Option<CharString>,
    pub label_component_3: Option<CharString>,
    pub label_component_4: Option<CharString>,
    pub label_component_5: Option<CharString>,
    pub label_component_6: Option<CharString>,
    pub occupancy: Option<Numeric>,
    pub refinement_flags: Option<RefinementFlag>,
    pub refinement_flags_adp: Option<AdpRefineFlag>,
    pub refinement_flags_occupancy: Option<OccupancyRefineFlag>,
    pub refinement_flags_posn: Option<PosnRefineFlag>,
    pub restraints: Option<CharString>,
    pub site_symmetry_multiplicity: Option<SymMultiplicity>,
    pub site_symmetry_order: Option<Numeric>,
    pub symmetry_multiplicity: Option<SymMultiplicity>,
    pub thermal_displace_type: Option<AdpType>,
    pub type_symbol: Option<CharString>,
    pub u_equiv_geom_mean: Option<Numeric>,
    pub u_iso_or_equiv: Option<Numeric>,
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
