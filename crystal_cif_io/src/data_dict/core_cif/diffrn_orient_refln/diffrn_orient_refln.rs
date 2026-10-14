//! The `diffrn_orient_refln` category of the typed document layer (roadmap D12, tier T5).
//!
//! The loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<DiffrnOrientRefln>` group: it emits
//! one `MultiValues` loop and fills the rows from the columns.

use crystal_cif_fmt::grammar::{Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct DiffrnOrientRefln {
    pub angle_chi: Option<Numeric>,
    pub angle_kappa: Option<Numeric>,
    pub angle_omega: Option<Numeric>,
    pub angle_phi: Option<Numeric>,
    pub angle_psi: Option<Numeric>,
    pub angle_theta: Option<Numeric>,
    pub index_h: Option<Numeric>,
    pub index_k: Option<Numeric>,
    pub index_l: Option<Numeric>,
}
impl CifCategory for Vec<DiffrnOrientRefln> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.angle_chi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_chi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_angle_chi".to_string()), values));
        }
        if self.iter().any(|r| r.angle_kappa.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_kappa {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_angle_kappa".to_string()), values));
        }
        if self.iter().any(|r| r.angle_omega.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_omega {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_angle_omega".to_string()), values));
        }
        if self.iter().any(|r| r.angle_phi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_phi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_angle_phi".to_string()), values));
        }
        if self.iter().any(|r| r.angle_psi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_psi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_angle_psi".to_string()), values));
        }
        if self.iter().any(|r| r.angle_theta.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_theta {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_angle_theta".to_string()), values));
        }
        if self.iter().any(|r| r.index_h.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_h {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_index_h".to_string()), values));
        }
        if self.iter().any(|r| r.index_k.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_k {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_index_k".to_string()), values));
        }
        if self.iter().any(|r| r.index_l.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_l {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_orient_refln_index_l".to_string()), values));
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
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_orient_refln_angle_chi",
                    "diffrn_orient_refln_angle_kappa",
                    "diffrn_orient_refln_angle_omega",
                    "diffrn_orient_refln_angle_phi",
                    "diffrn_orient_refln_angle_psi",
                    "diffrn_orient_refln_angle_theta",
                    "diffrn_orient_refln_index_h",
                    "diffrn_orient_refln_index_k",
                    "diffrn_orient_refln_index_l",];
                let is_ours = cols
                    .iter()
                    .any(|c| own_tags.contains(&c.tag().as_ref().to_lowercase().as_str()));
                if is_ours {
                    merge_rows(self, cols, &mut errors);
                } else {
                    residue.push(m.clone());
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}

fn merge_rows(rows: &mut Vec<DiffrnOrientRefln>, cols: &[LoopColumn], errors: &mut Vec<CifError>) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnOrientRefln::default());
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_angle_chi") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_angle_chi", v) {
                Ok(val) => rows[i].angle_chi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_angle_kappa") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_angle_kappa", v) {
                Ok(val) => rows[i].angle_kappa = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_angle_omega") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_angle_omega", v) {
                Ok(val) => rows[i].angle_omega = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_angle_phi") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_angle_phi", v) {
                Ok(val) => rows[i].angle_phi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_angle_psi") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_angle_psi", v) {
                Ok(val) => rows[i].angle_psi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_angle_theta") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_angle_theta", v) {
                Ok(val) => rows[i].angle_theta = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_index_h") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_index_h", v) {
                Ok(val) => rows[i].index_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_index_k") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_index_k", v) {
                Ok(val) => rows[i].index_k = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_orient_refln_index_l") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_orient_refln_index_l", v) {
                Ok(val) => rows[i].index_l = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
