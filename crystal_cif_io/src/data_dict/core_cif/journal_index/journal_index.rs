//! The `journal_index` category of the typed document layer (roadmap D12).
//!
//! The `JournalIndex` struct owns the single-value items (one `Option` field
//! per registry item) and the loop rows (`rows`). One `CifCategory`
//! impl covers both. Char items read leniently: the tier corpus
//! records them as text fields. The closed sets keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::journal_index::Type;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct JournalIndex {
    pub id: Option<CharString>,
    pub rows: Vec<JournalIndexRow>,
}

impl CifCategory for JournalIndex {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("journal_index_id".to_string()),
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
                        "journal_index_id" => {
                            match convert::read_char_text("journal_index_id", value) {
                                Ok(v) => self.id = v,
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
                        "journal_index_subterm",
                        "journal_index_term",
                        "journal_index_type",
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

/// The `journal_index` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct JournalIndexRow {
    pub subterm: Option<CharString>,
    pub term: Option<CharString>,
    pub type_: Option<Type>,
}

fn rows_to_members(rows: &[JournalIndexRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.subterm.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.subterm {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("journal_index_subterm".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.term.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.term {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("journal_index_term".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.type_.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.type_ {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("journal_index_type".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<JournalIndexRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![JournalIndexRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "journal_index_subterm")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_index_subterm", v) {
                Ok(val) => rows[i].subterm = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "journal_index_term")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_index_term", v) {
                Ok(val) => rows[i].term = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "journal_index_type")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("journal_index_type", v, Type::parse, "Type") {
                Ok(val) => rows[i].type_ = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
