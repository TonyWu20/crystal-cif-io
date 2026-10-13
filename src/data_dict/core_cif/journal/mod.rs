mod paper_pages;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::paper_pages::PaperPages;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum JournalTerms {
    Journal_data_validation_number(CharString),
    Paper_number(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Paper_pages(PaperPages),
    Paper_url(CharString),
}

impl CifTerm for JournalTerms {
    fn tag(&self) -> Tag {
        let tag = match self {
            JournalTerms::Journal_data_validation_number(_) => "journal.data_validation_number",
            JournalTerms::Paper_number(_) => "journal_paper_number",
            JournalTerms::Paper_pages(_) => "journal_paper_pages",
            JournalTerms::Paper_url(_) => "journal_paper_url",
        };
        Tag::new(tag.to_string())
    }
}

impl SingleValueTerm for JournalTerms {
    fn value(&self) -> Value {
        match self {
            JournalTerms::Journal_data_validation_number(s)
            | JournalTerms::Paper_number(s)
            | JournalTerms::Paper_url(s) => Value::CharString(s.clone()),
            JournalTerms::Paper_pages(v) => Value::from(*v),
        }
    }
}

impl Display for JournalTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum JournalLoopItem {
    Coden_ASTM(Vec<CharString>),
    Coden_Cambridge(Vec<CharString>),
    Coeditor_address(Vec<CharString>),
    Coeditor_code(Vec<CharString>),
    Coeditor_email(Vec<CharString>),
    Coeditor_fax(Vec<CharString>),
    Coeditor_name(Vec<CharString>),
    Coeditor_notes(Vec<CharString>),
    Coeditor_phone(Vec<CharString>),
    Data_validation_number(Vec<CharString>),
    Date_accepted(Vec<CharString>),
    Date_from_coeditor(Vec<CharString>),
    Date_printers_final(Vec<CharString>),
    Date_printers_first(Vec<CharString>),
    Date_proofs_in(Vec<CharString>),
    Date_proofs_out(Vec<CharString>),
    Date_recd_copyright(Vec<CharString>),
    Date_recd_electronic(Vec<CharString>),
    Date_recd_hard_copy(Vec<CharString>),
    Date_to_coeditor(Vec<CharString>),
    Issue(Vec<CharString>),
    Language(Vec<CharString>),
    Name_full(Vec<CharString>),
    Page_first(Vec<CharString>),
    Page_last(Vec<CharString>),
    Paper_category(Vec<CharString>),
    Paper_doi(Vec<CharString>),
    Suppl_publ_number(Vec<CharString>),
    Suppl_publ_pages(Vec<CharString>),
    Techeditor_address(Vec<CharString>),
    Techeditor_code(Vec<CharString>),
    Techeditor_email(Vec<CharString>),
    Techeditor_fax(Vec<CharString>),
    Techeditor_name(Vec<CharString>),
    Techeditor_notes(Vec<CharString>),
    Techeditor_phone(Vec<CharString>),
    Volume(Vec<CharString>),
    Year(Vec<CharString>),
}

impl CifTerm for JournalLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            JournalLoopItem::Coden_ASTM(_) => "coden_astm",
            JournalLoopItem::Coden_Cambridge(_) => "coden_cambridge",
            JournalLoopItem::Coeditor_address(_) => "coeditor_address",
            JournalLoopItem::Coeditor_code(_) => "coeditor_code",
            JournalLoopItem::Coeditor_email(_) => "coeditor_email",
            JournalLoopItem::Coeditor_fax(_) => "coeditor_fax",
            JournalLoopItem::Coeditor_name(_) => "coeditor_name",
            JournalLoopItem::Coeditor_notes(_) => "coeditor_notes",
            JournalLoopItem::Coeditor_phone(_) => "coeditor_phone",
            JournalLoopItem::Data_validation_number(_) => "data_validation_number",
            JournalLoopItem::Date_accepted(_) => "date_accepted",
            JournalLoopItem::Date_from_coeditor(_) => "date_from_coeditor",
            JournalLoopItem::Date_printers_final(_) => "date_printers_final",
            JournalLoopItem::Date_printers_first(_) => "date_printers_first",
            JournalLoopItem::Date_proofs_in(_) => "date_proofs_in",
            JournalLoopItem::Date_proofs_out(_) => "date_proofs_out",
            JournalLoopItem::Date_recd_copyright(_) => "date_recd_copyright",
            JournalLoopItem::Date_recd_electronic(_) => "date_recd_electronic",
            JournalLoopItem::Date_recd_hard_copy(_) => "date_recd_hard_copy",
            JournalLoopItem::Date_to_coeditor(_) => "date_to_coeditor",
            JournalLoopItem::Issue(_) => "issue",
            JournalLoopItem::Language(_) => "language",
            JournalLoopItem::Name_full(_) => "name_full",
            JournalLoopItem::Page_first(_) => "page_first",
            JournalLoopItem::Page_last(_) => "page_last",
            JournalLoopItem::Paper_category(_) => "paper_category",
            JournalLoopItem::Paper_doi(_) => "paper_doi",
            JournalLoopItem::Suppl_publ_number(_) => "suppl_publ_number",
            JournalLoopItem::Suppl_publ_pages(_) => "suppl_publ_pages",
            JournalLoopItem::Techeditor_address(_) => "techeditor_address",
            JournalLoopItem::Techeditor_code(_) => "techeditor_code",
            JournalLoopItem::Techeditor_email(_) => "techeditor_email",
            JournalLoopItem::Techeditor_fax(_) => "techeditor_fax",
            JournalLoopItem::Techeditor_name(_) => "techeditor_name",
            JournalLoopItem::Techeditor_notes(_) => "techeditor_notes",
            JournalLoopItem::Techeditor_phone(_) => "techeditor_phone",
            JournalLoopItem::Volume(_) => "volume",
            JournalLoopItem::Year(_) => "year",
        };
        Tag::new(format!("journal_{suffix}"))
    }
}

