//! The `space_group_symop` category of the typed document layer (roadmap D12).
//!
//! The `space_group_symop` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<Symop>` group: it
//! emits one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::data_dict::matrix;
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
pub struct Symop {
    pub id: Option<CharString>,
    pub operation_description: Option<CharString>,
    pub operation_xyz: Option<CharString>,
    pub r: Option<Matrix>,
    pub rt: Option<Matrix>,
    pub seitz_matrix: Option<Matrix>,
    pub sg_id: Option<Numeric>,
    pub t: Option<Matrix>,
}

impl CifCategory for Vec<Symop> {
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
                Tag::new("space_group_symop_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.operation_description.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.operation_description {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("space_group_symop_operation_description".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.operation_xyz.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.operation_xyz {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("space_group_symop_operation_xyz".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.r.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r {
                    Some(v) => matrix::to_value(v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("space_group_symop_r".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.rt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.rt {
                    Some(v) => matrix::to_value(v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("space_group_symop_rt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.seitz_matrix.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.seitz_matrix {
                    Some(v) => matrix::to_value(v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("space_group_symop_seitz_matrix".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.sg_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.sg_id {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("space_group_symop_sg_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.t.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.t {
                    Some(v) => matrix::to_value(v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("space_group_symop_t".to_string()),
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
                    // A loop is this category only when it carries a required
                    // identity tag. A sibling loop that carries other items of
                    // this category (for example the atom_site_aniso loop) stays
                    // in the residue instead of overwriting the primary rows.
                    let required = ["space_group_symop_operation_xyz"];
                    let is_ours = cols
                        .iter()
                        .any(|c| required.contains(&c.tag().as_ref().to_lowercase().as_str()));
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<Symop> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![Symop::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("space_group_symop_id", v) {
                Ok(val) => rows[i].id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_operation_description")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("space_group_symop_operation_description", v) {
                Ok(val) => rows[i].operation_description = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_operation_xyz")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("space_group_symop_operation_xyz", v) {
                Ok(val) => rows[i].operation_xyz = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_r")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_matrix("space_group_symop_r", v, (3, 3)) {
                Ok(val) => rows[i].r = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_rt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_matrix("space_group_symop_rt", v, (3, 3)) {
                Ok(val) => rows[i].rt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_seitz_matrix")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_matrix("space_group_symop_seitz_matrix", v, (4, 4)) {
                Ok(val) => rows[i].seitz_matrix = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_sg_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("space_group_symop_sg_id", v) {
                Ok(val) => rows[i].sg_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "space_group_symop_t")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_matrix("space_group_symop_t", v, (3, 1)) {
                Ok(val) => rows[i].t = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
