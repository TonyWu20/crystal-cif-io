mod citation;
mod coordinate_linkage;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

pub use self::citation::{Citation, CitationRow};
pub use self::coordinate_linkage::CoordinateLinkage;

/// The single-value terms of the `citation` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationTerms {
    /// The `_citation_url` item. The variant holds the single value.
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

/// The loop-value terms of the `citation` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CitationLoopItem {
    /// The `_citation_abstract` item. The variant holds the column values, one per loop row.
    ///
    /// Abstract for the citation. This is used most when the citation is
    /// extracted from a bibliographic database that contains full text or
    /// abstract information.
    Abstract(Vec<CharString>),
    /// The `_citation_abstract_id_CAS` item. The variant holds the column values, one per loop row.
    ///
    /// The Chemical Abstracts Service (CAS) abstract identifier; relevant
    /// for journal articles.
    Abstract_id_CAS(Vec<CharString>),
    /// The `_citation_book_id_ISBN` item. The variant holds the column values, one per loop row.
    ///
    /// The International Standard Book Number (ISBN) code assigned to the
    /// book cited; relevant for books or book chapters.
    Book_id_ISBN(Vec<CharString>),
    /// The `_citation_book_publisher` item. The variant holds the column values, one per loop row.
    ///
    /// The name of the publisher of the citation; relevant for books or
    /// book chapters.
    Book_publisher(Vec<CharString>),
    /// The `_citation_book_publisher_city` item. The variant holds the column values, one per loop row.
    ///
    /// The location of the publisher of the citation; relevant for books or
    /// book chapters.
    Book_publisher_city(Vec<CharString>),
    /// The `_citation_book_title` item. The variant holds the column values, one per loop row.
    ///
    /// The title of the book in which the citation appeared; relevant for
    /// books or book chapters.
    Book_title(Vec<CharString>),
    /// The `_citation_coordinate_linkage` item. The variant holds the column values, one per loop row.
    ///
    /// _citation_coordinate_linkage states whether or not this citation is
    /// concerned with precisely the set of coordinates given in the data
    /// block. If, for instance, the publication described the same
    /// structure, but the coordinates had undergone further refinement
    /// prior to creation of the data block, the value of this data item
    /// would be 'no'.
    /// closed code set recorded in the dictionary _enumeration
    Coordinate_linkage(Vec<CoordinateLinkage>),
    /// The `_citation_country` item. The variant holds the column values, one per loop row.
    ///
    /// The country of publication; relevant for books and book chapters.
    Country(Vec<CharString>),
    /// The `_citation_database_id_CSD` item. The variant holds the column values, one per loop row.
    ///
    /// Identifier ('refcode') of the database record in the Cambridge
    /// Structural Database that contains details of the cited structure.
    Database_id_CSD(Vec<CharString>),
    /// The `_citation_database_id_Medline` item. The variant holds the column values, one per loop row.
    ///
    /// Accession number used by Medline to categorize a specific
    /// bibliographic entry.
    Database_id_Medline(Vec<CharString>),
    /// The `_citation_doi` item. The variant holds the column values, one per loop row.
    ///
    /// The Digital Object Identifier (DOI) of the cited work. A DOI is a
    /// unique character string identifying any object of intellectual
    /// property. It provides a persistent identifier for an object on a
    /// digital network and permits the association of related current data
    /// in a structured extensible way. A DOI is an implementation of the
    /// Internet concepts of Uniform Resource Name and Universal Resource
    /// Locator managed according to the specifications of the International
    /// DOI Foundation (see `http://www.doi.org`).
    Doi(Vec<CharString>),
    /// The `_citation_id` item. The variant holds the column values, one per loop row.
    ///
    /// The value of _citation_id must uniquely identify a record in the
    /// _citation_ list. The _citation_id 'primary' should be used to
    /// indicate the citation that the author(s) consider to be the most
    /// pertinent to the contents of the data block. Note that this item
    /// need not be a number; it can be any unique identifier.
    Id(Vec<CharString>),
    /// The `_citation_journal_abbrev` item. The variant holds the column values, one per loop row.
    ///
    /// Abbreviated name of the journal cited as given in the Chemical
    /// Abstracts Service Source Index.
    Journal_abbrev(Vec<CharString>),
    /// The `_citation_journal_full` item. The variant holds the column values, one per loop row.
    ///
    /// Full name of the journal cited; relevant for journal articles.
    Journal_full(Vec<CharString>),
    /// The `_citation_journal_id_ASTM` item. The variant holds the column values, one per loop row.
    ///
    /// The American Society for Testing and Materials (ASTM) code assigned
    /// to the journal cited (also referred to as the CODEN designator of
    /// the Chemical Abstracts Service); relevant for journal articles.
    Journal_id_ASTM(Vec<CharString>),
    /// The `_citation_journal_id_CSD` item. The variant holds the column values, one per loop row.
    ///
    /// The Cambridge Structural Database (CSD) code assigned to the journal
    /// cited; relevant for journal articles. This is also the system used
    /// at the Protein Data Bank (PDB).
    Journal_id_CSD(Vec<CharString>),
    /// The `_citation_journal_id_ISSN` item. The variant holds the column values, one per loop row.
    ///
    /// The International Standard Serial Number (ISSN) code assigned to the
    /// journal cited; relevant for journal articles.
    Journal_id_ISSN(Vec<CharString>),
    /// The `_citation_journal_issue` item. The variant holds the column values, one per loop row.
    ///
    /// Issue number of the journal cited; relevant for journal articles.
    Journal_issue(Vec<CharString>),
    /// The `_citation_journal_volume` item. The variant holds the column values, one per loop row.
    ///
    /// Volume number of the journal cited; relevant for journal articles.
    Journal_volume(Vec<CharString>),
    /// The `_citation_language` item. The variant holds the column values, one per loop row.
    ///
    /// Language in which the cited article is written.
    Language(Vec<CharString>),
    /// The `_citation_page_first` item. The variant holds the column values, one per loop row.
    ///
    /// The first and last pages of the citation; relevant for journal
    /// articles, books and book chapters.
    Page_first(Vec<CharString>),
    /// The `_citation_page_last` item. The variant holds the column values, one per loop row.
    ///
    /// The first and last pages of the citation; relevant for journal
    /// articles, books and book chapters.
    Page_last(Vec<CharString>),
    /// The `_citation_publisher` item. The variant holds the column values, one per loop row.
    ///
    /// The name of the publisher of the cited work. This should be used for
    /// citations of journal articles or datasets (in the latter case the
    /// publisher could be a curated database). For books or book chapters
    /// use _citation_book_publisher.
    Publisher(Vec<CharString>),
    /// The `_citation_special_details` item. The variant holds the column values, one per loop row.
    ///
    /// A description of special aspects of the relationship of the contents
    /// of the data block to the literature item cited.
    Special_details(Vec<CharString>),
    /// The `_citation_title` item. The variant holds the column values, one per loop row.
    ///
    /// The title of the citation; relevant for journal articles, books and
    /// book chapters.
    Title(Vec<CharString>),
    /// The `_citation_year` item. The variant holds the column values, one per loop row.
    ///
    /// The year of the citation; relevant for journal articles, books and
    /// book chapters.
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
