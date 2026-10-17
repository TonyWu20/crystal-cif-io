//! The `diffrn_attenuator` category of the typed document layer (roadmap D12, tier T5).
//!
//! The loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<DiffrnAttenuator>` group: it emits
//! one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::diffrn_attenuator::Scale;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
/// The `diffrn_attenuator` category struct. One field per registry item.
pub struct DiffrnAttenuator {
    /// The `_diffrn_attenuator_code` item. The field holds the column values, one per loop row.
    ///
    /// A code associated with a particular attenuator setting. This code is
    /// referenced by the _diffrn_refln_attenuator_code which is stored with
    /// the intensities. See _diffrn_attenuator_scale.
    pub code: Option<CharString>,
    /// The `_diffrn_attenuator_material` item. The field holds the column values, one per loop row.
    ///
    /// Material from which the attenuator is made.
    pub material: Option<CharString>,
    /// The `_diffrn_attenuator_scale` item. The field holds the column values, one per loop row.
    ///
    /// The scale factor applied when an intensity measurement is reduced by
    /// an attenuator identified by _diffrn_attenuator_code. The measured
    /// intensity must be multiplied by this scale to convert it to the same
    /// scale as unattenuated intensities.
    pub scale: Option<Scale>,
}
impl CifCategory for Vec<DiffrnAttenuator> {
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
            columns.push(LoopColumn::new(
                Tag::new("diffrn_attenuator_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.material.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.material {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_attenuator_material".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scale.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scale {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_attenuator_scale".to_string()),
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
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_attenuator_code",
                    "diffrn_attenuator_material",
                    "diffrn_attenuator_scale",
                ];
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

fn merge_rows(rows: &mut Vec<DiffrnAttenuator>, cols: &[LoopColumn], errors: &mut Vec<CifError>) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnAttenuator::default());
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_attenuator_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_attenuator_code", v) {
                Ok(val) => rows[i].code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_attenuator_material")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_attenuator_material", v) {
                Ok(val) => rows[i].material = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_attenuator_scale")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_attenuator_scale", v, Scale::new) {
                Ok(val) => rows[i].scale = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
