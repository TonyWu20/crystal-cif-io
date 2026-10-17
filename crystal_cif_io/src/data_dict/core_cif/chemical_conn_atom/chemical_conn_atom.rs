//! The `chemical_conn_atom` category of the typed document layer (roadmap D12).
//!
//! The `ChemicalConnAtomRow` loop row of the connection-table atom
//! loop: the atom index, the type symbol, the charge, the hydrogen
//! and neighbour counts, and the display coordinates.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::chemical_conn_atom::{
    ConnAtomCharge, ConnAtomCount, ConnAtomDisplay, ConnAtomNumber,
};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
/// One loop row of the `chemical_conn_atom` category: the per-observation values of the loop items.
pub struct ChemicalConnAtomRow {
    /// The `_chemical_conn_atom_charge` item. The field holds the column values, one per loop row.
    ///
    /// The net integer charge assigned to this atom. This is the formal
    /// charge assignment normally found in chemical diagrams.
    pub charge: Option<ConnAtomCharge>,
    /// The `_chemical_conn_atom_display_x` item. The field holds the column values, one per loop row.
    ///
    /// The 2D Cartesian coordinates (x,y) of the position of this atom in a
    /// recognizable chemical diagram. The coordinate origin is at the lower
    /// left corner, the x axis is horizontal and the y axis is vertical.
    /// The coordinates must lie in the range 0.0 to 1.0. These coordinates
    /// can be obtained from projections of a suitable uncluttered view of
    /// the molecular structure.
    pub display_x: Option<ConnAtomDisplay>,
    /// The `_chemical_conn_atom_display_y` item. The field holds the column values, one per loop row.
    ///
    /// The 2D Cartesian coordinates (x,y) of the position of this atom in a
    /// recognizable chemical diagram. The coordinate origin is at the lower
    /// left corner, the x axis is horizontal and the y axis is vertical.
    /// The coordinates must lie in the range 0.0 to 1.0. These coordinates
    /// can be obtained from projections of a suitable uncluttered view of
    /// the molecular structure.
    pub display_y: Option<ConnAtomDisplay>,
    /// The `_chemical_conn_atom_NCA` item. The field holds the column values, one per loop row.
    ///
    /// The number of connected atoms excluding terminal hydrogen atoms.
    pub nca: Option<ConnAtomCount>,
    /// The `_chemical_conn_atom_NH` item. The field holds the column values, one per loop row.
    ///
    /// The total number of hydrogen atoms attached to this atom, regardless
    /// of whether they are included in the refinement or the _atom_site_
    /// list. This number will be the same as _atom_site_attached_hydrogens
    /// only if none of the hydrogen atoms appear in the _atom_site_ list.
    pub nh: Option<ConnAtomCount>,
    /// The `_chemical_conn_atom_number` item. The field holds the column values, one per loop row.
    ///
    /// The chemical sequence number to be associated with this atom.
    pub number: Option<ConnAtomNumber>,
    /// The `_chemical_conn_atom_type_symbol` item. The field holds the column values, one per loop row.
    ///
    /// A code identifying the atom type. This code must match an
    /// _atom_type_symbol code in the _atom_type_ list or be a recognizable
    /// element symbol.
    pub type_symbol: Option<CharString>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "chemical_conn_atom_charge",
    "chemical_conn_atom_display_x",
    "chemical_conn_atom_display_y",
    "chemical_conn_atom_nca",
    "chemical_conn_atom_nh",
    "chemical_conn_atom_number",
    "chemical_conn_atom_type_symbol",
];

impl CifCategory for Vec<ChemicalConnAtomRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.charge.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.charge {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_conn_atom_charge".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.display_x.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.display_x {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_conn_atom_display_x".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.display_y.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.display_y {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_conn_atom_display_y".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.nca.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.nca {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_conn_atom_NCA".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.nh.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.nh {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_conn_atom_NH".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_conn_atom_number".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.type_symbol.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.type_symbol {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_conn_atom_type_symbol".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ChemicalConnAtomRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ChemicalConnAtomRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_atom_charge")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_conn_atom_charge("chemical_conn_atom_charge", v) {
                Ok(val) => rows[i].charge = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_atom_display_x")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_conn_atom_display("chemical_conn_atom_display_x", v) {
                Ok(val) => rows[i].display_x = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_atom_display_y")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_conn_atom_display("chemical_conn_atom_display_y", v) {
                Ok(val) => rows[i].display_y = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_atom_nca")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_conn_atom_count("chemical_conn_atom_nca", v) {
                Ok(val) => rows[i].nca = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_atom_nh")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_conn_atom_count("chemical_conn_atom_nh", v) {
                Ok(val) => rows[i].nh = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_atom_number")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_conn_atom_number("chemical_conn_atom_number", v) {
                Ok(val) => rows[i].number = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_atom_type_symbol")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("chemical_conn_atom_type_symbol", v) {
                Ok(val) => rows[i].type_symbol = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
