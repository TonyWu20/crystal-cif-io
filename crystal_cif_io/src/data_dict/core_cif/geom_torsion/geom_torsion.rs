//! The `geom_torsion` category of the typed document layer (roadmap D12).
//!
//! The `GeomTorsion` single-value struct (the T6 shell items) and
//! the `GeomTorsionRow` loop row of the torsion special list.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::geom_torsion::PublFlag;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct GeomTorsion {
    pub distances: Option<Vec<Numeric>>,
    pub id: Option<CharString>,
}

impl CifCategory for GeomTorsion {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.distances {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_torsion_distances".to_string()),
                    Value::List(v.iter().cloned().map(Value::from).collect()),
                )),
            )));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_torsion_id".to_string()),
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
                    "geom_torsion_distances" => {
                        match convert::read_numeric_list("geom_torsion_distances", value) {
                            Ok(v) => self.distances = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "geom_torsion_id" => match convert::read_char("geom_torsion_id", value) {
                        Ok(v) => self.id = v,
                        Err(e) => errors.push(e),
                    },
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
pub struct GeomTorsionRow {
    pub atom_site_label_1: Option<CharString>,
    pub atom_site_label_2: Option<CharString>,
    pub atom_site_label_3: Option<CharString>,
    pub atom_site_label_4: Option<CharString>,
    pub publ_flag: Option<PublFlag>,
    pub site_symmetry_1: Option<CharString>,
    pub site_symmetry_2: Option<CharString>,
    pub site_symmetry_3: Option<CharString>,
    pub site_symmetry_4: Option<CharString>,
    pub torsion: Option<Numeric>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "geom_torsion",
    "geom_torsion_atom_site_label_1",
    "geom_torsion_atom_site_label_2",
    "geom_torsion_atom_site_label_3",
    "geom_torsion_atom_site_label_4",
    "geom_torsion_publ_flag",
    "geom_torsion_site_symmetry_1",
    "geom_torsion_site_symmetry_2",
    "geom_torsion_site_symmetry_3",
    "geom_torsion_site_symmetry_4",
];

impl CifCategory for Vec<GeomTorsionRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.atom_site_label_1.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_site_label_1 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_atom_site_label_1".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.atom_site_label_2.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_site_label_2 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_atom_site_label_2".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.atom_site_label_3.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_site_label_3 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_atom_site_label_3".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.atom_site_label_4.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.atom_site_label_4 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_atom_site_label_4".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.publ_flag.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.publ_flag {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_publ_flag".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.site_symmetry_1.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.site_symmetry_1 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_site_symmetry_1".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.site_symmetry_2.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.site_symmetry_2 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_site_symmetry_2".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.site_symmetry_3.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.site_symmetry_3 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_site_symmetry_3".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.site_symmetry_4.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.site_symmetry_4 {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion_site_symmetry_4".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.torsion.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.torsion {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("geom_torsion".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<GeomTorsionRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![GeomTorsionRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_1", v) {
                Ok(val) => rows[i].atom_site_label_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_2", v) {
                Ok(val) => rows[i].atom_site_label_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_3", v) {
                Ok(val) => rows[i].atom_site_label_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_4")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_4", v) {
                Ok(val) => rows[i].atom_site_label_4 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_publ_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("geom_torsion_publ_flag", v, PublFlag::parse, "PublFlag")
            {
                Ok(val) => rows[i].publ_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_1", v) {
                Ok(val) => rows[i].site_symmetry_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_2", v) {
                Ok(val) => rows[i].site_symmetry_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_3", v) {
                Ok(val) => rows[i].site_symmetry_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_4")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_4", v) {
                Ok(val) => rows[i].site_symmetry_4 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("geom_torsion", v) {
                Ok(val) => rows[i].torsion = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
