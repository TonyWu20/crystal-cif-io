//! The `chemical_formula` category of the typed document layer (roadmap D12).
//!
//! The `chemical_formula` category struct. One `Option` field per
//! registry item. `weight` and `weight_meas` keep the `FormulaWeight`
//! range guard.

use crystal_cif_fmt::grammar::{Tag, Value, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};
use {crate::data_dict::core_cif::chemical_formula::FormulaWeight};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct ChemicalFormula {
    pub analytical: Option<CharString>,
    pub iupac: Option<CharString>,
    pub moiety: Option<CharString>,
    pub structural: Option<CharString>,
    pub sum: Option<CharString>,
    pub weight: Option<FormulaWeight>,
    pub weight_meas: Option<FormulaWeight>,
}

impl CifCategory for ChemicalFormula {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.analytical {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("chemical_formula_analytical".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.iupac {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("chemical_formula_iupac".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.moiety {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("chemical_formula_moiety".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.structural {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("chemical_formula_structural".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.sum {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("chemical_formula_sum".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.weight {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("chemical_formula_weight".to_string()),
                Value::from(*v),
            )))));
        }
        if let Some(v) = &self.weight_meas {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("chemical_formula_weight_meas".to_string()),
                Value::from(*v),
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
                    "chemical_formula_analytical" => {
                        match convert::read_char("chemical_formula_analytical", value) {
                            Ok(v) => self.analytical = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_iupac" => {
                        match convert::read_char("chemical_formula_iupac", value) {
                            Ok(v) => self.iupac = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_moiety" => {
                        match convert::read_char("chemical_formula_moiety", value) {
                            Ok(v) => self.moiety = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_structural" => {
                        match convert::read_char("chemical_formula_structural", value) {
                            Ok(v) => self.structural = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_sum" => {
                        match convert::read_char("chemical_formula_sum", value) {
                            Ok(v) => self.sum = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_weight" => {
                        match convert::read_formula_weight("chemical_formula_weight", value) {
                            Ok(v) => self.weight = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_weight_meas" => {
                        match convert::read_formula_weight("chemical_formula_weight_meas", value) {
                            Ok(v) => self.weight_meas = v,
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
