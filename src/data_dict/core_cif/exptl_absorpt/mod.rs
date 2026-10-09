use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ExptlAbsorptTerms {
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
