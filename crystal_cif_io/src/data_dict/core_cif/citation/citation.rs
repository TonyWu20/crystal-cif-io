//! The `citation` category of the typed document layer (roadmap D12).
//!
//! The `Citation` struct owns the single-value items (one `Option` field
//! per registry item) and the loop rows (`rows`). One `CifCategory`
//! impl covers both. Char items read leniently: the tier corpus
//! records them as text fields. The closed sets keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::citation::CoordinateLinkage;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `citation` category struct. One field per registry item.
pub struct Citation {
    /// The `_citation_URL` item. The field holds the single value.
    pub url: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<CitationRow>,
}

impl CifCategory for Citation {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.url {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("citation_url".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        out.extend(rows_to_members(&self.rows));
        out
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::SingleValue(sv)) => {
                    let tag = sv.tag().as_ref().to_lowercase();
                    let value = sv.value();
                    match tag.as_str() {
                        "citation_url" => match convert::read_char_text("citation_url", value) {
                            Ok(v) => self.url = v,
                            Err(e) => errors.push(e),
                        },
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries one of
                    // the category own loop items. A sibling loop that
                    // carries none of them stays in the residue.
                    let ours: &[&str] = &[
                        "citation_abstract",
                        "citation_abstract_id_cas",
                        "citation_book_id_isbn",
                        "citation_book_publisher",
                        "citation_book_publisher_city",
                        "citation_book_title",
                        "citation_coordinate_linkage",
                        "citation_country",
                        "citation_database_id_csd",
                        "citation_database_id_medline",
                        "citation_doi",
                        "citation_id",
                        "citation_journal_abbrev",
                        "citation_journal_full",
                        "citation_journal_id_astm",
                        "citation_journal_id_csd",
                        "citation_journal_id_issn",
                        "citation_journal_issue",
                        "citation_journal_volume",
                        "citation_language",
                        "citation_page_first",
                        "citation_page_last",
                        "citation_publisher",
                        "citation_special_details",
                        "citation_title",
                        "citation_year",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        self.rows = rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

/// The `citation` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
/// One loop row of the `citation` category: the per-observation values of the loop items.
pub struct CitationRow {
    /// The `_citation_abstract` item. The field holds the column values, one per loop row.
    ///
    /// Abstract for the citation. This is used most when the citation is
    /// extracted from a bibliographic database that contains full text or
    /// abstract information.
    pub abstract_: Option<CharString>,
    /// The `_citation_abstract_id_CAS` item. The field holds the column values, one per loop row.
    ///
    /// The Chemical Abstracts Service (CAS) abstract identifier; relevant
    /// for journal articles.
    pub abstract_id_cas: Option<CharString>,
    /// The `_citation_book_id_ISBN` item. The field holds the column values, one per loop row.
    ///
    /// The International Standard Book Number (ISBN) code assigned to the
    /// book cited; relevant for books or book chapters.
    pub book_id_isbn: Option<CharString>,
    /// The `_citation_book_publisher` item. The field holds the column values, one per loop row.
    ///
    /// The name of the publisher of the citation; relevant for books or
    /// book chapters.
    pub book_publisher: Option<CharString>,
    /// The `_citation_book_publisher_city` item. The field holds the column values, one per loop row.
    ///
    /// The location of the publisher of the citation; relevant for books or
    /// book chapters.
    pub book_publisher_city: Option<CharString>,
    /// The `_citation_book_title` item. The field holds the column values, one per loop row.
    ///
    /// The title of the book in which the citation appeared; relevant for
    /// books or book chapters.
    pub book_title: Option<CharString>,
    /// The `_citation_coordinate_linkage` item. The field holds the column values, one per loop row.
    ///
    /// _citation_coordinate_linkage states whether or not this citation is
    /// concerned with precisely the set of coordinates given in the data
    /// block. If, for instance, the publication described the same
    /// structure, but the coordinates had undergone further refinement
    /// prior to creation of the data block, the value of this data item
    /// would be 'no'.
    pub coordinate_linkage: Option<CoordinateLinkage>,
    /// The `_citation_country` item. The field holds the column values, one per loop row.
    ///
    /// The country of publication; relevant for books and book chapters.
    pub country: Option<CharString>,
    /// The `_citation_database_id_CSD` item. The field holds the column values, one per loop row.
    ///
    /// Identifier ('refcode') of the database record in the Cambridge
    /// Structural Database that contains details of the cited structure.
    pub database_id_csd: Option<CharString>,
    /// The `_citation_database_id_Medline` item. The field holds the column values, one per loop row.
    ///
    /// Accession number used by Medline to categorize a specific
    /// bibliographic entry.
    pub database_id_medline: Option<CharString>,
    /// The `_citation_doi` item. The field holds the column values, one per loop row.
    ///
    /// The Digital Object Identifier (DOI) of the cited work. A DOI is a
    /// unique character string identifying any object of intellectual
    /// property. It provides a persistent identifier for an object on a
    /// digital network and permits the association of related current data
    /// in a structured extensible way. A DOI is an implementation of the
    /// Internet concepts of Uniform Resource Name and Universal Resource
    /// Locator managed according to the specifications of the International
    /// DOI Foundation (see `http://www.doi.org`).
    pub doi: Option<CharString>,
    /// The `_citation_id` item. The field holds the column values, one per loop row.
    ///
    /// The value of _citation_id must uniquely identify a record in the
    /// _citation_ list. The _citation_id 'primary' should be used to
    /// indicate the citation that the author(s) consider to be the most
    /// pertinent to the contents of the data block. Note that this item
    /// need not be a number; it can be any unique identifier.
    pub id: Option<CharString>,
    /// The `_citation_journal_abbrev` item. The field holds the column values, one per loop row.
    ///
    /// Abbreviated name of the journal cited as given in the Chemical
    /// Abstracts Service Source Index.
    pub journal_abbrev: Option<CharString>,
    /// The `_citation_journal_full` item. The field holds the column values, one per loop row.
    ///
    /// Full name of the journal cited; relevant for journal articles.
    pub journal_full: Option<CharString>,
    /// The `_citation_journal_id_ASTM` item. The field holds the column values, one per loop row.
    ///
    /// The American Society for Testing and Materials (ASTM) code assigned
    /// to the journal cited (also referred to as the CODEN designator of
    /// the Chemical Abstracts Service); relevant for journal articles.
    pub journal_id_astm: Option<CharString>,
    /// The `_citation_journal_id_CSD` item. The field holds the column values, one per loop row.
    ///
    /// The Cambridge Structural Database (CSD) code assigned to the journal
    /// cited; relevant for journal articles. This is also the system used
    /// at the Protein Data Bank (PDB).
    pub journal_id_csd: Option<CharString>,
    /// The `_citation_journal_id_ISSN` item. The field holds the column values, one per loop row.
    ///
    /// The International Standard Serial Number (ISSN) code assigned to the
    /// journal cited; relevant for journal articles.
    pub journal_id_issn: Option<CharString>,
    /// The `_citation_journal_issue` item. The field holds the column values, one per loop row.
    ///
    /// Issue number of the journal cited; relevant for journal articles.
    pub journal_issue: Option<CharString>,
    /// The `_citation_journal_volume` item. The field holds the column values, one per loop row.
    ///
    /// Volume number of the journal cited; relevant for journal articles.
    pub journal_volume: Option<CharString>,
    /// The `_citation_language` item. The field holds the column values, one per loop row.
    ///
    /// Language in which the cited article is written.
    pub language: Option<CharString>,
    /// The `_citation_page_first` item. The field holds the column values, one per loop row.
    ///
    /// The first and last pages of the citation; relevant for journal
    /// articles, books and book chapters.
    pub page_first: Option<CharString>,
    /// The `_citation_page_last` item. The field holds the column values, one per loop row.
    ///
    /// The first and last pages of the citation; relevant for journal
    /// articles, books and book chapters.
    pub page_last: Option<CharString>,
    /// The `_citation_publisher` item. The field holds the column values, one per loop row.
    ///
    /// The name of the publisher of the cited work. This should be used for
    /// citations of journal articles or datasets (in the latter case the
    /// publisher could be a curated database). For books or book chapters
    /// use _citation_book_publisher.
    pub publisher: Option<CharString>,
    /// The `_citation_special_details` item. The field holds the column values, one per loop row.
    ///
    /// A description of special aspects of the relationship of the contents
    /// of the data block to the literature item cited.
    pub special_details: Option<CharString>,
    /// The `_citation_title` item. The field holds the column values, one per loop row.
    ///
    /// The title of the citation; relevant for journal articles, books and
    /// book chapters.
    pub title: Option<CharString>,
    /// The `_citation_year` item. The field holds the column values, one per loop row.
    ///
    /// The year of the citation; relevant for journal articles, books and
    /// book chapters.
    pub year: Option<Numeric>,
}

fn rows_to_members(rows: &[CitationRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.abstract_.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.abstract_ {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_abstract".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.abstract_id_cas.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.abstract_id_cas {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_abstract_id_cas".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.book_id_isbn.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.book_id_isbn {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_book_id_isbn".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.book_publisher.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.book_publisher {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_book_publisher".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.book_publisher_city.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.book_publisher_city {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_book_publisher_city".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.book_title.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.book_title {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_book_title".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.coordinate_linkage.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.coordinate_linkage {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_coordinate_linkage".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.country.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.country {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_country".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.database_id_csd.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.database_id_csd {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_database_id_csd".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.database_id_medline.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.database_id_medline {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_database_id_medline".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.doi.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.doi {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_doi".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.id.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.id {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("citation_id".to_string()), values));
    }
    if rows.iter().any(|r| r.journal_abbrev.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_abbrev {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_journal_abbrev".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_full.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_full {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_journal_full".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_id_astm.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_id_astm {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_journal_id_astm".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_id_csd.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_id_csd {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_journal_id_csd".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_id_issn.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_id_issn {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_journal_id_issn".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_issue.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_issue {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_journal_issue".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_volume.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_volume {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_journal_volume".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.language.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.language {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_language".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.page_first.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.page_first {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_page_first".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.page_last.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.page_last {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_page_last".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.publisher.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.publisher {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_publisher".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.special_details.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.special_details {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_special_details".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.title.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.title {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_title".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.year.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.year {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("citation_year".to_string()),
            values,
        ));
    }
    if columns.is_empty() {
        return Vec::new();
    }
    vec![DataBlockMember::DataItems(DataItems::MultiValues(
        LoopColumns::new(columns),
    ))]
}

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<CitationRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![CitationRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_abstract")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_abstract", v) {
                Ok(val) => rows[i].abstract_ = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_abstract_id_cas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_abstract_id_cas", v) {
                Ok(val) => rows[i].abstract_id_cas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_book_id_isbn")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_book_id_isbn", v) {
                Ok(val) => rows[i].book_id_isbn = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_book_publisher")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_book_publisher", v) {
                Ok(val) => rows[i].book_publisher = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_book_publisher_city")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_book_publisher_city", v) {
                Ok(val) => rows[i].book_publisher_city = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_book_title")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_book_title", v) {
                Ok(val) => rows[i].book_title = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_coordinate_linkage")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "citation_coordinate_linkage",
                v,
                CoordinateLinkage::parse,
                "CoordinateLinkage",
            ) {
                Ok(val) => rows[i].coordinate_linkage = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_country")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_country", v) {
                Ok(val) => rows[i].country = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_database_id_csd")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_database_id_csd", v) {
                Ok(val) => rows[i].database_id_csd = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_database_id_medline")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_database_id_medline", v) {
                Ok(val) => rows[i].database_id_medline = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_doi")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_doi", v) {
                Ok(val) => rows[i].doi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_id", v) {
                Ok(val) => rows[i].id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_journal_abbrev")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_journal_abbrev", v) {
                Ok(val) => rows[i].journal_abbrev = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_journal_full")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_journal_full", v) {
                Ok(val) => rows[i].journal_full = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_journal_id_astm")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_journal_id_astm", v) {
                Ok(val) => rows[i].journal_id_astm = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_journal_id_csd")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_journal_id_csd", v) {
                Ok(val) => rows[i].journal_id_csd = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_journal_id_issn")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_journal_id_issn", v) {
                Ok(val) => rows[i].journal_id_issn = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_journal_issue")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_journal_issue", v) {
                Ok(val) => rows[i].journal_issue = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_journal_volume")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_journal_volume", v) {
                Ok(val) => rows[i].journal_volume = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_language")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_language", v) {
                Ok(val) => rows[i].language = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_page_first")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_page_first", v) {
                Ok(val) => rows[i].page_first = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_page_last")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_page_last", v) {
                Ok(val) => rows[i].page_last = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_publisher")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_publisher", v) {
                Ok(val) => rows[i].publisher = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_special_details")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_special_details", v) {
                Ok(val) => rows[i].special_details = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_title")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("citation_title", v) {
                Ok(val) => rows[i].title = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "citation_year")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("citation_year", v) {
                Ok(val) => rows[i].year = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
