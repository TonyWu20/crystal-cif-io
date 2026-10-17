//! The `chemical_conn_atom` category (7 items, all loop columns).
//!
//! One loop lists the atoms of the connection table. Each column is
//! a property of an atom: its index, its type symbol, its charge,
//! the hydrogen and neighbour counts, and the display coordinates.

mod chemical_conn_atom;
mod ranges;

pub use chemical_conn_atom::ChemicalConnAtomRow;
pub use ranges::{ConnAtomCharge, ConnAtomCount, ConnAtomDisplay, ConnAtomNumber};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// Loop columns of the `chemical_conn_atom` category (7 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `chemical_conn_atom` category: one variant per

/// registry loop-value item, in item order.
pub enum ChemicalConnAtomLoopItem {
    /// The `_chemical_conn_atom_NCA` item. The variant holds the column values, one per loop row.
    ///
    /// The number of connected atoms excluding terminal hydrogen atoms.
    Nca(Vec<ConnAtomCount>),
    /// The `_chemical_conn_atom_NH` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of hydrogen atoms attached to this atom, regardless
    /// of whether they are included in the refinement or the _atom_site_
    /// list. This number will be the same as _atom_site_attached_hydrogens
    /// only if none of the hydrogen atoms appear in the _atom_site_ list.
    Nh(Vec<ConnAtomCount>),
    /// The `_chemical_conn_atom_charge` item. The variant holds the column values, one per loop row.
    ///
    /// The net integer charge assigned to this atom. This is the formal
    /// charge assignment normally found in chemical diagrams.
    Charge(Vec<ConnAtomCharge>),
    /// The `_chemical_conn_atom_display_x` item. The variant holds the column values, one per loop row.
    ///
    /// The 2D Cartesian coordinates (x,y) of the position of this atom in a
    /// recognizable chemical diagram. The coordinate origin is at the lower
    /// left corner, the x axis is horizontal and the y axis is vertical.
    /// The coordinates must lie in the range 0.0 to 1.0. These coordinates
    /// can be obtained from projections of a suitable uncluttered view of
    /// the molecular structure.
    Display_x(Vec<ConnAtomDisplay>),
    /// The `_chemical_conn_atom_display_y` item. The variant holds the column values, one per loop row.
    ///
    /// The 2D Cartesian coordinates (x,y) of the position of this atom in a
    /// recognizable chemical diagram. The coordinate origin is at the lower
    /// left corner, the x axis is horizontal and the y axis is vertical.
    /// The coordinates must lie in the range 0.0 to 1.0. These coordinates
    /// can be obtained from projections of a suitable uncluttered view of
    /// the molecular structure.
    Display_y(Vec<ConnAtomDisplay>),
    /// The `_chemical_conn_atom_number` item. The variant holds the column values, one per loop row.
    ///
    /// The chemical sequence number to be associated with this atom.
    Number(Vec<ConnAtomNumber>),
    /// The `_chemical_conn_atom_type_symbol` item. The variant holds the column values, one per loop row.
    ///
    /// A code identifying the atom type. This code must match an
    /// _atom_type_symbol code in the _atom_type_ list or be a recognizable
    /// element symbol.
    Type_symbol(Vec<CharString>),
}

impl CifTerm for ChemicalConnAtomLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ChemicalConnAtomLoopItem::Nca(_) => "NCA",
            ChemicalConnAtomLoopItem::Nh(_) => "NH",
            ChemicalConnAtomLoopItem::Charge(_) => "charge",
            ChemicalConnAtomLoopItem::Display_x(_) => "display_x",
            ChemicalConnAtomLoopItem::Display_y(_) => "display_y",
            ChemicalConnAtomLoopItem::Number(_) => "number",
            ChemicalConnAtomLoopItem::Type_symbol(_) => "type_symbol",
        };
        Tag::new(format!("chemical_conn_atom_{suffix}"))
    }
}

impl LoopValueTerm for ChemicalConnAtomLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ChemicalConnAtomLoopItem::Nca(v) | ChemicalConnAtomLoopItem::Nh(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            ChemicalConnAtomLoopItem::Charge(v) => v.iter().cloned().map(Value::from).collect(),
            ChemicalConnAtomLoopItem::Display_x(v) | ChemicalConnAtomLoopItem::Display_y(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            ChemicalConnAtomLoopItem::Number(v) => v.iter().cloned().map(Value::from).collect(),
            ChemicalConnAtomLoopItem::Type_symbol(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for ChemicalConnAtomLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
