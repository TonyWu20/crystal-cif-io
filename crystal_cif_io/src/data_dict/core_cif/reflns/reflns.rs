//! The `reflns` category of the typed document layer (roadmap D12).
//!
//! The `reflns` category struct: the nine single-value items and the
//! `reflns_d_resolution` and `reflns_limit` loop families. `CifCategory`
//! is on the `Reflns` group. The loop families keep one row struct each.

use crystal_cif_fmt::grammar::{Tag, Value, Numeric, UnsignedInteger, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};
use crate::data_dict::core_cif::reflns::{FriedelFraction};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct Reflns {
    pub friedel_coverage: Option<FriedelFraction>,
    pub friedel_fraction_full: Option<FriedelFraction>,
    pub friedel_fraction_max: Option<FriedelFraction>,
    pub number_gt: Option<UnsignedInteger>,
    pub number_observed: Option<UnsignedInteger>,
    pub number_total: Option<UnsignedInteger>,
    pub observed_criterion: Option<CharString>,
    pub special_details: Option<CharString>,
    pub threshold_expression: Option<CharString>,
    pub d_resolution: Vec<ReflnsDResolution>,
    pub limit: Vec<ReflnsLimit>,
}

#[derive(Debug, Clone, Default)]
pub struct ReflnsDResolution {
    pub high: Option<Numeric>,
    pub low: Option<Numeric>,
}

#[derive(Debug, Clone, Default)]
pub struct ReflnsLimit {
    pub h_max: Option<Numeric>,
    pub h_min: Option<Numeric>,
    pub k_max: Option<Numeric>,
    pub k_min: Option<Numeric>,
    pub l_max: Option<Numeric>,
    pub l_min: Option<Numeric>,
}

impl CifCategory for Reflns {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.friedel_coverage {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_Friedel_coverage".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.friedel_fraction_full {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_Friedel_fraction_full".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.friedel_fraction_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_Friedel_fraction_max".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.number_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_number_gt".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.number_observed {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_number_observed".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.number_total {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_number_total".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.observed_criterion {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_observed_criterion".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_special_details".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.threshold_expression {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_threshold_expression".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if !self.d_resolution.is_empty() {
            out.extend(d_resolution_to_loop_members(&self.d_resolution));
        }
        if !self.limit.is_empty() {
            out.extend(limit_to_loop_members(&self.limit));
        }
        out
    }

    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::SingleValue(sv)) => {
                    let tag = sv.tag().as_ref().to_lowercase();
                    let value = sv.value();
                    match tag.as_str() {
                        "reflns_friedel_coverage" => {
                            match convert::read_friedel_fraction("reflns_friedel_coverage", value) {
                                Ok(v) => self.friedel_coverage = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_friedel_fraction_full" => {
                            match convert::read_friedel_fraction("reflns_friedel_fraction_full", value) {
                                Ok(v) => self.friedel_fraction_full = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_friedel_fraction_max" => {
                            match convert::read_friedel_fraction("reflns_friedel_fraction_max", value) {
                                Ok(v) => self.friedel_fraction_max = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_number_gt" => {
                            match convert::read_unsigned("reflns_number_gt", value) {
                                Ok(v) => self.number_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_number_observed" => {
                            match convert::read_unsigned("reflns_number_observed", value) {
                                Ok(v) => self.number_observed = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_number_total" => {
                            match convert::read_unsigned("reflns_number_total", value) {
                                Ok(v) => self.number_total = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_observed_criterion" => {
                            match convert::read_char("reflns_observed_criterion", value) {
                                Ok(v) => self.observed_criterion = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_special_details" => {
                            match convert::read_char("reflns_special_details", value) {
                                Ok(v) => self.special_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "reflns_threshold_expression" => {
                            match convert::read_char("reflns_threshold_expression", value) {
                                Ok(v) => self.threshold_expression = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop belongs to this category only when it carries one
                    // of the loop-family column tags. A sibling loop of
                    // another category stays in the residue.
                    let d_resolution = [
                        "reflns_d_resolution_high",
                        "reflns_d_resolution_low",
                    ];
                    let limit = [
                        "reflns_limit_h_max",
                        "reflns_limit_h_min",
                        "reflns_limit_k_max",
                        "reflns_limit_k_min",
                        "reflns_limit_l_max",
                        "reflns_limit_l_min",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| {
                            let t = c.tag().as_ref().to_lowercase();
                            d_resolution.contains(&t.as_str()) || limit.contains(&t.as_str())
                        });
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                    if cols.iter().any(|c| d_resolution.contains(&c.tag().as_ref().to_lowercase().as_str())) {
                        self.d_resolution = d_resolution_rows_from_columns(cols, &mut errors);
                    }
                    if cols.iter().any(|c| limit.contains(&c.tag().as_ref().to_lowercase().as_str())) {
                        self.limit = limit_rows_from_columns(cols, &mut errors);
                    }
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn d_resolution_to_loop_members(rows: &[ReflnsDResolution]) -> Vec<DataBlockMember> {
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.high.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.high {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_d_resolution_high".to_string()), values));
    }
    if rows.iter().any(|r| r.low.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.low {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_d_resolution_low".to_string()), values));
    }
    if columns.is_empty() {
        return Vec::new();
    }
    vec![DataBlockMember::DataItems(DataItems::MultiValues(LoopColumns::new(columns)))]
}

fn d_resolution_rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ReflnsDResolution> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ReflnsDResolution::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_d_resolution_high") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_d_resolution_high", v) {
                Ok(val) => rows[i].high = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_d_resolution_low") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_d_resolution_low", v) {
                Ok(val) => rows[i].low = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}

fn limit_to_loop_members(rows: &[ReflnsLimit]) -> Vec<DataBlockMember> {
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.h_max.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.h_max {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_limit_h_max".to_string()), values));
    }
    if rows.iter().any(|r| r.h_min.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.h_min {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_limit_h_min".to_string()), values));
    }
    if rows.iter().any(|r| r.k_max.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.k_max {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_limit_k_max".to_string()), values));
    }
    if rows.iter().any(|r| r.k_min.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.k_min {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_limit_k_min".to_string()), values));
    }
    if rows.iter().any(|r| r.l_max.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.l_max {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_limit_l_max".to_string()), values));
    }
    if rows.iter().any(|r| r.l_min.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.l_min {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("reflns_limit_l_min".to_string()), values));
    }
    if columns.is_empty() {
        return Vec::new();
    }
    vec![DataBlockMember::DataItems(DataItems::MultiValues(LoopColumns::new(columns)))]
}

fn limit_rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ReflnsLimit> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ReflnsLimit::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_limit_h_max") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_limit_h_max", v) {
                Ok(val) => rows[i].h_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_limit_h_min") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_limit_h_min", v) {
                Ok(val) => rows[i].h_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_limit_k_max") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_limit_k_max", v) {
                Ok(val) => rows[i].k_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_limit_k_min") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_limit_k_min", v) {
                Ok(val) => rows[i].k_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_limit_l_max") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_limit_l_max", v) {
                Ok(val) => rows[i].l_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "reflns_limit_l_min") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_limit_l_min", v) {
                Ok(val) => rows[i].l_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
