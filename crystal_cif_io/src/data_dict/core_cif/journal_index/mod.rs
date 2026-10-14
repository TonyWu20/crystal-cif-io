mod index_type;
mod journal_index;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::index_type::Type;
pub use self::journal_index::{JournalIndex, JournalIndexRow};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum JournalIndexTerms {
    Id(CharString),
}

impl CifTerm for JournalIndexTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            JournalIndexTerms::Id(_) => "id",
        };
        Tag::new(format!("journal_index_{suffix}"))
    }
}

impl SingleValueTerm for JournalIndexTerms {
    fn value(&self) -> Value {
        match self {
            JournalIndexTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for JournalIndexTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum JournalIndexLoopItem {
    Subterm(Vec<CharString>),
    Term(Vec<CharString>),
    /// closed code set recorded in the dictionary _enumeration
    Type(Vec<Type>),
}

impl CifTerm for JournalIndexLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            JournalIndexLoopItem::Subterm(_) => "subterm",
            JournalIndexLoopItem::Term(_) => "term",
            JournalIndexLoopItem::Type(_) => "type",
        };
        Tag::new(format!("journal_index_{suffix}"))
    }
}

impl LoopValueTerm for JournalIndexLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            JournalIndexLoopItem::Subterm(s) | JournalIndexLoopItem::Term(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
            JournalIndexLoopItem::Type(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}
