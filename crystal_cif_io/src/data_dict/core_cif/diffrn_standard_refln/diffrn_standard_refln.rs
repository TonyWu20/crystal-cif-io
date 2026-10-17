//! The `diffrn_standard_refln` category of the typed document layer (roadmap D12, tier T5).
//!
//! The `DiffrnStandardRefln` struct owns the single-value items and the loop
//! rows (`rows`). One `CifCategory` impl: the scalar items, then
//! the `MultiValues` loop of the `DiffrnStandardReflnRow` columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `diffrn_standard_refln` category struct. One field per registry item.
pub struct DiffrnStandardRefln {
    /// The `_diffrn_standard_refln_code` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying a reflection measured as a standard reflection
    /// with the indices _diffrn_standard_refln_index_. This is the same
    /// code as the _diffrn_refln_standard_code in the _diffrn_refln_ list.
    pub code: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<DiffrnStandardReflnRow>,
}
impl CifCategory for DiffrnStandardRefln {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.code {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standard_refln_code".to_string()),
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
                        "diffrn_standard_refln_code" => {
                            match convert::read_char("diffrn_standard_refln_code", value) {
                                Ok(v) => self.code = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries one of its
                    // own loop column tags. A loop without our tags stays in
                    // the residue instead of overwriting the rows.
                    let own_tags = [
                        "diffrn_standard_refln_index_h",
                        "diffrn_standard_refln_index_k",
                        "diffrn_standard_refln_index_l",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| own_tags.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if is_ours {
                        merge_rows(&mut self.rows, cols, &mut errors);
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
/// One loop row of the `diffrn_standard_refln` category: the per-observation values of the loop items.
pub struct DiffrnStandardReflnRow {
    /// The `_diffrn_standard_refln_index_h` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of standard reflections used in the diffraction
    /// measurement process.
    pub index_h: Option<Numeric>,
    /// The `_diffrn_standard_refln_index_k` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of standard reflections used in the diffraction
    /// measurement process.
    pub index_k: Option<Numeric>,
    /// The `_diffrn_standard_refln_index_l` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of standard reflections used in the diffraction
    /// measurement process.
    pub index_l: Option<Numeric>,
}
fn rows_to_members(rows: &[DiffrnStandardReflnRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.index_h.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.index_h {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_standard_refln_index_h".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.index_k.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.index_k {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_standard_refln_index_k".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.index_l.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.index_l {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("diffrn_standard_refln_index_l".to_string()),
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

fn merge_rows(
    rows: &mut Vec<DiffrnStandardReflnRow>,
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnStandardReflnRow::default());
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_standard_refln_index_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_standard_refln_index_h", v) {
                Ok(val) => rows[i].index_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_standard_refln_index_k")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_standard_refln_index_k", v) {
                Ok(val) => rows[i].index_k = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_standard_refln_index_l")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_standard_refln_index_l", v) {
                Ok(val) => rows[i].index_l = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
