//! The `symmetry_equiv` category of the typed document layer (roadmap D12).
//!
//! The `symmetry_equiv` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<SymmetryEquiv>`
//! group: it emits one `MultiValues` loop and fills the rows from the
//! columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
/// The `symmetry_equiv` category struct. One field per registry item.
pub struct SymmetryEquiv {
    /// The `_symmetry_equiv_pos_as_xyz` item. The field holds the column values, one per loop row.
    ///
    /// Symmetry-equivalent position in the 'xyz' representation. Except for
    /// the space group P1, these data will be repeated in a loop. The
    /// format of the data item is as per International Tables for
    /// Crystallography Vol. A. (2002). All equivalent positions should be
    /// entered, including those for lattice centring and a centre of
    /// symmetry, if present. In order for the defaults to work correctly,
    /// the identity operation should have _space_group_symop_id or
    /// _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    pub pos_as_xyz: Option<CharString>,
    /// The `_symmetry_equiv_pos_site_id` item. The field holds the column values, one per loop row.
    ///
    /// A code identifying each entry in the _symmetry_equiv_pos_as_xyz
    /// list. It is normally the sequence number of the entry in that list,
    /// and should be identified with the code 'n' in _geom_*_symmetry_
    /// codes of the form 'n_klm'. In order for the defaults to work
    /// correctly, the identity operation should have _space_group_symop_id
    /// or _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    pub pos_site_id: Option<Numeric>,
}

impl CifCategory for Vec<SymmetryEquiv> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.pos_as_xyz.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.pos_as_xyz {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("symmetry_equiv_pos_as_xyz".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.pos_site_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.pos_site_id {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("symmetry_equiv_pos_site_id".to_string()),
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
                    // the category column tags. A sibling loop of another
                    // category stays in the residue.
                    let ours = ["symmetry_equiv_pos_as_xyz", "symmetry_equiv_pos_site_id"];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = symmetryequiv_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn symmetryequiv_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<SymmetryEquiv> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![SymmetryEquiv::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "symmetry_equiv_pos_as_xyz")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("symmetry_equiv_pos_as_xyz", v) {
                Ok(val) => rows[i].pos_as_xyz = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "symmetry_equiv_pos_site_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("symmetry_equiv_pos_site_id", v) {
                Ok(val) => rows[i].pos_site_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
