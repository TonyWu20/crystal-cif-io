use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DatabaseTerms {
    CSD_history(CharString),
    Code_depnum_ccdc_archive(CharString),
    Code_depnum_ccdc_fiz(CharString),
    Code_depnum_ccdc_journal(CharString),
    Dataset_doi(CharString),
}

impl CifTerm for DatabaseTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DatabaseTerms::CSD_history(_) => "csd_history",
            DatabaseTerms::Code_depnum_ccdc_archive(_) => "code_depnum_ccdc_archive",
            DatabaseTerms::Code_depnum_ccdc_fiz(_) => "code_depnum_ccdc_fiz",
            DatabaseTerms::Code_depnum_ccdc_journal(_) => "code_depnum_ccdc_journal",
            DatabaseTerms::Dataset_doi(_) => "dataset_doi",
        };
        Tag::new(format!("database_{suffix}"))
    }
}

impl SingleValueTerm for DatabaseTerms {
    fn value(&self) -> Value {
        match self {
            DatabaseTerms::CSD_history(s)
            | DatabaseTerms::Code_depnum_ccdc_archive(s)
            | DatabaseTerms::Code_depnum_ccdc_fiz(s)
            | DatabaseTerms::Code_depnum_ccdc_journal(s)
            | DatabaseTerms::Dataset_doi(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for DatabaseTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DatabaseLoopItem {
    Code_CAS(Vec<CharString>),
    Code_COD(Vec<CharString>),
    Code_CSD(Vec<CharString>),
    Code_ICSD(Vec<CharString>),
    Code_MDF(Vec<CharString>),
    Code_NBS(Vec<CharString>),
    Code_PDB(Vec<CharString>),
    Code_PDF(Vec<CharString>),
    Journal_ASTM(Vec<CharString>),
    Journal_CSD(Vec<CharString>),
}

impl CifTerm for DatabaseLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DatabaseLoopItem::Code_CAS(_) => "code_cas",
            DatabaseLoopItem::Code_COD(_) => "code_cod",
            DatabaseLoopItem::Code_CSD(_) => "code_csd",
            DatabaseLoopItem::Code_ICSD(_) => "code_icsd",
            DatabaseLoopItem::Code_MDF(_) => "code_mdf",
            DatabaseLoopItem::Code_NBS(_) => "code_nbs",
            DatabaseLoopItem::Code_PDB(_) => "code_pdb",
            DatabaseLoopItem::Code_PDF(_) => "code_pdf",
            DatabaseLoopItem::Journal_ASTM(_) => "journal_astm",
            DatabaseLoopItem::Journal_CSD(_) => "journal_csd",
        };
        Tag::new(format!("database_{suffix}"))
    }
}

impl LoopValueTerm for DatabaseLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DatabaseLoopItem::Code_CAS(s)
            | DatabaseLoopItem::Code_COD(s)
            | DatabaseLoopItem::Code_CSD(s)
            | DatabaseLoopItem::Code_ICSD(s)
            | DatabaseLoopItem::Code_MDF(s)
            | DatabaseLoopItem::Code_NBS(s)
            | DatabaseLoopItem::Code_PDB(s)
            | DatabaseLoopItem::Code_PDF(s)
            | DatabaseLoopItem::Journal_ASTM(s)
            | DatabaseLoopItem::Journal_CSD(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
