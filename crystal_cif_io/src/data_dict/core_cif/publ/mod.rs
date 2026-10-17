mod publ;
mod requested_category;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::publ::{Publ, PublRow};
pub use self::requested_category::RequestedCategory;

/// The single-value terms of the `publ` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublTerms {
    /// The `_publ_contact_author` item. The variant holds the single value.
    ///
    /// The name and address of the author submitting the manuscript and
    /// data block. This is the person contacted by the journal editorial
    /// staff. It is preferable to use the separate data items
    /// _publ_contact_author_name and _publ_contact_author_address.
    Contact_author(CharString),
    /// The `_publ_contact_author_address` item. The variant holds the single value.
    ///
    /// The address of the author submitting the manuscript and data block.
    /// This is the person contacted by the journal editorial staff.
    Contact_author_address(CharString),
    /// The `_publ_contact_author_email` item. The variant holds the single value.
    ///
    /// E-mail address in a form recognizable to international networks. The
    /// format of e-mail addresses is given in Section 3.4, Address
    /// Specification, of Internet Message Format, RFC 2822, P. Resnick
    /// (Editor), Network Standards Group, April 2001.
    Contact_author_email(CharString),
    /// The `_publ_contact_author_fax` item. The variant holds the single value.
    ///
    /// Facsimile telephone number of the author submitting the manuscript
    /// and data block. The recommended style is the international dialing
    /// prefix, followed by the area code in parentheses, followed by the
    /// local number with no spaces. The earlier convention of including the
    /// international dialing prefix in parentheses is no longer
    /// recommended.
    Contact_author_fax(CharString),
    /// The `_publ_contact_author_id_iucr` item. The variant holds the single value.
    ///
    /// Identifier in the IUCr contact database of the author submitting the
    /// manuscript and data block. This identifier may be available from the
    /// World Directory of Crystallographers (`http://wdc.iucr.org`).
    Contact_author_id_iucr(CharString),
    /// The `_publ_contact_author_id_orcid` item. The variant holds the single value.
    ///
    /// Identifier in the ORCID Registry of the author submitting the
    /// manuscript and data block. ORCID is an open, non-profit,
    /// community-driven service to provide a registry of unique researcher
    /// identifiers (`http://orcid.org`).
    Contact_author_id_orcid(CharString),
    /// The `_publ_contact_author_name` item. The variant holds the single value.
    ///
    /// The name of the author submitting the manuscript and data block.
    /// This is the person contacted by the journal editorial staff.
    Contact_author_name(CharString),
    /// The `_publ_contact_author_phone` item. The variant holds the single value.
    ///
    /// Telephone number of the author submitting the manuscript and data
    /// block. The recommended style is the international dialing prefix,
    /// followed by the area code in parentheses, followed by the local
    /// number and any extension number prefixed by 'x', with no spaces. The
    /// earlier convention of including the international dialing prefix in
    /// parentheses is no longer recommended.
    Contact_author_phone(CharString),
    /// The `_publ_contact_letter` item. The variant holds the single value.
    ///
    /// A letter submitted to the journal editor by the contact author.
    Contact_letter(CharString),
    /// The `_publ_manuscript_creation` item. The variant holds the single value.
    ///
    /// A description of the word-processor package and computer used to
    /// create the word-processed manuscript stored as
    /// _publ_manuscript_processed.
    Manuscript_creation(CharString),
    /// The `_publ_manuscript_processed` item. The variant holds the single value.
    ///
    /// The full manuscript of a paper (excluding possibly the figures and
    /// the tables) output in ASCII characters from a word processor.
    /// Information about the generation of this data item must be specified
    /// in the data item _publ_manuscript_creation.
    Manuscript_processed(CharString),
    /// The `_publ_manuscript_text` item. The variant holds the single value.
    ///
    /// The full manuscript of a paper (excluding figures and possibly the
    /// tables) output as standard ASCII text.
    Manuscript_text(CharString),
    /// The `_publ_requested_category` item. The variant holds the single value.
    ///
    /// The category of paper submitted. For submission to Acta
    /// Crystallographica Section C or Acta Crystallographica Section E,
    /// ONLY those codes indicated for use with those journals should be
    /// used.
    /// closed code set recorded in the dictionary _enumeration
    Requested_category(RequestedCategory),
    /// The `_publ_requested_coeditor_name` item. The variant holds the single value.
    ///
    /// The name of the co-editor whom the authors would like to handle the
    /// submitted manuscript.
    Requested_coeditor_name(CharString),
    /// The `_publ_requested_journal` item. The variant holds the single value.
    ///
    /// The name of the journal to which the manuscript is being submitted.
    Requested_journal(CharString),
}

impl CifTerm for PublTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            PublTerms::Contact_author(_) => "contact_author",
            PublTerms::Contact_author_address(_) => "contact_author_address",
            PublTerms::Contact_author_email(_) => "contact_author_email",
            PublTerms::Contact_author_fax(_) => "contact_author_fax",
            PublTerms::Contact_author_id_iucr(_) => "contact_author_id_iucr",
            PublTerms::Contact_author_id_orcid(_) => "contact_author_id_orcid",
            PublTerms::Contact_author_name(_) => "contact_author_name",
            PublTerms::Contact_author_phone(_) => "contact_author_phone",
            PublTerms::Contact_letter(_) => "contact_letter",
            PublTerms::Manuscript_creation(_) => "manuscript_creation",
            PublTerms::Manuscript_processed(_) => "manuscript_processed",
            PublTerms::Manuscript_text(_) => "manuscript_text",
            PublTerms::Requested_category(_) => "requested_category",
            PublTerms::Requested_coeditor_name(_) => "requested_coeditor_name",
            PublTerms::Requested_journal(_) => "requested_journal",
        };
        Tag::new(format!("publ_{suffix}"))
    }
}

