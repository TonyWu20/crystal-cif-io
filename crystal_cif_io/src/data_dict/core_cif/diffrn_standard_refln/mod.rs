use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnStandardReflnTerms {
    Hkl(Matrix),
}

impl CifTerm for DiffrnStandardReflnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnStandardReflnTerms::Hkl(_) => "hkl",
        };
        Tag::new(format!("diffrn_standard_refln_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnStandardReflnTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnStandardReflnTerms::Hkl(m) => matrix::to_value(m),
        }
    }
}

impl Display for DiffrnStandardReflnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnStandardReflnLoopItem {
    Code(Vec<CharString>),
    Index_h(Vec<Numeric>),
    Index_k(Vec<Numeric>),
    Index_l(Vec<Numeric>),
}

impl CifTerm for DiffrnStandardReflnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnStandardReflnLoopItem::Code(_) => "code",
            DiffrnStandardReflnLoopItem::Index_h(_) => "index_h",
            DiffrnStandardReflnLoopItem::Index_k(_) => "index_k",
            DiffrnStandardReflnLoopItem::Index_l(_) => "index_l",
        };
        Tag::new(format!("diffrn_standard_refln_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnStandardReflnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnStandardReflnLoopItem::Code(s) => s.iter().cloned().map(Value::from).collect(),
            DiffrnStandardReflnLoopItem::Index_h(n)
            | DiffrnStandardReflnLoopItem::Index_k(n)
            | DiffrnStandardReflnLoopItem::Index_l(n) => {
                n.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
