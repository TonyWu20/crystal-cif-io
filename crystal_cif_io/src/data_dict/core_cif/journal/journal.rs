//! The `journal` category of the typed document layer (roadmap D12).
//!
//! The `journal` single-value struct and the loop-row struct. One `Option`
//! field per registry item. `CifCategory` is on the `Journal` single
//! group and on the `Vec<JournalRow>` loop group. Char items read leniently:
//! the tier corpus records them as text fields. The closed sets keep
//! the typed enum.

use crystal_cif_fmt::grammar::{Tag, Value, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::journal::PaperPages;
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct Journal {
    pub journal_data_validation_number: Option<CharString>,
    pub paper_number: Option<CharString>,
    pub paper_pages: Option<PaperPages>,
    pub paper_url: Option<CharString>,
}

impl CifCategory for Journal {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.journal_data_validation_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("journal.data_validation_number".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.paper_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("journal_paper_number".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.paper_pages {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("journal_paper_pages".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.paper_url {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("journal_paper_url".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        out
    }

    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "journal.data_validation_number" => {
                        match convert::read_char_text("journal.data_validation_number", value) {
                            Ok(v) => self.journal_data_validation_number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "journal_paper_number" => {
                        match convert::read_char_text("journal_paper_number", value) {
                            Ok(v) => self.paper_number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "journal_paper_pages" => {
                        match convert::read_paper_pages("journal_paper_pages", value) {
                            Ok(v) => self.paper_pages = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "journal_paper_url" => {
                        match convert::read_char_text("journal_paper_url", value) {
                            Ok(v) => self.paper_url = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    _ => residue.push(m.clone()),
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}

/// The `journal` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct JournalRow {
    pub coden_astm: Option<CharString>,
    pub coden_cambridge: Option<CharString>,
    pub coeditor_address: Option<CharString>,
    pub coeditor_code: Option<CharString>,
    pub coeditor_email: Option<CharString>,
    pub coeditor_fax: Option<CharString>,
    pub coeditor_name: Option<CharString>,
    pub coeditor_notes: Option<CharString>,
    pub coeditor_phone: Option<CharString>,
    pub data_validation_number: Option<CharString>,
    pub date_accepted: Option<CharString>,
    pub date_from_coeditor: Option<CharString>,
    pub date_printers_final: Option<CharString>,
    pub date_printers_first: Option<CharString>,
    pub date_proofs_in: Option<CharString>,
    pub date_proofs_out: Option<CharString>,
    pub date_recd_copyright: Option<CharString>,
    pub date_recd_electronic: Option<CharString>,
    pub date_recd_hard_copy: Option<CharString>,
    pub date_to_coeditor: Option<CharString>,
    pub issue: Option<CharString>,
    pub language: Option<CharString>,
    pub name_full: Option<CharString>,
    pub page_first: Option<CharString>,
    pub page_last: Option<CharString>,
    pub paper_category: Option<CharString>,
    pub paper_doi: Option<CharString>,
    pub suppl_publ_number: Option<CharString>,
    pub suppl_publ_pages: Option<CharString>,
    pub techeditor_address: Option<CharString>,
    pub techeditor_code: Option<CharString>,
    pub techeditor_email: Option<CharString>,
    pub techeditor_fax: Option<CharString>,
    pub techeditor_name: Option<CharString>,
    pub techeditor_notes: Option<CharString>,
    pub techeditor_phone: Option<CharString>,
    pub volume: Option<CharString>,
    pub year: Option<CharString>,
}

impl CifCategory for Vec<JournalRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.coden_astm.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coden_astm {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coden_astm".to_string()), values));
        }
        if self.iter().any(|r| r.coden_cambridge.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coden_cambridge {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coden_cambridge".to_string()), values));
        }
        if self.iter().any(|r| r.coeditor_address.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coeditor_address {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coeditor_address".to_string()), values));
        }
        if self.iter().any(|r| r.coeditor_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coeditor_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coeditor_code".to_string()), values));
        }
        if self.iter().any(|r| r.coeditor_email.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coeditor_email {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coeditor_email".to_string()), values));
        }
        if self.iter().any(|r| r.coeditor_fax.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coeditor_fax {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coeditor_fax".to_string()), values));
        }
        if self.iter().any(|r| r.coeditor_name.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coeditor_name {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coeditor_name".to_string()), values));
        }
        if self.iter().any(|r| r.coeditor_notes.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coeditor_notes {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coeditor_notes".to_string()), values));
        }
        if self.iter().any(|r| r.coeditor_phone.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.coeditor_phone {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_coeditor_phone".to_string()), values));
        }
        if self.iter().any(|r| r.data_validation_number.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.data_validation_number {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_data_validation_number".to_string()), values));
        }
        if self.iter().any(|r| r.date_accepted.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_accepted {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_accepted".to_string()), values));
        }
        if self.iter().any(|r| r.date_from_coeditor.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_from_coeditor {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_from_coeditor".to_string()), values));
        }
        if self.iter().any(|r| r.date_printers_final.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_printers_final {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_printers_final".to_string()), values));
        }
        if self.iter().any(|r| r.date_printers_first.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_printers_first {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_printers_first".to_string()), values));
        }
        if self.iter().any(|r| r.date_proofs_in.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_proofs_in {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_proofs_in".to_string()), values));
        }
        if self.iter().any(|r| r.date_proofs_out.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_proofs_out {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_proofs_out".to_string()), values));
        }
        if self.iter().any(|r| r.date_recd_copyright.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_recd_copyright {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_recd_copyright".to_string()), values));
        }
        if self.iter().any(|r| r.date_recd_electronic.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_recd_electronic {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_recd_electronic".to_string()), values));
        }
        if self.iter().any(|r| r.date_recd_hard_copy.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_recd_hard_copy {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_recd_hard_copy".to_string()), values));
        }
        if self.iter().any(|r| r.date_to_coeditor.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.date_to_coeditor {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_date_to_coeditor".to_string()), values));
        }
        if self.iter().any(|r| r.issue.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.issue {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_issue".to_string()), values));
        }
        if self.iter().any(|r| r.language.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.language {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_language".to_string()), values));
        }
        if self.iter().any(|r| r.name_full.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.name_full {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_name_full".to_string()), values));
        }
        if self.iter().any(|r| r.page_first.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.page_first {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_page_first".to_string()), values));
        }
        if self.iter().any(|r| r.page_last.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.page_last {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_page_last".to_string()), values));
        }
        if self.iter().any(|r| r.paper_category.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.paper_category {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_paper_category".to_string()), values));
        }
        if self.iter().any(|r| r.paper_doi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.paper_doi {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_paper_doi".to_string()), values));
        }
        if self.iter().any(|r| r.suppl_publ_number.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.suppl_publ_number {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_suppl_publ_number".to_string()), values));
        }
        if self.iter().any(|r| r.suppl_publ_pages.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.suppl_publ_pages {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_suppl_publ_pages".to_string()), values));
        }
        if self.iter().any(|r| r.techeditor_address.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.techeditor_address {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_techeditor_address".to_string()), values));
        }
        if self.iter().any(|r| r.techeditor_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.techeditor_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_techeditor_code".to_string()), values));
        }
        if self.iter().any(|r| r.techeditor_email.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.techeditor_email {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_techeditor_email".to_string()), values));
        }
        if self.iter().any(|r| r.techeditor_fax.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.techeditor_fax {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_techeditor_fax".to_string()), values));
        }
        if self.iter().any(|r| r.techeditor_name.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.techeditor_name {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_techeditor_name".to_string()), values));
        }
        if self.iter().any(|r| r.techeditor_notes.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.techeditor_notes {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_techeditor_notes".to_string()), values));
        }
        if self.iter().any(|r| r.techeditor_phone.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.techeditor_phone {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_techeditor_phone".to_string()), values));
        }
        if self.iter().any(|r| r.volume.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.volume {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_volume".to_string()), values));
        }
        if self.iter().any(|r| r.year.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.year {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("journal_year".to_string()), values));
        }
        if columns.is_empty() {
            return Vec::new();
        }
        vec![DataBlockMember::DataItems(DataItems::MultiValues(LoopColumns::new(columns)))]
    }

    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries one of
                    // the category own loop items. A sibling loop that
                    // carries none of them stays in the residue.
                    let ours: &[&str] = &[
                        "journal_coden_astm",
                        "journal_coden_cambridge",
                        "journal_coeditor_address",
                        "journal_coeditor_code",
                        "journal_coeditor_email",
                        "journal_coeditor_fax",
                        "journal_coeditor_name",
                        "journal_coeditor_notes",
                        "journal_coeditor_phone",
                        "journal_data_validation_number",
                        "journal_date_accepted",
                        "journal_date_from_coeditor",
                        "journal_date_printers_final",
                        "journal_date_printers_first",
                        "journal_date_proofs_in",
                        "journal_date_proofs_out",
                        "journal_date_recd_copyright",
                        "journal_date_recd_electronic",
                        "journal_date_recd_hard_copy",
                        "journal_date_to_coeditor",
                        "journal_issue",
                        "journal_language",
                        "journal_name_full",
                        "journal_page_first",
                        "journal_page_last",
                        "journal_paper_category",
                        "journal_paper_doi",
                        "journal_suppl_publ_number",
                        "journal_suppl_publ_pages",
                        "journal_techeditor_address",
                        "journal_techeditor_code",
                        "journal_techeditor_email",
                        "journal_techeditor_fax",
                        "journal_techeditor_name",
                        "journal_techeditor_notes",
                        "journal_techeditor_phone",
                        "journal_volume",
                        "journal_year",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<JournalRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![JournalRow::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coden_astm") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coden_astm", v) {
                Ok(val) => rows[i].coden_astm = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coden_cambridge") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coden_cambridge", v) {
                Ok(val) => rows[i].coden_cambridge = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coeditor_address") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coeditor_address", v) {
                Ok(val) => rows[i].coeditor_address = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coeditor_code") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coeditor_code", v) {
                Ok(val) => rows[i].coeditor_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coeditor_email") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coeditor_email", v) {
                Ok(val) => rows[i].coeditor_email = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coeditor_fax") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coeditor_fax", v) {
                Ok(val) => rows[i].coeditor_fax = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coeditor_name") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coeditor_name", v) {
                Ok(val) => rows[i].coeditor_name = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coeditor_notes") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coeditor_notes", v) {
                Ok(val) => rows[i].coeditor_notes = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_coeditor_phone") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_coeditor_phone", v) {
                Ok(val) => rows[i].coeditor_phone = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_data_validation_number") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_data_validation_number", v) {
                Ok(val) => rows[i].data_validation_number = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_accepted") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_accepted", v) {
                Ok(val) => rows[i].date_accepted = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_from_coeditor") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_from_coeditor", v) {
                Ok(val) => rows[i].date_from_coeditor = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_printers_final") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_printers_final", v) {
                Ok(val) => rows[i].date_printers_final = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_printers_first") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_printers_first", v) {
                Ok(val) => rows[i].date_printers_first = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_proofs_in") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_proofs_in", v) {
                Ok(val) => rows[i].date_proofs_in = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_proofs_out") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_proofs_out", v) {
                Ok(val) => rows[i].date_proofs_out = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_recd_copyright") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_recd_copyright", v) {
                Ok(val) => rows[i].date_recd_copyright = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_recd_electronic") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_recd_electronic", v) {
                Ok(val) => rows[i].date_recd_electronic = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_recd_hard_copy") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_recd_hard_copy", v) {
                Ok(val) => rows[i].date_recd_hard_copy = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_date_to_coeditor") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_date_to_coeditor", v) {
                Ok(val) => rows[i].date_to_coeditor = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_issue") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_issue", v) {
                Ok(val) => rows[i].issue = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_language") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_language", v) {
                Ok(val) => rows[i].language = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_name_full") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_name_full", v) {
                Ok(val) => rows[i].name_full = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_page_first") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_page_first", v) {
                Ok(val) => rows[i].page_first = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_page_last") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_page_last", v) {
                Ok(val) => rows[i].page_last = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_paper_category") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_paper_category", v) {
                Ok(val) => rows[i].paper_category = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_paper_doi") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_paper_doi", v) {
                Ok(val) => rows[i].paper_doi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_suppl_publ_number") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_suppl_publ_number", v) {
                Ok(val) => rows[i].suppl_publ_number = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_suppl_publ_pages") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_suppl_publ_pages", v) {
                Ok(val) => rows[i].suppl_publ_pages = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_techeditor_address") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_techeditor_address", v) {
                Ok(val) => rows[i].techeditor_address = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_techeditor_code") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_techeditor_code", v) {
                Ok(val) => rows[i].techeditor_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_techeditor_email") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_techeditor_email", v) {
                Ok(val) => rows[i].techeditor_email = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_techeditor_fax") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_techeditor_fax", v) {
                Ok(val) => rows[i].techeditor_fax = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_techeditor_name") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_techeditor_name", v) {
                Ok(val) => rows[i].techeditor_name = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_techeditor_notes") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_techeditor_notes", v) {
                Ok(val) => rows[i].techeditor_notes = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_techeditor_phone") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_techeditor_phone", v) {
                Ok(val) => rows[i].techeditor_phone = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_volume") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_volume", v) {
                Ok(val) => rows[i].volume = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "journal_year") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("journal_year", v) {
                Ok(val) => rows[i].year = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
