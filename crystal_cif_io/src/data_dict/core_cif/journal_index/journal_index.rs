//! The `journal_index` category of the typed document layer (roadmap D12).
//!
//! The `journal_index` single-value struct and the loop-row struct. One `Option`
//! field per registry item. `CifCategory` is on the `JournalIndex` single
//! group and on the `Vec<JournalIndexRow>` loop group. Char items read leniently:
//! the tier corpus records them as text fields. The closed sets keep
//! the typed enum.

use crystal_cif_fmt::grammar::{Tag, Value, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::journal_index::Type;
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct JournalIndex {
    pub id: Option<CharString>,
}

impl CifCategory for JournalIndex {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("journal_index_id".to_string()),
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
                    "journal_index_id" => {
                        match convert::read_char_text("journal_index_id", value) {
                            Ok(v) => self.id = v,
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

/// The `journal_index` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct JournalIndexRow {
    pub subterm: Option<CharString>,
    pub term: Option<CharString>,
    pub type_: Option<Type>,
}

impl CifCategory for Vec<JournalIndexRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.subterm.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.subterm {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_index_subterm".to_string()), values));
        }
        if self.iter().any(|r| r.term.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.term {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_index_term".to_string()), values));
        }
        if self.iter().any(|r| r.type_.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.type_ {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_index_type".to_string()), values));
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
                        *self = rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<JournalIndexRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![JournalIndexRow::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_index_subterm") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_index_subterm", v) {
                Ok(val) => rows[i].subterm = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_index_term") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_index_term", v) {
                Ok(val) => rows[i].term = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_index_type") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("journal_index_type", v, Type::parse, "Type") {
                Ok(val) => rows[i].type_ = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
