mod relation;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::relation::Relation;

/// The single-value terms of the `database_related` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DatabaseRelatedTerms {
    /// The `_database_related_database_id` item. The variant holds the single value.
    Database_id(CharString),
    /// The `_database_related_entry_code` item. The variant holds the single value.
    Entry_code(CharString),
    /// The `_database_related_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_database_related_relation` item. The variant holds the single value.
    /// closed code set recorded in the dictionary _enumeration_set
    Relation(Relation),
    /// The `_database_related_special_details` item. The variant holds the single value.
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
