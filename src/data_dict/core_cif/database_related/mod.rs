mod relation;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::relation::Relation;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DatabaseRelatedTerms {
    Database_id(CharString),
    Entry_code(CharString),
    Id(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Relation(Relation),
    Special_details(CharString),
}

impl CifTerm for DatabaseRelatedTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DatabaseRelatedTerms::Database_id(_) => "database_id",
            DatabaseRelatedTerms::Entry_code(_) => "entry_code",
            DatabaseRelatedTerms::Id(_) => "id",
            DatabaseRelatedTerms::Relation(_) => "relation",
            DatabaseRelatedTerms::Special_details(_) => "special_details",
        };
        Tag::new(format!("database_related_{suffix}"))
    }
}

impl SingleValueTerm for DatabaseRelatedTerms {
    fn value(&self) -> Value {
        match self {
            DatabaseRelatedTerms::Database_id(s)
            | DatabaseRelatedTerms::Entry_code(s)
            | DatabaseRelatedTerms::Id(s)
            | DatabaseRelatedTerms::Special_details(s) => Value::CharString(s.clone()),
            DatabaseRelatedTerms::Relation(v) => Value::from(*v),
        }
    }
}

impl Display for DatabaseRelatedTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
