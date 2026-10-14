//! The `refine` category of the typed document layer (roadmap D12).
//!
//! The `refine` category struct: the forty-three single-value items
//! and the `refine_diff_density` loop family. `CifCategory` is on the
//! `Refine` group. The range-checked items keep the entry newtypes;
//! the closed sets keep the typed enums.

use crate::core_cif::CifError;
use crate::data_dict::Ineq;
use crate::data_dict::core_cif::refine::{
    Flack, HydrogenTreatment, MatrixType, NonNeg, Rogers, StructureFactorCoef, WeightingScheme,
};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, UnsignedInteger, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct Refine {
    pub ls_abs_structure_details: Option<CharString>,
    pub ls_abs_structure_flack: Option<Flack>,
    pub ls_abs_structure_rogers: Option<Rogers>,
    pub ls_d_res_high: Option<NonNeg>,
    pub ls_d_res_low: Option<NonNeg>,
    pub ls_extinction_coef: Option<Numeric>,
    pub ls_extinction_expression: Option<CharString>,
    pub ls_extinction_method: Option<CharString>,
    pub ls_f_calc_details: Option<CharString>,
    pub ls_f_calc_formula: Option<CharString>,
    pub ls_f_calc_precision: Option<NonNeg>,
    pub ls_goodness_of_fit_all: Option<NonNeg>,
    pub ls_goodness_of_fit_gt: Option<NonNeg>,
    pub ls_goodness_of_fit_obs: Option<NonNeg>,
    pub ls_goodness_of_fit_ref: Option<NonNeg>,
    pub ls_hydrogen_treatment: Option<HydrogenTreatment>,
    pub ls_matrix_type: Option<MatrixType>,
    pub ls_number_constraints: Option<UnsignedInteger>,
    pub ls_number_parameters: Option<UnsignedInteger>,
    pub ls_number_reflns: Option<UnsignedInteger>,
    pub ls_number_restraints: Option<UnsignedInteger>,
    pub ls_r_factor_all: Option<NonNeg>,
    pub ls_r_factor_gt: Option<NonNeg>,
    pub ls_r_factor_obs: Option<NonNeg>,
    pub ls_r_fsqd_factor: Option<NonNeg>,
    pub ls_r_i_factor: Option<NonNeg>,
    pub ls_restrained_s_all: Option<NonNeg>,
    pub ls_restrained_s_gt: Option<NonNeg>,
    pub ls_restrained_s_obs: Option<NonNeg>,
    pub ls_shift_esd_max: Option<NonNeg>,
    pub ls_shift_esd_mean: Option<NonNeg>,
    pub ls_shift_su_max: Option<Ineq<NonNeg>>,
    pub ls_shift_su_max_lt: Option<NonNeg>,
    pub ls_shift_su_mean: Option<NonNeg>,
    pub ls_shift_su_mean_lt: Option<NonNeg>,
    pub ls_structure_factor_coef: Option<StructureFactorCoef>,
    pub ls_weighting_details: Option<CharString>,
    pub ls_weighting_scheme: Option<WeightingScheme>,
    pub ls_wr_factor_all: Option<NonNeg>,
    pub ls_wr_factor_gt: Option<NonNeg>,
    pub ls_wr_factor_obs: Option<NonNeg>,
    pub ls_wr_factor_ref: Option<NonNeg>,
    pub special_details: Option<CharString>,
    pub diff_density: Vec<RefineDiffDensity>,
}

#[derive(Debug, Clone, Default)]
pub struct RefineDiffDensity {
    pub max: Option<Numeric>,
    pub min: Option<Numeric>,
    pub rms: Option<Numeric>,
}

