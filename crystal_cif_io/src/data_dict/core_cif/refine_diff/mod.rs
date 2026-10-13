use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Numeric, Tag, Value};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RefineDiffTerms {
    Potential_max(Numeric),
    Potential_min(Numeric),
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
