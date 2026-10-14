//! The `diffrn_source` category of the typed document layer (roadmap D12, tier T5).
//!
//! The single-value category struct. One `Option` field per standard
//! dictionary item. The ranged items keep the entry newtype; the closed-set items
//! //! keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};
use {
    crate::data_dict::core_cif::diffrn_source::Current,
    crate::data_dict::core_cif::diffrn_source::Power,
    crate::data_dict::core_cif::diffrn_source::TakeOffAngle,
    crate::data_dict::core_cif::diffrn_source::Target,
    crate::data_dict::core_cif::diffrn_source::Voltage,
};

#[derive(Debug, Clone, Default)]
pub struct DiffrnSource {
    pub current: Option<Current>,
    pub details: Option<CharString>,
    pub diffrn_radiation_source: Option<CharString>,
    pub diffrn_source: Option<CharString>,
    pub power: Option<Power>,
    pub size: Option<CharString>,
    pub take_off_angle: Option<TakeOffAngle>,
    pub target: Option<Target>,
    pub r#type: Option<CharString>,
    pub voltage: Option<Voltage>,
}
impl CifCategory for DiffrnSource {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.current {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_current".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.diffrn_radiation_source {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_source".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.diffrn_source {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.power {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_power".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.size {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_size".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.take_off_angle {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_take-off_angle".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.target {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_target".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.r#type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_type".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.voltage {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_source_voltage".to_string()),
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
                    "diffrn_source_current" => {
                        match convert::read_ranged_newtype(
                            "diffrn_source_current",
                            value,
                            Current::new,
                        ) {
                            Ok(v) => self.current = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_source_details" => {
                        match convert::read_char("diffrn_source_details", value) {
                            Ok(v) => self.details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_source" => {
                        match convert::read_char("diffrn_radiation_source", value) {
                            Ok(v) => self.diffrn_radiation_source = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_source" => match convert::read_char("diffrn_source", value) {
                        Ok(v) => self.diffrn_source = v,
                        Err(e) => errors.push(e),
                    },
                    "diffrn_source_power" => {
                        match convert::read_ranged_newtype("diffrn_source_power", value, Power::new)
                        {
                            Ok(v) => self.power = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_source_size" => match convert::read_char("diffrn_source_size", value) {
                        Ok(v) => self.size = v,
                        Err(e) => errors.push(e),
                    },
                    "diffrn_source_take-off_angle" => {
                        match convert::read_ranged_newtype(
                            "diffrn_source_take-off_angle",
                            value,
                            TakeOffAngle::new,
                        ) {
                            Ok(v) => self.take_off_angle = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_source_target" => {
                        match convert::read_closed_set(
                            "diffrn_source_target",
                            value,
                            Target::parse,
                            "Target",
                        ) {
                            Ok(v) => self.target = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_source_type" => match convert::read_char("diffrn_source_type", value) {
                        Ok(v) => self.r#type = v,
                        Err(e) => errors.push(e),
                    },
                    "diffrn_source_voltage" => {
                        match convert::read_ranged_newtype(
                            "diffrn_source_voltage",
                            value,
                            Voltage::new,
                        ) {
                            Ok(v) => self.voltage = v,
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
