//! The `publ_author` category of the typed document layer (roadmap D12).
//!
//! The `PublAuthor` struct owns the single-value items (one `Option` field
//! per registry item) and the loop rows (`rows`). One `CifCategory`
//! impl covers both. Char items read leniently: the tier corpus
//! records them as text fields. The closed sets keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct PublAuthor {
    pub id: Option<CharString>,
    pub id_audit: Option<CharString>,
    pub phone: Option<CharString>,
    pub rows: Vec<PublAuthorRow>,
}

impl CifCategory for PublAuthor {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_author_id".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.id_audit {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_author_id_audit".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.phone {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_author_phone".to_string()),
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
                        "publ_author_id" => match convert::read_char_text("publ_author_id", value) {
                            Ok(v) => self.id = v,
                            Err(e) => errors.push(e),
                        },
                        "publ_author_id_audit" => {
                            match convert::read_char_text("publ_author_id_audit", value) {
                                Ok(v) => self.id_audit = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_author_phone" => {
                            match convert::read_char_text("publ_author_phone", value) {
                                Ok(v) => self.phone = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries one of
                    // the category own loop items. A sibling loop that
                    // carries none of them stays in the residue.
                    let ours: &[&str] = &[
                        "publ_author_address",
                        "publ_author_email",
                        "publ_author_footnote",
                        "publ_author_id_iucr",
                        "publ_author_id_orcid",
                        "publ_author_name",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        self.rows = rows_from_columns(cols, &mut errors);
                    }

                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

/// The `publ_author` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct PublAuthorRow {
    pub address: Option<CharString>,
    pub email: Option<CharString>,
    pub footnote: Option<CharString>,
    pub id_iucr: Option<CharString>,
    pub id_orcid: Option<CharString>,
    pub name: Option<CharString>,
}

fn rows_to_members(rows: &[PublAuthorRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.address.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.address {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_author_address".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.email.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.email {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_author_email".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.footnote.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.footnote {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_author_footnote".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.id_iucr.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.id_iucr {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_author_id_iucr".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.id_orcid.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.id_orcid {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_author_id_orcid".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.name.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.name {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_author_name".to_string()),
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


fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<PublAuthorRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![PublAuthorRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_author_address")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_author_address", v) {
                Ok(val) => rows[i].address = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_author_email")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_author_email", v) {
                Ok(val) => rows[i].email = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_author_footnote")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_author_footnote", v) {
                Ok(val) => rows[i].footnote = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_author_id_iucr")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_author_id_iucr", v) {
                Ok(val) => rows[i].id_iucr = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_author_id_orcid")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_author_id_orcid", v) {
                Ok(val) => rows[i].id_orcid = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_author_name")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_author_name", v) {
                Ok(val) => rows[i].name = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
