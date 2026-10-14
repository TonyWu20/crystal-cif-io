//! The `chemical_conn_bond` category (3 items, all loop columns).
//!
//! One loop lists the bonds of the connection table: the two atom
//! indices and the bond type code.

mod bond_type;
mod chemical_conn_bond;
mod ranges;

pub use chemical_conn_bond::{ChemicalConnBond, ChemicalConnBondRow};
pub use bond_type::BondType;
pub use ranges::BondAtomNumber;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

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

// The T6 single-value items of the `chemical_conn_bond` category.

/// The single-value items of the `chemical_conn_bond` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalConnBondTerms {
    /// The dotted item name is recorded as written in the shell
    /// dictionary; the tag keeps the dot. The range constraint is
    /// recorded in the shell `_enumeration` (`0.0:`); the plain
    /// numeric form is kept.
    Chem_comp_bond_value_dist(Numeric),
    Id(CharString),
}

impl CifTerm for ChemicalConnBondTerms {
    fn tag(&self) -> Tag {
        let name = match self {
            ChemicalConnBondTerms::Chem_comp_bond_value_dist(_) => "chem_comp_bond.value_dist",
            ChemicalConnBondTerms::Id(_) => "chemical_conn_bond_id",
        };
        Tag::new(name.to_string())
    }
}

impl SingleValueTerm for ChemicalConnBondTerms {
    fn value(&self) -> Value {
        match self {
            ChemicalConnBondTerms::Chem_comp_bond_value_dist(n) => Value::Numeric(*n),
            ChemicalConnBondTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for ChemicalConnBondTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
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
