//! The `geom_hbond` category of the typed document layer (roadmap D12).
//!
//! The `GeomHbond` struct owns the single-value item (the T6 shell
//! item) and the loop rows (`rows`, the hydrogen-bond special list).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::geom_hbond::{HbondDistance, PublFlag};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct GeomHbond {
    pub id: Option<CharString>,
    pub rows: Vec<GeomHbondRow>,
}

impl CifCategory for GeomHbond {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_hbond_id".to_string()),
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
                        "geom_hbond_id" => match convert::read_char("geom_hbond_id", value) {
                            Ok(v) => self.id = v,
                            Err(e) => errors.push(e),
                        },
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    let is_ours = cols
                        .iter()
                        .any(|c| LOOP_TAGS.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if is_ours {
                        self.rows = rows_from_columns(cols, &mut errors);
                    } else {
                        residue.push(m.clone());
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeomHbondRow {
    pub angle_dha: Option<Numeric>,
    pub atom_site_label_a: Option<CharString>,
    pub atom_site_label_d: Option<CharString>,
    pub atom_site_label_h: Option<CharString>,
    pub distance_da: Option<HbondDistance>,
    pub distance_dh: Option<HbondDistance>,
    pub distance_ha: Option<HbondDistance>,
    pub publ_flag: Option<PublFlag>,
    pub site_symmetry_a: Option<CharString>,
    pub site_symmetry_d: Option<CharString>,
    pub site_symmetry_h: Option<CharString>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "geom_hbond_angle_dha",
    "geom_hbond_atom_site_label_a",
    "geom_hbond_atom_site_label_d",
    "geom_hbond_atom_site_label_h",
    "geom_hbond_distance_da",
    "geom_hbond_distance_dh",
    "geom_hbond_distance_ha",
    "geom_hbond_publ_flag",
    "geom_hbond_site_symmetry_a",
    "geom_hbond_site_symmetry_d",
    "geom_hbond_site_symmetry_h",
];

fn rows_to_members(rows: &[GeomHbondRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.angle_dha.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.angle_dha {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_angle_DHA".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_a.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_a {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_atom_site_label_A".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_d.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_d {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_atom_site_label_D".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_h.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_h {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_atom_site_label_H".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance_da.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance_da {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_distance_DA".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance_dh.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance_dh {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_distance_DH".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance_ha.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance_ha {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_distance_HA".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.publ_flag.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.publ_flag {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_publ_flag".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_a.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_a {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_site_symmetry_A".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_d.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_d {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_site_symmetry_D".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_h.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_h {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_site_symmetry_H".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<GeomHbondRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![GeomHbondRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_angle_dha")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("geom_hbond_angle_dha", v) {
                Ok(val) => rows[i].angle_dha = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_atom_site_label_a")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_atom_site_label_a", v) {
                Ok(val) => rows[i].atom_site_label_a = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_atom_site_label_d")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_atom_site_label_d", v) {
                Ok(val) => rows[i].atom_site_label_d = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_atom_site_label_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_atom_site_label_h", v) {
                Ok(val) => rows[i].atom_site_label_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_distance_da")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_hbond_distance("geom_hbond_distance_da", v) {
                Ok(val) => rows[i].distance_da = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_distance_dh")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_hbond_distance("geom_hbond_distance_dh", v) {
                Ok(val) => rows[i].distance_dh = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_distance_ha")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_hbond_distance("geom_hbond_distance_ha", v) {
                Ok(val) => rows[i].distance_ha = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_publ_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("geom_hbond_publ_flag", v, PublFlag::parse, "PublFlag") {
                Ok(val) => rows[i].publ_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_site_symmetry_a")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_site_symmetry_a", v) {
                Ok(val) => rows[i].site_symmetry_a = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_site_symmetry_d")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_site_symmetry_d", v) {
                Ok(val) => rows[i].site_symmetry_d = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_site_symmetry_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_site_symmetry_h", v) {
                Ok(val) => rows[i].site_symmetry_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
