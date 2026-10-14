//! The `diffrn` category of the typed document layer (roadmap D12, tier T5).
//!
//! The single-value struct `Diffrn` and the loop-row struct
//! `DiffrnRow`. `CifCategory` is on each group: the row group
//! emits one `MultiValues` loop and fills the rows from the columns.

use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData, LoopColumn, LoopColumns};
use {crate::data_dict::core_cif::diffrn::AmbientPressure, crate::data_dict::core_cif::diffrn::AmbientPressureGt, crate::data_dict::core_cif::diffrn::AmbientPressureLt, crate::data_dict::core_cif::diffrn::AmbientTemperature, crate::data_dict::core_cif::diffrn::AmbientTemperatureGt, crate::data_dict::core_cif::diffrn::AmbientTemperatureLt, crate::data_dict::core_cif::diffrn::MeasuredFractionThetaFull, crate::data_dict::core_cif::diffrn::MeasuredFractionThetaMax};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct Diffrn {
    pub ambient_environment: Option<CharString>,
    pub ambient_pressure: Option<AmbientPressure>,
    pub ambient_temperature: Option<AmbientTemperature>,
    pub crystal_treatment: Option<CharString>,
    pub measured_fraction_theta_full: Option<MeasuredFractionThetaFull>,
    pub measured_fraction_theta_max: Option<MeasuredFractionThetaMax>,
    pub special_details: Option<CharString>,
    pub symmetry_description: Option<CharString>,
}
impl CifCategory for Diffrn {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.ambient_environment {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_ambient_environment".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.ambient_pressure {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_ambient_pressure".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.ambient_temperature {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_ambient_temperature".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.crystal_treatment {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_crystal_treatment".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.measured_fraction_theta_full {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_measured_fraction_theta_full".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.measured_fraction_theta_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_measured_fraction_theta_max".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_special_details".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.symmetry_description {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("diffrn_symmetry_description".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        out
    }
    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "diffrn_ambient_environment" => {
                        match convert::read_char("diffrn_ambient_environment", value) {
                            Ok(v) => self.ambient_environment = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_ambient_pressure" => {
                        match convert::read_ranged_newtype("diffrn_ambient_pressure", value, AmbientPressure::new) {
                            Ok(v) => self.ambient_pressure = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_ambient_temperature" => {
                        match convert::read_ranged_newtype("diffrn_ambient_temperature", value, AmbientTemperature::new) {
                            Ok(v) => self.ambient_temperature = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_crystal_treatment" => {
                        match convert::read_char("diffrn_crystal_treatment", value) {
                            Ok(v) => self.crystal_treatment = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_measured_fraction_theta_full" => {
                        match convert::read_ranged_newtype("diffrn_measured_fraction_theta_full", value, MeasuredFractionThetaFull::new) {
                            Ok(v) => self.measured_fraction_theta_full = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_measured_fraction_theta_max" => {
                        match convert::read_ranged_newtype("diffrn_measured_fraction_theta_max", value, MeasuredFractionThetaMax::new) {
                            Ok(v) => self.measured_fraction_theta_max = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_special_details" => {
                        match convert::read_char("diffrn_special_details", value) {
                            Ok(v) => self.special_details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_symmetry_description" => {
                        match convert::read_char("diffrn_symmetry_description", value) {
                            Ok(v) => self.symmetry_description = v,
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
pub struct DiffrnRow {
    pub ambient_pressure_gt: Option<AmbientPressureGt>,
    pub ambient_pressure_lt: Option<AmbientPressureLt>,
    pub ambient_temperature_gt: Option<AmbientTemperatureGt>,
    pub ambient_temperature_lt: Option<AmbientTemperatureLt>,
}
impl CifCategory for Vec<DiffrnRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.ambient_pressure_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ambient_pressure_gt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_ambient_pressure_gt".to_string()), values));
        }
        if self.iter().any(|r| r.ambient_pressure_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ambient_pressure_lt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_ambient_pressure_lt".to_string()), values));
        }
        if self.iter().any(|r| r.ambient_temperature_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ambient_temperature_gt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_ambient_temperature_gt".to_string()), values));
        }
        if self.iter().any(|r| r.ambient_temperature_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.ambient_temperature_lt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("diffrn_ambient_temperature_lt".to_string()), values));
        }
        if columns.is_empty() {
            return Vec::new();
        }
        vec![DataBlockMember::DataItems(DataItems::MultiValues(LoopColumns::new(columns)))]
    }
    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_ambient_pressure_gt",
                    "diffrn_ambient_pressure_lt",
                    "diffrn_ambient_temperature_gt",
                    "diffrn_ambient_temperature_lt",];
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

fn merge_rows(rows: &mut Vec<DiffrnRow>, cols: &[LoopColumn], errors: &mut Vec<CifError>) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnRow::default());
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_ambient_pressure_gt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_ambient_pressure_gt", v, AmbientPressureGt::new) {
                Ok(val) => rows[i].ambient_pressure_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_ambient_pressure_lt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_ambient_pressure_lt", v, AmbientPressureLt::new) {
                Ok(val) => rows[i].ambient_pressure_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_ambient_temperature_gt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_ambient_temperature_gt", v, AmbientTemperatureGt::new) {
                Ok(val) => rows[i].ambient_temperature_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "diffrn_ambient_temperature_lt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_ambient_temperature_lt", v, AmbientTemperatureLt::new) {
                Ok(val) => rows[i].ambient_temperature_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
