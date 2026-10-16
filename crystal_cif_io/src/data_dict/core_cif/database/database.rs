//! The `database` category of the typed document layer (roadmap D12).
//!
//! The `Database` struct owns the single-value items (one `Option` field
//! per registry item) and the loop rows (`rows`). One `CifCategory`
//! impl covers both. Char items read leniently: the tier corpus
//! records them as text fields. The closed sets keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub csd_history: Option<CharString>,
    pub code_depnum_ccdc_archive: Option<CharString>,
    pub code_depnum_ccdc_fiz: Option<CharString>,
    pub code_depnum_ccdc_journal: Option<CharString>,
    pub dataset_doi: Option<CharString>,
    pub rows: Vec<DatabaseRow>,
}

impl CifCategory for Database {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.csd_history {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("database_csd_history".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.code_depnum_ccdc_archive {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("database_code_depnum_ccdc_archive".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.code_depnum_ccdc_fiz {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("database_code_depnum_ccdc_fiz".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.code_depnum_ccdc_journal {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("database_code_depnum_ccdc_journal".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.dataset_doi {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("database_dataset_doi".to_string()),
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
                        "database_csd_history" => {
                            match convert::read_char_text("database_csd_history", value) {
                                Ok(v) => self.csd_history = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "database_code_depnum_ccdc_archive" => {
                            match convert::read_char_text(
                                "database_code_depnum_ccdc_archive",
                                value,
                            ) {
                                Ok(v) => self.code_depnum_ccdc_archive = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "database_code_depnum_ccdc_fiz" => {
                            match convert::read_char_text("database_code_depnum_ccdc_fiz", value) {
                                Ok(v) => self.code_depnum_ccdc_fiz = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "database_code_depnum_ccdc_journal" => {
                            match convert::read_char_text(
                                "database_code_depnum_ccdc_journal",
                                value,
                            ) {
                                Ok(v) => self.code_depnum_ccdc_journal = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "database_dataset_doi" => {
                            match convert::read_char_text("database_dataset_doi", value) {
                                Ok(v) => self.dataset_doi = v,
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
                        "database_code_cas",
                        "database_code_cod",
                        "database_code_csd",
                        "database_code_icsd",
                        "database_code_mdf",
                        "database_code_nbs",
                        "database_code_pdb",
                        "database_code_pdf",
                        "database_journal_astm",
                        "database_journal_csd",
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

/// The `database` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct DatabaseRow {
    pub code_cas: Option<CharString>,
    pub code_cod: Option<CharString>,
    pub code_csd: Option<CharString>,
    pub code_icsd: Option<CharString>,
    pub code_mdf: Option<CharString>,
    pub code_nbs: Option<CharString>,
    pub code_pdb: Option<CharString>,
    pub code_pdf: Option<CharString>,
    pub journal_astm: Option<CharString>,
    pub journal_csd: Option<CharString>,
}

fn rows_to_members(rows: &[DatabaseRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.code_cas.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_cas {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_cas".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.code_cod.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_cod {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_cod".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.code_csd.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_csd {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_csd".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.code_icsd.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_icsd {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_icsd".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.code_mdf.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_mdf {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_mdf".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.code_nbs.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_nbs {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_nbs".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.code_pdb.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_pdb {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_pdb".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.code_pdf.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.code_pdf {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_code_pdf".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_astm.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_astm {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_journal_astm".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.journal_csd.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.journal_csd {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("database_journal_csd".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<DatabaseRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![DatabaseRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_cas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_cas", v) {
                Ok(val) => rows[i].code_cas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_cod")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_cod", v) {
                Ok(val) => rows[i].code_cod = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_csd")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_csd", v) {
                Ok(val) => rows[i].code_csd = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_icsd")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_icsd", v) {
                Ok(val) => rows[i].code_icsd = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_mdf")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_mdf", v) {
                Ok(val) => rows[i].code_mdf = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_nbs")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_nbs", v) {
                Ok(val) => rows[i].code_nbs = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_pdb")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_pdb", v) {
                Ok(val) => rows[i].code_pdb = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_code_pdf")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_code_pdf", v) {
                Ok(val) => rows[i].code_pdf = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_journal_astm")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_journal_astm", v) {
                Ok(val) => rows[i].journal_astm = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "database_journal_csd")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("database_journal_csd", v) {
                Ok(val) => rows[i].journal_csd = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
