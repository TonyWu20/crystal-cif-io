//! Write-time range payloads of the `atom_type` category.
//!
//! Each type wraps a `Numeric` and checks the dictionary range of
//! its item in `try_new` (roadmap section 6, T3 rule: range
//! constraints get a write-time check).

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_atom_type_analytical_mass_%`. The dictionary records the range
/// `0.0:100.0` (percent of the formula-unit mass).
#[derive(Debug, Clone, Copy)]
pub struct AnalyticalMass(Numeric);

impl AnalyticalMass {
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), Some(100.0))
            .map_err(|e| RangeError::new(format!("atom_type_analytical_mass_percent: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
    }

    /// Build without the range guard. The read path stores the
    /// recorded value as written. The builder write path applies the
    /// guard. The document `to_members` writes the stored value as
    /// is.
    pub fn from_numeric(value: Numeric) -> Self {
        Self(value)
    }
}

impl From<AnalyticalMass> for Value {
    fn from(value: AnalyticalMass) -> Self {
        Value::Numeric(value.0)
    }
}

/// `_atom_type_number_in_cell`. The dictionary records the range
/// `0:` (site count times the site multiplicity).
#[derive(Debug, Clone, Copy)]
pub struct NumberInCell(Numeric);

impl NumberInCell {
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("atom_type_number_in_cell: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
    }

    /// Build without the range guard. The read path stores the
    /// recorded value as written. The builder write path applies the
    /// guard. The document `to_members` writes the stored value as
    /// is.
    pub fn from_numeric(value: Numeric) -> Self {
        Self(value)
    }
}

impl From<NumberInCell> for Value {
    fn from(value: NumberInCell) -> Self {
        Value::Numeric(value.0)
    }
}

/// `_atom_type_oxidation_number`. The dictionary records the range
/// `-8:8`.
#[derive(Debug, Clone, Copy)]
pub struct OxidationNumber(Numeric);

impl OxidationNumber {
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(-8.0), Some(8.0))
            .map_err(|e| RangeError::new(format!("atom_type_oxidation_number: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
    }

    /// Build without the range guard. The read path stores the
    /// recorded value as written. The builder write path applies the
    /// guard. The document `to_members` writes the stored value as
    /// is.
    pub fn from_numeric(value: Numeric) -> Self {
        Self(value)
    }
}

impl From<OxidationNumber> for Value {
    fn from(value: OxidationNumber) -> Self {
        Value::Numeric(value.0)
    }
}

/// `_atom_type_radius_bond` and `_atom_type_radius_contact`. The
/// dictionary records the range `0.0:5.0` for both (radius in
/// angstroms).
#[derive(Debug, Clone, Copy)]
pub struct Radius(Numeric);

impl Radius {
    pub fn try_new_bond(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), Some(5.0))
            .map_err(|e| RangeError::new(format!("atom_type_radius_bond: {e}")))?;
        Ok(Self(value))
    }

    pub fn try_new_contact(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), Some(5.0))
            .map_err(|e| RangeError::new(format!("atom_type_radius_contact: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
    }

    /// Build without the range guard. The read path stores the
    /// recorded value as written. The builder write path applies the
    /// guard. The document `to_members` writes the stored value as
    /// is.
    pub fn from_numeric(value: Numeric) -> Self {
        Self(value)
    }
}

impl From<Radius> for Value {
    fn from(value: Radius) -> Self {
        Value::Numeric(value.0)
    }
}
