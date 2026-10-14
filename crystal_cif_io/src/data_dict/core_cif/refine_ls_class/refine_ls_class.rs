//! The `refine_ls_class` category of the typed document layer (roadmap D12).
//!
//! The `refine_ls_class` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<RefineLsClass>`
//! group: it emits one `MultiValues` loop and fills the rows from the
//! columns.

use crystal_cif_fmt::grammar::{Tag, Value, Numeric, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct RefineLsClass {
    pub code: Option<CharString>,
    pub d_res_high: Option<Numeric>,
    pub d_res_low: Option<Numeric>,
    pub r_factor_all: Option<Numeric>,
    pub r_factor_gt: Option<Numeric>,
    pub r_fsqd_factor: Option<Numeric>,
    pub r_i_factor: Option<Numeric>,
    pub wr_factor_all: Option<Numeric>,
}

impl CifCategory for Vec<RefineLsClass> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_code".to_string()), values));
        }
        if self.iter().any(|r| r.d_res_high.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_res_high {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_d_res_high".to_string()), values));
        }
        if self.iter().any(|r| r.d_res_low.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_res_low {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_d_res_low".to_string()), values));
        }
        if self.iter().any(|r| r.r_factor_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_factor_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_R_factor_all".to_string()), values));
        }
        if self.iter().any(|r| r.r_factor_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_factor_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_R_factor_gt".to_string()), values));
        }
        if self.iter().any(|r| r.r_fsqd_factor.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_fsqd_factor {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_R_Fsqd_factor".to_string()), values));
        }
        if self.iter().any(|r| r.r_i_factor.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_i_factor {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_R_I_factor".to_string()), values));
        }
        if self.iter().any(|r| r.wr_factor_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wr_factor_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("refine_ls_class_wR_factor_all".to_string()), values));
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
                        "refine_ls_class_code",
                        "refine_ls_class_d_res_high",
                        "refine_ls_class_d_res_low",
                        "refine_ls_class_r_factor_all",
                        "refine_ls_class_r_factor_gt",
                        "refine_ls_class_r_fsqd_factor",
                        "refine_ls_class_r_i_factor",
                        "refine_ls_class_wr_factor_all",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = refinelsclass_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn refinelsclass_rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<RefineLsClass> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![RefineLsClass::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_code") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refine_ls_class_code", v) {
                Ok(val) => rows[i].code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_d_res_high") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_d_res_high", v) {
                Ok(val) => rows[i].d_res_high = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_d_res_low") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_d_res_low", v) {
                Ok(val) => rows[i].d_res_low = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_factor_all") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_factor_all", v) {
                Ok(val) => rows[i].r_factor_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_factor_gt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_factor_gt", v) {
                Ok(val) => rows[i].r_factor_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_fsqd_factor") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_fsqd_factor", v) {
                Ok(val) => rows[i].r_fsqd_factor = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_i_factor") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_i_factor", v) {
                Ok(val) => rows[i].r_i_factor = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_wr_factor_all") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_wr_factor_all", v) {
                Ok(val) => rows[i].wr_factor_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
