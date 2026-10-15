//! The `geom_bond` category of the typed document layer (roadmap D12).
//!
//! The `GeomBond` struct owns the single-value item (the T6 shell
//! item) and the loop rows (`rows`, the bond special list).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::geom_bond::{BondDistance, BondMultiplicity, PublFlag};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct GeomBond {
    pub id: Option<CharString>,    pub rows: Vec<GeomBondRow>,

}

impl CifCategory for GeomBond {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_bond_id".to_string()),
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
                        "geom_bond_id" => match convert::read_char("geom_bond_id", value) {
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
pub struct GeomBondRow {
    pub atom_site_label_1: Option<CharString>,
    pub atom_site_label_2: Option<CharString>,
    pub distance: Option<BondDistance>,
    pub multiplicity: Option<BondMultiplicity>,
    pub publ_flag: Option<PublFlag>,
    pub site_symmetry_1: Option<CharString>,
    pub site_symmetry_2: Option<CharString>,
    pub valence: Option<Numeric>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "geom_bond_atom_site_label_1",
    "geom_bond_atom_site_label_2",
    "geom_bond_distance",
    "geom_bond_multiplicity",
    "geom_bond_publ_flag",
    "geom_bond_site_symmetry_1",
    "geom_bond_site_symmetry_2",
    "geom_bond_valence",
];

fn rows_to_members(rows: &[GeomBondRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.atom_site_label_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_1 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_bond_atom_site_label_1".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_2.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_2 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_bond_atom_site_label_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_bond_distance".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.multiplicity.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.multiplicity {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_bond_multiplicity".to_string()),
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
            Tag::new("geom_bond_publ_flag".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_1 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_bond_site_symmetry_1".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_2.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_2 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_bond_site_symmetry_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.valence.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.valence {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_bond_valence".to_string()),
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


fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<GeomBondRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![GeomBondRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_atom_site_label_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_bond_atom_site_label_1", v) {
                Ok(val) => rows[i].atom_site_label_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_atom_site_label_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_bond_atom_site_label_2", v) {
                Ok(val) => rows[i].atom_site_label_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_distance")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_bond_distance("geom_bond_distance", v) {
                Ok(val) => rows[i].distance = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_multiplicity")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_bond_multiplicity("geom_bond_multiplicity", v) {
                Ok(val) => rows[i].multiplicity = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_publ_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("geom_bond_publ_flag", v, PublFlag::parse, "PublFlag") {
                Ok(val) => rows[i].publ_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_site_symmetry_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_bond_site_symmetry_1", v) {
                Ok(val) => rows[i].site_symmetry_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_site_symmetry_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_bond_site_symmetry_2", v) {
                Ok(val) => rows[i].site_symmetry_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_bond_valence")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("geom_bond_valence", v) {
                Ok(val) => rows[i].valence = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
