//! The `chemical_conn_bond` category (3 items, all loop columns).
//!
//! One loop lists the bonds of the connection table: the two atom
//! indices and the bond type code.

mod bond_type;
mod ranges;

pub use bond_type::BondType;
pub use ranges::BondAtomNumber;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{Tag, Value},
};

/// Loop columns of the `chemical_conn_bond` category (3 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalConnBondLoopItem {
    Atom_1(Vec<BondAtomNumber>),
    Atom_2(Vec<BondAtomNumber>),
    Type(Vec<BondType>),
}

impl CifTerm for ChemicalConnBondLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ChemicalConnBondLoopItem::Atom_1(_) => "atom_1",
            ChemicalConnBondLoopItem::Atom_2(_) => "atom_2",
            ChemicalConnBondLoopItem::Type(_) => "type",
        };
        Tag::new(format!("chemical_conn_bond_{suffix}"))
    }
}

impl LoopValueTerm for ChemicalConnBondLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ChemicalConnBondLoopItem::Atom_1(v) | ChemicalConnBondLoopItem::Atom_2(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            ChemicalConnBondLoopItem::Type(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

impl Display for ChemicalConnBondLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
