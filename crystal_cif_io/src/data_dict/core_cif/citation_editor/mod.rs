mod citation_editor;

pub use self::citation_editor::{CitationEditor, CitationEditorRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationEditorTerms {
    Citation_editor(CharString),
}

impl CifTerm for CitationEditorTerms {
    fn tag(&self) -> Tag {
        let tag = match self {
            CitationEditorTerms::Citation_editor(_) => "citation_editor",
        };
        Tag::new(tag.to_string())
    }
}

impl SingleValueTerm for CitationEditorTerms {
    fn value(&self) -> Value {
        match self {
            CitationEditorTerms::Citation_editor(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for CitationEditorTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

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
