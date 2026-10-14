//! The `diffrn_scale_group` category of the typed document layer (roadmap D12, tier T5).
//!
//! The loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<DiffrnScaleGroup>` group: it emits
//! one `MultiValues` loop and fills the rows from the columns.

use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use {crate::data_dict::core_cif::diffrn_scale_group::INet};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct DiffrnScaleGroup {
    pub code: Option<CharString>,
    pub i_net: Option<INet>,
}
impl CifCategory for Vec<DiffrnScaleGroup> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_scale_group_code".to_string()), values));
        }
        if self.iter().any(|r| r.i_net.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.i_net {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_scale_group_I_net".to_string()), values));
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
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_scale_group_code",
                    "diffrn_scale_group_i_net",];
                let is_ours = cols
                    .iter()
                    .any(|c| own_tags.contains(&c.tag().as_ref().to_lowercase().as_str()));
                if is_ours {
                    merge_rows(self, cols, &mut errors);
                } else {
                    residue.push(m.clone());
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}

fn merge_rows(rows: &mut Vec<DiffrnScaleGroup>, cols: &[LoopColumn], errors: &mut Vec<CifError>) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnScaleGroup::default());
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_scale_group_code") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_scale_group_code", v) {
                Ok(val) => rows[i].code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_scale_group_i_net") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_scale_group_I_net", v, INet::new) {
                Ok(val) => rows[i].i_net = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
