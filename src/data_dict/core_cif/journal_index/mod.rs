use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum JournalIndexLoopItem {
    Subterm(Vec<CharString>),
    Term(Vec<CharString>),
    Type(Vec<CharString>),
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
            JournalIndexLoopItem::Subterm(s)
            | JournalIndexLoopItem::Term(s)
            | JournalIndexLoopItem::Type(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
