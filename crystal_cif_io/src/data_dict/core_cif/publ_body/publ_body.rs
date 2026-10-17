//! The `publ_body` category of the typed document layer (roadmap D12).
//!
//! The `publ_body` loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<PublBody>` group: it emits one `MultiValues`
//! loop and fills the rows from the columns. Char items read leniently:
//! the tier corpus records them as text fields.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::publ_body::{Element, Format};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

/// The `publ_body` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
/// The `publ_body` category struct. One field per registry item.
pub struct PublBody {
    /// The `_publ_body_contents` item. The field holds the column values, one per loop row.
    ///
    /// A text section of a paper.
    pub contents: Option<CharString>,
    /// The `_publ_body_element` item. The field holds the column values, one per loop row.
    ///
    /// The functional role of the associated text section.
    pub element: Option<Element>,
    /// The `_publ_body_format` item. The field holds the column values, one per loop row.
    ///
    /// Code indicating the appropriate typesetting conventions for accented
    /// characters and special symbols in the text section.
    pub format: Option<Format>,
    /// The `_publ_body_label` item. The field holds the column values, one per loop row.
    ///
    /// Code identifying the section of text. The combination of this with
    /// _publ_body_element must be unique.
    pub label: Option<CharString>,
    /// The `_publ_body_title` item. The field holds the column values, one per loop row.
    ///
    /// Title of the associated section of text.
    pub title: Option<CharString>,
}

impl CifCategory for Vec<PublBody> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.contents.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.contents {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_body_contents".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.element.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.element {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_body_element".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.format.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.format {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_body_format".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.label.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.label {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_body_label".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.title.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.title {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_body_title".to_string()),
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
                        "publ_body_contents",
                        "publ_body_element",
                        "publ_body_format",
                        "publ_body_label",
                        "publ_body_title",
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<PublBody> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![PublBody::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_body_contents")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_body_contents", v) {
                Ok(val) => rows[i].contents = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_body_element")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("publ_body_element", v, Element::parse, "Element") {
                Ok(val) => rows[i].element = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_body_format")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("publ_body_format", v, Format::parse, "Format") {
                Ok(val) => rows[i].format = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_body_label")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_body_label", v) {
                Ok(val) => rows[i].label = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_body_title")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_body_title", v) {
                Ok(val) => rows[i].title = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
