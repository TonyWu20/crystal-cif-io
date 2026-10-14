//! The `reflns_shell` category of the typed document layer (roadmap D12).
//!
//! The `reflns_shell` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<ReflnsShell>` group:
//! it emits one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
pub struct ReflnsShell {
    pub d_res_high: Option<Numeric>,
    pub d_res_low: Option<Numeric>,
    pub meani_over_sigi_all: Option<Numeric>,
    pub meani_over_sigi_gt: Option<Numeric>,
    pub meani_over_sigi_obs: Option<Numeric>,
    pub meani_over_ui_all: Option<Numeric>,
    pub meani_over_ui_gt: Option<Numeric>,
    pub number_measured_all: Option<Numeric>,
    pub number_measured_gt: Option<Numeric>,
    pub number_measured_obs: Option<Numeric>,
    pub number_possible: Option<Numeric>,
    pub number_unique_all: Option<Numeric>,
    pub number_unique_gt: Option<Numeric>,
    pub number_unique_obs: Option<Numeric>,
    pub percent_possible_all: Option<Numeric>,
    pub percent_possible_gt: Option<Numeric>,
    pub percent_possible_obs: Option<Numeric>,
    pub rmerge_f_all: Option<Numeric>,
    pub rmerge_f_gt: Option<Numeric>,
    pub rmerge_f_obs: Option<Numeric>,
    pub rmerge_i_all: Option<Numeric>,
    pub rmerge_i_gt: Option<Numeric>,
    pub rmerge_i_obs: Option<Numeric>,
}

impl CifCategory for Vec<ReflnsShell> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.d_res_high.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.d_res_high {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_d_res_high".to_string()),
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
                Tag::new("reflns_shell_d_res_low".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meani_over_sigi_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meani_over_sigi_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_meanI_over_sigI_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meani_over_sigi_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meani_over_sigi_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_meanI_over_sigI_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meani_over_sigi_obs.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meani_over_sigi_obs {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_meanI_over_sigI_obs".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meani_over_ui_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meani_over_ui_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_meanI_over_uI_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.meani_over_ui_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.meani_over_ui_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_meanI_over_uI_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number_measured_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_measured_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_number_measured_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number_measured_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_measured_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_number_measured_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number_measured_obs.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_measured_obs {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_number_measured_obs".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number_possible.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_possible {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_number_possible".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number_unique_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_unique_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_number_unique_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number_unique_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_unique_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_number_unique_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.number_unique_obs.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_unique_obs {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_number_unique_obs".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.percent_possible_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.percent_possible_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_percent_possible_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.percent_possible_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.percent_possible_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_percent_possible_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.percent_possible_obs.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.percent_possible_obs {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_percent_possible_obs".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.rmerge_f_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.rmerge_f_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_Rmerge_F_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.rmerge_f_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.rmerge_f_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_Rmerge_F_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.rmerge_f_obs.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.rmerge_f_obs {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_Rmerge_F_obs".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.rmerge_i_all.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.rmerge_i_all {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_Rmerge_I_all".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.rmerge_i_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.rmerge_i_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_Rmerge_I_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.rmerge_i_obs.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.rmerge_i_obs {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("reflns_shell_Rmerge_I_obs".to_string()),
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
                        "reflns_shell_d_res_high",
                        "reflns_shell_d_res_low",
                        "reflns_shell_meani_over_sigi_all",
                        "reflns_shell_meani_over_sigi_gt",
                        "reflns_shell_meani_over_sigi_obs",
                        "reflns_shell_meani_over_ui_all",
                        "reflns_shell_meani_over_ui_gt",
                        "reflns_shell_number_measured_all",
                        "reflns_shell_number_measured_gt",
                        "reflns_shell_number_measured_obs",
                        "reflns_shell_number_possible",
                        "reflns_shell_number_unique_all",
                        "reflns_shell_number_unique_gt",
                        "reflns_shell_number_unique_obs",
                        "reflns_shell_percent_possible_all",
                        "reflns_shell_percent_possible_gt",
                        "reflns_shell_percent_possible_obs",
                        "reflns_shell_rmerge_f_all",
                        "reflns_shell_rmerge_f_gt",
                        "reflns_shell_rmerge_f_obs",
                        "reflns_shell_rmerge_i_all",
                        "reflns_shell_rmerge_i_gt",
                        "reflns_shell_rmerge_i_obs",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = reflnsshell_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn reflnsshell_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<ReflnsShell> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ReflnsShell::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_d_res_high")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_d_res_high", v) {
                Ok(val) => rows[i].d_res_high = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_d_res_low")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_d_res_low", v) {
                Ok(val) => rows[i].d_res_low = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_meani_over_sigi_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_meani_over_sigi_all", v) {
                Ok(val) => rows[i].meani_over_sigi_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_meani_over_sigi_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_meani_over_sigi_gt", v) {
                Ok(val) => rows[i].meani_over_sigi_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_meani_over_sigi_obs")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_meani_over_sigi_obs", v) {
                Ok(val) => rows[i].meani_over_sigi_obs = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_meani_over_ui_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_meani_over_ui_all", v) {
                Ok(val) => rows[i].meani_over_ui_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_meani_over_ui_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_meani_over_ui_gt", v) {
                Ok(val) => rows[i].meani_over_ui_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_number_measured_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_number_measured_all", v) {
                Ok(val) => rows[i].number_measured_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_number_measured_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_number_measured_gt", v) {
                Ok(val) => rows[i].number_measured_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_number_measured_obs")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_number_measured_obs", v) {
                Ok(val) => rows[i].number_measured_obs = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_number_possible")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_number_possible", v) {
                Ok(val) => rows[i].number_possible = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_number_unique_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_number_unique_all", v) {
                Ok(val) => rows[i].number_unique_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_number_unique_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_number_unique_gt", v) {
                Ok(val) => rows[i].number_unique_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_number_unique_obs")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_number_unique_obs", v) {
                Ok(val) => rows[i].number_unique_obs = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_percent_possible_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_percent_possible_all", v) {
                Ok(val) => rows[i].percent_possible_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_percent_possible_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_percent_possible_gt", v) {
                Ok(val) => rows[i].percent_possible_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_percent_possible_obs")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_percent_possible_obs", v) {
                Ok(val) => rows[i].percent_possible_obs = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_rmerge_f_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_rmerge_f_all", v) {
                Ok(val) => rows[i].rmerge_f_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_rmerge_f_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_rmerge_f_gt", v) {
                Ok(val) => rows[i].rmerge_f_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_rmerge_f_obs")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_rmerge_f_obs", v) {
                Ok(val) => rows[i].rmerge_f_obs = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_rmerge_i_all")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_rmerge_i_all", v) {
                Ok(val) => rows[i].rmerge_i_all = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_rmerge_i_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_rmerge_i_gt", v) {
                Ok(val) => rows[i].rmerge_i_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "reflns_shell_rmerge_i_obs")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("reflns_shell_rmerge_i_obs", v) {
                Ok(val) => rows[i].rmerge_i_obs = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
