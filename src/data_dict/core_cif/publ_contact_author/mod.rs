use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublContactAuthorTerms {
    Id(CharString),
}

impl CifTerm for PublContactAuthorTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            PublContactAuthorTerms::Id(_) => "id",
        };
        Tag::new(format!("publ_contact_author_{suffix}"))
    }
}

impl SingleValueTerm for PublContactAuthorTerms {
    fn value(&self) -> Value {
        match self {
            PublContactAuthorTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for PublContactAuthorTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