impl LoopValueTerm for JournalLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            JournalLoopItem::Coden_ASTM(s)
            | JournalLoopItem::Coden_Cambridge(s)
            | JournalLoopItem::Coeditor_address(s)
            | JournalLoopItem::Coeditor_code(s)
            | JournalLoopItem::Coeditor_email(s)
            | JournalLoopItem::Coeditor_fax(s)
            | JournalLoopItem::Coeditor_name(s)
            | JournalLoopItem::Coeditor_notes(s)
            | JournalLoopItem::Coeditor_phone(s)
            | JournalLoopItem::Data_validation_number(s)
            | JournalLoopItem::Date_accepted(s)
            | JournalLoopItem::Date_from_coeditor(s)
            | JournalLoopItem::Date_printers_final(s)
            | JournalLoopItem::Date_printers_first(s)
            | JournalLoopItem::Date_proofs_in(s)
            | JournalLoopItem::Date_proofs_out(s)
            | JournalLoopItem::Date_recd_copyright(s)
            | JournalLoopItem::Date_recd_electronic(s)
            | JournalLoopItem::Date_recd_hard_copy(s)
            | JournalLoopItem::Date_to_coeditor(s)
            | JournalLoopItem::Issue(s)
            | JournalLoopItem::Language(s)
            | JournalLoopItem::Name_full(s)
            | JournalLoopItem::Page_first(s)
            | JournalLoopItem::Page_last(s)
            | JournalLoopItem::Paper_category(s)
            | JournalLoopItem::Paper_doi(s)
            | JournalLoopItem::Suppl_publ_number(s)
            | JournalLoopItem::Suppl_publ_pages(s)
            | JournalLoopItem::Techeditor_address(s)
            | JournalLoopItem::Techeditor_code(s)
            | JournalLoopItem::Techeditor_email(s)
            | JournalLoopItem::Techeditor_fax(s)
            | JournalLoopItem::Techeditor_name(s)
            | JournalLoopItem::Techeditor_notes(s)
            | JournalLoopItem::Techeditor_phone(s)
            | JournalLoopItem::Volume(s)
            | JournalLoopItem::Year(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
