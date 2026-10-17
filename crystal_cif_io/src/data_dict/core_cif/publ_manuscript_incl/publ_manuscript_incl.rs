//! The `publ_manuscript_incl` category of the typed document layer (roadmap D12).
//!
//! The `publ_manuscript_incl` loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<PublManuscriptIncl>` group: it emits one `MultiValues`
//! loop and fills the rows from the columns. Char items read leniently:
//! the tier corpus records them as text fields.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::publ_manuscript_incl::ExtraDefn;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

/// The `publ_manuscript_incl` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
/// The `publ_manuscript_incl` category struct. One field per registry item.
pub struct PublManuscriptIncl {
    /// The `_publ_manuscript_incl_extra_defn` item. The field holds the column values, one per loop row.
    ///
    /// Flags whether the corresponding data item marked for inclusion in a
    /// journal request list is a standard CIF definition or not.
    pub extra_defn: Option<ExtraDefn>,
    /// The `_publ_manuscript_incl_extra_info` item. The field holds the column values, one per loop row.
    ///
    /// A short note indicating the reason why the author wishes the
    /// corresponding data item marked for inclusion in the journal request
    /// list to be published.
    pub extra_info: Option<CharString>,
    /// The `_publ_manuscript_incl_extra_item` item. The field holds the column values, one per loop row.
    ///
    /// Specifies the inclusion of specific data into a manuscript which are
    /// not normally requested by the journal. The values of this item are
    /// the extra data names (which MUST be enclosed in single quotes) that
    /// will be added to the journal request list.
    pub extra_item: Option<CharString>,
}

impl CifCategory for Vec<PublManuscriptIncl> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.extra_defn.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.extra_defn {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_manuscript_incl_extra_defn".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.extra_info.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.extra_info {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_manuscript_incl_extra_info".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.extra_item.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.extra_item {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("publ_manuscript_incl_extra_item".to_string()),
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
                        "publ_manuscript_incl_extra_defn",
                        "publ_manuscript_incl_extra_info",
                        "publ_manuscript_incl_extra_item",
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<PublManuscriptIncl> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![PublManuscriptIncl::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_manuscript_incl_extra_defn")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "publ_manuscript_incl_extra_defn",
                v,
                ExtraDefn::parse,
                "ExtraDefn",
            ) {
                Ok(val) => rows[i].extra_defn = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_manuscript_incl_extra_info")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_manuscript_incl_extra_info", v) {
                Ok(val) => rows[i].extra_info = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_manuscript_incl_extra_item")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_manuscript_incl_extra_item", v) {
                Ok(val) => rows[i].extra_item = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
