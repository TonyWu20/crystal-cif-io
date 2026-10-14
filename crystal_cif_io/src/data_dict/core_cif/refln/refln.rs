//! The `refln` category of the typed document layer (roadmap D12).
//!
//! The `refln` loop-row struct. One `Option` field per registry
//! column. `CifCategory` is on the `Vec<Refln>` group: it emits one
//! `MultiValues` loop and fills the rows from the columns. The status
//! closed sets keep the typed enums.

use crystal_cif_fmt::grammar::{Tag, Value, Numeric, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::refln::{IncludeStatus, ObservedStatus, RefinementStatus};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct Refln {
    pub a_calc: Option<Numeric>,
    pub a_meas: Option<Numeric>,
    pub b_calc: Option<Numeric>,
    pub b_meas: Option<Numeric>,
    pub class_code: Option<CharString>,
    pub crystal_id: Option<CharString>,
    pub d_spacing: Option<Numeric>,
    pub f_calc: Option<Numeric>,
    pub f_meas: Option<Numeric>,
    pub f_sigma: Option<Numeric>,
    pub f_squared_calc: Option<Numeric>,
    pub f_squared_meas: Option<Numeric>,
    pub f_squared_sigma: Option<Numeric>,
    pub include_status: Option<IncludeStatus>,
    pub index_h: Option<Numeric>,
    pub index_k: Option<Numeric>,
    pub index_l: Option<Numeric>,
    pub intensity_calc: Option<Numeric>,
    pub intensity_meas: Option<Numeric>,
    pub intensity_sigma: Option<Numeric>,
    pub mean_path_length_tbar: Option<Numeric>,
    pub observed_status: Option<ObservedStatus>,
    pub phase_calc: Option<Numeric>,
    pub phase_meas: Option<Numeric>,
    pub refinement_status: Option<RefinementStatus>,
    pub scale_group_code: Option<CharString>,
    pub sint_lambda: Option<Numeric>,
    pub symmetry_epsilon: Option<Numeric>,
    pub symmetry_multiplicity: Option<Numeric>,
    pub wavelength: Option<Numeric>,
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
            columns.push(LoopColumn::new(Tag::new("refln_A_calc".to_string()), values));
        }
        if self.iter().any(|r| r.a_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.a_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_A_meas".to_string()), values));
        }
        if self.iter().any(|r| r.b_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.b_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_B_calc".to_string()), values));
        }
        if self.iter().any(|r| r.b_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.b_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_B_meas".to_string()), values));
        }
        if self.iter().any(|r| r.class_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.class_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_class_code".to_string()), values));
        }
        if self.iter().any(|r| r.crystal_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.crystal_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_crystal_id".to_string()), values));
        }
        if self.iter().any(|r| r.d_spacing.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_spacing {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_d_spacing".to_string()), values));
        }
        if self.iter().any(|r| r.f_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_F_calc".to_string()), values));
        }
        if self.iter().any(|r| r.f_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_F_meas".to_string()), values));
        }
        if self.iter().any(|r| r.f_sigma.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_sigma {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_F_sigma".to_string()), values));
        }
        if self.iter().any(|r| r.f_squared_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_squared_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_F_squared_calc".to_string()), values));
        }
        if self.iter().any(|r| r.f_squared_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_squared_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_F_squared_meas".to_string()), values));
        }
        if self.iter().any(|r| r.f_squared_sigma.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_squared_sigma {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_F_squared_sigma".to_string()), values));
        }
        if self.iter().any(|r| r.include_status.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.include_status {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_include_status".to_string()), values));
        }
        if self.iter().any(|r| r.index_h.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_h {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_index_h".to_string()), values));
        }
        if self.iter().any(|r| r.index_k.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_k {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_index_k".to_string()), values));
        }
        if self.iter().any(|r| r.index_l.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_l {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_index_l".to_string()), values));
        }
        if self.iter().any(|r| r.intensity_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_intensity_calc".to_string()), values));
        }
        if self.iter().any(|r| r.intensity_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_intensity_meas".to_string()), values));
        }
        if self.iter().any(|r| r.intensity_sigma.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_sigma {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_intensity_sigma".to_string()), values));
        }
        if self.iter().any(|r| r.mean_path_length_tbar.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.mean_path_length_tbar {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_mean_path_length_tbar".to_string()), values));
        }
        if self.iter().any(|r| r.observed_status.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.observed_status {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_observed_status".to_string()), values));
        }
        if self.iter().any(|r| r.phase_calc.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.phase_calc {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_phase_calc".to_string()), values));
        }
        if self.iter().any(|r| r.phase_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.phase_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_phase_meas".to_string()), values));
        }
        if self.iter().any(|r| r.refinement_status.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.refinement_status {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_refinement_status".to_string()), values));
        }
        if self.iter().any(|r| r.scale_group_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scale_group_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_scale_group_code".to_string()), values));
        }
        if self.iter().any(|r| r.sint_lambda.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.sint_lambda {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_sint/lambda".to_string()), values));
        }
        if self.iter().any(|r| r.symmetry_epsilon.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.symmetry_epsilon {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_symmetry_epsilon".to_string()), values));
        }
        if self.iter().any(|r| r.symmetry_multiplicity.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.symmetry_multiplicity {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_symmetry_multiplicity".to_string()), values));
        }
        if self.iter().any(|r| r.wavelength.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wavelength {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_wavelength".to_string()), values));
        }
        if self.iter().any(|r| r.wavelength_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wavelength_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refln_wavelength_id".to_string()), values));
        }
        if columns.is_empty() {
            return Vec::new();
        }
        vec![DataBlockMember::DataItems(DataItems::MultiValues(LoopColumns::new(columns)))]
    }

    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
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
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_a_calc") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_a_calc", v) {
                Ok(val) => rows[i].a_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_a_meas") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_a_meas", v) {
                Ok(val) => rows[i].a_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_b_calc") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_b_calc", v) {
                Ok(val) => rows[i].b_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_b_meas") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_b_meas", v) {
                Ok(val) => rows[i].b_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_class_code") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_class_code", v) {
                Ok(val) => rows[i].class_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_crystal_id") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_crystal_id", v) {
                Ok(val) => rows[i].crystal_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_d_spacing") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_d_spacing", v) {
                Ok(val) => rows[i].d_spacing = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_f_calc") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_calc", v) {
                Ok(val) => rows[i].f_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_f_meas") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_meas", v) {
                Ok(val) => rows[i].f_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_f_sigma") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_sigma", v) {
                Ok(val) => rows[i].f_sigma = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_f_squared_calc") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_squared_calc", v) {
                Ok(val) => rows[i].f_squared_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_f_squared_meas") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_squared_meas", v) {
                Ok(val) => rows[i].f_squared_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_f_squared_sigma") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_f_squared_sigma", v) {
                Ok(val) => rows[i].f_squared_sigma = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_include_status") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("refln_include_status", v, |c| IncludeStatus::try_from(c).ok(), "IncludeStatus") {
                Ok(val) => rows[i].include_status = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_index_h") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_index_h", v) {
                Ok(val) => rows[i].index_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_index_k") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_index_k", v) {
                Ok(val) => rows[i].index_k = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_index_l") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_index_l", v) {
                Ok(val) => rows[i].index_l = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_intensity_calc") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_intensity_calc", v) {
                Ok(val) => rows[i].intensity_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_intensity_meas") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_intensity_meas", v) {
                Ok(val) => rows[i].intensity_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_intensity_sigma") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_intensity_sigma", v) {
                Ok(val) => rows[i].intensity_sigma = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_mean_path_length_tbar") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_mean_path_length_tbar", v) {
                Ok(val) => rows[i].mean_path_length_tbar = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_observed_status") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("refln_observed_status", v, |c| ObservedStatus::try_from(c).ok(), "ObservedStatus") {
                Ok(val) => rows[i].observed_status = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_phase_calc") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_phase_calc", v) {
                Ok(val) => rows[i].phase_calc = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_phase_meas") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_phase_meas", v) {
                Ok(val) => rows[i].phase_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_refinement_status") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("refln_refinement_status", v, |c| RefinementStatus::try_from(c).ok(), "RefinementStatus") {
                Ok(val) => rows[i].refinement_status = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_scale_group_code") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_scale_group_code", v) {
                Ok(val) => rows[i].scale_group_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_sint/lambda") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_sint/lambda", v) {
                Ok(val) => rows[i].sint_lambda = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_symmetry_epsilon") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_symmetry_epsilon", v) {
                Ok(val) => rows[i].symmetry_epsilon = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_symmetry_multiplicity") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_symmetry_multiplicity", v) {
                Ok(val) => rows[i].symmetry_multiplicity = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_wavelength") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refln_wavelength", v) {
                Ok(val) => rows[i].wavelength = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refln_wavelength_id") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refln_wavelength_id", v) {
                Ok(val) => rows[i].wavelength_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
