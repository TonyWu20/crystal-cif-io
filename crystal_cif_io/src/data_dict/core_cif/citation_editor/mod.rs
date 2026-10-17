mod citation_editor;

pub use self::citation_editor::{CitationEditor, CitationEditorRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `citation_editor` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationEditorTerms {
    /// The `_citation_editor` item. The variant holds the single value.
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

/// The loop-value terms of the `citation_editor` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationEditorLoopItem {
    /// The `_citation_editor_citation_id` item. The variant holds the column values, one per loop row.
    ///
    /// The value of _citation_editor_citation_id must match an identifier
    /// specified by _citation_id in the _citation_ list.
    Citation_id(Vec<CharString>),
    /// The `_citation_editor_name` item. The variant holds the column values, one per loop row.
    ///
    /// Name of an editor of the citation; relevant for books and book
    /// chapters. The family name(s), followed by a comma and including any
    /// dynastic components, precedes the first name(s) or initial(s).
    Name(Vec<CharString>),
    /// The `_citation_editor_ordinal` item. The variant holds the column values, one per loop row.
    ///
    /// This data name defines the order of the editor's name in the list of
    /// editors of a citation.
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
