use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationEditorLoopItem {
    Citation_id(Vec<CharString>),
    Name(Vec<CharString>),
    Ordinal(Vec<CharString>),
}

impl CifTerm for CitationEditorLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            CitationEditorLoopItem::Citation_id(_) => "citation_id",
            CitationEditorLoopItem::Name(_) => "name",
            CitationEditorLoopItem::Ordinal(_) => "ordinal",
        };
        Tag::new(format!("citation_editor_{suffix}"))
    }
}

impl LoopValueTerm for CitationEditorLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            CitationEditorLoopItem::Citation_id(s)
            | CitationEditorLoopItem::Name(s)
            | CitationEditorLoopItem::Ordinal(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
