//! The `audit_link` category of the typed document layer (roadmap D12).
//!
//! The `audit_link` loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<AuditLink>` group: it emits one `MultiValues`
//! loop and fills the rows from the columns. Char items read leniently:
//! the tier corpus records them as text fields.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

/// The `audit_link` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct AuditLink {
    pub block_code: Option<CharString>,
    pub block_description: Option<CharString>,
}

impl CifCategory for Vec<AuditLink> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.block_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.block_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("audit_link_block_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.block_description.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.block_description {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("audit_link_block_description".to_string()),
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
                    let ours: &[&str] = &["audit_link_block_code", "audit_link_block_description"];
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<AuditLink> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![AuditLink::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "audit_link_block_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("audit_link_block_code", v) {
                Ok(val) => rows[i].block_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "audit_link_block_description")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("audit_link_block_description", v) {
                Ok(val) => rows[i].block_description = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
