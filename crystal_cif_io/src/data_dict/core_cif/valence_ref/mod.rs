//! The `valence_ref` category (2 items, all loop columns).
//!
//! One loop lists the references of the valence models: the model
//! tag and the reference string.

mod valence_ref;

pub use valence_ref::ValenceRefRow;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// Loop columns of the `valence_ref` category (2 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ValenceRefLoopItem {
    Id(Vec<CharString>),
    Reference(Vec<CharString>),
}

impl CifTerm for ValenceRefLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ValenceRefLoopItem::Id(_) => "id",
            ValenceRefLoopItem::Reference(_) => "reference",
        };
        Tag::new(format!("valence_ref_{suffix}"))
    }
}

impl LoopValueTerm for ValenceRefLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ValenceRefLoopItem::Id(v) | ValenceRefLoopItem::Reference(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for ValenceRefLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
