//! The `diffrn_reflns_class` category of the typed document layer (roadmap D12, tier T5).
//!
//! The loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<DiffrnReflnsClass>` group: it emits
//! one `MultiValues` loop and fills the rows from the columns.

use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use {crate::data_dict::core_cif::diffrn_reflns_class::AvREq, crate::data_dict::core_cif::diffrn_reflns_class::AvSgIOverI, crate::data_dict::core_cif::diffrn_reflns_class::AvUIOverI, crate::data_dict::core_cif::diffrn_reflns_class::DResHigh, crate::data_dict::core_cif::diffrn_reflns_class::DResLow, crate::data_dict::core_cif::diffrn_reflns_class::ReflnsClassNumber};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct DiffrnReflnsClass {
    pub av_r_eq: Option<AvREq>,
    pub av_sg_i_over_i: Option<AvSgIOverI>,
    pub av_u_i_over_i: Option<AvUIOverI>,
    pub code: Option<CharString>,
    pub d_res_high: Option<DResHigh>,
    pub d_res_low: Option<DResLow>,
    pub description: Option<CharString>,
    pub number: Option<ReflnsClassNumber>,
}
impl CifCategory for Vec<DiffrnReflnsClass> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.av_r_eq.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.av_r_eq {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_av_R_eq".to_string()), values));
        }
        if self.iter().any(|r| r.av_sg_i_over_i.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.av_sg_i_over_i {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_av_sgI/I".to_string()), values));
        }
        if self.iter().any(|r| r.av_u_i_over_i.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.av_u_i_over_i {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_av_uI/I".to_string()), values));
        }
        if self.iter().any(|r| r.code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_code".to_string()), values));
        }
        if self.iter().any(|r| r.d_res_high.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_res_high {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_d_res_high".to_string()), values));
        }
        if self.iter().any(|r| r.d_res_low.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_res_low {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_d_res_low".to_string()), values));
        }
        if self.iter().any(|r| r.description.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.description {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_description".to_string()), values));
        }
        if self.iter().any(|r| r.number.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_reflns_class_number".to_string()), values));
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
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_reflns_class_av_r_eq",
                    "diffrn_reflns_class_av_sgi/i",
                    "diffrn_reflns_class_av_ui/i",
                    "diffrn_reflns_class_code",
                    "diffrn_reflns_class_d_res_high",
                    "diffrn_reflns_class_d_res_low",
                    "diffrn_reflns_class_description",
                    "diffrn_reflns_class_number",];
                let is_ours = cols
                    .iter()
                    .any(|c| own_tags.contains(&c.tag().as_ref().to_lowercase().as_str()));
                if is_ours {
                    merge_rows(self, cols, &mut errors);
                } else {
                    residue.push(m.clone());
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}

fn merge_rows(rows: &mut Vec<DiffrnReflnsClass>, cols: &[LoopColumn], errors: &mut Vec<CifError>) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnReflnsClass::default());
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_av_r_eq") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_reflns_class_av_R_eq", v, AvREq::new) {
                Ok(val) => rows[i].av_r_eq = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_av_sgi/i") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_reflns_class_av_sgI/I", v, AvSgIOverI::new) {
                Ok(val) => rows[i].av_sg_i_over_i = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_av_ui/i") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_reflns_class_av_uI/I", v, AvUIOverI::new) {
                Ok(val) => rows[i].av_u_i_over_i = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_code") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_reflns_class_code", v) {
                Ok(val) => rows[i].code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_d_res_high") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_reflns_class_d_res_high", v, DResHigh::new) {
                Ok(val) => rows[i].d_res_high = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_d_res_low") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_reflns_class_d_res_low", v, DResLow::new) {
                Ok(val) => rows[i].d_res_low = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_description") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_reflns_class_description", v) {
                Ok(val) => rows[i].description = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_class_number") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_reflns_class_number", v, ReflnsClassNumber::new) {
                Ok(val) => rows[i].number = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
