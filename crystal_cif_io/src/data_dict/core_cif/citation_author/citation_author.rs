//! The `citation_author` category of the typed document layer (roadmap D12).
//!
//! The `citation_author` loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<CitationAuthor>` group: it emits one `MultiValues`
//! loop and fills the rows from the columns. Char items read leniently:
//! the tier corpus records them as text fields.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

/// The `citation_author` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
/// The `citation_author` category struct. One field per registry item.
pub struct CitationAuthor {
    /// The `_citation_author_citation_id` item. The field holds the column values, one per loop row.
    ///
    /// The value of _citation_author_citation_id must match an identifier
    /// specified by _citation_id in the _citation_ list.
    pub citation_id: Option<CharString>,
    /// The `_citation_author_name` item. The field holds the column values, one per loop row.
    ///
    /// Name of an author of the citation; relevant for journal articles,
    /// books and book chapters. The family name(s), followed by a comma and
    /// including any dynastic components, precedes the first name(s) or
    /// initial(s).
    pub name: Option<CharString>,
    /// The `_citation_author_ordinal` item. The field holds the column values, one per loop row.
    ///
    /// This data name defines the order of the author's name in the list of
    /// authors of a citation.
    pub ordinal: Option<CharString>,
}

impl CifCategory for Vec<CitationAuthor> {
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
            columns.push(LoopColumn::new(
                Tag::new("citation_author_citation_id".to_string()),
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
                Tag::new("citation_author_name".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.ordinal.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ordinal {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("citation_author_ordinal".to_string()),
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
                    let ours: &[&str] = &[
                        "citation_author_citation_id",
                        "citation_author_name",
                        "citation_author_ordinal",
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<CitationAuthor> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![CitationAuthor::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_author_citation_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_author_citation_id", v) {
                Ok(val) => rows[i].citation_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_author_name")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_author_name", v) {
                Ok(val) => rows[i].name = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_author_ordinal")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_author_ordinal", v) {
                Ok(val) => rows[i].ordinal = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
