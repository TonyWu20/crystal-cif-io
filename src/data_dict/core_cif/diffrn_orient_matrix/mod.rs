use std::fmt::Display;

use crate::{
    data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Matrix, Numeric, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientMatrixTerms {
    Type(CharString),
    Ubij(Matrix),
}

impl CifTerm for DiffrnOrientMatrixTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientMatrixTerms::Type(_) => "type",
            DiffrnOrientMatrixTerms::Ubij(_) => "ubij",
        };
        Tag::new(format!("diffrn_orient_matrix_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnOrientMatrixTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnOrientMatrixTerms::Type(s) => Value::CharString(s.clone()),
            DiffrnOrientMatrixTerms::Ubij(m) => matrix::to_value(m),
        }
    }
}

impl Display for DiffrnOrientMatrixTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientMatrixLoopItem {
    UB_11(Vec<Numeric>),
    UB_12(Vec<Numeric>),
    UB_13(Vec<Numeric>),
    UB_21(Vec<Numeric>),
    UB_22(Vec<Numeric>),
    UB_23(Vec<Numeric>),
    UB_31(Vec<Numeric>),
    UB_32(Vec<Numeric>),
    UB_33(Vec<Numeric>),
}

impl CifTerm for DiffrnOrientMatrixLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientMatrixLoopItem::UB_11(_) => "ub_11",
            DiffrnOrientMatrixLoopItem::UB_12(_) => "ub_12",
            DiffrnOrientMatrixLoopItem::UB_13(_) => "ub_13",
            DiffrnOrientMatrixLoopItem::UB_21(_) => "ub_21",
            DiffrnOrientMatrixLoopItem::UB_22(_) => "ub_22",
            DiffrnOrientMatrixLoopItem::UB_23(_) => "ub_23",
            DiffrnOrientMatrixLoopItem::UB_31(_) => "ub_31",
            DiffrnOrientMatrixLoopItem::UB_32(_) => "ub_32",
            DiffrnOrientMatrixLoopItem::UB_33(_) => "ub_33",
        };
        Tag::new(format!("diffrn_orient_matrix_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnOrientMatrixLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnOrientMatrixLoopItem::UB_11(n)
            | DiffrnOrientMatrixLoopItem::UB_12(n)
            | DiffrnOrientMatrixLoopItem::UB_13(n)
            | DiffrnOrientMatrixLoopItem::UB_21(n)
            | DiffrnOrientMatrixLoopItem::UB_22(n)
            | DiffrnOrientMatrixLoopItem::UB_23(n)
            | DiffrnOrientMatrixLoopItem::UB_31(n)
            | DiffrnOrientMatrixLoopItem::UB_32(n)
            | DiffrnOrientMatrixLoopItem::UB_33(n) => n.iter().cloned().map(Value::from).collect(),
        }
    }
}
