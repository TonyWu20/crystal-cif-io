//! The `audit_author` category of the typed document layer (roadmap D12).
//!
//! The `audit_author` single-value struct and the loop-row struct. One `Option`
//! field per registry item. `CifCategory` is on the `AuditAuthor` single
//! group and on the `Vec<AuditAuthorRow>` loop group. Char items read leniently:
//! the tier corpus records them as text fields. The closed sets keep
//! the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct AuditAuthor {
    pub email: Option<CharString>,
    pub fax: Option<CharString>,
    pub id: Option<CharString>,
    pub id_orcid: Option<CharString>,
    pub phone: Option<CharString>,
}

impl CifCategory for AuditAuthor {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.email {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_author_email".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.fax {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_author_fax".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_author_id".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.id_orcid {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_author_id_orcid".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.phone {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_author_phone".to_string()),
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
                    "audit_author_email" => {
                        match convert::read_char_text("audit_author_email", value) {
                            Ok(v) => self.email = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_author_fax" => {
                        match convert::read_char_text("audit_author_fax", value) {
                            Ok(v) => self.fax = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_author_id" => match convert::read_char_text("audit_author_id", value) {
                        Ok(v) => self.id = v,
                        Err(e) => errors.push(e),
                    },
                    "audit_author_id_orcid" => {
                        match convert::read_char_text("audit_author_id_orcid", value) {
                            Ok(v) => self.id_orcid = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_author_phone" => {
                        match convert::read_char_text("audit_author_phone", value) {
                            Ok(v) => self.phone = v,
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

/// The `audit_author` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct AuditAuthorRow {
    pub address: Option<CharString>,
    pub name: Option<CharString>,
}

impl CifCategory for Vec<AuditAuthorRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.address.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.address {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("audit_author_address".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.name.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.name {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("audit_author_name".to_string()),
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
                    // the category own loop items. A sibling loop that
                    // carries none of them stays in the residue.
                    let ours: &[&str] = &["audit_author_address", "audit_author_name"];
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<AuditAuthorRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![AuditAuthorRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "audit_author_address")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("audit_author_address", v) {
                Ok(val) => rows[i].address = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "audit_author_name")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("audit_author_name", v) {
                Ok(val) => rows[i].name = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
