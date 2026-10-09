use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomAnalyticalSourceTerms {
    Equipment_make(CharString),
    Id(CharString),
    Special_details(CharString),
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
