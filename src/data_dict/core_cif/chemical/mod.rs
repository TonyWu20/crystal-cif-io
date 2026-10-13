//! The `chemical` category (25 items).
//!
//! Single-value items describe the compound: names, identifiers,
//! the source, the melting and decomposition temperatures, and the
//! enantiomeric excess with its technique code. The loop items are
//! the `_gt` and `_lt` companions of the temperature items, which
//! the dictionary marks as loop columns.

mod codes;
mod ranges;

pub use codes::{AbsoluteConfiguration, EnantioexcessBulkTechnique, EnantioexcessCrystalTechnique};
pub use ranges::{Enantioexcess, Temperature};

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

/// Single-value items of the `chemical` category (19 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalItem {
    Absolute_configuration(AbsoluteConfiguration),
    Compound_source(CharString),
    Enantioexcess_bulk(Enantioexcess),
    Enantioexcess_bulk_technique(EnantioexcessBulkTechnique),
    Enantioexcess_crystal(Enantioexcess),
    Enantioexcess_crystal_technique(EnantioexcessCrystalTechnique),
    Identifier_inchi(CharString),
    Identifier_inchi_key(CharString),
    Identifier_inchi_version(CharString),
    Melting_point(Temperature),
    Name_common(CharString),
    Name_mineral(CharString),
    Name_structure_type(CharString),
    Name_systematic(CharString),
    Optical_rotation(CharString),
    Properties_biological(CharString),
    Properties_physical(CharString),
    Temperature_decomposition(Temperature),
    Temperature_sublimation(Temperature),
}

impl CifTerm for ChemicalItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ChemicalItem::Absolute_configuration(_) => "absolute_configuration",
            ChemicalItem::Compound_source(_) => "compound_source",
            ChemicalItem::Enantioexcess_bulk(_) => "enantioexcess_bulk",
            ChemicalItem::Enantioexcess_bulk_technique(_) => "enantioexcess_bulk_technique",
            ChemicalItem::Enantioexcess_crystal(_) => "enantioexcess_crystal",
            ChemicalItem::Enantioexcess_crystal_technique(_) => "enantioexcess_crystal_technique",
            ChemicalItem::Identifier_inchi(_) => "identifier_inchi",
            ChemicalItem::Identifier_inchi_key(_) => "identifier_inchi_key",
            ChemicalItem::Identifier_inchi_version(_) => "identifier_inchi_version",
            ChemicalItem::Melting_point(_) => "melting_point",
            ChemicalItem::Name_common(_) => "name_common",
            ChemicalItem::Name_mineral(_) => "name_mineral",
            ChemicalItem::Name_structure_type(_) => "name_structure_type",
            ChemicalItem::Name_systematic(_) => "name_systematic",
            ChemicalItem::Optical_rotation(_) => "optical_rotation",
            ChemicalItem::Properties_biological(_) => "properties_biological",
            ChemicalItem::Properties_physical(_) => "properties_physical",
            ChemicalItem::Temperature_decomposition(_) => "temperature_decomposition",
            ChemicalItem::Temperature_sublimation(_) => "temperature_sublimation",
        };
        Tag::new(format!("chemical_{suffix}"))
    }
}

impl SingleValueTerm for ChemicalItem {
    fn value(&self) -> Value {
        match self {
            ChemicalItem::Absolute_configuration(v) => Value::from(*v),
            ChemicalItem::Compound_source(s)
            | ChemicalItem::Identifier_inchi(s)
            | ChemicalItem::Identifier_inchi_key(s)
            | ChemicalItem::Identifier_inchi_version(s)
            | ChemicalItem::Name_common(s)
            | ChemicalItem::Name_mineral(s)
            | ChemicalItem::Name_structure_type(s)
            | ChemicalItem::Name_systematic(s)
            | ChemicalItem::Optical_rotation(s)
            | ChemicalItem::Properties_biological(s)
            | ChemicalItem::Properties_physical(s) => Value::CharString(s.clone()),
            ChemicalItem::Enantioexcess_bulk(v) | ChemicalItem::Enantioexcess_crystal(v) => {
                Value::from(*v)
            }
            ChemicalItem::Enantioexcess_bulk_technique(v) => Value::from(*v),
            ChemicalItem::Enantioexcess_crystal_technique(v) => Value::from(*v),
            ChemicalItem::Melting_point(v)
            | ChemicalItem::Temperature_decomposition(v)
            | ChemicalItem::Temperature_sublimation(v) => Value::from(*v),
        }
    }
}

impl Display for ChemicalItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop companions of the `chemical` temperature items (6
/// items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalLoopItem {
    Melting_point_gt(Vec<Temperature>),
    Melting_point_lt(Vec<Temperature>),
    Temperature_decomposition_gt(Vec<Temperature>),
    Temperature_decomposition_lt(Vec<Temperature>),
    Temperature_sublimation_gt(Vec<Temperature>),
    Temperature_sublimation_lt(Vec<Temperature>),
}

impl CifTerm for ChemicalLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ChemicalLoopItem::Melting_point_gt(_) => "melting_point_gt",
            ChemicalLoopItem::Melting_point_lt(_) => "melting_point_lt",
            ChemicalLoopItem::Temperature_decomposition_gt(_) => "temperature_decomposition_gt",
            ChemicalLoopItem::Temperature_decomposition_lt(_) => "temperature_decomposition_lt",
            ChemicalLoopItem::Temperature_sublimation_gt(_) => "temperature_sublimation_gt",
            ChemicalLoopItem::Temperature_sublimation_lt(_) => "temperature_sublimation_lt",
        };
        Tag::new(format!("chemical_{suffix}"))
    }
}

impl LoopValueTerm for ChemicalLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ChemicalLoopItem::Melting_point_gt(v)
            | ChemicalLoopItem::Melting_point_lt(v)
            | ChemicalLoopItem::Temperature_decomposition_gt(v)
            | ChemicalLoopItem::Temperature_decomposition_lt(v)
            | ChemicalLoopItem::Temperature_sublimation_gt(v)
            | ChemicalLoopItem::Temperature_sublimation_lt(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for ChemicalLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
