//! The `diffrn_standards` category of the typed document layer (roadmap D12, tier T5).
//!
//! The single-value category struct. One `Option` field per standard
//! dictionary item. The ranged items keep the entry newtype.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};
use {
    crate::data_dict::core_cif::diffrn_standards::DecayPercent,
    crate::data_dict::core_cif::diffrn_standards::IntervalCount,
    crate::data_dict::core_cif::diffrn_standards::IntervalTime,
    crate::data_dict::core_cif::diffrn_standards::ScaleSigma,
    crate::data_dict::core_cif::diffrn_standards::ScaleU,
    crate::data_dict::core_cif::diffrn_standards::StandardsNumber,
};

#[derive(Debug, Clone, Default)]
/// The `diffrn_standards` category struct. One field per registry item.
pub struct DiffrnStandards {
    /// The `_diffrn_standards_decay_percent` item. The observed decay
    /// percent of the reflection intensities over the exposure.
    pub decay_percent: Option<DecayPercent>,
    /// The `_diffrn_standards_interval_count` item. The field holds the single value.
    ///
    /// The number of reflection intensities between the measurements of
    /// standard reflection intensities.
    pub interval_count: Option<IntervalCount>,
    /// The `_diffrn_standards_interval_time` item. The field holds the single value.
    ///
    /// The time in minutes between the measurements of standard reflection
    /// intensities.
    /// Units: min (minute).
    pub interval_time: Option<IntervalTime>,
    /// The `_diffrn_standards_number` item. The field holds the single value.
    ///
    /// The number of unique standard reflections used during the
    /// measurement of the diffraction intensities.
    pub number: Option<StandardsNumber>,
    /// The `_diffrn_standards_scale_sigma` item. The field holds the single value.
    ///
    /// The standard uncertainty (e.s.d.) of the individual mean standard
    /// scales applied to the intensity data.
    pub scale_sigma: Option<ScaleSigma>,
    /// The `_diffrn_standards_scale_u` item. The field holds the single value.
    ///
    /// The standard uncertainty of the individual mean standard scales
    /// applied to the intensity data.
    pub scale_u: Option<ScaleU>,
}
impl CifCategory for DiffrnStandards {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.decay_percent {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standards_decay_%".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.interval_count {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standards_interval_count".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.interval_time {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standards_interval_time".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standards_number".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.scale_sigma {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standards_scale_sigma".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.scale_u {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_standards_scale_u".to_string()),
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
                    "diffrn_standards_decay_%" => {
                        match convert::read_ranged_newtype(
                            "diffrn_standards_decay_%",
                            value,
                            DecayPercent::new,
                        ) {
                            Ok(v) => self.decay_percent = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_standards_interval_count" => {
                        match convert::read_ranged_newtype(
                            "diffrn_standards_interval_count",
                            value,
                            IntervalCount::new,
                        ) {
                            Ok(v) => self.interval_count = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_standards_interval_time" => {
                        match convert::read_ranged_newtype(
                            "diffrn_standards_interval_time",
                            value,
                            IntervalTime::new,
                        ) {
                            Ok(v) => self.interval_time = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_standards_number" => {
                        match convert::read_ranged_newtype(
                            "diffrn_standards_number",
                            value,
                            StandardsNumber::new,
                        ) {
                            Ok(v) => self.number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_standards_scale_sigma" => {
                        match convert::read_ranged_newtype(
                            "diffrn_standards_scale_sigma",
                            value,
                            ScaleSigma::new,
                        ) {
                            Ok(v) => self.scale_sigma = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_standards_scale_u" => {
                        match convert::read_ranged_newtype(
                            "diffrn_standards_scale_u",
                            value,
                            ScaleU::new,
                        ) {
                            Ok(v) => self.scale_u = v,
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
