//! The `chemical_formula` category (7 items, all single values).
//!
//! The formula in its recorded forms (sum, structural, analytical,
//! IUPAC, moiety) and the molar mass, with the measured companion.

mod chemical_formula;
mod ranges;

pub use chemical_formula::ChemicalFormula;
pub use ranges::FormulaWeight;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// Single-value items of the `chemical_formula` category (7 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalFormulaItem {
    /// The `_chemical_formula_analytical` item. The variant holds the single value.
    Analytical(CharString),
    /// The `_chemical_formula_iupac` item. The variant holds the single value.
    Iupac(CharString),
    /// The `_chemical_formula_moiety` item. The variant holds the single value.
    Moiety(CharString),
    /// The `_chemical_formula_structural` item. The variant holds the single value.
    Structural(CharString),
    /// The `_chemical_formula_sum` item. The variant holds the single value.
    Sum(CharString),
    /// The `_chemical_formula_weight` item. The variant holds the single value.
    Weight(FormulaWeight),
    /// The `_chemical_formula_weight_meas` item. The variant holds the single value.
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
