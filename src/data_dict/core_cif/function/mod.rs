mod symkey;

use std::fmt::Display;

use crate::{
    data_dict::{matrix, CifTerm, SingleValueTerm},
    grammar::{CharString, Matrix, Tag, Value},
};

pub use self::symkey::Symkey;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum FunctionTerms {
    Atomtype(CharString),
    Closest(Matrix),
    Seitzfromjones(Matrix),
    Symequiv(Matrix),
    /// range constraint recorded in the dictionary _enumeration
    Symkey(Symkey),
    Symlat(Matrix),
    Symop(CharString),
}

impl CifTerm for FunctionTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            FunctionTerms::Atomtype(_) => "atomtype",
            FunctionTerms::Closest(_) => "closest",
            FunctionTerms::Seitzfromjones(_) => "seitzfromjones",
            FunctionTerms::Symequiv(_) => "symequiv",
            FunctionTerms::Symkey(_) => "symkey",
            FunctionTerms::Symlat(_) => "symlat",
            FunctionTerms::Symop(_) => "symop",
        };
        Tag::new(format!("function_{suffix}"))
    }
}

impl SingleValueTerm for FunctionTerms {
    fn value(&self) -> Value {
        match self {
            FunctionTerms::Atomtype(s) | FunctionTerms::Symop(s) => Value::CharString(s.clone()),
            FunctionTerms::Closest(m)
            | FunctionTerms::Seitzfromjones(m)
            | FunctionTerms::Symequiv(m)
            | FunctionTerms::Symlat(m) => matrix::to_value(m),
            FunctionTerms::Symkey(v) => Value::from(*v),
        }
    }
}

impl Display for FunctionTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
