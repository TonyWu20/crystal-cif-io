use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `exptl_absorpt` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ExptlAbsorptTerms {
    /// The `_exptl_absorpt_special_details` item. The variant holds the single value.
    Special_details(CharString),
}

impl CifTerm for ExptlAbsorptTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ExptlAbsorptTerms::Special_details(_) => "special_details",
        };
        Tag::new(format!("exptl_absorpt_{suffix}"))
    }
}

impl SingleValueTerm for ExptlAbsorptTerms {
    fn value(&self) -> Value {
        match self {
            ExptlAbsorptTerms::Special_details(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for ExptlAbsorptTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
