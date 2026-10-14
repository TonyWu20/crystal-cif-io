//! The `diffrn_standard_refln` category of the typed document layer (roadmap D12, tier T5).
//!
//! The single-value struct `DiffrnStandardRefln` and the loop-row struct
//! `DiffrnStandardReflnRow`. `CifCategory` is on each group: the row group
//! emits one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct DiffrnStandardRefln {
    pub code: Option<CharString>,
}
impl CifCategory for DiffrnStandardRefln {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.code {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standard_refln_code".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        out
    }
    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "diffrn_standard_refln_code" => {
                        match convert::read_char("diffrn_standard_refln_code", value) {
                            Ok(v) => self.code = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    _ => residue.push(m.clone()),
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiffrnStandardReflnRow {
    pub index_h: Option<Numeric>,
    pub index_k: Option<Numeric>,
    pub index_l: Option<Numeric>,
}
impl CifCategory for Vec<DiffrnStandardReflnRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.index_h.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_h {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_standard_refln_index_h".to_string()),
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
                Tag::new("diffrn_standard_refln_index_k".to_string()),
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
                Tag::new("diffrn_standard_refln_index_l".to_string()),
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
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_standard_refln_index_h",
                    "diffrn_standard_refln_index_k",
                    "diffrn_standard_refln_index_l",
                ];
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

fn merge_rows(
    rows: &mut Vec<DiffrnStandardReflnRow>,
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnStandardReflnRow::default());
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_standard_refln_index_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_standard_refln_index_h", v) {
                Ok(val) => rows[i].index_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_standard_refln_index_k")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_standard_refln_index_k", v) {
                Ok(val) => rows[i].index_k = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_standard_refln_index_l")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_standard_refln_index_l", v) {
                Ok(val) => rows[i].index_l = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
