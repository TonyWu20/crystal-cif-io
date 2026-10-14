//! The `chemical` category of the typed document layer (roadmap D12).
//!
//! The `Chemical` single-value struct (the compound names, the
//! temperatures, and the enantiomeric-excess items) and the
//! `ChemicalRow` loop row (the temperature range companions).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::chemical::{
    AbsoluteConfiguration, Enantioexcess, EnantioexcessBulkTechnique,
    EnantioexcessCrystalTechnique, Temperature,
};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct Chemical {
    pub absolute_configuration: Option<AbsoluteConfiguration>,
    pub compound_source: Option<CharString>,
    pub enantioexcess_bulk: Option<Enantioexcess>,
    pub enantioexcess_bulk_technique: Option<EnantioexcessBulkTechnique>,
    pub enantioexcess_crystal: Option<Enantioexcess>,
    pub enantioexcess_crystal_technique: Option<EnantioexcessCrystalTechnique>,
    pub identifier_inchi: Option<CharString>,
    pub identifier_inchi_key: Option<CharString>,
    pub identifier_inchi_version: Option<CharString>,
    pub melting_point: Option<Temperature>,
    pub name_common: Option<CharString>,
    pub name_mineral: Option<CharString>,
    pub name_structure_type: Option<CharString>,
    pub name_systematic: Option<CharString>,
    pub optical_rotation: Option<CharString>,
    pub properties_biological: Option<CharString>,
    pub properties_physical: Option<CharString>,
    pub temperature_decomposition: Option<Temperature>,
    pub temperature_sublimation: Option<Temperature>,
}

