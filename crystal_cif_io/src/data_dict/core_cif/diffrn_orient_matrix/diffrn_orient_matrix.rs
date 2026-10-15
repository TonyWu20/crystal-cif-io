//! The `diffrn_orient_matrix` category of the typed document layer (roadmap D12, tier T5).
//!
//! The `DiffrnOrientMatrix` struct owns the single-value items and the loop
//! rows (`rows`). One `CifCategory` impl: the scalar items, then
//! the `MultiValues` loop of the `DiffrnOrientMatrixRow` columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct DiffrnOrientMatrix {
    pub r#type: Option<CharString>,
    pub rows: Vec<DiffrnOrientMatrixRow>,
}
impl CifCategory for DiffrnOrientMatrix {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.r#type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_orient_matrix_type".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        out.extend(rows_to_members(&self.rows));
        out
    }
    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::SingleValue(sv)) => {
                    let tag = sv.tag().as_ref().to_lowercase();
                    let value = sv.value();
                    match tag.as_str() {
                        "diffrn_orient_matrix_type" => {
                            match convert::read_char("diffrn_orient_matrix_type", value) {
                                Ok(v) => self.r#type = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries one of its
                    // own loop column tags. A loop without our tags stays in
                    // the residue instead of overwriting the rows.
                    let own_tags = [
                        "diffrn_orient_matrix_ub_11",
                        "diffrn_orient_matrix_ub_12",
                        "diffrn_orient_matrix_ub_13",
                        "diffrn_orient_matrix_ub_21",
                        "diffrn_orient_matrix_ub_22",
                        "diffrn_orient_matrix_ub_23",
                        "diffrn_orient_matrix_ub_31",
                        "diffrn_orient_matrix_ub_32",
                        "diffrn_orient_matrix_ub_33",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| own_tags.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if is_ours {
                        merge_rows(&mut self.rows, cols, &mut errors);
                    } else {
                        residue.push(m.clone());
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiffrnOrientMatrixRow {
    pub ub_11: Option<Numeric>,
    pub ub_12: Option<Numeric>,
    pub ub_13: Option<Numeric>,
    pub ub_21: Option<Numeric>,
    pub ub_22: Option<Numeric>,
    pub ub_23: Option<Numeric>,
    pub ub_31: Option<Numeric>,
    pub ub_32: Option<Numeric>,
    pub ub_33: Option<Numeric>,
}
fn rows_to_members(rows: &[DiffrnOrientMatrixRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.ub_11.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_11 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_11".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_12.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_12 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_12".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_13.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_13 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_13".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_21.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_21 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_21".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_22.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_22 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_22".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_23.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_23 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_23".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_31.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_31 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_31".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_32.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_32 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_32".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.ub_33.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.ub_33 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_orient_matrix_UB_33".to_string()),
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

fn merge_rows(
    rows: &mut Vec<DiffrnOrientMatrixRow>,
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnOrientMatrixRow::default());
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_11")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_11", v) {
                Ok(val) => rows[i].ub_11 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_12")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_12", v) {
                Ok(val) => rows[i].ub_12 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_13")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_13", v) {
                Ok(val) => rows[i].ub_13 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_21")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_21", v) {
                Ok(val) => rows[i].ub_21 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_22")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_22", v) {
                Ok(val) => rows[i].ub_22 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_23")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_23", v) {
                Ok(val) => rows[i].ub_23 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_31")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_31", v) {
                Ok(val) => rows[i].ub_31 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_32")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_32", v) {
                Ok(val) => rows[i].ub_32 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_matrix_ub_33")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_matrix_UB_33", v) {
                Ok(val) => rows[i].ub_33 = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
