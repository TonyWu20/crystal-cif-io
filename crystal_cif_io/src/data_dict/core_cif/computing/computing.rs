//! The `computing` category of the typed document layer (roadmap D12).
//!
//! The `computing` single-value struct and the loop-row struct. One `Option`
//! field per registry item. `CifCategory` is on the `Computing` single
//! group and on the `Vec<ComputingRow>` loop group. Char items read leniently:
//! the tier corpus records them as text fields. The closed sets keep
//! the typed enum.

use crystal_cif_fmt::grammar::{Tag, Value, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct Computing {
    pub sample_tracking: Option<CharString>,
}

impl CifCategory for Computing {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.sample_tracking {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("computing_sample_tracking".to_string()),
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
                    "computing_sample_tracking" => {
                        match convert::read_char_text("computing_sample_tracking", value) {
                            Ok(v) => self.sample_tracking = v,
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

/// The `computing` loop-row struct. One `Option` field per registry column.
#[derive(Debug, Clone, Default)]
pub struct ComputingRow {
    pub cell_refinement: Option<CharString>,
    pub data_collection: Option<CharString>,
    pub data_reduction: Option<CharString>,
    pub molecular_graphics: Option<CharString>,
    pub publication_material: Option<CharString>,
    pub structure_refinement: Option<CharString>,
    pub structure_solution: Option<CharString>,
}

impl CifCategory for Vec<ComputingRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.cell_refinement.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.cell_refinement {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("computing_cell_refinement".to_string()), values));
        }
        if self.iter().any(|r| r.data_collection.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.data_collection {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("computing_data_collection".to_string()), values));
        }
        if self.iter().any(|r| r.data_reduction.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.data_reduction {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("computing_data_reduction".to_string()), values));
        }
        if self.iter().any(|r| r.molecular_graphics.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.molecular_graphics {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("computing_molecular_graphics".to_string()), values));
        }
        if self.iter().any(|r| r.publication_material.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.publication_material {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("computing_publication_material".to_string()), values));
        }
        if self.iter().any(|r| r.structure_refinement.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.structure_refinement {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("computing_structure_refinement".to_string()), values));
        }
        if self.iter().any(|r| r.structure_solution.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.structure_solution {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("computing_structure_solution".to_string()), values));
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
                        "computing_cell_refinement",
                        "computing_data_collection",
                        "computing_data_reduction",
                        "computing_molecular_graphics",
                        "computing_publication_material",
                        "computing_structure_refinement",
                        "computing_structure_solution",
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ComputingRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ComputingRow::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "computing_cell_refinement") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("computing_cell_refinement", v) {
                Ok(val) => rows[i].cell_refinement = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "computing_data_collection") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("computing_data_collection", v) {
                Ok(val) => rows[i].data_collection = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "computing_data_reduction") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("computing_data_reduction", v) {
                Ok(val) => rows[i].data_reduction = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "computing_molecular_graphics") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("computing_molecular_graphics", v) {
                Ok(val) => rows[i].molecular_graphics = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "computing_publication_material") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("computing_publication_material", v) {
                Ok(val) => rows[i].publication_material = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "computing_structure_refinement") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("computing_structure_refinement", v) {
                Ok(val) => rows[i].structure_refinement = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "computing_structure_solution") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char_text("computing_structure_solution", v) {
                Ok(val) => rows[i].structure_solution = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
