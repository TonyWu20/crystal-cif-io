mod journal;
mod paper_pages;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::journal::{Journal, JournalRow};
pub use self::paper_pages::PaperPages;

/// The single-value terms of the `journal` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum JournalTerms {
    /// The `_journal.data_validation_number` item. The variant holds the single value.
    Journal_data_validation_number(CharString),
    /// The `_journal_paper_number` item. The variant holds the single value.
    Paper_number(CharString),
    /// The `_journal_paper_pages` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Paper_pages(PaperPages),
    /// The `_journal_paper_url` item. The variant holds the single value.
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

/// The loop-value terms of the `journal` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum JournalLoopItem {
    /// The `_journal_coden_ASTM` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coden_ASTM(Vec<CharString>),
    /// The `_journal_coden_Cambridge` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coden_Cambridge(Vec<CharString>),
    /// The `_journal_coeditor_address` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coeditor_address(Vec<CharString>),
    /// The `_journal_coeditor_code` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coeditor_code(Vec<CharString>),
    /// The `_journal_coeditor_email` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coeditor_email(Vec<CharString>),
    /// The `_journal_coeditor_fax` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coeditor_fax(Vec<CharString>),
    /// The `_journal_coeditor_name` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coeditor_name(Vec<CharString>),
    /// The `_journal_coeditor_notes` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coeditor_notes(Vec<CharString>),
    /// The `_journal_coeditor_phone` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Coeditor_phone(Vec<CharString>),
    /// The `_journal_data_validation_number` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Data_validation_number(Vec<CharString>),
    /// The `_journal_date_accepted` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_accepted(Vec<CharString>),
    /// The `_journal_date_from_coeditor` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_from_coeditor(Vec<CharString>),
    /// The `_journal_date_printers_final` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_printers_final(Vec<CharString>),
    /// The `_journal_date_printers_first` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_printers_first(Vec<CharString>),
    /// The `_journal_date_proofs_in` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_proofs_in(Vec<CharString>),
    /// The `_journal_date_proofs_out` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_proofs_out(Vec<CharString>),
    /// The `_journal_date_recd_copyright` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_recd_copyright(Vec<CharString>),
    /// The `_journal_date_recd_electronic` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_recd_electronic(Vec<CharString>),
    /// The `_journal_date_recd_hard_copy` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_recd_hard_copy(Vec<CharString>),
    /// The `_journal_date_to_coeditor` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Date_to_coeditor(Vec<CharString>),
    /// The `_journal_issue` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Issue(Vec<CharString>),
    /// The `_journal_language` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Language(Vec<CharString>),
    /// The `_journal_name_full` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Name_full(Vec<CharString>),
    /// The `_journal_page_first` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Page_first(Vec<CharString>),
    /// The `_journal_page_last` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Page_last(Vec<CharString>),
    /// The `_journal_paper_category` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Paper_category(Vec<CharString>),
    /// The `_journal_paper_doi` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Paper_doi(Vec<CharString>),
    /// The `_journal_suppl_publ_number` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Suppl_publ_number(Vec<CharString>),
    /// The `_journal_suppl_publ_pages` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Suppl_publ_pages(Vec<CharString>),
    /// The `_journal_techeditor_address` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Techeditor_address(Vec<CharString>),
    /// The `_journal_techeditor_code` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Techeditor_code(Vec<CharString>),
    /// The `_journal_techeditor_email` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Techeditor_email(Vec<CharString>),
    /// The `_journal_techeditor_fax` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Techeditor_fax(Vec<CharString>),
    /// The `_journal_techeditor_name` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Techeditor_name(Vec<CharString>),
    /// The `_journal_techeditor_notes` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Techeditor_notes(Vec<CharString>),
    /// The `_journal_techeditor_phone` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Techeditor_phone(Vec<CharString>),
    /// The `_journal_volume` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
    Volume(Vec<CharString>),
    /// The `_journal_year` item. The variant holds the column values, one per loop row.
    ///
    /// Data items specified by the journal staff.
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
