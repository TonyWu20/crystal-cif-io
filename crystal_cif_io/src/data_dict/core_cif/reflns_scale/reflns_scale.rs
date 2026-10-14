//! The `reflns_scale` category of the typed document layer (roadmap D12).
//!
//! The `reflns_scale` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<ReflnsScale>` group:
//! it emits one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
pub struct ReflnsScale {
    pub group_code: Option<CharString>,
    pub meas_f: Option<Numeric>,
    pub meas_f_squared: Option<Numeric>,
    pub meas_intensity: Option<Numeric>,
}

impl CifCategory for Vec<ReflnsScale> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.group_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.group_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_scale_group_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meas_f.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meas_f {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_scale_meas_F".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meas_f_squared.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meas_f_squared {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_scale_meas_F_squared".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meas_intensity.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meas_intensity {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_scale_meas_intensity".to_string()),
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
                        "reflns_scale_group_code",
                        "reflns_scale_meas_f",
                        "reflns_scale_meas_f_squared",
                        "reflns_scale_meas_intensity",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = reflnsscale_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn reflnsscale_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<ReflnsScale> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ReflnsScale::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_scale_group_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("reflns_scale_group_code", v) {
                Ok(val) => rows[i].group_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_scale_meas_f")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_scale_meas_f", v) {
                Ok(val) => rows[i].meas_f = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_scale_meas_f_squared")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_scale_meas_f_squared", v) {
                Ok(val) => rows[i].meas_f_squared = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_scale_meas_intensity")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_scale_meas_intensity", v) {
                Ok(val) => rows[i].meas_intensity = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
