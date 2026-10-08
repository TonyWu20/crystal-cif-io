//! The `chemical_conn_atom` category (7 items, all loop columns).
//!
//! One loop lists the atoms of the connection table. Each column is
//! a property of an atom: its index, its type symbol, its charge,
//! the hydrogen and neighbour counts, and the display coordinates.

mod ranges;

pub use ranges::{ConnAtomCharge, ConnAtomCount, ConnAtomDisplay, ConnAtomNumber};

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

/// Loop columns of the `chemical_conn_atom` category (7 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalConnAtomLoopItem {
    Nca(Vec<ConnAtomCount>),
    Nh(Vec<ConnAtomCount>),
    Charge(Vec<ConnAtomCharge>),
    Display_x(Vec<ConnAtomDisplay>),
    Display_y(Vec<ConnAtomDisplay>),
    Number(Vec<ConnAtomNumber>),
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
