//! The `valence_param` category (9 items, all loop columns).
//!
//! One loop lists the parameter sets of a valence model: the model
//! tag, the reference tag, the atom and valence pairs, the
//! `B`/`Ro` coefficients, and the details text.

mod valence_param;

pub use valence_param::{ValenceParam, ValenceParamRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Loop columns of the `valence_param` category (9 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ValenceParamLoopItem {
    B(Vec<Numeric>),
    Ro(Vec<Numeric>),
    Atom_1(Vec<CharString>),
    Atom_1_valence(Vec<Numeric>),
    Atom_2(Vec<CharString>),
    Atom_2_valence(Vec<Numeric>),
    Details(Vec<CharString>),
    Id(Vec<CharString>),
    Ref_id(Vec<CharString>),
}

impl CifTerm for ValenceParamLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ValenceParamLoopItem::B(_) => "B",
            ValenceParamLoopItem::Ro(_) => "Ro",
            ValenceParamLoopItem::Atom_1(_) => "atom_1",
            ValenceParamLoopItem::Atom_1_valence(_) => "atom_1_valence",
            ValenceParamLoopItem::Atom_2(_) => "atom_2",
            ValenceParamLoopItem::Atom_2_valence(_) => "atom_2_valence",
            ValenceParamLoopItem::Details(_) => "details",
            ValenceParamLoopItem::Id(_) => "id",
            ValenceParamLoopItem::Ref_id(_) => "ref_id",
        };
        Tag::new(format!("valence_param_{suffix}"))
    }
}

impl LoopValueTerm for ValenceParamLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ValenceParamLoopItem::B(v)
            | ValenceParamLoopItem::Ro(v)
            | ValenceParamLoopItem::Atom_1_valence(v)
            | ValenceParamLoopItem::Atom_2_valence(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            ValenceParamLoopItem::Atom_1(v)
            | ValenceParamLoopItem::Atom_2(v)
            | ValenceParamLoopItem::Details(v)
            | ValenceParamLoopItem::Id(v)
            | ValenceParamLoopItem::Ref_id(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}
// The T6 single-value item of the `valence_param` category.

/// The single-value item of the `valence_param` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ValenceParamTerms {
    Citation_id(CharString),
}

impl CifTerm for ValenceParamTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ValenceParamTerms::Citation_id(_) => "citation_id",
        };
        Tag::new(format!("valence_param_{suffix}"))
    }
}

impl SingleValueTerm for ValenceParamTerms {
    fn value(&self) -> Value {
        match self {
            ValenceParamTerms::Citation_id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for ValenceParamTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for ValenceParamLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
