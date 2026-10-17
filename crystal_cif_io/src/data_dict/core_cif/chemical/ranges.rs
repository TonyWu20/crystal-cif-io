//! Write-time range payloads of the `chemical` category.
//!
//! Each type wraps a `Numeric` and checks the dictionary range of
//! its item in `try_new` (roadmap section 6, T3 rule: range
//! constraints get a write-time check).

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_chemical_enantioexcess_bulk` and `_chemical_enantioexcess_crystal`.
/// The dictionary records the range `0.0:1.0` for both (the
/// enantiomeric excess as a fraction).
#[derive(Debug, Clone, Copy)]
pub struct Enantioexcess(Numeric);

impl Enantioexcess {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_bulk(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), Some(1.0))
            .map_err(|e| RangeError::new(format!("chemical_enantioexcess_bulk: {e}")))?;
        Ok(Self(value))
    }

    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_crystal(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), Some(1.0))
            .map_err(|e| RangeError::new(format!("chemical_enantioexcess_crystal: {e}")))?;
        Ok(Self(value))
    }

    /// The stored value.
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

impl From<Enantioexcess> for Value {
    fn from(value: Enantioexcess) -> Self {
        Value::Numeric(value.0)
    }
}

/// The temperature items of the `chemical` category:
/// `_chemical_melting_point`, `_chemical_temperature_decomposition`,
/// and `_chemical_temperature_sublimation`, with their loop `_gt`
/// and `_lt` companions. The dictionary records the range `0.0:`
/// (temperature in kelvin).
#[derive(Debug, Clone, Copy)]
pub struct Temperature(Numeric);

impl Temperature {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new(item: &str, value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("chemical_{item}: {e}")))?;
        Ok(Self(value))
    }

    /// The stored value.
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

impl From<Temperature> for Value {
    fn from(value: Temperature) -> Self {
        Value::Numeric(value.0)
    }
}
