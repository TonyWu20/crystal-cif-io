use std::fmt::Display;

use crate::{
    data_dict::{matrix, CifTerm, SingleValueTerm},
    grammar::{Matrix, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsTransfMatrixTerms {
    Tij(Matrix),
}

impl CifTerm for DiffrnReflnsTransfMatrixTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnReflnsTransfMatrixTerms::Tij(_) => "tij",
        };
        Tag::new(format!("diffrn_reflns_transf_matrix_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnReflnsTransfMatrixTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnReflnsTransfMatrixTerms::Tij(m) => matrix::to_value(m),
        }
    }
}

impl Display for DiffrnReflnsTransfMatrixTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
