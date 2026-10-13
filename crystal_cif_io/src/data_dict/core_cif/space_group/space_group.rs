//! The `space_group` category of the typed document layer (roadmap D12).
//!
//! The `space_group` category struct. One `Option` field per registry
//! item. `it_number` keeps the clamped `ITNumber` newtype and
//! `crystal_system` the `CrystalSystemCif` closed set.

use crystal_cif_fmt::grammar::{Tag, Value, Numeric, CharString, Number, UnquotedString};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};
use {crate::data_dict::core_cif::space_group::CrystalSystemCif, crate::data_dict::core_cif::space_group::ITNumber};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct SpaceGroup {
    pub bravais_type: Option<CharString>,
    pub centring_type: Option<CharString>,
    pub crystal_system: Option<CrystalSystemCif>,
    pub id: Option<Numeric>,
    pub it_number: Option<ITNumber>,
    pub it_coordinate_system_code: Option<CharString>,
    pub laue_class: Option<CharString>,
    pub multiplicity: Option<Numeric>,
    pub name_h_m_alt: Option<CharString>,
    pub name_hall: Option<CharString>,
    pub name_h_m_alt_description: Option<CharString>,
    pub name_h_m_ref: Option<CharString>,
    pub name_schoenflies: Option<CharString>,
    pub patterson_name_h_m: Option<CharString>,
    pub point_group_h_m: Option<CharString>,
    pub reference_setting: Option<CharString>,
    pub transform_pp_abc: Option<CharString>,
    pub transform_qq_xyz: Option<CharString>,
    pub symmetry_name_h_m: Option<CharString>,
}

impl CifCategory for SpaceGroup {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.bravais_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_bravais_type".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.centring_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_centring_type".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.crystal_system {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_crystal_system".to_string()),
                Value::CharString(CharString::Unquoted(UnquotedString::new(v.to_string()))),
            )))));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_id".to_string()),
                Value::Numeric(*v),
            )))));
        }
        if let Some(v) = &self.it_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_IT_number".to_string()),
                Value::Numeric(Numeric::new(Number::from(u64::from(*v) as u32), None)),
            )))));
        }
        if let Some(v) = &self.it_coordinate_system_code {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_it_coordinate_system_code".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.laue_class {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_laue_class".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.multiplicity {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_multiplicity".to_string()),
                Value::Numeric(*v),
            )))));
        }
        if let Some(v) = &self.name_h_m_alt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_name_H-M_alt".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.name_hall {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_name_Hall".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.name_h_m_alt_description {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_name_h_m_alt_description".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.name_h_m_ref {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_name_h_m_ref".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.name_schoenflies {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_name_schoenflies".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.patterson_name_h_m {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_patterson_name_h_m".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.point_group_h_m {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_point_group_h_m".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.reference_setting {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_reference_setting".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.transform_pp_abc {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_transform_pp_abc".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.transform_qq_xyz {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("space_group_transform_qq_xyz".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.symmetry_name_h_m {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("symmetry.space_group_name_h-m".to_string()),
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
                    "space_group_bravais_type" => {
                        match convert::read_char("space_group_bravais_type", value) {
                            Ok(v) => self.bravais_type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_centring_type" => {
                        match convert::read_char("space_group_centring_type", value) {
                            Ok(v) => self.centring_type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_crystal_system" => {
                        match convert::read_closed_set("space_group_crystal_system", value, |c| CrystalSystemCif::try_from(c).ok(), "CrystalSystemCif") {
                            Ok(v) => self.crystal_system = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_id" => {
                        match convert::read_numeric("space_group_id", value) {
                            Ok(v) => self.id = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_it_number" | "symmetry_int_tables_number" => {
                        match convert::read_it_number("space_group_it_number", value) {
                            Ok(v) => self.it_number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_it_coordinate_system_code" => {
                        match convert::read_char("space_group_it_coordinate_system_code", value) {
                            Ok(v) => self.it_coordinate_system_code = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_laue_class" => {
                        match convert::read_char("space_group_laue_class", value) {
                            Ok(v) => self.laue_class = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_multiplicity" => {
                        match convert::read_numeric("space_group_multiplicity", value) {
                            Ok(v) => self.multiplicity = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_h-m_alt" => {
                        match convert::read_char("space_group_name_h-m_alt", value) {
                            Ok(v) => self.name_h_m_alt = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_hall" | "symmetry_space_group_name_hall" => {
                        match convert::read_char("space_group_name_hall", value) {
                            Ok(v) => self.name_hall = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_h_m_alt_description" => {
                        match convert::read_char("space_group_name_h_m_alt_description", value) {
                            Ok(v) => self.name_h_m_alt_description = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_h_m_ref" => {
                        match convert::read_char("space_group_name_h_m_ref", value) {
                            Ok(v) => self.name_h_m_ref = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_schoenflies" => {
                        match convert::read_char("space_group_name_schoenflies", value) {
                            Ok(v) => self.name_schoenflies = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_patterson_name_h_m" => {
                        match convert::read_char("space_group_patterson_name_h_m", value) {
                            Ok(v) => self.patterson_name_h_m = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_point_group_h_m" => {
                        match convert::read_char("space_group_point_group_h_m", value) {
                            Ok(v) => self.point_group_h_m = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_reference_setting" => {
                        match convert::read_char("space_group_reference_setting", value) {
                            Ok(v) => self.reference_setting = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_transform_pp_abc" => {
                        match convert::read_char("space_group_transform_pp_abc", value) {
                            Ok(v) => self.transform_pp_abc = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_transform_qq_xyz" => {
                        match convert::read_char("space_group_transform_qq_xyz", value) {
                            Ok(v) => self.transform_qq_xyz = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "symmetry.space_group_name_h-m" | "symmetry_space_group_name_h-m" => {
                        match convert::read_char("symmetry_space_group_name_h-m", value) {
                            Ok(v) => self.symmetry_name_h_m = v,
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
