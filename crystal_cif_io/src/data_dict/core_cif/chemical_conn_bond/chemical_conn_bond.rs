//! The `chemical_conn_bond` category of the typed document layer (roadmap D12).
//!
//! The `ChemicalConnBond` struct owns the single-value items (the T6
//! shell items) and the loop rows (`rows`, the connection-table
//! bond loop: the two atom indices and the bond type code).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::chemical_conn_bond::{BondAtomNumber, BondType};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct ChemicalConnBond {
    pub chem_comp_bond_value_dist: Option<Numeric>,
    pub id: Option<CharString>,    pub rows: Vec<ChemicalConnBondRow>,

}

impl CifCategory for ChemicalConnBond {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.chem_comp_bond_value_dist {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chem_comp_bond.value_dist".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_conn_bond_id".to_string()),
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
                        "chem_comp_bond.value_dist" => {
                            match convert::read_numeric("chem_comp_bond.value_dist", value) {
                                Ok(v) => self.chem_comp_bond_value_dist = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_conn_bond_id" => {
                            match convert::read_char("chemical_conn_bond_id", value) {
                                Ok(v) => self.id = v,
                                Err(e) => errors.push(e),
                            }
                        }
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
pub struct ChemicalConnBondRow {
    pub atom_1: Option<BondAtomNumber>,
    pub atom_2: Option<BondAtomNumber>,
    pub r#type: Option<BondType>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "chemical_conn_bond_atom_1",
    "chemical_conn_bond_atom_2",
    "chemical_conn_bond_type",
];

fn rows_to_members(rows: &[ChemicalConnBondRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.atom_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_1 {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_conn_bond_atom_1".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_2.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_2 {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_conn_bond_atom_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.r#type.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.r#type {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_conn_bond_type".to_string()),
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


fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ChemicalConnBondRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ChemicalConnBondRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_bond_atom_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_bond_atom_number("chemical_conn_bond_atom_1", v) {
                Ok(val) => rows[i].atom_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_bond_atom_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_bond_atom_number("chemical_conn_bond_atom_2", v) {
                Ok(val) => rows[i].atom_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_conn_bond_type")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "chemical_conn_bond_type",
                v,
                BondType::parse,
                "BondType",
            ) {
                Ok(val) => rows[i].r#type = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
