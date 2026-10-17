//! The `diffrn_measurement` category of the typed document layer
//! (roadmap D12, tier T5).
//!
//! One `Option` field per standard-dictionary item.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `diffrn_measurement` category struct. One field per registry item.
pub struct DiffrnMeasurement {
    /// The `_diffrn_measurement_details` item. The field holds the single value.
    ///
    /// A description of special aspects of the intensity measurement.
    pub details: Option<CharString>,
    /// The `_diffrn_measurement_device` item. The field holds the single value.
    ///
    /// The general class of goniometer or device used to support and orient
    /// the specimen.
    pub device: Option<CharString>,
    /// The `_diffrn_measurement_device_details` item. The field holds the single value.
    ///
    /// A description of special aspects of the device used to measure the
    /// diffraction intensities.
    pub device_details: Option<CharString>,
    /// The `_diffrn_measurement_device_type` item. The field holds the single value.
    ///
    /// The make, model or name of the measurement device (goniometer) used.
    pub device_type: Option<CharString>,
    /// The `_diffrn_measurement_method` item. The field holds the single value.
    ///
    /// Method used to measure the intensities.
    pub method: Option<CharString>,
    /// The `_diffrn_measurement_specimen_support` item. The field holds the single value.
    ///
    /// The physical device used to support the crystal during data
    /// collection.
    pub specimen_support: Option<CharString>,
}

impl CifCategory for DiffrnMeasurement {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_measurement_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.device {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_measurement_device".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.device_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_measurement_device_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.device_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_measurement_device_type".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.method {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_measurement_method".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.specimen_support {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_measurement_specimen_support".to_string()),
                    Value::CharString(v.clone()),
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
                    "diffrn_measurement_details" => {
                        match convert::read_char("diffrn_measurement_details", value) {
                            Ok(v) => self.details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_measurement_device" => {
                        match convert::read_char("diffrn_measurement_device", value) {
                            Ok(v) => self.device = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_measurement_device_details" => {
                        match convert::read_char("diffrn_measurement_device_details", value) {
                            Ok(v) => self.device_details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_measurement_device_type" => {
                        match convert::read_char("diffrn_measurement_device_type", value) {
                            Ok(v) => self.device_type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_measurement_method" => {
                        match convert::read_char("diffrn_measurement_method", value) {
                            Ok(v) => self.method = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_measurement_specimen_support" => {
                        match convert::read_char("diffrn_measurement_specimen_support", value) {
                            Ok(v) => self.specimen_support = v,
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
