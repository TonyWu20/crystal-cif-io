//! The `refine_ls_class` category of the typed document layer (roadmap D12).
//!
//! The `refine_ls_class` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<RefineLsClass>`
//! group: it emits one `MultiValues` loop and fills the rows from the
//! columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
/// The `refine_ls_class` category struct. One field per registry item.
pub struct RefineLsClass {
    /// The `_refine_ls_class_code` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying a certain reflection class. This code must
    /// match a _reflns_class_code.
    pub code: Option<CharString>,
    /// The `_refine_ls_class_d_res_high` item. The field holds the column values, one per loop row.
    ///
    /// For each reflection class, the highest resolution in angstroms for
    /// the reflections used in the refinement. This is the lowest d value
    /// in a reflection class.
    /// Units: A (angstrom).
    pub d_res_high: Option<Numeric>,
    /// The `_refine_ls_class_d_res_low` item. The field holds the column values, one per loop row.
    ///
    /// For each reflection class, the lowest resolution in angstroms for
    /// the reflections used in the refinement. This is the highest d value
    /// in a reflection class.
    /// Units: A (angstrom).
    pub d_res_low: Option<Numeric>,
    /// The `_refine_ls_class_R_factor_all` item. The field holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factors for all reflections,
    /// and for significantly intense reflections (see
    /// _reflns_threshold_expression), included in the refinement. The
    /// reflections also satisfy the resolution limits established by
    /// _refine_ls_class_d_res_high and _refine_ls_class_d_res_low. This is
    /// the conventional R factor. sum | F(obs) - F(calc) | R =
    /// ------------------------ sum | F(obs) | F(obs) = the observed
    /// structure-factor amplitudes F(calc) = the calculated
    /// structure-factor amplitudes and the sum is taken over the
    /// reflections of this class. See also _refine_ls_class_wR_factor_all
    /// definitions.
    pub r_factor_all: Option<Numeric>,
    /// The `_refine_ls_class_R_factor_gt` item. The field holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factors for all reflections,
    /// and for significantly intense reflections (see
    /// _reflns_threshold_expression), included in the refinement. The
    /// reflections also satisfy the resolution limits established by
    /// _refine_ls_class_d_res_high and _refine_ls_class_d_res_low. This is
    /// the conventional R factor. sum | F(obs) - F(calc) | R =
    /// ------------------------ sum | F(obs) | F(obs) = the observed
    /// structure-factor amplitudes F(calc) = the calculated
    /// structure-factor amplitudes and the sum is taken over the
    /// reflections of this class. See also _refine_ls_class_wR_factor_all
    /// definitions.
    pub r_factor_gt: Option<Numeric>,
    /// The `_refine_ls_class_R_Fsqd_factor` item. The field holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factor R(F^2^) calculated on
    /// the squared amplitudes of the observed and calculated structure
    /// factors for the reflections judged significantly intense (i.e.
    /// satisfying the threshold specified by _reflns_threshold_expression)
    /// and included in the refinement. The reflections also satisfy the
    /// resolution limits established by _refine_ls_class_d_res_high and
    /// _refine_ls_class_d_res_low. sum | F(obs)^2^ - F(calc)^2^ | R(Fsqd) =
    /// ------------------------------- sum F(obs)^2^ F(obs)^2^ = squares of
    /// the observed structure-factor amplitudes F(calc)^2^ = squares of the
    /// calculated structure-factor amplitudes and the sum is taken over the
    /// reflections of this class.
    pub r_fsqd_factor: Option<Numeric>,
    /// The `_refine_ls_class_R_I_factor` item. The field holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factor R(I) for the
    /// reflections judged significantly intense (i.e. satisfying the
    /// threshold specified by _reflns_threshold_expression) and included in
    /// the refinement. This is most often calculated in Rietveld
    /// refinements against powder data, where it is referred to as R~B~ or
    /// R~Bragg~. sum | I(obs) - I(calc) | R(I) = ------------------------
    /// sum | I(obs) | I(obs) = the net observed intensities I(calc) = the
    /// net calculated intensities and the sum is taken over the reflections
    /// of this class.
    pub r_i_factor: Option<Numeric>,
    /// The `_refine_ls_class_wR_factor_all` item. The field holds the column values, one per loop row.
    ///
    /// For each reflection class, the weighted residual factors for all
    /// reflections included in the refinement. The reflections also satisfy
    /// the resolution limits established by _refine_ls_class_d_res_high and
    /// _refine_ls_class_d_res_low. ( sum w [ Y(obs) - Y(calc) ]^2^ )^1/2^
    /// wR = ( ------------------------------ ) ( sum w Y(obs)^2^ ) Y(obs) =
    /// the observed amplitudes specified by
    /// _refine_ls_structure_factor_coef Y(calc) = the calculated amplitudes
    /// specified by _refine_ls_structure_factor_coef w = the least-squares
    /// weights and the sum is taken over the reflections of this class. See
    /// also _refine_ls_class_R_factor_ definitions.
    pub wr_factor_all: Option<Numeric>,
}

impl CifCategory for Vec<RefineLsClass> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.d_res_high.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_res_high {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_d_res_high".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.d_res_low.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_res_low {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_d_res_low".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.r_factor_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_factor_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_R_factor_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.r_factor_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_factor_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_R_factor_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.r_fsqd_factor.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_fsqd_factor {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_R_Fsqd_factor".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.r_i_factor.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.r_i_factor {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_R_I_factor".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.wr_factor_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wr_factor_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("refine_ls_class_wR_factor_all".to_string()),
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
                        "refine_ls_class_code",
                        "refine_ls_class_d_res_high",
                        "refine_ls_class_d_res_low",
                        "refine_ls_class_r_factor_all",
                        "refine_ls_class_r_factor_gt",
                        "refine_ls_class_r_fsqd_factor",
                        "refine_ls_class_r_i_factor",
                        "refine_ls_class_wr_factor_all",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = refinelsclass_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn refinelsclass_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<RefineLsClass> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![RefineLsClass::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("refine_ls_class_code", v) {
                Ok(val) => rows[i].code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_d_res_high")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_d_res_high", v) {
                Ok(val) => rows[i].d_res_high = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_d_res_low")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_d_res_low", v) {
                Ok(val) => rows[i].d_res_low = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_factor_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_factor_all", v) {
                Ok(val) => rows[i].r_factor_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_factor_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_factor_gt", v) {
                Ok(val) => rows[i].r_factor_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_fsqd_factor")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_fsqd_factor", v) {
                Ok(val) => rows[i].r_fsqd_factor = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_r_i_factor")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_r_i_factor", v) {
                Ok(val) => rows[i].r_i_factor = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_ls_class_wr_factor_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_ls_class_wr_factor_all", v) {
                Ok(val) => rows[i].wr_factor_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
