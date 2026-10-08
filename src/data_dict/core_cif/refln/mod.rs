//! The `refln` category: the loop items that describe one
//! reflection (roadmap T2, DDL1 `cif_core.dic`).
//!
//! All 31 items are loop columns. Three items carry closed code
//! sets; the rest are open `char` or `numb` values.

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Numeric, Tag, Value},
};

mod codes;

pub use codes::{IncludeStatus, ObservedStatus, RefinementStatus, ReflnCifError};

/// The loop items of the `refln` category. Each variant holds one
/// column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ReflnLoopItem {
    A_calc(Vec<Numeric>),
    A_meas(Vec<Numeric>),
    B_calc(Vec<Numeric>),
    B_meas(Vec<Numeric>),
    F_calc(Vec<Numeric>),
    F_meas(Vec<Numeric>),
    F_sigma(Vec<Numeric>),
    F_squared_calc(Vec<Numeric>),
    F_squared_meas(Vec<Numeric>),
    F_squared_sigma(Vec<Numeric>),
    Class_code(Vec<CharString>),
    Crystal_id(Vec<CharString>),
    D_spacing(Vec<Numeric>),
    Include_status(Vec<IncludeStatus>),
    Index_h(Vec<Numeric>),
    Index_k(Vec<Numeric>),
    Index_l(Vec<Numeric>),
    Intensity_calc(Vec<Numeric>),
    Intensity_meas(Vec<Numeric>),
    Intensity_sigma(Vec<Numeric>),
    Mean_path_length_tbar(Vec<Numeric>),
    Observed_status(Vec<ObservedStatus>),
    Phase_calc(Vec<Numeric>),
    Phase_meas(Vec<Numeric>),
    Refinement_status(Vec<RefinementStatus>),
    Scale_group_code(Vec<CharString>),
    Sint_lambda(Vec<Numeric>),
    Symmetry_epsilon(Vec<Numeric>),
    Symmetry_multiplicity(Vec<Numeric>),
    Wavelength(Vec<Numeric>),
    Wavelength_id(Vec<CharString>),
}

impl CifTerm for ReflnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnLoopItem::A_calc(_) => "A_calc",
            ReflnLoopItem::A_meas(_) => "A_meas",
            ReflnLoopItem::B_calc(_) => "B_calc",
            ReflnLoopItem::B_meas(_) => "B_meas",
            ReflnLoopItem::F_calc(_) => "F_calc",
            ReflnLoopItem::F_meas(_) => "F_meas",
            ReflnLoopItem::F_sigma(_) => "F_sigma",
            ReflnLoopItem::F_squared_calc(_) => "F_squared_calc",
            ReflnLoopItem::F_squared_meas(_) => "F_squared_meas",
            ReflnLoopItem::F_squared_sigma(_) => "F_squared_sigma",
            ReflnLoopItem::Class_code(_) => "class_code",
            ReflnLoopItem::Crystal_id(_) => "crystal_id",
            ReflnLoopItem::D_spacing(_) => "d_spacing",
            ReflnLoopItem::Include_status(_) => "include_status",
            ReflnLoopItem::Index_h(_) => "index_h",
            ReflnLoopItem::Index_k(_) => "index_k",
            ReflnLoopItem::Index_l(_) => "index_l",
            ReflnLoopItem::Intensity_calc(_) => "intensity_calc",
            ReflnLoopItem::Intensity_meas(_) => "intensity_meas",
            ReflnLoopItem::Intensity_sigma(_) => "intensity_sigma",
            ReflnLoopItem::Mean_path_length_tbar(_) => "mean_path_length_tbar",
            ReflnLoopItem::Observed_status(_) => "observed_status",
            ReflnLoopItem::Phase_calc(_) => "phase_calc",
            ReflnLoopItem::Phase_meas(_) => "phase_meas",
            ReflnLoopItem::Refinement_status(_) => "refinement_status",
            ReflnLoopItem::Scale_group_code(_) => "scale_group_code",
            ReflnLoopItem::Sint_lambda(_) => "sint/lambda",
            ReflnLoopItem::Symmetry_epsilon(_) => "symmetry_epsilon",
            ReflnLoopItem::Symmetry_multiplicity(_) => "symmetry_multiplicity",
            ReflnLoopItem::Wavelength(_) => "wavelength",
            ReflnLoopItem::Wavelength_id(_) => "wavelength_id",
        };
        Tag::new(format!("refln_{suffix}"))
    }
}

