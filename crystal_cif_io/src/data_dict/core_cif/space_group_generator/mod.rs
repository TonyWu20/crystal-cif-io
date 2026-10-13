use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupGeneratorTerms {
    Id(CharString),
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
