//! The `chemical` category (25 items).
//!
//! Single-value items describe the compound: names, identifiers,
//! the source, the melting and decomposition temperatures, and the
//! enantiomeric excess with its technique code. The loop items are
//! the `_gt` and `_lt` companions of the temperature items, which
//! the dictionary marks as loop columns.

mod chemical;
mod codes;
mod ranges;

pub use chemical::{Chemical, ChemicalRow};
pub use codes::{AbsoluteConfiguration, EnantioexcessBulkTechnique, EnantioexcessCrystalTechnique};
pub use ranges::{Enantioexcess, Temperature};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// Single-value items of the `chemical` category (19 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ChemicalItem {
    /// The `_chemical_absolute_configuration` item. The variant holds the single value.
    Absolute_configuration(AbsoluteConfiguration),
    /// The `_chemical_compound_source` item. The variant holds the single value.
    Compound_source(CharString),
    /// The `_chemical_enantioexcess_bulk` item. The variant holds the single value.
    Enantioexcess_bulk(Enantioexcess),
    /// The `_chemical_enantioexcess_bulk_technique` item. The variant holds the single value.
    Enantioexcess_bulk_technique(EnantioexcessBulkTechnique),
    /// The `_chemical_enantioexcess_crystal` item. The variant holds the single value.
    Enantioexcess_crystal(Enantioexcess),
    /// The `_chemical_enantioexcess_crystal_technique` item. The variant holds the single value.
    Enantioexcess_crystal_technique(EnantioexcessCrystalTechnique),
    /// The `_chemical_identifier_inchi` item. The variant holds the single value.
    Identifier_inchi(CharString),
    /// The `_chemical_identifier_inchi_key` item. The variant holds the single value.
    Identifier_inchi_key(CharString),
    /// The `_chemical_identifier_inchi_version` item. The variant holds the single value.
    Identifier_inchi_version(CharString),
    /// The `_chemical_melting_point` item. The variant holds the single value.
    Melting_point(Temperature),
    /// The `_chemical_name_common` item. The variant holds the single value.
    Name_common(CharString),
    /// The `_chemical_name_mineral` item. The variant holds the single value.
    Name_mineral(CharString),
    /// The `_chemical_name_structure_type` item. The variant holds the single value.
    Name_structure_type(CharString),
    /// The `_chemical_name_systematic` item. The variant holds the single value.
    Name_systematic(CharString),
    /// The `_chemical_optical_rotation` item. The variant holds the single value.
    Optical_rotation(CharString),
    /// The `_chemical_properties_biological` item. The variant holds the single value.
    Properties_biological(CharString),
    /// The `_chemical_properties_physical` item. The variant holds the single value.
    Properties_physical(CharString),
    /// The `_chemical_temperature_decomposition` item. The variant holds the single value.
    Temperature_decomposition(Temperature),
    /// The `_chemical_temperature_sublimation` item. The variant holds the single value.
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
/// The loop-value terms of the `chemical` category: one variant per

/// registry loop-value item, in item order.
pub enum ChemicalLoopItem {
    /// The `_chemical_melting_point_gt` item. The variant holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the melting point (the temperature at which the crystalline solid
    /// changes to a liquid) lies. These items allow a range of temperatures
    /// to be given. _chemical_melting_point should always be used in
    /// preference to these items whenever possible.
    /// Units: K (kelvin).
    Melting_point_gt(Vec<Temperature>),
    /// The `_chemical_melting_point_lt` item. The variant holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the melting point (the temperature at which the crystalline solid
    /// changes to a liquid) lies. These items allow a range of temperatures
    /// to be given. _chemical_melting_point should always be used in
    /// preference to these items whenever possible.
    /// Units: K (kelvin).
    Melting_point_lt(Vec<Temperature>),
    /// The `_chemical_temperature_decomposition_gt` item. The variant holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to decompose. These items allow a range of
    /// temperatures to be given. _chemical_temperature_decomposition should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    Temperature_decomposition_gt(Vec<Temperature>),
    /// The `_chemical_temperature_decomposition_lt` item. The variant holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to decompose. These items allow a range of
    /// temperatures to be given. _chemical_temperature_decomposition should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    Temperature_decomposition_lt(Vec<Temperature>),
    /// The `_chemical_temperature_sublimation_gt` item. The variant holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to sublime. These items allow a range of
    /// temperatures to be given. _chemical_temperature_sublimation should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    Temperature_sublimation_gt(Vec<Temperature>),
    /// The `_chemical_temperature_sublimation_lt` item. The variant holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to sublime. These items allow a range of
    /// temperatures to be given. _chemical_temperature_sublimation should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
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
