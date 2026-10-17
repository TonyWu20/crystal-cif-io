mod database;

pub use self::database::{Database, DatabaseRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `database` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DatabaseTerms {
    /// The `_database_CSD_history` item. The variant holds the single value.
    ///
    /// A history of changes made by the Cambridge Crystallographic Data
    /// Centre and incorporated into the Cambridge Structural Database
    /// (CSD).
    CSD_history(CharString),
    /// The `_database_code_depnum_ccdc_archive` item. The variant holds the single value.
    ///
    /// Deposition numbers assigned by the Cambridge Crystallographic Data
    /// Centre (CCDC) to files containing structural information archived by
    /// the CCDC.
    Code_depnum_ccdc_archive(CharString),
    /// The `_database_code_depnum_ccdc_fiz` item. The variant holds the single value.
    ///
    /// Deposition numbers assigned by the Fachinformationszentrum Karlsruhe
    /// (FIZ) to files containing structural information archived by the
    /// Cambridge Crystallographic Data Centre (CCDC).
    Code_depnum_ccdc_fiz(CharString),
    /// The `_database_code_depnum_ccdc_journal` item. The variant holds the single value.
    ///
    /// Deposition numbers assigned by various journals to files containing
    /// structural information archived by the Cambridge Crystallographic
    /// Data Centre (CCDC).
    Code_depnum_ccdc_journal(CharString),
    /// The `_database_dataset_doi` item. The variant holds the single value.
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

/// The loop-value terms of the `database` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DatabaseLoopItem {
    /// The `_database_code_CAS` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_CAS(Vec<CharString>),
    /// The `_database_code_COD` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_COD(Vec<CharString>),
    /// The `_database_code_CSD` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_CSD(Vec<CharString>),
    /// The `_database_code_ICSD` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_ICSD(Vec<CharString>),
    /// The `_database_code_MDF` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_MDF(Vec<CharString>),
    /// The `_database_code_NBS` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_NBS(Vec<CharString>),
    /// The `_database_code_PDB` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_PDB(Vec<CharString>),
    /// The `_database_code_PDF` item. The variant holds the column values, one per loop row.
    ///
    /// The codes are assigned by databases: Chemical Abstracts;
    /// Crystallography Open Database (COD); Cambridge Structural Database
    /// (organic and metal-organic compounds); Inorganic Crystal Structure
    /// Database; Metals Data File (metal structures); NBS (NIST) Crystal
    /// Data Database (lattice parameters); Protein Data Bank; and the
    /// Powder Diffraction File (JCPDS/ICDD).
    Code_PDF(Vec<CharString>),
    /// The `_database_journal_ASTM` item. The variant holds the column values, one per loop row.
    ///
    /// The ASTM CODEN designator for a journal as given in the Chemical
    /// Source List maintained by the Chemical Abstracts Service, and the
    /// journal code used in the Cambridge Structural Database.
    Journal_ASTM(Vec<CharString>),
    /// The `_database_journal_CSD` item. The variant holds the column values, one per loop row.
    ///
    /// The ASTM CODEN designator for a journal as given in the Chemical
    /// Source List maintained by the Chemical Abstracts Service, and the
    /// journal code used in the Cambridge Structural Database.
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
