use std::fmt::Display;

use crate::{
    data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Matrix, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupSymopTerms {
    Operation_description(CharString),
    R(Matrix),
    Rt(Matrix),
    Seitz_matrix(Matrix),
    T(Matrix),
}

impl CifTerm for SpaceGroupSymopTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupSymopTerms::Operation_description(_) => "operation_description",
            SpaceGroupSymopTerms::R(_) => "r",
            SpaceGroupSymopTerms::Rt(_) => "rt",
            SpaceGroupSymopTerms::Seitz_matrix(_) => "seitz_matrix",
            SpaceGroupSymopTerms::T(_) => "t",
        };
        Tag::new(format!("space_group_symop_{suffix}"))
    }
}

impl SingleValueTerm for SpaceGroupSymopTerms {
    fn value(&self) -> Value {
        match self {
            SpaceGroupSymopTerms::Operation_description(s) => Value::CharString(s.clone()),
            SpaceGroupSymopTerms::R(m)
            | SpaceGroupSymopTerms::Rt(m)
            | SpaceGroupSymopTerms::Seitz_matrix(m)
            | SpaceGroupSymopTerms::T(m) => matrix::to_value(m),
        }
    }
}

impl Display for SpaceGroupSymopTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupSymopLoopItem {
    Id(Vec<CharString>),
    Operation_xyz(Vec<CharString>),
}

impl CifTerm for SpaceGroupSymopLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupSymopLoopItem::Id(_) => "id",
            SpaceGroupSymopLoopItem::Operation_xyz(_) => "operation_xyz",
        };
        Tag::new(format!("space_group_symop_{suffix}"))
    }
}

impl LoopValueTerm for SpaceGroupSymopLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            SpaceGroupSymopLoopItem::Id(s) | SpaceGroupSymopLoopItem::Operation_xyz(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
