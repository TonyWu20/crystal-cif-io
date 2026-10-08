//! The closed code sets of the `chemical` category.

use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_chemical_absolute_configuration`. The
/// dictionary records the single code `rm` (relative configuration
/// established by a chiral reference compound).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsoluteConfiguration {
    /// `rm`: relative configuration of a chiral reference compound.
    Rm,
}

impl Display for AbsoluteConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("rm")
    }
}

impl From<AbsoluteConfiguration> for UnquotedString {
    fn from(value: AbsoluteConfiguration) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<AbsoluteConfiguration> for CharString {
    fn from(value: AbsoluteConfiguration) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<AbsoluteConfiguration> for Value {
    fn from(value: AbsoluteConfiguration) -> Self {
        Value::CharString(CharString::from(value))
    }
}

/// The closed code set of `_chemical_enantioexcess_bulk_technique`.
/// The dictionary records the single code `OA` (optical activity).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnantioexcessBulkTechnique {
    /// `OA`: specific rotation of the bulk compound in solution.
    Oa,
}

impl Display for EnantioexcessBulkTechnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OA")
    }
}

impl From<EnantioexcessBulkTechnique> for UnquotedString {
    fn from(value: EnantioexcessBulkTechnique) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<EnantioexcessBulkTechnique> for CharString {
    fn from(value: EnantioexcessBulkTechnique) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<EnantioexcessBulkTechnique> for Value {
    fn from(value: EnantioexcessBulkTechnique) -> Self {
        Value::CharString(CharString::from(value))
    }
}

/// The closed code set of `_chemical_enantioexcess_crystal_technique`.
/// The dictionary records the single code `CD` (circular
/// dichroism).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnantioexcessCrystalTechnique {
    /// `CD`: circular dichroism of the crystal.
    Cd,
}

impl Display for EnantioexcessCrystalTechnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CD")
    }
}

impl From<EnantioexcessCrystalTechnique> for UnquotedString {
    fn from(value: EnantioexcessCrystalTechnique) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<EnantioexcessCrystalTechnique> for CharString {
    fn from(value: EnantioexcessCrystalTechnique) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<EnantioexcessCrystalTechnique> for Value {
    fn from(value: EnantioexcessCrystalTechnique) -> Self {
        Value::CharString(CharString::from(value))
    }
}
