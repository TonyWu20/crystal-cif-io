//! The `refln` category of the typed document layer (roadmap D12).
//!
//! The `refln` loop-row struct. One `Option` field per registry
//! column. `CifCategory` is on the `Vec<Refln>` group: it emits one
//! `MultiValues` loop and fills the rows from the columns. The status
//! closed sets keep the typed enums.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::refln::{IncludeStatus, ObservedStatus, RefinementStatus};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
/// The `refln` category struct. One field per registry item.
pub struct Refln {
    /// The `_refln_A_calc` item. The field holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component A (in
    /// electrons for X-ray diffraction). A =|F|cos(phase)
    pub a_calc: Option<Numeric>,
    /// The `_refln_A_meas` item. The field holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component A (in
    /// electrons for X-ray diffraction). A =|F|cos(phase)
    pub a_meas: Option<Numeric>,
    /// The `_refln_B_calc` item. The field holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component B (in
    /// electrons for X-ray diffraction). B =|F|sin(phase)
    pub b_calc: Option<Numeric>,
    /// The `_refln_B_meas` item. The field holds the column values, one per loop row.
    ///
    /// The calculated and measured structure-factor component B (in
    /// electrons for X-ray diffraction). B =|F|sin(phase)
    pub b_meas: Option<Numeric>,
    /// The `_refln_class_code` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying the class to which this reflection has been
    /// assigned. This code must match a value of _reflns_class_code.
    /// Reflections may be grouped into classes for a variety of purposes.
    /// For example, for modulated structures each reflection class may be
    /// defined by the number m=sum|m~i~|, where the m~i~ are the integer
    /// coefficients that, in addition to h,k,l, index the corresponding
    /// diffraction vector in the basis defined for the reciprocal lattice.
    pub class_code: Option<CharString>,
    /// The `_refln_crystal_id` item. The field holds the column values, one per loop row.
    ///
    /// Code identifying each crystal if multiple crystals are used. Is used
    /// to link with _exptl_crystal_id in the _exptl_crystal_ list.
    pub crystal_id: Option<CharString>,
    /// The `_refln_d_spacing` item. The field holds the column values, one per loop row.
    ///
    /// The d spacing in angstroms for this reflection. This is related to
    /// the (sin theta)/lambda value by the expression _refln_d_spacing =
    /// 2/(_refln_sint/lambda)
    /// Units: A (angstrom).
    pub d_spacing: Option<Numeric>,
    /// The `_refln_F_calc` item. The field holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the structure factors (in electrons for X-ray
    /// diffraction).
    pub f_calc: Option<Numeric>,
    /// The `_refln_F_meas` item. The field holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the structure factors (in electrons for X-ray
    /// diffraction).
    pub f_meas: Option<Numeric>,
    /// The `_refln_F_sigma` item. The field holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the structure factors (in electrons for X-ray
    /// diffraction).
    pub f_sigma: Option<Numeric>,
    /// The `_refln_F_squared_calc` item. The field holds the column values, one per loop row.
    ///
    /// Calculated, measured and estimated standard uncertainty (derived
    /// from measurement) of the squared structure factors (in electrons
    /// squared for X-ray diffraction).
    pub f_squared_calc: Option<Numeric>,
    /// The `_refln_F_squared_meas` item. The field holds the column values, one per loop row.
    ///
    /// Calculated, measured and estimated standard uncertainty (derived
    /// from measurement) of the squared structure factors (in electrons
    /// squared for X-ray diffraction).
    pub f_squared_meas: Option<Numeric>,
    /// The `_refln_F_squared_sigma` item. The field holds the column values, one per loop row.
    ///
    /// Calculated, measured and estimated standard uncertainty (derived
    /// from measurement) of the squared structure factors (in electrons
    /// squared for X-ray diffraction).
    pub f_squared_sigma: Option<Numeric>,
    /// The `_refln_include_status` item. The field holds the column values, one per loop row.
    ///
    /// Classification of a reflection indicating its status with respect to
    /// inclusion in the refinement and the calculation of R factors.
    pub include_status: Option<IncludeStatus>,
    /// The `_refln_index_h` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of the reflection. The values of the Miller indices
    /// in the REFLN category must correspond to the cell defined by the
    /// cell lengths and cell angles in the CELL category.
    pub index_h: Option<Numeric>,
    /// The `_refln_index_k` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of the reflection. The values of the Miller indices
    /// in the REFLN category must correspond to the cell defined by the
    /// cell lengths and cell angles in the CELL category.
    pub index_k: Option<Numeric>,
    /// The `_refln_index_l` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of the reflection. The values of the Miller indices
    /// in the REFLN category must correspond to the cell defined by the
    /// cell lengths and cell angles in the CELL category.
    pub index_l: Option<Numeric>,
    /// The `_refln_intensity_calc` item. The field holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the intensity, all in the same arbitrary units as
    /// _refln_intensity_meas.
    pub intensity_calc: Option<Numeric>,
    /// The `_refln_intensity_meas` item. The field holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the intensity, all in the same arbitrary units as
    /// _refln_intensity_meas.
    pub intensity_meas: Option<Numeric>,
    /// The `_refln_intensity_sigma` item. The field holds the column values, one per loop row.
    ///
    /// The calculated, measured and standard uncertainty (derived from
    /// measurement) of the intensity, all in the same arbitrary units as
    /// _refln_intensity_meas.
    pub intensity_sigma: Option<Numeric>,
    /// The `_refln_mean_path_length_tbar` item. The field holds the column values, one per loop row.
    ///
    /// Mean path length in millimetres through the crystal for this
    /// reflection.
    /// Units: mm (millimetre).
    pub mean_path_length_tbar: Option<Numeric>,
    /// The `_refln_observed_status` item. The field holds the column values, one per loop row.
    ///
    /// Classification of a reflection indicating its status with respect to
    /// inclusion in the refinement and the calculation of R factors.
    pub observed_status: Option<ObservedStatus>,
    /// The `_refln_phase_calc` item. The field holds the column values, one per loop row.
    ///
    /// The calculated structure-factor phase in degrees.
    /// Units: deg (degree of arc).
    pub phase_calc: Option<Numeric>,
    /// The `_refln_phase_meas` item. The field holds the column values, one per loop row.
    ///
    /// The measured structure-factor phase in degrees.
    /// Units: deg (degree of arc).
    pub phase_meas: Option<Numeric>,
    /// The `_refln_refinement_status` item. The field holds the column values, one per loop row.
    ///
    /// Status of a reflection in the structure-refinement process.
    pub refinement_status: Option<RefinementStatus>,
    /// The `_refln_scale_group_code` item. The field holds the column values, one per loop row.
    ///
    /// Code identifying the structure-factor scale. This code must
    /// correspond to one of the _reflns_scale_group_code values.
    pub scale_group_code: Option<CharString>,
    /// The `_refln_sint/lambda` item. The field holds the column values, one per loop row.
    ///
    /// The (sin theta)/lambda value in reciprocal angstroms for this
    /// reflection.
    /// Units: A^-1^ (reciprocal angstrom).
    pub sint_lambda: Option<Numeric>,
    /// The `_refln_symmetry_epsilon` item. The field holds the column values, one per loop row.
    ///
    /// The symmetry reinforcement factor corresponding to the number of
    /// times the reflection indices are generated identically from the
    /// space-group symmetry operations.
    pub symmetry_epsilon: Option<Numeric>,
    /// The `_refln_symmetry_multiplicity` item. The field holds the column values, one per loop row.
    ///
    /// The number of reflections symmetry-equivalent under the Laue
    /// symmetry to the present reflection. In the Laue symmetry, Friedel
    /// opposites (h k l and -h -k -l) are equivalent. Tables of
    /// symmetry-equivalent reflections are available in International
    /// Tables for Crystallography Volume A (2002), Chapter 10.1.
    pub symmetry_multiplicity: Option<Numeric>,
    /// The `_refln_wavelength` item. The field holds the column values, one per loop row.
    ///
    /// The mean wavelength in angstroms of the radiation used to measure
    /// this reflection. This is an important parameter for data collected
    /// using energy-dispersive detectors or the Laue method.
    /// Units: A (angstrom).
    pub wavelength: Option<Numeric>,
    /// The `_refln_wavelength_id` item. The field holds the column values, one per loop row.
    ///
    /// Code identifying the wavelength in the _diffrn_radiation_ list. See
    /// _diffrn_radiation_wavelength_id.
    pub wavelength_id: Option<CharString>,
}

