//! The `audit_conform` category of the typed document layer (roadmap D12).
//!
//! The `audit_conform` single-value struct and the loop-row struct. One `Option`
//! field per registry item. `CifCategory` is on the `AuditConform` single
//! group and on the `Vec<AuditConformRow>` loop group. Char items read leniently:
//! the tier corpus records them as text fields. The closed sets keep
//! the typed enum.

use crystal_cif_fmt::grammar::{Tag, Value, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct AuditConform {
    pub dict_doi: Option<CharString>,
}

impl CifCategory for AuditConform {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.dict_doi {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("audit_conform_dict_doi".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        out
    }

    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "audit_conform_dict_doi" => {
                        match convert::read_char_text("audit_conform_dict_doi", value) {
                            Ok(v) => self.dict_doi = v,
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

/// The `audit_conform` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct AuditConformRow {
    pub dict_location: Option<CharString>,
    pub dict_name: Option<CharString>,
    pub dict_version: Option<CharString>,
}

impl CifCategory for Vec<AuditConformRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.dict_location.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.dict_location {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("audit_conform_dict_location".to_string()), values));
        }
        if self.iter().any(|r| r.dict_name.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.dict_name {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("audit_conform_dict_name".to_string()), values));
        }
        if self.iter().any(|r| r.dict_version.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.dict_version {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("audit_conform_dict_version".to_string()), values));
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
                    // the category own loop items. A sibling loop that
                    // carries none of them stays in the residue.
                    let ours: &[&str] = &[
                        "audit_conform_dict_location",
                        "audit_conform_dict_name",
                        "audit_conform_dict_version",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<AuditConformRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![AuditConformRow::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "audit_conform_dict_location") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("audit_conform_dict_location", v) {
                Ok(val) => rows[i].dict_location = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "audit_conform_dict_name") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("audit_conform_dict_name", v) {
                Ok(val) => rows[i].dict_name = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "audit_conform_dict_version") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("audit_conform_dict_version", v) {
                Ok(val) => rows[i].dict_version = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
