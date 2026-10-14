//! The `citation_editor` category of the typed document layer (roadmap D12).
//!
//! The `citation_editor` single-value struct and the loop-row struct. One `Option`
//! field per registry item. `CifCategory` is on the `CitationEditor` single
//! group and on the `Vec<CitationEditorRow>` loop group. Char items read leniently:
//! the tier corpus records them as text fields. The closed sets keep
//! the typed enum.

use crystal_cif_fmt::grammar::{Tag, Value, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct CitationEditor {
    pub editor: Option<CharString>,
}

impl CifCategory for CitationEditor {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.editor {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("citation_editor".to_string()),
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
                    "citation_editor" => {
                        match convert::read_char_text("citation_editor", value) {
                            Ok(v) => self.editor = v,
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

/// The `citation_editor` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct CitationEditorRow {
    pub citation_id: Option<CharString>,
    pub name: Option<CharString>,
    pub ordinal: Option<CharString>,
}

impl CifCategory for Vec<CitationEditorRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.citation_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.citation_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("citation_editor_citation_id".to_string()), values));
        }
        if self.iter().any(|r| r.name.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.name {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("citation_editor_name".to_string()), values));
        }
        if self.iter().any(|r| r.ordinal.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ordinal {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("citation_editor_ordinal".to_string()), values));
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
                        "citation_editor_citation_id",
                        "citation_editor_name",
                        "citation_editor_ordinal",
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<CitationEditorRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![CitationEditorRow::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "citation_editor_citation_id") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_editor_citation_id", v) {
                Ok(val) => rows[i].citation_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "citation_editor_name") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_editor_name", v) {
                Ok(val) => rows[i].name = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "citation_editor_ordinal") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_editor_ordinal", v) {
                Ok(val) => rows[i].ordinal = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
