//! The `refln` category: the loop items that describe one
//! reflection (roadmap T2, DDL1 `cif_core.dic`).
//!
//! All 31 items are loop columns. Three items carry closed code
//! sets; the rest are open `char` or `numb` values.

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};

mod codes;
mod refln;
pub use refln::Refln;

pub use codes::{IncludeStatus, ObservedStatus, RefinementStatus, ReflnCifError};

/// The loop items of the `refln` category. Each variant holds one
/// column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `refln` category: one variant per

/// registry loop-value item, in item order.
pub enum ReflnLoopItem {
    /// The `_refln_A_calc` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component A (in
    /// electrons for X-ray diffraction). A =|F|cos(phase)
    A_calc(Vec<Numeric>),
    /// The `_refln_A_meas` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component A (in
    /// electrons for X-ray diffraction). A =|F|cos(phase)
    A_meas(Vec<Numeric>),
    /// The `_refln_B_calc` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component B (in
    /// electrons for X-ray diffraction). B =|F|sin(phase)
    B_calc(Vec<Numeric>),
    /// The `_refln_B_meas` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component B (in
    /// electrons for X-ray diffraction). B =|F|sin(phase)
    B_meas(Vec<Numeric>),
    /// The `_refln_F_calc` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the structure factors (in electrons for X-ray
    /// diffraction).
    F_calc(Vec<Numeric>),
    /// The `_refln_F_meas` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the structure factors (in electrons for X-ray
    /// diffraction).
    F_meas(Vec<Numeric>),
    /// The `_refln_F_sigma` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the structure factors (in electrons for X-ray
    /// diffraction).
    F_sigma(Vec<Numeric>),
    /// The `_refln_F_squared_calc` item. The variant holds the column values, one per loop row.
    ///
    /// Calculated, measured and estimated standard uncertainty (derived
    /// from measurement) of the squared structure factors (in electrons
    /// squared for X-ray diffraction).
    F_squared_calc(Vec<Numeric>),
    /// The `_refln_F_squared_meas` item. The variant holds the column values, one per loop row.
    ///
    /// Calculated, measured and estimated standard uncertainty (derived
    /// from measurement) of the squared structure factors (in electrons
    /// squared for X-ray diffraction).
    F_squared_meas(Vec<Numeric>),
    /// The `_refln_F_squared_sigma` item. The variant holds the column values, one per loop row.
    ///
    /// Calculated, measured and estimated standard uncertainty (derived
    /// from measurement) of the squared structure factors (in electrons
    /// squared for X-ray diffraction).
    F_squared_sigma(Vec<Numeric>),
    /// The `_refln_class_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying the class to which this reflection has been
    /// assigned. This code must match a value of _reflns_class_code.
    /// Reflections may be grouped into classes for a variety of purposes.
    /// For example, for modulated structures each reflection class may be
    /// defined by the number m=sum|m~i~|, where the m~i~ are the integer
    /// coefficients that, in addition to h,k,l, index the corresponding
    /// diffraction vector in the basis defined for the reciprocal lattice.
    Class_code(Vec<CharString>),
    /// The `_refln_crystal_id` item. The variant holds the column values, one per loop row.
    ///
    /// Code identifying each crystal if multiple crystals are used. Is used
    /// to link with _exptl_crystal_id in the _exptl_crystal_ list.
    Crystal_id(Vec<CharString>),
    /// The `_refln_d_spacing` item. The variant holds the column values, one per loop row.
    ///
    /// The d spacing in angstroms for this reflection. This is related to
    /// the (sin theta)/lambda value by the expression _refln_d_spacing =
    /// 2/(_refln_sint/lambda)
    /// Units: A (angstrom).
    D_spacing(Vec<Numeric>),
    /// The `_refln_include_status` item. The variant holds the column values, one per loop row.
    ///
    /// Classification of a reflection indicating its status with respect to
    /// inclusion in the refinement and the calculation of R factors.
    Include_status(Vec<IncludeStatus>),
    /// The `_refln_index_h` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of the reflection. The values of the Miller indices
    /// in the REFLN category must correspond to the cell defined by the
    /// cell lengths and cell angles in the CELL category.
    Index_h(Vec<Numeric>),
    /// The `_refln_index_k` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of the reflection. The values of the Miller indices
    /// in the REFLN category must correspond to the cell defined by the
    /// cell lengths and cell angles in the CELL category.
    Index_k(Vec<Numeric>),
    /// The `_refln_index_l` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of the reflection. The values of the Miller indices
    /// in the REFLN category must correspond to the cell defined by the
    /// cell lengths and cell angles in the CELL category.
    Index_l(Vec<Numeric>),
    /// The `_refln_intensity_calc` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the intensity, all in the same arbitrary units as
    /// _refln_intensity_meas.
    Intensity_calc(Vec<Numeric>),
    /// The `_refln_intensity_meas` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the intensity, all in the same arbitrary units as
    /// _refln_intensity_meas.
    Intensity_meas(Vec<Numeric>),
    /// The `_refln_intensity_sigma` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the intensity, all in the same arbitrary units as
    /// _refln_intensity_meas.
    Intensity_sigma(Vec<Numeric>),
    /// The `_refln_mean_path_length_tbar` item. The variant holds the column values, one per loop row.
    ///
    /// Mean path length in millimetres through the crystal for this
    /// reflection.
    /// Units: mm (millimetre).
    Mean_path_length_tbar(Vec<Numeric>),
    /// The `_refln_observed_status` item. The variant holds the column values, one per loop row.
    ///
    /// Classification of a reflection indicating its status with respect to
    /// inclusion in the refinement and the calculation of R factors.
    Observed_status(Vec<ObservedStatus>),
    /// The `_refln_phase_calc` item. The variant holds the column values, one per loop row.
    ///
    /// The calculated structure-factor phase in degrees.
    /// Units: deg (degree of arc).
    Phase_calc(Vec<Numeric>),
    /// The `_refln_phase_meas` item. The variant holds the column values, one per loop row.
    ///
    /// The measured structure-factor phase in degrees.
    /// Units: deg (degree of arc).
    Phase_meas(Vec<Numeric>),
    /// The `_refln_refinement_status` item. The variant holds the column values, one per loop row.
    ///
    /// Status of a reflection in the structure-refinement process.
    Refinement_status(Vec<RefinementStatus>),
    /// The `_refln_scale_group_code` item. The variant holds the column values, one per loop row.
    ///
    /// Code identifying the structure-factor scale. This code must
    /// correspond to one of the _reflns_scale_group_code values.
    Scale_group_code(Vec<CharString>),
    /// The `_refln_sint/lambda` item. The variant holds the column values, one per loop row.
    ///
    /// The (sin theta)/lambda value in reciprocal angstroms for this
    /// reflection.
    /// Units: A^-1^ (reciprocal angstrom).
    Sint_lambda(Vec<Numeric>),
    /// The `_refln_symmetry_epsilon` item. The variant holds the column values, one per loop row.
    ///
    /// The symmetry reinforcement factor corresponding to the number of
    /// times the reflection indices are generated identically from the
    /// space-group symmetry operations.
    Symmetry_epsilon(Vec<Numeric>),
    /// The `_refln_symmetry_multiplicity` item. The variant holds the column values, one per loop row.
    ///
    /// The number of reflections symmetry-equivalent under the Laue
    /// symmetry to the present reflection. In the Laue symmetry, Friedel
    /// opposites (h k l and -h -k -l) are equivalent. Tables of
    /// symmetry-equivalent reflections are available in International
    /// Tables for Crystallography Volume A (2002), Chapter 10.1.
    Symmetry_multiplicity(Vec<Numeric>),
    /// The `_refln_wavelength` item. The variant holds the column values, one per loop row.
    ///
    /// The mean wavelength in angstroms of the radiation used to measure
    /// this reflection. This is an important parameter for data collected
    /// using energy-dispersive detectors or the Laue method.
    /// Units: A (angstrom).
    Wavelength(Vec<Numeric>),
    /// The `_refln_wavelength_id` item. The variant holds the column values, one per loop row.
    ///
    /// Code identifying the wavelength in the _diffrn_radiation_ list. See
    /// _diffrn_radiation_wavelength_id.
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

// The T6 single-value items of the `refln` category.

/// The single-value items of the `refln` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `refln` category: one variant per

/// registry single-value item, in item order.
pub enum ReflnTerms {
    /// The dictionary contents are Complex; the value layer has
    /// no complex type (roadmap section 9), so the item binds
    /// the Numeric payload.
    /// The `_refln_F_complex` item. The variant holds the column values, one per loop row.
    F_complex(Numeric),
    /// The `_refln_fom` item. The variant holds the column values, one per loop row.
    Fom(Numeric),
    /// The dictionary records a table; the value layer has no
    /// table type (roadmap section 9), so the item binds the
    /// Numeric payload.
    /// The `_refln_form_factor_table` item. The variant holds the column values, one per loop row.
    Form_factor_table(Numeric),
    /// The shell records a 3-vector (`_type.dimension` `[3]`);
    /// the `Matrix` payload holds the row list.
    /// The `_refln_hkl` item. The variant holds the column values, one per loop row.
    Hkl(Matrix),
    /// The `_refln_id` item. The variant holds the column values, one per loop row.
    Id(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0.0:`); the plain numeric form is kept.
    /// The `_refln_Lp_factor` item. The variant holds the column values, one per loop row.
    Lp_factor(Numeric),
}

impl CifTerm for ReflnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnTerms::F_complex(_) => "F_complex",
            ReflnTerms::Fom(_) => "fom",
            ReflnTerms::Form_factor_table(_) => "form_factor_table",
            ReflnTerms::Hkl(_) => "hkl",
            ReflnTerms::Id(_) => "id",
            ReflnTerms::Lp_factor(_) => "Lp_factor",
        };
        Tag::new(format!("refln_{suffix}"))
    }
}

impl SingleValueTerm for ReflnTerms {
    fn value(&self) -> Value {
        match self {
            ReflnTerms::F_complex(n)
            | ReflnTerms::Fom(n)
            | ReflnTerms::Form_factor_table(n)
            | ReflnTerms::Lp_factor(n) => Value::Numeric(*n),
            ReflnTerms::Hkl(m) => matrix::to_value(m),
            ReflnTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for ReflnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
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
    use crystal_cif_fmt::grammar::UnquotedString;

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
