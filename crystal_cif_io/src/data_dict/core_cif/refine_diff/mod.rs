use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Numeric, Tag, Value};

/// The single-value terms of the `refine_diff` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RefineDiffTerms {
    /// The `_refine_diff_potential_max` item. The variant holds the single value.
    Potential_max(Numeric),
    /// The `_refine_diff_potential_min` item. The variant holds the single value.
    Potential_min(Numeric),
    /// The `_refine_diff_potential_rms` item. The variant holds the single value.
    Potential_rms(Numeric),
}

impl CifTerm for RefineDiffTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            RefineDiffTerms::Potential_max(_) => "potential_max",
            RefineDiffTerms::Potential_min(_) => "potential_min",
            RefineDiffTerms::Potential_rms(_) => "potential_rms",
        };
        Tag::new(format!("refine_diff_{suffix}"))
    }
}

impl SingleValueTerm for RefineDiffTerms {
    fn value(&self) -> Value {
        match self {
            RefineDiffTerms::Potential_max(n)
            | RefineDiffTerms::Potential_min(n)
            | RefineDiffTerms::Potential_rms(n) => Value::Numeric(*n),
        }
    }
}

impl Display for RefineDiffTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
