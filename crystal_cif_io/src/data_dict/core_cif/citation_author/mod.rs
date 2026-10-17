mod citation_author;

pub use self::citation_author::CitationAuthor;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The loop-value terms of the `citation_author` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationAuthorLoopItem {
    /// The `_citation_author_citation_id` item. The variant holds the column values, one per loop row.
    ///
    /// The value of _citation_author_citation_id must match an identifier
    /// specified by _citation_id in the _citation_ list.
    Citation_id(Vec<CharString>),
    /// The `_citation_author_name` item. The variant holds the column values, one per loop row.
    ///
    /// Name of an author of the citation; relevant for journal articles,
    /// books and book chapters. The family name(s), followed by a comma and
    /// including any dynastic components, precedes the first name(s) or
    /// initial(s).
    Name(Vec<CharString>),
    /// The `_citation_author_ordinal` item. The variant holds the column values, one per loop row.
    ///
    /// This data name defines the order of the author's name in the list of
    /// authors of a citation.
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
