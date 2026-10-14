//! The `exptl_crystal_face` category of the typed document layer (roadmap D12).
//!
//! The `exptl_crystal_face` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<ExptlCrystalFace>`
//! group: it emits one `MultiValues` loop and fills the rows from the
//! columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
pub struct ExptlCrystalFace {
    pub diffr_chi: Option<Numeric>,
    pub diffr_kappa: Option<Numeric>,
    pub diffr_phi: Option<Numeric>,
    pub diffr_psi: Option<Numeric>,
    pub index_h: Option<Numeric>,
    pub index_k: Option<Numeric>,
    pub index_l: Option<Numeric>,
    pub perp_dist: Option<Numeric>,
}

impl CifCategory for Vec<ExptlCrystalFace> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.diffr_chi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.diffr_chi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_diffr_chi".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.diffr_kappa.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.diffr_kappa {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_diffr_kappa".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.diffr_phi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.diffr_phi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_diffr_phi".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.diffr_psi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.diffr_psi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_diffr_psi".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_h.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_h {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_index_h".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_k.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_k {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_index_k".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_l.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_l {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_index_l".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.perp_dist.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.perp_dist {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_face_perp_dist".to_string()),
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
                    // A loop is this category only when it carries one of
                    // the category column tags. A sibling loop of another
                    // category stays in the residue.
                    let ours = [
                        "exptl_crystal_face_diffr_chi",
                        "exptl_crystal_face_diffr_kappa",
                        "exptl_crystal_face_diffr_phi",
                        "exptl_crystal_face_diffr_psi",
                        "exptl_crystal_face_index_h",
                        "exptl_crystal_face_index_k",
                        "exptl_crystal_face_index_l",
                        "exptl_crystal_face_perp_dist",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = exptlcrystalface_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn exptlcrystalface_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<ExptlCrystalFace> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ExptlCrystalFace::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_diffr_chi")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_diffr_chi", v) {
                Ok(val) => rows[i].diffr_chi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_diffr_kappa")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_diffr_kappa", v) {
                Ok(val) => rows[i].diffr_kappa = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_diffr_phi")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_diffr_phi", v) {
                Ok(val) => rows[i].diffr_phi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_diffr_psi")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_diffr_psi", v) {
                Ok(val) => rows[i].diffr_psi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_index_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_index_h", v) {
                Ok(val) => rows[i].index_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_index_k")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_index_k", v) {
                Ok(val) => rows[i].index_k = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_index_l")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_index_l", v) {
                Ok(val) => rows[i].index_l = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_face_perp_dist")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_face_perp_dist", v) {
                Ok(val) => rows[i].perp_dist = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
