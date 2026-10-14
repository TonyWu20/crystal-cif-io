//! The `diffrn_reflns` category of the typed document layer (roadmap D12, tier T5).
//!
//! The single-value struct `DiffrnReflns` and the loop-row struct
//! `DiffrnReflnsRow`. `CifCategory` is on each group: the row group
//! emits one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};
use {
    crate::data_dict::core_cif::diffrn_reflns::AvREquivalents,
    crate::data_dict::core_cif::diffrn_reflns::AvSigmaIOverNetI,
    crate::data_dict::core_cif::diffrn_reflns::AvUnetIOverNetI,
    crate::data_dict::core_cif::diffrn_reflns::LaueMeasuredFractionFull,
    crate::data_dict::core_cif::diffrn_reflns::LaueMeasuredFractionMax,
    crate::data_dict::core_cif::diffrn_reflns::PointGroupMeasuredFractionFull,
    crate::data_dict::core_cif::diffrn_reflns::PointGroupMeasuredFractionMax,
    crate::data_dict::core_cif::diffrn_reflns::ReflnsNumber,
    crate::data_dict::core_cif::diffrn_reflns::ResolutionFull,
    crate::data_dict::core_cif::diffrn_reflns::ResolutionMax,
    crate::data_dict::core_cif::diffrn_reflns::ThetaFull,
    crate::data_dict::core_cif::diffrn_reflns::ThetaMax,
    crate::data_dict::core_cif::diffrn_reflns::ThetaMin,
};

