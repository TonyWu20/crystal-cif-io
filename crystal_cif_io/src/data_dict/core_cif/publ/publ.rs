//! The `publ` category of the typed document layer (roadmap D12).
//!
//! The `Publ` struct owns the single-value items (one `Option` field
//! per registry item) and the loop rows (`rows`). One `CifCategory`
//! impl covers both. Char items read leniently: the tier corpus
//! records them as text fields. The closed sets keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::publ::RequestedCategory;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `publ` category struct. One field per registry item.
pub struct Publ {
    /// The `_publ_contact_author` item. The field holds the single value.
    ///
    /// The name and address of the author submitting the manuscript and
    /// data block. This is the person contacted by the journal editorial
    /// staff. It is preferable to use the separate data items
    /// _publ_contact_author_name and _publ_contact_author_address.
    pub contact_author: Option<CharString>,
    /// The `_publ_contact_author_address` item. The field holds the single value.
    ///
    /// The address of the author submitting the manuscript and data block.
    /// This is the person contacted by the journal editorial staff.
    pub contact_author_address: Option<CharString>,
    /// The `_publ_contact_author_email` item. The field holds the single value.
    ///
    /// E-mail address in a form recognizable to international networks. The
    /// format of e-mail addresses is given in Section 3.4, Address
    /// Specification, of Internet Message Format, RFC 2822, P. Resnick
    /// (Editor), Network Standards Group, April 2001.
    pub contact_author_email: Option<CharString>,
    /// The `_publ_contact_author_fax` item. The field holds the single value.
    ///
    /// Facsimile telephone number of the author submitting the manuscript
    /// and data block. The recommended style is the international dialing
    /// prefix, followed by the area code in parentheses, followed by the
    /// local number with no spaces. The earlier convention of including the
    /// international dialing prefix in parentheses is no longer
    /// recommended.
    pub contact_author_fax: Option<CharString>,
    /// The `_publ_contact_author_id_iucr` item. The field holds the single value.
    ///
    /// Identifier in the IUCr contact database of the author submitting the
    /// manuscript and data block. This identifier may be available from the
    /// World Directory of Crystallographers (`http://wdc.iucr.org`).
    pub contact_author_id_iucr: Option<CharString>,
    /// The `_publ_contact_author_id_orcid` item. The field holds the single value.
    ///
    /// Identifier in the ORCID Registry of the author submitting the
    /// manuscript and data block. ORCID is an open, non-profit,
    /// community-driven service to provide a registry of unique researcher
    /// identifiers (`http://orcid.org`).
    pub contact_author_id_orcid: Option<CharString>,
    /// The `_publ_contact_author_name` item. The field holds the single value.
    ///
    /// The name of the author submitting the manuscript and data block.
    /// This is the person contacted by the journal editorial staff.
    pub contact_author_name: Option<CharString>,
    /// The `_publ_contact_author_phone` item. The field holds the single value.
    ///
    /// Telephone number of the author submitting the manuscript and data
    /// block. The recommended style is the international dialing prefix,
    /// followed by the area code in parentheses, followed by the local
    /// number and any extension number prefixed by 'x', with no spaces. The
    /// earlier convention of including the international dialing prefix in
    /// parentheses is no longer recommended.
    pub contact_author_phone: Option<CharString>,
    /// The `_publ_contact_letter` item. The field holds the single value.
    ///
    /// A letter submitted to the journal editor by the contact author.
    pub contact_letter: Option<CharString>,
    /// The `_publ_manuscript_creation` item. The field holds the single value.
    ///
    /// A description of the word-processor package and computer used to
    /// create the word-processed manuscript stored as
    /// _publ_manuscript_processed.
    pub manuscript_creation: Option<CharString>,
    /// The `_publ_manuscript_processed` item. The field holds the single value.
    ///
    /// The full manuscript of a paper (excluding possibly the figures and
    /// the tables) output in ASCII characters from a word processor.
    /// Information about the generation of this data item must be specified
    /// in the data item _publ_manuscript_creation.
    pub manuscript_processed: Option<CharString>,
    /// The `_publ_manuscript_text` item. The field holds the single value.
    ///
    /// The full manuscript of a paper (excluding figures and possibly the
    /// tables) output as standard ASCII text.
    pub manuscript_text: Option<CharString>,
    /// The `_publ_requested_category` item. The field holds the single value.
    ///
    /// The category of paper submitted. For submission to Acta
    /// Crystallographica Section C or Acta Crystallographica Section E,
    /// ONLY those codes indicated for use with those journals should be
    /// used.
    pub requested_category: Option<RequestedCategory>,
    /// The `_publ_requested_coeditor_name` item. The field holds the single value.
    ///
    /// The name of the co-editor whom the authors would like to handle the
    /// submitted manuscript.
    pub requested_coeditor_name: Option<CharString>,
    /// The `_publ_requested_journal` item. The field holds the single value.
    ///
    /// The name of the journal to which the manuscript is being submitted.
    pub requested_journal: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<PublRow>,
}

