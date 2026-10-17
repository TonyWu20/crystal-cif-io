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
/// The loop-value terms of the `valence_param` category: one variant per

/// registry loop-value item, in item order.
pub enum ValenceParamLoopItem {
    /// The `_valence_param_B` item. The variant holds the column values, one per loop row.
    ///
    /// The bond-valence parameter B used in the expression s = exp[(Ro -
    /// R)/B] where s is the valence of a bond of length R.
    /// Units: A (angstrom).
    B(Vec<Numeric>),
    /// The `_valence_param_Ro` item. The variant holds the column values, one per loop row.
    ///
    /// The bond-valence parameter Ro used in the expression s = exp[(Ro -
    /// R)/B] where s is the valence of a bond of length R.
    /// Units: A (angstrom).
    Ro(Vec<Numeric>),
    /// The `_valence_param_atom_1` item. The variant holds the column values, one per loop row.
    ///
    /// The element symbol of the first atom forming the bond whose
    /// bond-valence parameters are given in this category.
    Atom_1(Vec<CharString>),
    /// The `_valence_param_atom_1_valence` item. The variant holds the column values, one per loop row.
    ///
    /// The valence (formal charge) of the first atom whose bond-valence
    /// parameters are given in this category.
    Atom_1_valence(Vec<Numeric>),
    /// The `_valence_param_atom_2` item. The variant holds the column values, one per loop row.
    ///
    /// The element symbol of the second atom forming the bond whose
    /// bond-valence parameters are given in this category.
    Atom_2(Vec<CharString>),
    /// The `_valence_param_atom_2_valence` item. The variant holds the column values, one per loop row.
    ///
    /// The valence (formal charge) of the second atom whose bond-valence
    /// parameters are given in this category.
    Atom_2_valence(Vec<Numeric>),
    /// The `_valence_param_details` item. The variant holds the column values, one per loop row.
    ///
    /// Details of or comments on the bond-valence parameters.
    Details(Vec<CharString>),
    /// The `_valence_param_id` item. The variant holds the column values, one per loop row.
    ///
    /// An identifier for the valence parameters of a bond between the given
    /// atoms.
    Id(Vec<CharString>),
    /// The `_valence_param_ref_id` item. The variant holds the column values, one per loop row.
    ///
    /// An identifier which links to the reference to the source from which
    /// the bond-valence parameters are taken. A child of _valence_ref_id,
    /// which it must match.
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
/// The single-value terms of the `valence_param` category: one variant per

/// registry single-value item, in item order.
pub enum ValenceParamTerms {
    /// The `_valence_param_citation_id` item. The variant holds the column values, one per loop row.
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
