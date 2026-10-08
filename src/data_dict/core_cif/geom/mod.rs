//! The `geom` category (1 item).
//!
//! `_geom_special_details` records information about the geometry
//! checks not coded elsewhere. The category also holds
//! [`PublFlag`], the closed code set shared by the `geom_*`
//! special-list flags.

mod publ_flag;

pub use publ_flag::PublFlag;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

/// Single-value items of the `geom` category (1 item).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomItem {
    Special_details(CharString),
}

impl CifTerm for GeomItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomItem::Special_details(_) => "special_details",
        };
        Tag::new(format!("geom_{suffix}"))
    }
}

impl SingleValueTerm for GeomItem {
    fn value(&self) -> Value {
        match self {
            GeomItem::Special_details(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
