mod publ;
mod requested_category;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::publ::{Publ, PublRow};
pub use self::requested_category::RequestedCategory;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublTerms {
    Contact_author(CharString),
    Contact_author_address(CharString),
    Contact_author_email(CharString),
    Contact_author_fax(CharString),
    Contact_author_id_iucr(CharString),
    Contact_author_id_orcid(CharString),
    Contact_author_name(CharString),
    Contact_author_phone(CharString),
    Contact_letter(CharString),
    Manuscript_creation(CharString),
    Manuscript_processed(CharString),
    Manuscript_text(CharString),
    /// closed code set recorded in the dictionary _enumeration
    Requested_category(RequestedCategory),
    Requested_coeditor_name(CharString),
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

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublLoopItem {
    Section_abstract(Vec<CharString>),
    Section_acknowledgements(Vec<CharString>),
    Section_comment(Vec<CharString>),
    Section_discussion(Vec<CharString>),
    Section_experimental(Vec<CharString>),
    Section_exptl_prep(Vec<CharString>),
    Section_exptl_refinement(Vec<CharString>),
    Section_exptl_solution(Vec<CharString>),
    Section_figure_captions(Vec<CharString>),
    Section_introduction(Vec<CharString>),
    Section_keywords(Vec<CharString>),
    Section_references(Vec<CharString>),
    Section_related_literature(Vec<CharString>),
    Section_synopsis(Vec<CharString>),
    Section_table_legends(Vec<CharString>),
    Section_title(Vec<CharString>),
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