impl CifCategory for Refine {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.ls_abs_structure_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_abs_structure_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_abs_structure_flack {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_abs_structure_Flack".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_abs_structure_rogers {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_abs_structure_Rogers".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_d_res_high {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_d_res_high".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_d_res_low {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_d_res_low".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_extinction_coef {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_extinction_coef".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_extinction_expression {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_extinction_expression".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_extinction_method {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_extinction_method".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_f_calc_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_F_calc_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_f_calc_formula {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_F_calc_formula".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_f_calc_precision {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_F_calc_precision".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_ref {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_ref".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_hydrogen_treatment {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_hydrogen_treatment".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_matrix_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_matrix_type".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_constraints {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_constraints".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_parameters {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_parameters".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_reflns {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_reflns".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_restraints {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_restraints".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_factor_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_factor_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_factor_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_factor_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_factor_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_factor_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_fsqd_factor {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_Fsqd_factor".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_i_factor {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_I_factor".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_restrained_s_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_restrained_S_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_restrained_s_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_restrained_S_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_restrained_s_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_restrained_S_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_esd_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/esd_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_esd_mean {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/esd_mean".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_max_lt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_max_lt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_mean {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_mean".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_mean_lt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_mean_lt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_structure_factor_coef {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_structure_factor_coef".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_weighting_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_weighting_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_weighting_scheme {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_weighting_scheme".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_ref {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_ref".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_special_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if !self.diff_density.is_empty() {
            out.extend(diff_density_to_loop_members(&self.diff_density));
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
            match m {
                DataBlockMember::DataItems(DataItems::SingleValue(sv)) => {
                    let tag = sv.tag().as_ref().to_lowercase();
                    let value = sv.value();
                    match tag.as_str() {
                        "refine_ls_abs_structure_details" => {
                            match convert::read_char("refine_ls_abs_structure_details", value) {
                                Ok(v) => self.ls_abs_structure_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_abs_structure_flack" => {
                            match convert::read_flack("refine_ls_abs_structure_flack", value) {
                                Ok(v) => self.ls_abs_structure_flack = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_abs_structure_rogers" => {
                            match convert::read_rogers("refine_ls_abs_structure_rogers", value) {
                                Ok(v) => self.ls_abs_structure_rogers = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_d_res_high" => {
                            match convert::read_non_neg("refine_ls_d_res_high", value) {
                                Ok(v) => self.ls_d_res_high = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_d_res_low" => {
                            match convert::read_non_neg("refine_ls_d_res_low", value) {
                                Ok(v) => self.ls_d_res_low = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_extinction_coef" => {
                            match convert::read_numeric("refine_ls_extinction_coef", value) {
                                Ok(v) => self.ls_extinction_coef = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_extinction_expression" => {
                            match convert::read_char("refine_ls_extinction_expression", value) {
                                Ok(v) => self.ls_extinction_expression = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_extinction_method" => {
                            match convert::read_char("refine_ls_extinction_method", value) {
                                Ok(v) => self.ls_extinction_method = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_f_calc_details" => {
                            match convert::read_char("refine_ls_f_calc_details", value) {
                                Ok(v) => self.ls_f_calc_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_f_calc_formula" => {
                            match convert::read_char("refine_ls_f_calc_formula", value) {
                                Ok(v) => self.ls_f_calc_formula = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_f_calc_precision" => {
                            match convert::read_non_neg("refine_ls_f_calc_precision", value) {
                                Ok(v) => self.ls_f_calc_precision = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_all" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_all", value) {
                                Ok(v) => self.ls_goodness_of_fit_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_gt" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_gt", value) {
                                Ok(v) => self.ls_goodness_of_fit_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_obs" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_obs", value) {
                                Ok(v) => self.ls_goodness_of_fit_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_ref" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_ref", value) {
                                Ok(v) => self.ls_goodness_of_fit_ref = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_hydrogen_treatment" => {
                            match convert::read_closed_set(
                                "refine_ls_hydrogen_treatment",
                                value,
                                |c| HydrogenTreatment::try_from(c).ok(),
                                "HydrogenTreatment",
                            ) {
                                Ok(v) => self.ls_hydrogen_treatment = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_matrix_type" => {
                            match convert::read_closed_set(
                                "refine_ls_matrix_type",
                                value,
                                |c| MatrixType::try_from(c).ok(),
                                "MatrixType",
                            ) {
                                Ok(v) => self.ls_matrix_type = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_constraints" => {
                            match convert::read_unsigned("refine_ls_number_constraints", value) {
                                Ok(v) => self.ls_number_constraints = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_parameters" => {
                            match convert::read_unsigned("refine_ls_number_parameters", value) {
                                Ok(v) => self.ls_number_parameters = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_reflns" => {
                            match convert::read_unsigned("refine_ls_number_reflns", value) {
                                Ok(v) => self.ls_number_reflns = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_restraints" => {
                            match convert::read_unsigned("refine_ls_number_restraints", value) {
                                Ok(v) => self.ls_number_restraints = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_factor_all" => {
                            match convert::read_non_neg("refine_ls_r_factor_all", value) {
                                Ok(v) => self.ls_r_factor_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_factor_gt" => {
                            match convert::read_non_neg("refine_ls_r_factor_gt", value) {
                                Ok(v) => self.ls_r_factor_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_factor_obs" => {
                            match convert::read_non_neg("refine_ls_r_factor_obs", value) {
                                Ok(v) => self.ls_r_factor_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_fsqd_factor" => {
                            match convert::read_non_neg("refine_ls_r_fsqd_factor", value) {
                                Ok(v) => self.ls_r_fsqd_factor = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_i_factor" => {
                            match convert::read_non_neg("refine_ls_r_i_factor", value) {
                                Ok(v) => self.ls_r_i_factor = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_restrained_s_all" => {
                            match convert::read_non_neg("refine_ls_restrained_s_all", value) {
                                Ok(v) => self.ls_restrained_s_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_restrained_s_gt" => {
                            match convert::read_non_neg("refine_ls_restrained_s_gt", value) {
                                Ok(v) => self.ls_restrained_s_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_restrained_s_obs" => {
                            match convert::read_non_neg("refine_ls_restrained_s_obs", value) {
                                Ok(v) => self.ls_restrained_s_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/esd_max" => {
                            match convert::read_non_neg("refine_ls_shift/esd_max", value) {
                                Ok(v) => self.ls_shift_esd_max = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/esd_mean" => {
                            match convert::read_non_neg("refine_ls_shift/esd_mean", value) {
                                Ok(v) => self.ls_shift_esd_mean = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_max" => {
                            // A real-data file may record the value as a
                            // "<N" assertion (the COD). The assertion is a
                            // less-than statement. It records in the payload
                            // of this field (roadmap D12l, option B).
                            if let Value::CharString(c) = value {
                                let num = c.content().strip_prefix('<');
                                if let Some(n) = num.and_then(|t| t.parse::<f64>().ok()) {
                                    self.ls_shift_su_max = Some(Ineq::LessThan(
                                        NonNeg::from_numeric(Numeric::from(n)),
                                    ));
                                    continue;
                                }
                            }
                            match convert::read_non_neg("refine_ls_shift/su_max", value) {
                                Ok(v) => self.ls_shift_su_max = v.map(Ineq::Value),
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_max_lt" => {
                            match convert::read_non_neg("refine_ls_shift/su_max_lt", value) {
                                Ok(v) => self.ls_shift_su_max_lt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_mean" => {
                            match convert::read_non_neg("refine_ls_shift/su_mean", value) {
                                Ok(v) => self.ls_shift_su_mean = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_mean_lt" => {
                            match convert::read_non_neg("refine_ls_shift/su_mean_lt", value) {
                                Ok(v) => self.ls_shift_su_mean_lt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_structure_factor_coef" => {
                            match convert::read_closed_set(
                                "refine_ls_structure_factor_coef",
                                value,
                                |c| StructureFactorCoef::try_from(c).ok(),
                                "StructureFactorCoef",
                            ) {
                                Ok(v) => self.ls_structure_factor_coef = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_weighting_details" => {
                            match convert::read_char("refine_ls_weighting_details", value) {
                                Ok(v) => self.ls_weighting_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_weighting_scheme" => {
                            match convert::read_closed_set(
                                "refine_ls_weighting_scheme",
                                value,
                                |c| WeightingScheme::try_from(c).ok(),
                                "WeightingScheme",
                            ) {
                                Ok(v) => self.ls_weighting_scheme = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_all" => {
                            match convert::read_non_neg("refine_ls_wr_factor_all", value) {
                                Ok(v) => self.ls_wr_factor_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_gt" => {
                            match convert::read_non_neg("refine_ls_wr_factor_gt", value) {
                                Ok(v) => self.ls_wr_factor_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_obs" => {
                            match convert::read_non_neg("refine_ls_wr_factor_obs", value) {
                                Ok(v) => self.ls_wr_factor_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_ref" => {
                            match convert::read_non_neg("refine_ls_wr_factor_ref", value) {
                                Ok(v) => self.ls_wr_factor_ref = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_special_details" => {
                            match convert::read_char_text("refine_special_details", value) {
                                Ok(v) => self.special_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop belongs to this category only when it carries one
                    // of the loop-family column tags. A sibling loop of
                    // another category stays in the residue.
                    let diff_density = [
                        "refine_diff_density_max",
                        "refine_diff_density_min",
                        "refine_diff_density_rms",
                    ];
                    let is_ours = cols.iter().any(|c| {
                        let t = c.tag().as_ref().to_lowercase();
                        diff_density.contains(&t.as_str())
                    });
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        if cols.iter().any(|c| {
                            diff_density.contains(&c.tag().as_ref().to_lowercase().as_str())
                        }) {
                            self.diff_density = diff_density_rows_from_columns(cols, &mut errors);
                        }
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn diff_density_to_loop_members(rows: &[RefineDiffDensity]) -> Vec<DataBlockMember> {
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.max.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.max {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("refine_diff_density_max".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.min.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.min {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("refine_diff_density_min".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.rms.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.rms {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("refine_diff_density_rms".to_string()),
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

fn diff_density_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<RefineDiffDensity> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![RefineDiffDensity::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_diff_density_max")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_diff_density_max", v) {
                Ok(val) => rows[i].max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_diff_density_min")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_diff_density_min", v) {
                Ok(val) => rows[i].min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_diff_density_rms")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_diff_density_rms", v) {
                Ok(val) => rows[i].rms = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::parse_cif;
    use crystal_cif_fmt::{DataBlockMember, DataItems};

    /// The single member of the tag, with the value as recorded.
    fn members_for(value: &str) -> Vec<DataBlockMember> {
        let text = format!("data_test\n_refine_ls_shift/su_max {value}\n");
        let doc = parse_cif(&text).expect("the snippet parses");
        let block = &doc.data_blocks().expect("the block is present")[0];
        assert_eq!(block.members().len(), 1, "the snippet holds one member");
        block.members().to_vec()
    }

    /// The emitted member of the written value.
    fn emitted(member: &DataBlockMember) -> (&Tag, &Value) {
        match member {
            DataBlockMember::DataItems(DataItems::SingleValue(sv)) => {
                (sv.tag(), sv.value())
            }
            other => panic!("the emitted member is a single value: {other:?}"),
        }
    }

    #[test]
    fn the_less_than_assertion_records_in_the_payload() {
        let mut refine = Refine::default();
        let (residue, errors) = refine.from_members(&members_for("<0.001"));
        assert!(residue.is_empty(), "the tag is consumed: {residue:?}");
        assert!(errors.is_empty(), "no mismatch: {errors:?}");
        let expected = Ineq::LessThan(NonNeg::from_numeric(Numeric::from(0.001)));
        assert_eq!(refine.ls_shift_su_max, Some(expected));
        assert_eq!(refine.ls_shift_su_max_lt, None, "the plain assertion stays in the payload");
        let member = &refine.to_members()[0];
        let (tag, value) = emitted(member);
        assert_eq!(tag.as_ref(), "refine_ls_shift/su_max");
        assert_eq!(
            format!("{value}"),
            "<0.001",
            "the assertion re-emits as written"
        );
    }

    #[test]
    fn the_plain_value_records_in_the_payload() {
        let mut refine = Refine::default();
        let (residue, errors) = refine.from_members(&members_for("0.001"));
        assert!(residue.is_empty(), "the tag is consumed: {residue:?}");
        assert!(errors.is_empty(), "no mismatch: {errors:?}");
        let expected = Ineq::Value(NonNeg::from_numeric(Numeric::from(0.001)));
        assert_eq!(refine.ls_shift_su_max, Some(expected));
        assert_eq!(refine.ls_shift_su_max_lt, None);
    }
}
