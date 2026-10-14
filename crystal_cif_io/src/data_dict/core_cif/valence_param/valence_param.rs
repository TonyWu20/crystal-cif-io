//! The `valence_param` category of the typed document layer (roadmap D12).
//!
//! The `ValenceParam` single-value struct (the T6 shell item) and
//! the `ValenceParamRow` loop row of the valence-model parameter
//! loop.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct ValenceParam {
    pub citation_id: Option<CharString>,
}

impl CifCategory for ValenceParam {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.citation_id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("valence_param_citation_id".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        out
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "valence_param_citation_id" => {
                        match convert::read_char("valence_param_citation_id", value) {
                            Ok(v) => self.citation_id = v,
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

#[derive(Debug, Clone, Default)]
pub struct ValenceParamRow {
    pub atom_1: Option<CharString>,
    pub atom_1_valence: Option<Numeric>,
    pub atom_2: Option<CharString>,
    pub atom_2_valence: Option<Numeric>,
    pub b: Option<Numeric>,
    pub details: Option<CharString>,
    pub id: Option<CharString>,
    pub ref_id: Option<CharString>,
    pub ro: Option<Numeric>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "valence_param_atom_1",
    "valence_param_atom_1_valence",
    "valence_param_atom_2",
    "valence_param_atom_2_valence",
    "valence_param_b",
    "valence_param_details",
    "valence_param_id",
    "valence_param_ref_id",
    "valence_param_ro",
];

impl CifCategory for Vec<ValenceParamRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.atom_1.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_1 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_atom_1".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.atom_1_valence.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_1_valence {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_atom_1_valence".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.atom_2.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_2 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_atom_2".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.atom_2_valence.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_2_valence {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_atom_2_valence".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.b.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.b {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_B".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.details.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.details {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_details".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.ref_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ref_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_ref_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.ro.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ro {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("valence_param_Ro".to_string()),
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

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    let is_ours = cols
                        .iter()
                        .any(|c| LOOP_TAGS.contains(&c.tag().as_ref().to_lowercase().as_str()));
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ValenceParamRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ValenceParamRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_atom_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("valence_param_atom_1", v) {
                Ok(val) => rows[i].atom_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_atom_1_valence")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("valence_param_atom_1_valence", v) {
                Ok(val) => rows[i].atom_1_valence = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_atom_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("valence_param_atom_2", v) {
                Ok(val) => rows[i].atom_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_atom_2_valence")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("valence_param_atom_2_valence", v) {
                Ok(val) => rows[i].atom_2_valence = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_b")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("valence_param_b", v) {
                Ok(val) => rows[i].b = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_details")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("valence_param_details", v) {
                Ok(val) => rows[i].details = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("valence_param_id", v) {
                Ok(val) => rows[i].id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_ref_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("valence_param_ref_id", v) {
                Ok(val) => rows[i].ref_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "valence_param_ro")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("valence_param_ro", v) {
                Ok(val) => rows[i].ro = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