impl CifCategory for Chemical {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.absolute_configuration {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_absolute_configuration".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.compound_source {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_compound_source".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_bulk {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_bulk".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_bulk_technique {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_bulk_technique".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_crystal {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_crystal".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_crystal_technique {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_crystal_technique".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.identifier_inchi {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_identifier_inchi".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.identifier_inchi_key {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_identifier_inchi_key".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.identifier_inchi_version {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_identifier_inchi_version".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.melting_point {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_melting_point".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.name_common {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_common".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_mineral {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_mineral".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_structure_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_structure_type".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_systematic {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_systematic".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.optical_rotation {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_optical_rotation".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.properties_biological {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_properties_biological".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.properties_physical {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_properties_physical".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.temperature_decomposition {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_temperature_decomposition".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.temperature_sublimation {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_temperature_sublimation".to_string()),
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
                    "chemical_absolute_configuration" => {
                        match convert::read_closed_set(
                            "chemical_absolute_configuration",
                            value,
                            AbsoluteConfiguration::parse,
                            "AbsoluteConfiguration",
                        ) {
                            Ok(v) => self.absolute_configuration = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_compound_source" => {
                        match convert::read_char("chemical_compound_source", value) {
                            Ok(v) => self.compound_source = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_enantioexcess_bulk" => {
                        match convert::read_enantioexcess("chemical_enantioexcess_bulk", value) {
                            Ok(v) => self.enantioexcess_bulk = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_enantioexcess_bulk_technique" => {
                        match convert::read_closed_set(
                            "chemical_enantioexcess_bulk_technique",
                            value,
                            EnantioexcessBulkTechnique::parse,
                            "EnantioexcessBulkTechnique",
                        ) {
                            Ok(v) => self.enantioexcess_bulk_technique = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_enantioexcess_crystal" => {
                        match convert::read_enantioexcess("chemical_enantioexcess_crystal", value) {
                            Ok(v) => self.enantioexcess_crystal = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_enantioexcess_crystal_technique" => {
                        match convert::read_closed_set(
                            "chemical_enantioexcess_crystal_technique",
                            value,
                            EnantioexcessCrystalTechnique::parse,
                            "EnantioexcessCrystalTechnique",
                        ) {
                            Ok(v) => self.enantioexcess_crystal_technique = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_identifier_inchi" => {
                        match convert::read_char("chemical_identifier_inchi", value) {
                            Ok(v) => self.identifier_inchi = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_identifier_inchi_key" => {
                        match convert::read_char("chemical_identifier_inchi_key", value) {
                            Ok(v) => self.identifier_inchi_key = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_identifier_inchi_version" => {
                        match convert::read_char("chemical_identifier_inchi_version", value) {
                            Ok(v) => self.identifier_inchi_version = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_melting_point" => {
                        match convert::read_temperature("chemical_melting_point", value) {
                            Ok(v) => self.melting_point = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_name_common" => {
                        match convert::read_char("chemical_name_common", value) {
                            Ok(v) => self.name_common = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_name_mineral" => {
                        match convert::read_char("chemical_name_mineral", value) {
                            Ok(v) => self.name_mineral = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_name_structure_type" => {
                        match convert::read_char("chemical_name_structure_type", value) {
                            Ok(v) => self.name_structure_type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_name_systematic" => {
                        match convert::read_char_text("chemical_name_systematic", value) {
                            Ok(v) => self.name_systematic = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_optical_rotation" => {
                        match convert::read_char("chemical_optical_rotation", value) {
                            Ok(v) => self.optical_rotation = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_properties_biological" => {
                        match convert::read_char("chemical_properties_biological", value) {
                            Ok(v) => self.properties_biological = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_properties_physical" => {
                        match convert::read_char("chemical_properties_physical", value) {
                            Ok(v) => self.properties_physical = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_temperature_decomposition" => {
                        match convert::read_temperature("chemical_temperature_decomposition", value)
                        {
                            Ok(v) => self.temperature_decomposition = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_temperature_sublimation" => {
                        match convert::read_temperature("chemical_temperature_sublimation", value) {
                            Ok(v) => self.temperature_sublimation = v,
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
pub struct ChemicalRow {
    pub melting_point_gt: Option<Temperature>,
    pub melting_point_lt: Option<Temperature>,
    pub temperature_decomposition_gt: Option<Temperature>,
    pub temperature_decomposition_lt: Option<Temperature>,
    pub temperature_sublimation_gt: Option<Temperature>,
    pub temperature_sublimation_lt: Option<Temperature>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "chemical_melting_point_gt",
    "chemical_melting_point_lt",
    "chemical_temperature_decomposition_gt",
    "chemical_temperature_decomposition_lt",
    "chemical_temperature_sublimation_gt",
    "chemical_temperature_sublimation_lt",
];

impl CifCategory for Vec<ChemicalRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.melting_point_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.melting_point_gt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_melting_point_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.melting_point_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.melting_point_lt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_melting_point_lt".to_string()),
                values,
            ));
        }
        if self
            .iter()
            .any(|r| r.temperature_decomposition_gt.is_some())
        {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.temperature_decomposition_gt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_temperature_decomposition_gt".to_string()),
                values,
            ));
        }
        if self
            .iter()
            .any(|r| r.temperature_decomposition_lt.is_some())
        {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.temperature_decomposition_lt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_temperature_decomposition_lt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.temperature_sublimation_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.temperature_sublimation_gt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_temperature_sublimation_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.temperature_sublimation_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.temperature_sublimation_lt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("chemical_temperature_sublimation_lt".to_string()),
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

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    let is_ours = cols
                        .iter()
                        .any(|c| LOOP_TAGS.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ChemicalRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ChemicalRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_melting_point_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_melting_point_gt", v) {
                Ok(val) => rows[i].melting_point_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_melting_point_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_melting_point_lt", v) {
                Ok(val) => rows[i].melting_point_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_decomposition_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_decomposition_gt", v) {
                Ok(val) => rows[i].temperature_decomposition_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_decomposition_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_decomposition_lt", v) {
                Ok(val) => rows[i].temperature_decomposition_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_sublimation_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_sublimation_gt", v) {
                Ok(val) => rows[i].temperature_sublimation_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_sublimation_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_sublimation_lt", v) {
                Ok(val) => rows[i].temperature_sublimation_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
