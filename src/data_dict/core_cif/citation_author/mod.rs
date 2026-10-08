use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationAuthorLoopItem {
    Citation_id(Vec<CharString>),
    Name(Vec<CharString>),
    Ordinal(Vec<CharString>),
}

impl CifTerm for CitationAuthorLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            CitationAuthorLoopItem::Citation_id(_) => "citation_id",
            CitationAuthorLoopItem::Name(_) => "name",
            CitationAuthorLoopItem::Ordinal(_) => "ordinal",
        };
        Tag::new(format!("citation_author_{suffix}"))
    }
}

impl LoopValueTerm for CitationAuthorLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            CitationAuthorLoopItem::Citation_id(s)
            | CitationAuthorLoopItem::Name(s)
            | CitationAuthorLoopItem::Ordinal(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