impl LoopValueTerm for ReflnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ReflnLoopItem::A_calc(v)
            | ReflnLoopItem::A_meas(v)
            | ReflnLoopItem::B_calc(v)
            | ReflnLoopItem::B_meas(v)
            | ReflnLoopItem::F_calc(v)
            | ReflnLoopItem::F_meas(v)
            | ReflnLoopItem::F_sigma(v)
            | ReflnLoopItem::F_squared_calc(v)
            | ReflnLoopItem::F_squared_meas(v)
            | ReflnLoopItem::F_squared_sigma(v)
            | ReflnLoopItem::D_spacing(v)
            | ReflnLoopItem::Index_h(v)
            | ReflnLoopItem::Index_k(v)
            | ReflnLoopItem::Index_l(v)
            | ReflnLoopItem::Intensity_calc(v)
            | ReflnLoopItem::Intensity_meas(v)
            | ReflnLoopItem::Intensity_sigma(v)
            | ReflnLoopItem::Mean_path_length_tbar(v)
            | ReflnLoopItem::Phase_calc(v)
            | ReflnLoopItem::Phase_meas(v)
            | ReflnLoopItem::Sint_lambda(v)
            | ReflnLoopItem::Symmetry_epsilon(v)
            | ReflnLoopItem::Symmetry_multiplicity(v)
            | ReflnLoopItem::Wavelength(v) => v.iter().cloned().map(Value::from).collect(),
            ReflnLoopItem::Class_code(v)
            | ReflnLoopItem::Crystal_id(v)
            | ReflnLoopItem::Scale_group_code(v)
            | ReflnLoopItem::Wavelength_id(v) => v.iter().cloned().map(Value::from).collect(),
            ReflnLoopItem::Include_status(v) => v.iter().cloned().map(Value::from).collect(),
            ReflnLoopItem::Observed_status(v) => v.iter().cloned().map(Value::from).collect(),
            ReflnLoopItem::Refinement_status(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

impl Display for ReflnLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::UnquotedString;

    fn cs() -> CharString {
        CharString::Unquoted(UnquotedString::new("1".into()))
    }

    fn n() -> Numeric {
        Numeric::from(1.5)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<ReflnLoopItem> = vec![
            ReflnLoopItem::A_calc(vec![n()]),
            ReflnLoopItem::A_meas(vec![n()]),
            ReflnLoopItem::B_calc(vec![n()]),
            ReflnLoopItem::B_meas(vec![n()]),
            ReflnLoopItem::F_calc(vec![n()]),
            ReflnLoopItem::F_meas(vec![n()]),
            ReflnLoopItem::F_sigma(vec![n()]),
            ReflnLoopItem::F_squared_calc(vec![n()]),
            ReflnLoopItem::F_squared_meas(vec![n()]),
            ReflnLoopItem::F_squared_sigma(vec![n()]),
            ReflnLoopItem::Class_code(vec![cs()]),
            ReflnLoopItem::Crystal_id(vec![cs()]),
            ReflnLoopItem::D_spacing(vec![n()]),
            ReflnLoopItem::Include_status(vec![IncludeStatus::Observed]),
            ReflnLoopItem::Index_h(vec![n()]),
            ReflnLoopItem::Index_k(vec![n()]),
            ReflnLoopItem::Index_l(vec![n()]),
            ReflnLoopItem::Intensity_calc(vec![n()]),
            ReflnLoopItem::Intensity_meas(vec![n()]),
            ReflnLoopItem::Intensity_sigma(vec![n()]),
            ReflnLoopItem::Mean_path_length_tbar(vec![n()]),
            ReflnLoopItem::Observed_status(vec![ObservedStatus::Observed]),
            ReflnLoopItem::Phase_calc(vec![n()]),
            ReflnLoopItem::Phase_meas(vec![n()]),
            ReflnLoopItem::Refinement_status(vec![RefinementStatus::Included]),
            ReflnLoopItem::Scale_group_code(vec![cs()]),
            ReflnLoopItem::Sint_lambda(vec![n()]),
            ReflnLoopItem::Symmetry_epsilon(vec![n()]),
            ReflnLoopItem::Symmetry_multiplicity(vec![n()]),
            ReflnLoopItem::Wavelength(vec![n()]),
            ReflnLoopItem::Wavelength_id(vec![cs()]),
        ];
        let expected: Vec<&str> = vec![
            "refln_A_calc",
            "refln_A_meas",
            "refln_B_calc",
            "refln_B_meas",
            "refln_F_calc",
            "refln_F_meas",
            "refln_F_sigma",
            "refln_F_squared_calc",
            "refln_F_squared_meas",
            "refln_F_squared_sigma",
            "refln_class_code",
            "refln_crystal_id",
            "refln_d_spacing",
            "refln_include_status",
            "refln_index_h",
            "refln_index_k",
            "refln_index_l",
            "refln_intensity_calc",
            "refln_intensity_meas",
            "refln_intensity_sigma",
            "refln_mean_path_length_tbar",
            "refln_observed_status",
            "refln_phase_calc",
            "refln_phase_meas",
            "refln_refinement_status",
            "refln_scale_group_code",
            "refln_sint/lambda",
            "refln_symmetry_epsilon",
            "refln_symmetry_multiplicity",
            "refln_wavelength",
            "refln_wavelength_id",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "refln tag drift");
        }
    }
}
