mod symkey;

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Tag, Value};

pub use self::symkey::Symkey;

/// The single-value terms of the `function` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum FunctionTerms {
    /// The `_function_atomtype` item. The variant holds the single value.
    Atomtype(CharString),
    /// The `_function_closest` item. The variant holds the single value.
    Closest(Matrix),
    /// The `_function_seitzfromjones` item. The variant holds the single value.
    Seitzfromjones(Matrix),
    /// The `_function_symequiv` item. The variant holds the single value.
    Symequiv(Matrix),
    /// The `_function_symkey` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Symkey(Symkey),
    /// The `_function_symlat` item. The variant holds the single value.
    Symlat(Matrix),
    /// The `_function_symop` item. The variant holds the single value.
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