#[derive(Debug, Clone, Default)]
pub struct DiffrnReflns {
    pub av_r_equivalents: Option<AvREquivalents>,
    pub av_sigma_i_over_net_i: Option<AvSigmaIOverNetI>,
    pub av_unet_i_over_net_i: Option<AvUnetIOverNetI>,
    pub laue_measured_fraction_full: Option<LaueMeasuredFractionFull>,
    pub laue_measured_fraction_max: Option<LaueMeasuredFractionMax>,
    pub number: Option<ReflnsNumber>,
    pub point_group_measured_fraction_full: Option<PointGroupMeasuredFractionFull>,
    pub point_group_measured_fraction_max: Option<PointGroupMeasuredFractionMax>,
    pub reduction_process: Option<CharString>,
    pub resolution_full: Option<ResolutionFull>,
    pub resolution_max: Option<ResolutionMax>,
    pub theta_full: Option<ThetaFull>,
    pub theta_max: Option<ThetaMax>,
    pub theta_min: Option<ThetaMin>,
}
impl CifCategory for DiffrnReflns {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.av_r_equivalents {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_av_R_equivalents".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.av_sigma_i_over_net_i {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_av_sigmaI/netI".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.av_unet_i_over_net_i {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_av_unetI/netI".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.laue_measured_fraction_full {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_Laue_measured_fraction_full".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.laue_measured_fraction_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_Laue_measured_fraction_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_number".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.point_group_measured_fraction_full {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_point_group_measured_fraction_full".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.point_group_measured_fraction_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_point_group_measured_fraction_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.reduction_process {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_reduction_process".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.resolution_full {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_resolution_full".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.resolution_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_resolution_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.theta_full {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_theta_full".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.theta_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_theta_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.theta_min {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_reflns_theta_min".to_string()),
                    Value::from(*v),
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
                    "diffrn_reflns_av_r_equivalents" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_av_R_equivalents",
                            value,
                            AvREquivalents::new,
                        ) {
                            Ok(v) => self.av_r_equivalents = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_av_sigmai/neti" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_av_sigmaI/netI",
                            value,
                            AvSigmaIOverNetI::new,
                        ) {
                            Ok(v) => self.av_sigma_i_over_net_i = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_av_uneti/neti" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_av_unetI/netI",
                            value,
                            AvUnetIOverNetI::new,
                        ) {
                            Ok(v) => self.av_unet_i_over_net_i = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_laue_measured_fraction_full" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_Laue_measured_fraction_full",
                            value,
                            LaueMeasuredFractionFull::new,
                        ) {
                            Ok(v) => self.laue_measured_fraction_full = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_laue_measured_fraction_max" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_Laue_measured_fraction_max",
                            value,
                            LaueMeasuredFractionMax::new,
                        ) {
                            Ok(v) => self.laue_measured_fraction_max = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_number" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_number",
                            value,
                            ReflnsNumber::new,
                        ) {
                            Ok(v) => self.number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_point_group_measured_fraction_full" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_point_group_measured_fraction_full",
                            value,
                            PointGroupMeasuredFractionFull::new,
                        ) {
                            Ok(v) => self.point_group_measured_fraction_full = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_point_group_measured_fraction_max" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_point_group_measured_fraction_max",
                            value,
                            PointGroupMeasuredFractionMax::new,
                        ) {
                            Ok(v) => self.point_group_measured_fraction_max = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_reduction_process" => {
                        match convert::read_char("diffrn_reflns_reduction_process", value) {
                            Ok(v) => self.reduction_process = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_resolution_full" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_resolution_full",
                            value,
                            ResolutionFull::new,
                        ) {
                            Ok(v) => self.resolution_full = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_resolution_max" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_resolution_max",
                            value,
                            ResolutionMax::new,
                        ) {
                            Ok(v) => self.resolution_max = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_theta_full" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_theta_full",
                            value,
                            ThetaFull::new,
                        ) {
                            Ok(v) => self.theta_full = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_theta_max" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_theta_max",
                            value,
                            ThetaMax::new,
                        ) {
                            Ok(v) => self.theta_max = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_reflns_theta_min" => {
                        match convert::read_ranged_newtype(
                            "diffrn_reflns_theta_min",
                            value,
                            ThetaMin::new,
                        ) {
                            Ok(v) => self.theta_min = v,
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
pub struct DiffrnReflnsRow {
    pub limit_h_max: Option<Numeric>,
    pub limit_h_min: Option<Numeric>,
    pub limit_k_max: Option<Numeric>,
    pub limit_k_min: Option<Numeric>,
    pub limit_l_max: Option<Numeric>,
    pub limit_l_min: Option<Numeric>,
    pub transf_matrix_11: Option<Numeric>,
    pub transf_matrix_12: Option<Numeric>,
    pub transf_matrix_13: Option<Numeric>,
    pub transf_matrix_21: Option<Numeric>,
    pub transf_matrix_22: Option<Numeric>,
    pub transf_matrix_23: Option<Numeric>,
    pub transf_matrix_31: Option<Numeric>,
    pub transf_matrix_32: Option<Numeric>,
    pub transf_matrix_33: Option<Numeric>,
}
impl CifCategory for Vec<DiffrnReflnsRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.limit_h_max.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.limit_h_max {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_limit_h_max".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.limit_h_min.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.limit_h_min {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_limit_h_min".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.limit_k_max.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.limit_k_max {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_limit_k_max".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.limit_k_min.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.limit_k_min {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_limit_k_min".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.limit_l_max.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.limit_l_max {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_limit_l_max".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.limit_l_min.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.limit_l_min {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_limit_l_min".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_11.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_11 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_11".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_12.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_12 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_12".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_13.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_13 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_13".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_21.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_21 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_21".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_22.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_22 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_22".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_23.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_23 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_23".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_31.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_31 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_31".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_32.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_32 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_32".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.transf_matrix_33.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.transf_matrix_33 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_reflns_transf_matrix_33".to_string()),
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
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_reflns_limit_h_max",
                    "diffrn_reflns_limit_h_min",
                    "diffrn_reflns_limit_k_max",
                    "diffrn_reflns_limit_k_min",
                    "diffrn_reflns_limit_l_max",
                    "diffrn_reflns_limit_l_min",
                    "diffrn_reflns_transf_matrix_11",
                    "diffrn_reflns_transf_matrix_12",
                    "diffrn_reflns_transf_matrix_13",
                    "diffrn_reflns_transf_matrix_21",
                    "diffrn_reflns_transf_matrix_22",
                    "diffrn_reflns_transf_matrix_23",
                    "diffrn_reflns_transf_matrix_31",
                    "diffrn_reflns_transf_matrix_32",
                    "diffrn_reflns_transf_matrix_33",
                ];
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

fn merge_rows(rows: &mut Vec<DiffrnReflnsRow>, cols: &[LoopColumn], errors: &mut Vec<CifError>) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnReflnsRow::default());
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_limit_h_max")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_limit_h_max", v) {
                Ok(val) => rows[i].limit_h_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_limit_h_min")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_limit_h_min", v) {
                Ok(val) => rows[i].limit_h_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_limit_k_max")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_limit_k_max", v) {
                Ok(val) => rows[i].limit_k_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_limit_k_min")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_limit_k_min", v) {
                Ok(val) => rows[i].limit_k_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_limit_l_max")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_limit_l_max", v) {
                Ok(val) => rows[i].limit_l_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_limit_l_min")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_limit_l_min", v) {
                Ok(val) => rows[i].limit_l_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_11")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_11", v) {
                Ok(val) => rows[i].transf_matrix_11 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_12")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_12", v) {
                Ok(val) => rows[i].transf_matrix_12 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_13")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_13", v) {
                Ok(val) => rows[i].transf_matrix_13 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_21")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_21", v) {
                Ok(val) => rows[i].transf_matrix_21 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_22")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_22", v) {
                Ok(val) => rows[i].transf_matrix_22 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_23")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_23", v) {
                Ok(val) => rows[i].transf_matrix_23 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_31")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_31", v) {
                Ok(val) => rows[i].transf_matrix_31 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_32")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_32", v) {
                Ok(val) => rows[i].transf_matrix_32 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_reflns_transf_matrix_33")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_reflns_transf_matrix_33", v) {
                Ok(val) => rows[i].transf_matrix_33 = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
