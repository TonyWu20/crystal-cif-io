//! The `diffrn_reflns` category of the typed document layer (roadmap D12, tier T5).
//!
//! The `DiffrnReflns` struct owns the single-value items and the loop
//! rows (`rows`). One `CifCategory` impl: the scalar items, then
//! the `MultiValues` loop of the `DiffrnReflnsRow` columns.

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
/// The `diffrn_reflns` category struct. One field per registry item.
pub struct DiffrnReflns {
    /// The `_diffrn_reflns_av_R_equivalents` item. The field holds the single value.
    ///
    /// The residual [sum av|del(I)| / sum |av(I)|] for symmetry-equivalent
    /// reflections used to calculate the average intensity av(I). The
    /// av|del(I)| term is the average absolute difference between av(I) and
    /// the individual symmetry-equivalent intensities.
    pub av_r_equivalents: Option<AvREquivalents>,
    /// The `_diffrn_reflns_av_sigma_I_over_net_I` item. The mean of
    /// the intensity uncertainties, over the mean net intensity.
    pub av_sigma_i_over_net_i: Option<AvSigmaIOverNetI>,
    /// The `_diffrn_reflns_av_u_net_I_over_net_I` item. The mean of the
    /// net-intensity uncertainties, over the mean net intensity.
    pub av_unet_i_over_net_i: Option<AvUnetIOverNetI>,
    /// The `_diffrn_reflns_Laue_measured_fraction_full` item. The field holds the single value.
    ///
    /// Fraction of Laue unique reflections (symmetry-independent in the
    /// Laue group) measured out to the resolution given in
    /// _diffrn_reflns_resolution_full or _diffrn_reflns_theta_full. The
    /// Laue group always contains a centre of symmetry so that the
    /// reflection h,k,l is always equivalent to the reflection -h,-k,-l
    /// even in space groups without a centre of symmetry. This number
    /// should not be less than 0.95, since it represents the fraction of
    /// reflections measured in the part of the diffraction pattern that is
    /// essentially complete.
    pub laue_measured_fraction_full: Option<LaueMeasuredFractionFull>,
    /// The `_diffrn_reflns_Laue_measured_fraction_max` item. The field holds the single value.
    ///
    /// Fraction of Laue unique reflections (symmetry-independent in the
    /// Laue group) measured out to the resolution given in
    /// _diffrn_reflns_resolution_max or _diffrn_reflns_theta_max. The Laue
    /// group always contains a centre of symmetry so that the reflection
    /// h,k,l is always equivalent to the reflection -h,-k,-l even in space
    /// groups without a centre of symmetry.
    pub laue_measured_fraction_max: Option<LaueMeasuredFractionMax>,
    /// The `_diffrn_reflns_number` item. The field holds the single value.
    ///
    /// The total number of measured intensities, excluding reflections that
    /// are classed as systematically absent arising from translational
    /// symmetry in the crystal unit cell.
    pub number: Option<ReflnsNumber>,
    /// The `_diffrn_reflns_point_group_measured_fraction_full` item. The field holds the single value.
    ///
    /// Fraction of crystal point-group unique reflections (i.e.
    /// symmetry-independent in the crystal point group) measured out to the
    /// resolution given in _diffrn_reflns_resolution_full or
    /// _diffrn_reflns_theta_full. For space groups that do not contain a
    /// centre of symmetry the reflections h,k,l and -h,-k,-l are
    /// independent. This number should not be less than 0.95, since it
    /// represents the fraction of reflections measured in the part of the
    /// diffraction pattern that is essentially complete.
    pub point_group_measured_fraction_full: Option<PointGroupMeasuredFractionFull>,
    /// The `_diffrn_reflns_point_group_measured_fraction_max` item. The field holds the single value.
    ///
    /// Fraction of crystal point-group unique reflections (i.e.
    /// symmetry-independent in the crystal point group) measured out to the
    /// resolution given in _diffrn_reflns_resolution_max or
    /// _diffrn_reflns_theta_max. For space groups that do not contain a
    /// centre of symmetry the reflections h,k,l and -h,-k,-l are
    /// independent.
    pub point_group_measured_fraction_max: Option<PointGroupMeasuredFractionMax>,
    /// The `_diffrn_reflns_reduction_process` item. The field holds the single value.
    ///
    /// A description of the process used to reduce the intensities into
    /// structure-factor magnitudes.
    pub reduction_process: Option<CharString>,
    /// The `_diffrn_reflns_resolution_full` item. The field holds the single value.
    ///
    /// The resolution in reciprocal angstroms at which the measured
    /// reflection count is close to complete. The fraction of unique
    /// reflections measured out to this angle is given by
    /// _diffrn_measured_fraction_theta_full.
    /// Units: A^-1^ (reciprocal angstrom).
    pub resolution_full: Option<ResolutionFull>,
    /// The `_diffrn_reflns_resolution_max` item. The field holds the single value.
    ///
    /// Maximum resolution in reciprocal angstroms of the measured
    /// diffraction pattern. The fraction of unique reflections measured out
    /// to this angle is given by _diffrn_measured_fraction_theta_max
    /// Units: A^-1^ (reciprocal angstrom).
    pub resolution_max: Option<ResolutionMax>,
    /// The `_diffrn_reflns_theta_full` item. The field holds the single value.
    ///
    /// The theta angle (in degrees) at which the measured reflection count
    /// is close to complete. The fraction of unique reflections measured
    /// out to this angle is given by _diffrn_measured_fraction_theta_full.
    /// Units: deg (degree of arc).
    pub theta_full: Option<ThetaFull>,
    /// The `_diffrn_reflns_theta_max` item. The field holds the single value.
    ///
    /// Maximum theta angle in degrees for the measured intensities. The
    /// fraction of unique reflections measured out to this angle is given
    /// by _diffrn_measured_fraction_theta_max
    /// Units: deg (degree of arc).
    pub theta_max: Option<ThetaMax>,
    /// The `_diffrn_reflns_theta_min` item. The field holds the single value.
    ///
    /// Minimum theta angle in degrees for the measured intensities.
    /// Units: deg (degree of arc).
    pub theta_min: Option<ThetaMin>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<DiffrnReflnsRow>,
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
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
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
/// One loop row of the `diffrn_reflns` category: the per-observation values of the loop items.
pub struct DiffrnReflnsRow {
    /// The `_diffrn_reflns_limit_h_max` item. The field holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    pub limit_h_max: Option<Numeric>,
    /// The `_diffrn_reflns_limit_h_min` item. The field holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    pub limit_h_min: Option<Numeric>,
    /// The `_diffrn_reflns_limit_k_max` item. The field holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    pub limit_k_max: Option<Numeric>,
    /// The `_diffrn_reflns_limit_k_min` item. The field holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    pub limit_k_min: Option<Numeric>,
    /// The `_diffrn_reflns_limit_l_max` item. The field holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    pub limit_l_max: Option<Numeric>,
    /// The `_diffrn_reflns_limit_l_min` item. The field holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    pub limit_l_min: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_11` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_11: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_12` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_12: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_13` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_13: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_21` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_21: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_22` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_22: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_23` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_23: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_31` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_31: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_32` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_32: Option<Numeric>,
    /// The `_diffrn_reflns_transf_matrix_33` item. The field holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    pub transf_matrix_33: Option<Numeric>,
}
fn rows_to_members(rows: &[DiffrnReflnsRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.limit_h_max.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.limit_h_min.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.limit_k_max.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.limit_k_min.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.limit_l_max.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.limit_l_min.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_11.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_12.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_13.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_21.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_22.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_23.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_31.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_32.is_some()) {
        let values: Vec<Value> = rows
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
    if rows.iter().any(|r| r.transf_matrix_33.is_some()) {
        let values: Vec<Value> = rows
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
