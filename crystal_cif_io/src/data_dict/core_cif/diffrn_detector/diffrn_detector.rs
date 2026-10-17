//! The `diffrn_detector` category of the typed document layer (roadmap D12, tier T5).
//!
//! The single-value category struct. One `Option` field per standard
//! dictionary item. The ranged items keep the entry newtype.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};
use {
    crate::data_dict::core_cif::diffrn_detector::AreaResolMean,
    crate::data_dict::core_cif::diffrn_detector::DiffrnRadiationDetectorDtime,
    crate::data_dict::core_cif::diffrn_detector::Dtime,
};

#[derive(Debug, Clone, Default)]
/// The `diffrn_detector` category struct. One field per registry item.
pub struct DiffrnDetector {
    /// The `_diffrn_detector` item. The field holds the single value.
    ///
    /// The general class of the radiation detector.
    pub diffrn_detector: Option<CharString>,
    /// The `_diffrn_detector_area_resol_mean` item. The field holds the single value.
    ///
    /// The resolution of an area detector, in pixels/mm.
    /// Units: pixel/mm (pixel per millimetre).
    pub area_resol_mean: Option<AreaResolMean>,
    /// The `_diffrn_detector_details` item. The field holds the single value.
    ///
    /// A description of special aspects of the radiation detector.
    pub details: Option<CharString>,
    /// The `_diffrn_detector_dtime` item. The field holds the single value.
    ///
    /// The deadtime in microseconds of the detector used to measure the
    /// diffraction intensities.
    /// Units: \ms (microsecond).
    pub dtime: Option<Dtime>,
    /// The `_diffrn_radiation_detector` item. The field holds the single value.
    ///
    /// The detector used to measure the diffraction intensities.
    pub diffrn_radiation_detector: Option<CharString>,
    /// The `_diffrn_detector_type` item. The field holds the single value.
    ///
    /// The make, model or name of the detector device used.
    pub r#type: Option<CharString>,
    /// The `_diffrn_radiation_detector_dtime` item. The field holds the single value.
    ///
    /// The deadtime in microseconds of the detector used to measure the
    /// diffraction intensities.
    /// Units: \ms (microsecond).
    pub diffrn_radiation_detector_dtime: Option<DiffrnRadiationDetectorDtime>,
}
impl CifCategory for DiffrnDetector {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.diffrn_detector {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_detector".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.area_resol_mean {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_detector_area_resol_mean".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_detector_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.dtime {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_detector_dtime".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.diffrn_radiation_detector {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_detector".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.r#type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_detector_type".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.diffrn_radiation_detector_dtime {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_detector_dtime".to_string()),
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
                    "diffrn_detector" => match convert::read_char("diffrn_detector", value) {
                        Ok(v) => self.diffrn_detector = v,
                        Err(e) => errors.push(e),
                    },
                    "diffrn_detector_area_resol_mean" => {
                        match convert::read_ranged_newtype(
                            "diffrn_detector_area_resol_mean",
                            value,
                            AreaResolMean::new,
                        ) {
                            Ok(v) => self.area_resol_mean = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_detector_details" => {
                        match convert::read_char("diffrn_detector_details", value) {
                            Ok(v) => self.details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_detector_dtime" => {
                        match convert::read_ranged_newtype(
                            "diffrn_detector_dtime",
                            value,
                            Dtime::new,
                        ) {
                            Ok(v) => self.dtime = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_detector" => {
                        match convert::read_char("diffrn_radiation_detector", value) {
                            Ok(v) => self.diffrn_radiation_detector = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_detector_type" => {
                        match convert::read_char("diffrn_detector_type", value) {
                            Ok(v) => self.r#type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_detector_dtime" => {
                        match convert::read_ranged_newtype(
                            "diffrn_radiation_detector_dtime",
                            value,
                            DiffrnRadiationDetectorDtime::new,
                        ) {
                            Ok(v) => self.diffrn_radiation_detector_dtime = v,
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
