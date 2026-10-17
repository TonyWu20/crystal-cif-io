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
/// The `database` category struct. One field per registry item.
pub struct Database {
    /// The `_database_CSD_history` item. The field holds the single value.
    ///
    /// A history of changes made by the Cambridge Crystallographic Data
    /// Centre and incorporated into the Cambridge Structural Database
    /// (CSD).
    pub csd_history: Option<CharString>,
    /// The `_database_code_depnum_ccdc_archive` item. The field holds the single value.
    ///
    /// Deposition numbers assigned by the Cambridge Crystallographic Data
    /// Centre (CCDC) to files containing structural information archived by
    /// the CCDC.
    pub code_depnum_ccdc_archive: Option<CharString>,
    /// The `_database_code_depnum_ccdc_fiz` item. The field holds the single value.
    ///
    /// Deposition numbers assigned by the Fachinformationszentrum Karlsruhe
    /// (FIZ) to files containing structural information archived by the
    /// Cambridge Crystallographic Data Centre (CCDC).
    pub code_depnum_ccdc_fiz: Option<CharString>,
    /// The `_database_code_depnum_ccdc_journal` item. The field holds the single value.
    ///
    /// Deposition numbers assigned by various journals to files containing
    /// structural information archived by the Cambridge Crystallographic
    /// Data Centre (CCDC).
    pub code_depnum_ccdc_journal: Option<CharString>,
    /// The `_database_dataset_doi` item. The field holds the single value.
    ///
    /// The digital object identifier (DOI) registered to identify a data
    /// set publication associated with the structure described in the
    /// current datablock. This should be used for a dataset obtained from a
    /// curated database such as CSD or PDB. A DOI is a unique character
    /// string identifying any object of intellectual property. It provides
    /// a persistent identifier for an object on a digital network and
    /// permits the association of related current data in a structured
    /// extensible way. A DOI is an implementation of the Internet concepts
    /// of Uniform Resource Name and Universal Resource Locator managed
    /// according to the specifications of the International DOI Foundation
    /// (see `http://www.doi.org`).
    pub dataset_doi: Option<CharString>,
    /// The loop rows of the category: one row per observation.
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
/// One loop row of the `database` category: the per-observation values of the loop items.
pub struct DatabaseRow {
    /// The `_database_code_CAS` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_cas: Option<CharString>,
    /// The `_database_code_COD` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_cod: Option<CharString>,
    /// The `_database_code_CSD` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_csd: Option<CharString>,
    /// The `_database_code_ICSD` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_icsd: Option<CharString>,
    /// The `_database_code_MDF` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_mdf: Option<CharString>,
    /// The `_database_code_NBS` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_nbs: Option<CharString>,
    /// The `_database_code_PDB` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_pdb: Option<CharString>,
    /// The `_database_code_PDF` item. The field holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    pub code_pdf: Option<CharString>,
    /// The `_database_journal_ASTM` item. The field holds the column values, one per loop row.
    ///
    /// The ASTM CODEN designator for a journal as given in the Chemical
    /// Source List maintained by the Chemical Abstracts Service, and the
    /// journal code used in the Cambridge Structural Database.
    pub journal_astm: Option<CharString>,
    /// The `_database_journal_CSD` item. The field holds the column values, one per loop row.
    ///
    /// The ASTM CODEN designator for a journal as given in the Chemical
    /// Source List maintained by the Chemical Abstracts Service, and the
    /// journal code used in the Cambridge Structural Database.
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
