//! The `diffrn_radiation` category of the typed document layer (roadmap D12, tier T5).
//!
//! The single-value category struct. One `Option` field per standard
//! dictionary item. The ranged items keep the entry newtype; the closed-set items
//! //! keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};
use {
    crate::data_dict::core_cif::diffrn_radiation::FilterEdge,
    crate::data_dict::core_cif::diffrn_radiation::Inhomogeneity,
    crate::data_dict::core_cif::diffrn_radiation::PolarisnNorm,
    crate::data_dict::core_cif::diffrn_radiation::PolarisnRatio,
    crate::data_dict::core_cif::diffrn_radiation::Probe,
    crate::data_dict::core_cif::diffrn_radiation::XraySymbol,
};

#[derive(Debug, Clone, Default)]
pub struct DiffrnRadiation {
    pub collimation: Option<CharString>,
    pub filter_edge: Option<FilterEdge>,
    pub inhomogeneity: Option<Inhomogeneity>,
    pub monochromator: Option<CharString>,
    pub polarisn_norm: Option<PolarisnNorm>,
    pub polarisn_ratio: Option<PolarisnRatio>,
    pub probe: Option<Probe>,
    pub r#type: Option<CharString>,
    pub xray_symbol: Option<XraySymbol>,
}
impl CifCategory for DiffrnRadiation {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.collimation {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_collimation".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.filter_edge {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_filter_edge".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.inhomogeneity {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_inhomogeneity".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.monochromator {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_monochromator".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.polarisn_norm {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_polarisn_norm".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.polarisn_ratio {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_polarisn_ratio".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.probe {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_probe".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.r#type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_type".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.xray_symbol {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("diffrn_radiation_xray_symbol".to_string()),
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
                    "diffrn_radiation_collimation" => {
                        match convert::read_char("diffrn_radiation_collimation", value) {
                            Ok(v) => self.collimation = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_filter_edge" => {
                        match convert::read_ranged_newtype(
                            "diffrn_radiation_filter_edge",
                            value,
                            FilterEdge::new,
                        ) {
                            Ok(v) => self.filter_edge = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_inhomogeneity" => {
                        match convert::read_ranged_newtype(
                            "diffrn_radiation_inhomogeneity",
                            value,
                            Inhomogeneity::new,
                        ) {
                            Ok(v) => self.inhomogeneity = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_monochromator" => {
                        match convert::read_char("diffrn_radiation_monochromator", value) {
                            Ok(v) => self.monochromator = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_polarisn_norm" => {
                        match convert::read_ranged_newtype(
                            "diffrn_radiation_polarisn_norm",
                            value,
                            PolarisnNorm::new,
                        ) {
                            Ok(v) => self.polarisn_norm = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_polarisn_ratio" => {
                        match convert::read_ranged_newtype(
                            "diffrn_radiation_polarisn_ratio",
                            value,
                            PolarisnRatio::new,
                        ) {
                            Ok(v) => self.polarisn_ratio = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_probe" => {
                        match convert::read_closed_set(
                            "diffrn_radiation_probe",
                            value,
                            Probe::parse,
                            "Probe",
                        ) {
                            Ok(v) => self.probe = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_type" => {
                        match convert::read_char("diffrn_radiation_type", value) {
                            Ok(v) => self.r#type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "diffrn_radiation_xray_symbol" => {
                        match convert::read_closed_set(
                            "diffrn_radiation_xray_symbol",
                            value,
                            XraySymbol::parse,
                            "XraySymbol",
                        ) {
                            Ok(v) => self.xray_symbol = v,
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
