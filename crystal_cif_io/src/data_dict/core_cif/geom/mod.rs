//! The `geom` category (1 item).
//!
//! `_geom_special_details` records information about the geometry
//! checks not coded elsewhere. The category also holds
//! [`PublFlag`], the closed code set shared by the `geom_*`
//! special-list flags.

mod publ_flag;

pub use publ_flag::PublFlag;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Single-value items of the `geom` category (1 item).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomItem {
    Bond_distance_incr(Numeric),
    Contact_distance_incr(Numeric),
    Min_bond_distance_cutoff(Numeric),
    Min_contact_distance_cutoff(Numeric),
    Special_details(CharString),
}

impl CifTerm for GeomItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomItem::Bond_distance_incr(_) => "bond_distance_incr",
            GeomItem::Contact_distance_incr(_) => "contact_distance_incr",
            GeomItem::Min_bond_distance_cutoff(_) => "min_bond_distance_cutoff",
            GeomItem::Min_contact_distance_cutoff(_) => "min_contact_distance_cutoff",
            GeomItem::Special_details(_) => "special_details",
        };
        Tag::new(format!("geom_{suffix}"))
    }
}

impl SingleValueTerm for GeomItem {
    fn value(&self) -> Value {
        match self {
            GeomItem::Bond_distance_incr(n)
            | GeomItem::Contact_distance_incr(n)
            | GeomItem::Min_bond_distance_cutoff(n)
            | GeomItem::Min_contact_distance_cutoff(n) => Value::Numeric(*n),
            GeomItem::Special_details(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
