//! The `geom_contact` category (6 items, all loop columns).
//!
//! One loop lists the intermolecular contacts of the special list:
//! the two site labels, the distance, the publication flag, and the
//! site symmetry of the two sites.

mod ranges;

pub use ranges::ContactDistance;

use std::fmt::Display;

use crate::data_dict::core_cif::geom::PublFlag;
use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crate::grammar::{CharString, Tag, Value};

/// Loop columns of the `geom_contact` category (6 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomContactLoopItem {
    Atom_site_label_1(Vec<CharString>),
    Atom_site_label_2(Vec<CharString>),
    Distance(Vec<ContactDistance>),
    Publ_flag(Vec<PublFlag>),
    Site_symmetry_1(Vec<CharString>),
    Site_symmetry_2(Vec<CharString>),
}

impl CifTerm for GeomContactLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomContactLoopItem::Atom_site_label_1(_) => "atom_site_label_1",
            GeomContactLoopItem::Atom_site_label_2(_) => "atom_site_label_2",
            GeomContactLoopItem::Distance(_) => "distance",
            GeomContactLoopItem::Publ_flag(_) => "publ_flag",
            GeomContactLoopItem::Site_symmetry_1(_) => "site_symmetry_1",
            GeomContactLoopItem::Site_symmetry_2(_) => "site_symmetry_2",
        };
        Tag::new(format!("geom_contact_{suffix}"))
    }
}

impl LoopValueTerm for GeomContactLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            GeomContactLoopItem::Atom_site_label_1(v)
            | GeomContactLoopItem::Atom_site_label_2(v)
            | GeomContactLoopItem::Site_symmetry_1(v)
            | GeomContactLoopItem::Site_symmetry_2(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            GeomContactLoopItem::Distance(v) => v.iter().cloned().map(Value::from).collect(),
            GeomContactLoopItem::Publ_flag(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

// The T6 single-value item of the `geom_contact` category.

/// The single-value item of the `geom_contact` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomContactTerms {
    Id(CharString),
}

impl CifTerm for GeomContactTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomContactTerms::Id(_) => "id",
        };
        Tag::new(format!("geom_contact_{suffix}"))
    }
}

impl SingleValueTerm for GeomContactTerms {
    fn value(&self) -> Value {
        match self {
            GeomContactTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomContactTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for GeomContactLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
