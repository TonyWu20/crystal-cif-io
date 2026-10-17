use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `atom_analytical_source` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomAnalyticalSourceTerms {
    /// The `_atom_analytical_source_equipment_make` item. The variant holds the single value.
    Equipment_make(CharString),
    /// The `_atom_analytical_source_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_atom_analytical_source_special_details` item. The variant holds the single value.
    Special_details(CharString),
    /// The `_atom_analytical_source_technique` item. The variant holds the single value.
    Technique(CharString),
}

impl CifTerm for AtomAnalyticalSourceTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomAnalyticalSourceTerms::Equipment_make(_) => "equipment_make",
            AtomAnalyticalSourceTerms::Id(_) => "id",
            AtomAnalyticalSourceTerms::Special_details(_) => "special_details",
            AtomAnalyticalSourceTerms::Technique(_) => "technique",
        };
        Tag::new(format!("atom_analytical_source_{suffix}"))
    }
}

impl SingleValueTerm for AtomAnalyticalSourceTerms {
    fn value(&self) -> Value {
        match self {
            AtomAnalyticalSourceTerms::Equipment_make(s)
            | AtomAnalyticalSourceTerms::Id(s)
            | AtomAnalyticalSourceTerms::Special_details(s)
            | AtomAnalyticalSourceTerms::Technique(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for AtomAnalyticalSourceTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
