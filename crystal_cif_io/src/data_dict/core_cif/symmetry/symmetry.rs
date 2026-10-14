//! The `symmetry` category of the typed document layer (roadmap D12).
//!
//! The `symmetry` category struct. One `Option` field per registry
//! item. `int_tables_number` keeps the `IntTablesNumber` newtype;
//! `cell_setting` keeps the closed set.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::symmetry::{CellSetting, IntTablesNumber};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct Symmetry {
    pub cell_setting: Option<CellSetting>,
    pub int_tables_number: Option<IntTablesNumber>,
    pub space_group_name_h_m: Option<CharString>,
    pub space_group_name_hall: Option<CharString>,
}

impl CifCategory for Symmetry {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.cell_setting {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("symmetry_cell_setting".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.int_tables_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("symmetry_Int_Tables_number".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.space_group_name_h_m {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("symmetry_space_group_name_H-M".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.space_group_name_hall {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("symmetry_space_group_name_Hall".to_string()),
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
                    "symmetry_cell_setting" => {
                        match convert::read_closed_set(
                            "symmetry_cell_setting",
                            value,
                            |c| CellSetting::try_from(c).ok(),
                            "CellSetting",
                        ) {
                            Ok(v) => self.cell_setting = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "symmetry_int_tables_number" => {
                        match convert::read_int_tables_number("symmetry_int_tables_number", value) {
                            Ok(v) => self.int_tables_number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "symmetry_space_group_name_h-m" => {
                        match convert::read_char("symmetry_space_group_name_h-m", value) {
                            Ok(v) => self.space_group_name_h_m = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "symmetry_space_group_name_hall" => {
                        match convert::read_char("symmetry_space_group_name_hall", value) {
                            Ok(v) => self.space_group_name_hall = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    _ => residue.push(m.clone()),
                }
            } else {
                residue.push(m.clone())
            }
        }
        (residue, errors)
    }
}
