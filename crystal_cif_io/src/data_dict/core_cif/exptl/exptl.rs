//! The `exptl` category of the typed document layer (roadmap D12).
//!
//! The `exptl` category struct: the seven single-value items and
//! the `exptl_absorpt_correction_T` loop family. `CifCategory` is on the
//! `Exptl` group. The loop keeps one row struct per family.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::exptl::{
    AbsorptCoefficient, AbsorptCorrectionType, CrystalCount, TransmissionFactor,
};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `exptl` category struct. One field per registry item.
pub struct Exptl {
    /// The `_exptl_absorpt_coefficient_mu` item. The field holds the single value.
    ///
    /// The absorption coefficient mu in reciprocal millimetres calculated
    /// from the atomic content of the cell, the density and the radiation
    /// wavelength.
    /// Units: mm^-1^ (reciprocal millimetre).
    pub absorpt_coefficient_mu: Option<AbsorptCoefficient>,
    /// The `_exptl_absorpt_correction_type` item. The field holds the single value.
    ///
    /// The absorption-correction type and method. The value 'empirical'
    /// should NOT be used unless more detailed information is not
    /// available.
    pub absorpt_correction_type: Option<AbsorptCorrectionType>,
    /// The `_exptl_absorpt_process_details` item. The field holds the single value.
    ///
    /// Description of the absorption process applied to the intensities. A
    /// literature reference should be supplied for psi-scan techniques.
    pub absorpt_process_details: Option<CharString>,
    /// The `_exptl_crystals_number` item. The field holds the single value.
    ///
    /// The total number of crystals used for the measurement of
    /// intensities.
    pub crystals_number: Option<CrystalCount>,
    /// The `_exptl_special_details` item. The field holds the single value.
    ///
    /// Any special information about the experimental work prior to the
    /// intensity measurements. See also _exptl_crystal_preparation.
    pub special_details: Option<CharString>,
    /// The `_exptl_transmission_factor_max` item. The field holds the single value.
    ///
    /// The calculated maximum value of the transmission factor for the
    /// specimen. Its value does not include the effects of absorption in
    /// the specimen mount. The presence of this item does not imply that
    /// the structure factors have been corrected for absorption. The
    /// applied correction should be given by
    /// _exptl_absorpt_correction_T_max.
    pub transmission_factor_max: Option<TransmissionFactor>,
    /// The `_exptl_transmission_factor_min` item. The field holds the single value.
    ///
    /// The calculated minimum value of the transmission factor for the
    /// specimen. Its value does not include the effects of absorption in
    /// the specimen mount. The presence of this item does not imply that
    /// the structure factors have been corrected for absorption. The
    /// applied correction should be given by
    /// _exptl_absorpt_correction_T_min.
    pub transmission_factor_min: Option<TransmissionFactor>,
    /// The rows of the `_exptl_absorpt_correction_T` loop family.
    pub absorpt_correction_t: Vec<ExptlAbsorptCorrectionT>,
}

/// One loop row of the `_exptl_absorpt_correction_T` loop family:
/// the correction factors at the two measurement limits.
#[derive(Debug, Clone, Default)]
pub struct ExptlAbsorptCorrectionT {
    /// The `_exptl_absorpt_correction_T_max` item. The field holds the column values, one per loop row.
    ///
    /// The maximum and minimum transmission factors applied to the
    /// diffraction pattern measured in this experiment. These factors are
    /// also referred to as the absorption correction A or 1/A*. As this
    /// value is the one that is applied to the measured intensities, it
    /// includes the correction for absorption by the specimen mount and
    /// diffractometer as well as by the specimen itself.
    pub max: Option<Numeric>,
    /// The `_exptl_absorpt_correction_T_min` item. The field holds the column values, one per loop row.
    ///
    /// The maximum and minimum transmission factors applied to the
    /// diffraction pattern measured in this experiment. These factors are
    /// also referred to as the absorption correction A or 1/A*. As this
    /// value is the one that is applied to the measured intensities, it
    /// includes the correction for absorption by the specimen mount and
    /// diffractometer as well as by the specimen itself.
    pub min: Option<Numeric>,
}

impl CifCategory for Exptl {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.absorpt_coefficient_mu {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("exptl_absorpt_coefficient_mu".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.absorpt_correction_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("exptl_absorpt_correction_type".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.absorpt_process_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("exptl_absorpt_process_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.crystals_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("exptl_crystals_number".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("exptl_special_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.transmission_factor_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("exptl_transmission_factor_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.transmission_factor_min {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("exptl_transmission_factor_min".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if !self.absorpt_correction_t.is_empty() {
            out.extend(absorpt_correction_t_to_loop_members(
                &self.absorpt_correction_t,
            ));
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
                        "exptl_absorpt_coefficient_mu" => {
                            match convert::read_absorpt_coefficient(
                                "exptl_absorpt_coefficient_mu",
                                value,
                            ) {
                                Ok(v) => self.absorpt_coefficient_mu = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "exptl_absorpt_correction_type" => {
                            match convert::read_closed_set(
                                "exptl_absorpt_correction_type",
                                value,
                                |c| AbsorptCorrectionType::try_from(c).ok(),
                                "AbsorptCorrectionType",
                            ) {
                                Ok(v) => self.absorpt_correction_type = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "exptl_absorpt_process_details" => {
                            match convert::read_char("exptl_absorpt_process_details", value) {
                                Ok(v) => self.absorpt_process_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "exptl_crystals_number" => {
                            match convert::read_crystal_count("exptl_crystals_number", value) {
                                Ok(v) => self.crystals_number = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "exptl_special_details" => {
                            match convert::read_char("exptl_special_details", value) {
                                Ok(v) => self.special_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "exptl_transmission_factor_max" => {
                            match convert::read_transmission_factor(
                                "exptl_transmission_factor_max",
                                value,
                            ) {
                                Ok(v) => self.transmission_factor_max = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "exptl_transmission_factor_min" => {
                            match convert::read_transmission_factor(
                                "exptl_transmission_factor_min",
                                value,
                            ) {
                                Ok(v) => self.transmission_factor_min = v,
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
                    let absorpt_correction_t = [
                        "exptl_absorpt_correction_t_max",
                        "exptl_absorpt_correction_t_min",
                    ];
                    let is_ours = cols.iter().any(|c| {
                        let t = c.tag().as_ref().to_lowercase();
                        absorpt_correction_t.contains(&t.as_str())
                    });
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        if cols.iter().any(|c| {
                            absorpt_correction_t.contains(&c.tag().as_ref().to_lowercase().as_str())
                        }) {
                            self.absorpt_correction_t =
                                absorpt_correction_t_rows_from_columns(cols, &mut errors);
                        }
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn absorpt_correction_t_to_loop_members(rows: &[ExptlAbsorptCorrectionT]) -> Vec<DataBlockMember> {
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
            Tag::new("exptl_absorpt_correction_T_max".to_string()),
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
            Tag::new("exptl_absorpt_correction_T_min".to_string()),
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

fn absorpt_correction_t_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<ExptlAbsorptCorrectionT> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ExptlAbsorptCorrectionT::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_absorpt_correction_t_max")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_absorpt_correction_t_max", v) {
                Ok(val) => rows[i].max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_absorpt_correction_t_min")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_absorpt_correction_t_min", v) {
                Ok(val) => rows[i].min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
