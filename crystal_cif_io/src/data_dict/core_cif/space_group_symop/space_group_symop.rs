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

/// One loop row of the `space_group_symop` category: the
/// per-operation values of the loop items.
#[derive(Debug, Clone, Default)]
pub struct Symop {
    /// The `_space_group_symop_id` item. The field holds the column values, one per loop row.
    ///
    /// An arbitrary identifier that uniquely labels each symmetry operation
    /// in the list. In order for the defaults to work correctly, the
    /// identity operation should have _space_group_symop_id or
    /// _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    pub id: Option<CharString>,
    /// The `_space_group_symop_operation_description` item. The field holds the single value.
    pub operation_description: Option<CharString>,
    /// The `_space_group_symop_operation_xyz` item. The field holds the column values, one per loop row.
    ///
    /// A parsable string giving one of the symmetry operations of the space
    /// group in algebraic form. If W is a matrix representation of the
    /// rotational part of the symmetry operation defined by the positions
    /// and signs of x, y and z, and w is a column of translations defined
    /// by fractions, an equivalent position X' is generated from a given
    /// position X by the equation X' = WX + w (Note: X is used to represent
    /// bold_italics_x in International Tables for Crystallography Vol. A,
    /// Part 5) When a list of symmetry operations is given, it must contain
    /// a complete set of coordinate representatives which generates all the
    /// operations of the space group by the addition of all primitive
    /// translations of the space group. Such representatives are to be
    /// found as the coordinates of the general-equivalent position in
    /// International Tables for Crystallography Vol. A (2002), to which it
    /// is necessary to add any centring translations shown above the
    /// general-equivalent position. That is to say, it is necessary to list
    /// explicitly all the symmetry operations required to generate all the
    /// atoms in the unit cell defined by the setting used. In order for the
    /// defaults to work correctly, the identity operation should have
    /// _space_group_symop_id or _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    pub operation_xyz: Option<CharString>,
    /// The `_space_group_symop_R` item. The field holds the single value.
    pub r: Option<Matrix>,
    /// The `_space_group_symop_RT` item. The field holds the single value.
    pub rt: Option<Matrix>,
    /// The `_space_group_symop_Seitz_matrix` item. The field holds the single value.
    pub seitz_matrix: Option<Matrix>,
    /// The `_space_group_symop_sg_id` item. The identifier of the
    /// space group the operation belongs to.
    pub sg_id: Option<Numeric>,
    /// The `_space_group_symop_T` item. The field holds the single value.
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