impl CifCategory for Publ {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.contact_author {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_author_address {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author_address".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_author_email {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author_email".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_author_fax {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author_fax".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_author_id_iucr {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author_id_iucr".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_author_id_orcid {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author_id_orcid".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_author_name {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author_name".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_author_phone {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_author_phone".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.contact_letter {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_contact_letter".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.manuscript_creation {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_manuscript_creation".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.manuscript_processed {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_manuscript_processed".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.manuscript_text {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_manuscript_text".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.requested_category {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_requested_category".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.requested_coeditor_name {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_requested_coeditor_name".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.requested_journal {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("publ_requested_journal".to_string()),
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
                        "publ_contact_author" => {
                            match convert::read_char_text("publ_contact_author", value) {
                                Ok(v) => self.contact_author = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_author_address" => {
                            match convert::read_char_text("publ_contact_author_address", value) {
                                Ok(v) => self.contact_author_address = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_author_email" => {
                            match convert::read_char_text("publ_contact_author_email", value) {
                                Ok(v) => self.contact_author_email = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_author_fax" => {
                            match convert::read_char_text("publ_contact_author_fax", value) {
                                Ok(v) => self.contact_author_fax = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_author_id_iucr" => {
                            match convert::read_char_text("publ_contact_author_id_iucr", value) {
                                Ok(v) => self.contact_author_id_iucr = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_author_id_orcid" => {
                            match convert::read_char_text("publ_contact_author_id_orcid", value) {
                                Ok(v) => self.contact_author_id_orcid = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_author_name" => {
                            match convert::read_char_text("publ_contact_author_name", value) {
                                Ok(v) => self.contact_author_name = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_author_phone" => {
                            match convert::read_char_text("publ_contact_author_phone", value) {
                                Ok(v) => self.contact_author_phone = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_contact_letter" => {
                            match convert::read_char_text("publ_contact_letter", value) {
                                Ok(v) => self.contact_letter = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_manuscript_creation" => {
                            match convert::read_char_text("publ_manuscript_creation", value) {
                                Ok(v) => self.manuscript_creation = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_manuscript_processed" => {
                            match convert::read_char_text("publ_manuscript_processed", value) {
                                Ok(v) => self.manuscript_processed = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_manuscript_text" => {
                            match convert::read_char_text("publ_manuscript_text", value) {
                                Ok(v) => self.manuscript_text = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_requested_category" => {
                            match convert::read_closed_set(
                                "publ_requested_category",
                                value,
                                RequestedCategory::parse,
                                "RequestedCategory",
                            ) {
                                Ok(v) => self.requested_category = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_requested_coeditor_name" => {
                            match convert::read_char_text("publ_requested_coeditor_name", value) {
                                Ok(v) => self.requested_coeditor_name = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "publ_requested_journal" => {
                            match convert::read_char_text("publ_requested_journal", value) {
                                Ok(v) => self.requested_journal = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries one of
                    // the category own loop items. A sibling loop that
                    // carries none of them stays in the residue.
                    let ours: &[&str] = &[
                        "publ_section_abstract",
                        "publ_section_acknowledgements",
                        "publ_section_comment",
                        "publ_section_discussion",
                        "publ_section_experimental",
                        "publ_section_exptl_prep",
                        "publ_section_exptl_refinement",
                        "publ_section_exptl_solution",
                        "publ_section_figure_captions",
                        "publ_section_introduction",
                        "publ_section_keywords",
                        "publ_section_references",
                        "publ_section_related_literature",
                        "publ_section_synopsis",
                        "publ_section_table_legends",
                        "publ_section_title",
                        "publ_section_title_footnote",
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

/// The `publ` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
/// One loop row of the `publ` category: the per-observation values of the loop items.
pub struct PublRow {
    /// The `_publ_section_abstract` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_abstract: Option<CharString>,
    /// The `_publ_section_acknowledgements` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_acknowledgements: Option<CharString>,
    /// The `_publ_section_comment` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_comment: Option<CharString>,
    /// The `_publ_section_discussion` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_discussion: Option<CharString>,
    /// The `_publ_section_experimental` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_experimental: Option<CharString>,
    /// The `_publ_section_exptl_prep` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_exptl_prep: Option<CharString>,
    /// The `_publ_section_exptl_refinement` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_exptl_refinement: Option<CharString>,
    /// The `_publ_section_exptl_solution` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_exptl_solution: Option<CharString>,
    /// The `_publ_section_figure_captions` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_figure_captions: Option<CharString>,
    /// The `_publ_section_introduction` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_introduction: Option<CharString>,
    /// The `_publ_section_keywords` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_keywords: Option<CharString>,
    /// The `_publ_section_references` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_references: Option<CharString>,
    /// The `_publ_section_related_literature` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_related_literature: Option<CharString>,
    /// The `_publ_section_synopsis` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_synopsis: Option<CharString>,
    /// The `_publ_section_table_legends` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_table_legends: Option<CharString>,
    /// The `_publ_section_title` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_title: Option<CharString>,
    /// The `_publ_section_title_footnote` item. The field holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    pub section_title_footnote: Option<CharString>,
}

fn rows_to_members(rows: &[PublRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.section_abstract.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_abstract {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_abstract".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_acknowledgements.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_acknowledgements {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_acknowledgements".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_comment.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_comment {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_comment".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_discussion.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_discussion {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_discussion".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_experimental.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_experimental {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_experimental".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_exptl_prep.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_exptl_prep {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_exptl_prep".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_exptl_refinement.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_exptl_refinement {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_exptl_refinement".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_exptl_solution.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_exptl_solution {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_exptl_solution".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_figure_captions.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_figure_captions {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_figure_captions".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_introduction.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_introduction {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_introduction".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_keywords.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_keywords {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_keywords".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_references.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_references {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_references".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_related_literature.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_related_literature {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_related_literature".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_synopsis.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_synopsis {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_synopsis".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_table_legends.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_table_legends {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_table_legends".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_title.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_title {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_title".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.section_title_footnote.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.section_title_footnote {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("publ_section_title_footnote".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<PublRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![PublRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_abstract")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_abstract", v) {
                Ok(val) => rows[i].section_abstract = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_acknowledgements")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_acknowledgements", v) {
                Ok(val) => rows[i].section_acknowledgements = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_comment")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_comment", v) {
                Ok(val) => rows[i].section_comment = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_discussion")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_discussion", v) {
                Ok(val) => rows[i].section_discussion = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_experimental")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_experimental", v) {
                Ok(val) => rows[i].section_experimental = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_exptl_prep")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_exptl_prep", v) {
                Ok(val) => rows[i].section_exptl_prep = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_exptl_refinement")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_exptl_refinement", v) {
                Ok(val) => rows[i].section_exptl_refinement = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_exptl_solution")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_exptl_solution", v) {
                Ok(val) => rows[i].section_exptl_solution = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_figure_captions")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_figure_captions", v) {
                Ok(val) => rows[i].section_figure_captions = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_introduction")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_introduction", v) {
                Ok(val) => rows[i].section_introduction = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_keywords")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_keywords", v) {
                Ok(val) => rows[i].section_keywords = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_references")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_references", v) {
                Ok(val) => rows[i].section_references = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_related_literature")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_related_literature", v) {
                Ok(val) => rows[i].section_related_literature = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_synopsis")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_synopsis", v) {
                Ok(val) => rows[i].section_synopsis = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_table_legends")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_table_legends", v) {
                Ok(val) => rows[i].section_table_legends = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_title")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_title", v) {
                Ok(val) => rows[i].section_title = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "publ_section_title_footnote")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("publ_section_title_footnote", v) {
                Ok(val) => rows[i].section_title_footnote = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
