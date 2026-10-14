mod citation;
mod coordinate_linkage;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

pub use self::citation::{Citation, CitationRow};
pub use self::coordinate_linkage::CoordinateLinkage;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationTerms {
    Url(CharString),
}

impl CifTerm for CitationTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            CitationTerms::Url(_) => "url",
        };
        Tag::new(format!("citation_{suffix}"))
    }
}

impl SingleValueTerm for CitationTerms {
    fn value(&self) -> Value {
        match self {
            CitationTerms::Url(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for CitationTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationLoopItem {
    Abstract(Vec<CharString>),
    Abstract_id_CAS(Vec<CharString>),
    Book_id_ISBN(Vec<CharString>),
    Book_publisher(Vec<CharString>),
    Book_publisher_city(Vec<CharString>),
    Book_title(Vec<CharString>),
    /// closed code set recorded in the dictionary _enumeration
    Coordinate_linkage(Vec<CoordinateLinkage>),
    Country(Vec<CharString>),
    Database_id_CSD(Vec<CharString>),
    Database_id_Medline(Vec<CharString>),
    Doi(Vec<CharString>),
    Id(Vec<CharString>),
    Journal_abbrev(Vec<CharString>),
    Journal_full(Vec<CharString>),
    Journal_id_ASTM(Vec<CharString>),
    Journal_id_CSD(Vec<CharString>),
    Journal_id_ISSN(Vec<CharString>),
    Journal_issue(Vec<CharString>),
    Journal_volume(Vec<CharString>),
    Language(Vec<CharString>),
    Page_first(Vec<CharString>),
    Page_last(Vec<CharString>),
    Publisher(Vec<CharString>),
    Special_details(Vec<CharString>),
    Title(Vec<CharString>),
    Year(Vec<Numeric>),
}

impl CifTerm for CitationLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            CitationLoopItem::Abstract(_) => "abstract",
            CitationLoopItem::Abstract_id_CAS(_) => "abstract_id_cas",
            CitationLoopItem::Book_id_ISBN(_) => "book_id_isbn",
            CitationLoopItem::Book_publisher(_) => "book_publisher",
            CitationLoopItem::Book_publisher_city(_) => "book_publisher_city",
            CitationLoopItem::Book_title(_) => "book_title",
            CitationLoopItem::Coordinate_linkage(_) => "coordinate_linkage",
            CitationLoopItem::Country(_) => "country",
            CitationLoopItem::Database_id_CSD(_) => "database_id_csd",
            CitationLoopItem::Database_id_Medline(_) => "database_id_medline",
            CitationLoopItem::Doi(_) => "doi",
            CitationLoopItem::Id(_) => "id",
            CitationLoopItem::Journal_abbrev(_) => "journal_abbrev",
            CitationLoopItem::Journal_full(_) => "journal_full",
            CitationLoopItem::Journal_id_ASTM(_) => "journal_id_astm",
            CitationLoopItem::Journal_id_CSD(_) => "journal_id_csd",
            CitationLoopItem::Journal_id_ISSN(_) => "journal_id_issn",
            CitationLoopItem::Journal_issue(_) => "journal_issue",
            CitationLoopItem::Journal_volume(_) => "journal_volume",
            CitationLoopItem::Language(_) => "language",
            CitationLoopItem::Page_first(_) => "page_first",
            CitationLoopItem::Page_last(_) => "page_last",
            CitationLoopItem::Publisher(_) => "publisher",
            CitationLoopItem::Special_details(_) => "special_details",
            CitationLoopItem::Title(_) => "title",
            CitationLoopItem::Year(_) => "year",
        };
        Tag::new(format!("citation_{suffix}"))
    }
}

impl LoopValueTerm for CitationLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            CitationLoopItem::Abstract(s)
            | CitationLoopItem::Abstract_id_CAS(s)
            | CitationLoopItem::Book_id_ISBN(s)
            | CitationLoopItem::Book_publisher(s)
            | CitationLoopItem::Book_publisher_city(s)
            | CitationLoopItem::Book_title(s)
            | CitationLoopItem::Country(s)
            | CitationLoopItem::Database_id_CSD(s)
            | CitationLoopItem::Database_id_Medline(s)
            | CitationLoopItem::Doi(s)
            | CitationLoopItem::Id(s)
            | CitationLoopItem::Journal_abbrev(s)
            | CitationLoopItem::Journal_full(s)
            | CitationLoopItem::Journal_id_ASTM(s)
            | CitationLoopItem::Journal_id_CSD(s)
            | CitationLoopItem::Journal_id_ISSN(s)
            | CitationLoopItem::Journal_issue(s)
            | CitationLoopItem::Journal_volume(s)
            | CitationLoopItem::Language(s)
            | CitationLoopItem::Page_first(s)
            | CitationLoopItem::Page_last(s)
            | CitationLoopItem::Publisher(s)
            | CitationLoopItem::Special_details(s)
            | CitationLoopItem::Title(s) => s.iter().cloned().map(Value::from).collect(),
            CitationLoopItem::Coordinate_linkage(v) => v.iter().cloned().map(Value::from).collect(),
            CitationLoopItem::Year(n) => n.iter().cloned().map(Value::from).collect(),
        }
    }
}
