//! The `geom_bond` category (8 items, all loop columns).
//!
//! One loop lists the bonds of the special list: the two site
//! labels, the distance, the bond order, the publication flag, the
//! site symmetry of the two sites, and the valence.

mod ranges;

pub use ranges::{BondDistance, BondMultiplicity};

use std::fmt::Display;

use crate::data_dict::core_cif::geom::PublFlag;
use crate::data_dict::{CifTerm, LoopValueTerm};
use crate::grammar::{CharString, Numeric, Tag, Value};

/// Loop columns of the `geom_bond` category (8 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomBondLoopItem {
    Atom_site_label_1(Vec<CharString>),
    Atom_site_label_2(Vec<CharString>),
    Distance(Vec<BondDistance>),
    Multiplicity(Vec<BondMultiplicity>),
    Publ_flag(Vec<PublFlag>),
    Site_symmetry_1(Vec<CharString>),
    Site_symmetry_2(Vec<CharString>),
    Valence(Vec<Numeric>),
}

impl CifTerm for GeomBondLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomBondLoopItem::Atom_site_label_1(_) => "atom_site_label_1",
            GeomBondLoopItem::Atom_site_label_2(_) => "atom_site_label_2",
            GeomBondLoopItem::Distance(_) => "distance",
            GeomBondLoopItem::Multiplicity(_) => "multiplicity",
            GeomBondLoopItem::Publ_flag(_) => "publ_flag",
            GeomBondLoopItem::Site_symmetry_1(_) => "site_symmetry_1",
            GeomBondLoopItem::Site_symmetry_2(_) => "site_symmetry_2",
            GeomBondLoopItem::Valence(_) => "valence",
        };
        Tag::new(format!("geom_bond_{suffix}"))
    }
}

impl LoopValueTerm for GeomBondLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            GeomBondLoopItem::Atom_site_label_1(v)
            | GeomBondLoopItem::Atom_site_label_2(v)
            | GeomBondLoopItem::Site_symmetry_1(v)
            | GeomBondLoopItem::Site_symmetry_2(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Valence(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Distance(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Multiplicity(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Publ_flag(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

impl Display for GeomBondLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