impl SingleValueTerm for PublTerms {
    fn value(&self) -> Value {
        match self {
            PublTerms::Contact_author(s)
            | PublTerms::Contact_author_address(s)
            | PublTerms::Contact_author_email(s)
            | PublTerms::Contact_author_fax(s)
            | PublTerms::Contact_author_id_iucr(s)
            | PublTerms::Contact_author_id_orcid(s)
            | PublTerms::Contact_author_name(s)
            | PublTerms::Contact_author_phone(s)
            | PublTerms::Contact_letter(s)
            | PublTerms::Manuscript_creation(s)
            | PublTerms::Manuscript_processed(s)
            | PublTerms::Manuscript_text(s)
            | PublTerms::Requested_coeditor_name(s)
            | PublTerms::Requested_journal(s) => Value::CharString(s.clone()),
            PublTerms::Requested_category(v) => Value::from(*v),
        }
    }
}

impl Display for PublTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `publ` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublLoopItem {
    /// The `_publ_section_abstract` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_abstract(Vec<CharString>),
    /// The `_publ_section_acknowledgements` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_acknowledgements(Vec<CharString>),
    /// The `_publ_section_comment` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_comment(Vec<CharString>),
    /// The `_publ_section_discussion` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_discussion(Vec<CharString>),
    /// The `_publ_section_experimental` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_experimental(Vec<CharString>),
    /// The `_publ_section_exptl_prep` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_exptl_prep(Vec<CharString>),
    /// The `_publ_section_exptl_refinement` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_exptl_refinement(Vec<CharString>),
    /// The `_publ_section_exptl_solution` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_exptl_solution(Vec<CharString>),
    /// The `_publ_section_figure_captions` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_figure_captions(Vec<CharString>),
    /// The `_publ_section_introduction` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_introduction(Vec<CharString>),
    /// The `_publ_section_keywords` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_keywords(Vec<CharString>),
    /// The `_publ_section_references` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_references(Vec<CharString>),
    /// The `_publ_section_related_literature` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_related_literature(Vec<CharString>),
    /// The `_publ_section_synopsis` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_synopsis(Vec<CharString>),
    /// The `_publ_section_table_legends` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_table_legends(Vec<CharString>),
    /// The `_publ_section_title` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_title(Vec<CharString>),
    /// The `_publ_section_title_footnote` item. The variant holds the column values, one per loop row.
    ///
    /// The sections of a manuscript if submitted in parts. As an
    /// alternative, see _publ_manuscript_text and
    /// _publ_manuscript_processed. The _publ_section_exptl_prep,
    /// _publ_section_exptl_refinement and _publ_section_exptl_solution
    /// items are preferred for separating the chemical preparation,
    /// refinement and structure solution aspects of the experimental
    /// description.
    Section_title_footnote(Vec<CharString>),
}

impl CifTerm for PublLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            PublLoopItem::Section_abstract(_) => "section_abstract",
            PublLoopItem::Section_acknowledgements(_) => "section_acknowledgements",
            PublLoopItem::Section_comment(_) => "section_comment",
            PublLoopItem::Section_discussion(_) => "section_discussion",
            PublLoopItem::Section_experimental(_) => "section_experimental",
            PublLoopItem::Section_exptl_prep(_) => "section_exptl_prep",
            PublLoopItem::Section_exptl_refinement(_) => "section_exptl_refinement",
            PublLoopItem::Section_exptl_solution(_) => "section_exptl_solution",
            PublLoopItem::Section_figure_captions(_) => "section_figure_captions",
            PublLoopItem::Section_introduction(_) => "section_introduction",
            PublLoopItem::Section_keywords(_) => "section_keywords",
            PublLoopItem::Section_references(_) => "section_references",
            PublLoopItem::Section_related_literature(_) => "section_related_literature",
            PublLoopItem::Section_synopsis(_) => "section_synopsis",
            PublLoopItem::Section_table_legends(_) => "section_table_legends",
            PublLoopItem::Section_title(_) => "section_title",
            PublLoopItem::Section_title_footnote(_) => "section_title_footnote",
        };
        Tag::new(format!("publ_{suffix}"))
    }
}

impl LoopValueTerm for PublLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            PublLoopItem::Section_abstract(s)
            | PublLoopItem::Section_acknowledgements(s)
            | PublLoopItem::Section_comment(s)
            | PublLoopItem::Section_discussion(s)
            | PublLoopItem::Section_experimental(s)
            | PublLoopItem::Section_exptl_prep(s)
            | PublLoopItem::Section_exptl_refinement(s)
            | PublLoopItem::Section_exptl_solution(s)
            | PublLoopItem::Section_figure_captions(s)
            | PublLoopItem::Section_introduction(s)
            | PublLoopItem::Section_keywords(s)
            | PublLoopItem::Section_references(s)
            | PublLoopItem::Section_related_literature(s)
            | PublLoopItem::Section_synopsis(s)
            | PublLoopItem::Section_table_legends(s)
            | PublLoopItem::Section_title(s)
            | PublLoopItem::Section_title_footnote(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
