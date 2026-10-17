use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `space_group_generator` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupGeneratorTerms {
    /// The `_space_group_generator_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_space_group_generator_xyz` item. The variant holds the single value.
    Xyz(CharString),
}

impl CifTerm for SpaceGroupGeneratorTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupGeneratorTerms::Id(_) => "id",
            SpaceGroupGeneratorTerms::Xyz(_) => "xyz",
        };
        Tag::new(format!("space_group_generator_{suffix}"))
    }
}

impl SingleValueTerm for SpaceGroupGeneratorTerms {
    fn value(&self) -> Value {
        match self {
            SpaceGroupGeneratorTerms::Id(s) | SpaceGroupGeneratorTerms::Xyz(s) => {
                Value::CharString(s.clone())
            }
        }
    }
}

impl Display for SpaceGroupGeneratorTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
