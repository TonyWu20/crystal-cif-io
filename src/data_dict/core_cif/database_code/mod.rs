use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DatabaseCodeTerms {
    Amcsd(CharString),
    Bincstrdb(CharString),
}

impl CifTerm for DatabaseCodeTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DatabaseCodeTerms::Amcsd(_) => "amcsd",
            DatabaseCodeTerms::Bincstrdb(_) => "bincstrdb",
        };
        Tag::new(format!("database_code_{suffix}"))
    }
}

impl SingleValueTerm for DatabaseCodeTerms {
    fn value(&self) -> Value {
        match self {
            DatabaseCodeTerms::Amcsd(s) | DatabaseCodeTerms::Bincstrdb(s) => {
                Value::CharString(s.clone())
            }
        }
    }
}

impl Display for DatabaseCodeTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
