//! The `chemical_formula` category (7 items, all single values).
//!
//! The formula in its recorded forms (sum, structural, analytical,
//! IUPAC, moiety) and the molar mass, with the measured companion.

mod ranges;

pub use ranges::FormulaWeight;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// Single-value items of the `chemical_formula` category (7 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalFormulaItem {
    Analytical(CharString),
    Iupac(CharString),
    Moiety(CharString),
    Structural(CharString),
    Sum(CharString),
    Weight(FormulaWeight),
    Weight_meas(FormulaWeight),
}

impl CifTerm for ChemicalFormulaItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ChemicalFormulaItem::Analytical(_) => "analytical",
            ChemicalFormulaItem::Iupac(_) => "iupac",
            ChemicalFormulaItem::Moiety(_) => "moiety",
            ChemicalFormulaItem::Structural(_) => "structural",
            ChemicalFormulaItem::Sum(_) => "sum",
            ChemicalFormulaItem::Weight(_) => "weight",
            ChemicalFormulaItem::Weight_meas(_) => "weight_meas",
        };
        Tag::new(format!("chemical_formula_{suffix}"))
    }
}

impl SingleValueTerm for ChemicalFormulaItem {
    fn value(&self) -> Value {
        match self {
            ChemicalFormulaItem::Analytical(s)
            | ChemicalFormulaItem::Iupac(s)
            | ChemicalFormulaItem::Moiety(s)
            | ChemicalFormulaItem::Structural(s)
            | ChemicalFormulaItem::Sum(s) => Value::CharString(s.clone()),
            ChemicalFormulaItem::Weight(v) | ChemicalFormulaItem::Weight_meas(v) => Value::from(*v),
        }
    }
}

impl Display for ChemicalFormulaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
