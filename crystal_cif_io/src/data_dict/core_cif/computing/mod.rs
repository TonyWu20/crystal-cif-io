mod computing;

pub use self::computing::{Computing, ComputingRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `computing` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ComputingTerms {
    /// The `_computing_sample_tracking` item. The variant holds the single value.
    Sample_tracking(CharString),
}

impl CifTerm for ComputingTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ComputingTerms::Sample_tracking(_) => "sample_tracking",
        };
        Tag::new(format!("computing_{suffix}"))
    }
}

impl SingleValueTerm for ComputingTerms {
    fn value(&self) -> Value {
        match self {
            ComputingTerms::Sample_tracking(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for ComputingTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `computing` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ComputingLoopItem {
    /// The `_computing_cell_refinement` item. The variant holds the column values, one per loop row.
    ///
    /// Software used in the processing of the data. Give the program or
    /// package name and a brief reference.
    Cell_refinement(Vec<CharString>),
    /// The `_computing_data_collection` item. The variant holds the column values, one per loop row.
    ///
    /// Software used in the processing of the data. Give the program or
    /// package name and a brief reference.
    Data_collection(Vec<CharString>),
    /// The `_computing_data_reduction` item. The variant holds the column values, one per loop row.
    ///
    /// Software used in the processing of the data. Give the program or
    /// package name and a brief reference.
    Data_reduction(Vec<CharString>),
    /// The `_computing_molecular_graphics` item. The variant holds the column values, one per loop row.
    ///
    /// Software used in the processing of the data. Give the program or
    /// package name and a brief reference.
    Molecular_graphics(Vec<CharString>),
    /// The `_computing_publication_material` item. The variant holds the column values, one per loop row.
    ///
    /// Software used in the processing of the data. Give the program or
    /// package name and a brief reference.
    Publication_material(Vec<CharString>),
    /// The `_computing_structure_refinement` item. The variant holds the column values, one per loop row.
    ///
    /// Software used in the processing of the data. Give the program or
    /// package name and a brief reference.
    Structure_refinement(Vec<CharString>),
    /// The `_computing_structure_solution` item. The variant holds the column values, one per loop row.
    ///
    /// Software used in the processing of the data. Give the program or
    /// package name and a brief reference.
    Structure_solution(Vec<CharString>),
}

impl CifTerm for ComputingLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ComputingLoopItem::Cell_refinement(_) => "cell_refinement",
            ComputingLoopItem::Data_collection(_) => "data_collection",
            ComputingLoopItem::Data_reduction(_) => "data_reduction",
            ComputingLoopItem::Molecular_graphics(_) => "molecular_graphics",
            ComputingLoopItem::Publication_material(_) => "publication_material",
            ComputingLoopItem::Structure_refinement(_) => "structure_refinement",
            ComputingLoopItem::Structure_solution(_) => "structure_solution",
        };
        Tag::new(format!("computing_{suffix}"))
    }
}

impl LoopValueTerm for ComputingLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ComputingLoopItem::Cell_refinement(s)
            | ComputingLoopItem::Data_collection(s)
            | ComputingLoopItem::Data_reduction(s)
            | ComputingLoopItem::Molecular_graphics(s)
            | ComputingLoopItem::Publication_material(s)
            | ComputingLoopItem::Structure_refinement(s)
            | ComputingLoopItem::Structure_solution(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
