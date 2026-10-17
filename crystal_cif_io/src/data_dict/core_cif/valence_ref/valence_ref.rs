//! The `valence_ref` category of the typed document layer (roadmap D12).
//!
//! The `ValenceRefRow` loop row of the valence-model reference
//! loop.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
/// One loop row of the `valence_ref` category: the per-observation values of the loop items.
pub struct ValenceRefRow {
    /// The `_valence_ref_id` item. The field holds the column values, one per loop row.
    ///
    /// An identifier for items in this category. Parent of
    /// _valence_param_ref_id, which must have the same value.
    pub id: Option<CharString>,
    /// The `_valence_ref_reference` item. The field holds the column values, one per loop row.
    ///
    /// Literature reference from which the valence parameters identified by
    /// _valence_param_id were taken.
    pub reference: Option<CharString>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &["valence_ref_id", "valence_ref_reference"];

impl CifCategory for Vec<ValenceRefRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_ref_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.reference.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.reference {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_ref_reference".to_string()),
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
                    let is_ours = cols
                        .iter()
                        .any(|c| LOOP_TAGS.contains(&c.tag().as_ref().to_lowercase().as_str()));
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ValenceRefRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ValenceRefRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_ref_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("valence_ref_id", v) {
                Ok(val) => rows[i].id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_ref_reference")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("valence_ref_reference", v) {
                Ok(val) => rows[i].reference = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