impl CifCategory for Vec<Refln> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.a_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.a_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_A_calc".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.a_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.a_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_A_meas".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.b_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.b_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_B_calc".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.b_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.b_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_B_meas".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.class_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.class_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_class_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.crystal_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.crystal_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_crystal_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.d_spacing.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_spacing {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_d_spacing".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.f_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_F_calc".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.f_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_F_meas".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.f_sigma.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_sigma {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_F_sigma".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.f_squared_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_squared_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_F_squared_calc".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.f_squared_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_squared_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_F_squared_meas".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.f_squared_sigma.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_squared_sigma {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_F_squared_sigma".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.include_status.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.include_status {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_include_status".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_h.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_h {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_index_h".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_k.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_k {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_index_k".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_l.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_l {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_index_l".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.intensity_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_intensity_calc".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.intensity_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_intensity_meas".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.intensity_sigma.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_sigma {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_intensity_sigma".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.mean_path_length_tbar.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.mean_path_length_tbar {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_mean_path_length_tbar".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.observed_status.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.observed_status {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_observed_status".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.phase_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.phase_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_phase_calc".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.phase_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.phase_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_phase_meas".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.refinement_status.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.refinement_status {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_refinement_status".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scale_group_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scale_group_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_scale_group_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.sint_lambda.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.sint_lambda {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_sint/lambda".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.symmetry_epsilon.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.symmetry_epsilon {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_symmetry_epsilon".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.symmetry_multiplicity.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.symmetry_multiplicity {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_symmetry_multiplicity".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.wavelength.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wavelength {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_wavelength".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.wavelength_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wavelength_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refln_wavelength_id".to_string()),
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
                    // A loop is this category only when it carries one of
                    // the category column tags. A sibling loop of another
                    // category stays in the residue.
                    let ours = [
                        "refln_a_calc",
                        "refln_a_meas",
                        "refln_b_calc",
                        "refln_b_meas",
                        "refln_class_code",
                        "refln_crystal_id",
                        "refln_d_spacing",
                        "refln_f_calc",
                        "refln_f_meas",
                        "refln_f_sigma",
                        "refln_f_squared_calc",
                        "refln_f_squared_meas",
                        "refln_f_squared_sigma",
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
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = refln_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn refln_rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<Refln> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![Refln::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_a_calc")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_a_calc", v) {
                Ok(val) => rows[i].a_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_a_meas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_a_meas", v) {
                Ok(val) => rows[i].a_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_b_calc")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_b_calc", v) {
                Ok(val) => rows[i].b_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_b_meas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_b_meas", v) {
                Ok(val) => rows[i].b_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_class_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_class_code", v) {
                Ok(val) => rows[i].class_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_crystal_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_crystal_id", v) {
                Ok(val) => rows[i].crystal_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_d_spacing")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_d_spacing", v) {
                Ok(val) => rows[i].d_spacing = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_f_calc")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_calc", v) {
                Ok(val) => rows[i].f_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_f_meas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_meas", v) {
                Ok(val) => rows[i].f_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_f_sigma")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_sigma", v) {
                Ok(val) => rows[i].f_sigma = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_f_squared_calc")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_squared_calc", v) {
                Ok(val) => rows[i].f_squared_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_f_squared_meas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_squared_meas", v) {
                Ok(val) => rows[i].f_squared_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_f_squared_sigma")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_squared_sigma", v) {
                Ok(val) => rows[i].f_squared_sigma = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_include_status")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "refln_include_status",
                v,
                |c| IncludeStatus::try_from(c).ok(),
                "IncludeStatus",
            ) {
                Ok(val) => rows[i].include_status = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_index_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_index_h", v) {
                Ok(val) => rows[i].index_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_index_k")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_index_k", v) {
                Ok(val) => rows[i].index_k = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_index_l")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_index_l", v) {
                Ok(val) => rows[i].index_l = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_intensity_calc")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_intensity_calc", v) {
                Ok(val) => rows[i].intensity_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_intensity_meas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_intensity_meas", v) {
                Ok(val) => rows[i].intensity_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_intensity_sigma")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_intensity_sigma", v) {
                Ok(val) => rows[i].intensity_sigma = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_mean_path_length_tbar")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_mean_path_length_tbar", v) {
                Ok(val) => rows[i].mean_path_length_tbar = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_observed_status")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "refln_observed_status",
                v,
                |c| ObservedStatus::try_from(c).ok(),
                "ObservedStatus",
            ) {
                Ok(val) => rows[i].observed_status = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_phase_calc")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_phase_calc", v) {
                Ok(val) => rows[i].phase_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_phase_meas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_phase_meas", v) {
                Ok(val) => rows[i].phase_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_refinement_status")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "refln_refinement_status",
                v,
                |c| RefinementStatus::try_from(c).ok(),
                "RefinementStatus",
            ) {
                Ok(val) => rows[i].refinement_status = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_scale_group_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_scale_group_code", v) {
                Ok(val) => rows[i].scale_group_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_sint/lambda")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_sint/lambda", v) {
                Ok(val) => rows[i].sint_lambda = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_symmetry_epsilon")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_symmetry_epsilon", v) {
                Ok(val) => rows[i].symmetry_epsilon = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_symmetry_multiplicity")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_symmetry_multiplicity", v) {
                Ok(val) => rows[i].symmetry_multiplicity = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_wavelength")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_wavelength", v) {
                Ok(val) => rows[i].wavelength = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refln_wavelength_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_wavelength_id", v) {
                Ok(val) => rows[i].wavelength_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
