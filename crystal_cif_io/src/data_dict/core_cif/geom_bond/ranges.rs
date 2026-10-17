//! Write-time range payloads of the `geom_bond` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_geom_bond_distance`. The dictionary records the range `0.0:`
/// (bond length in angstroms).
#[derive(Debug, Clone, Copy)]
pub struct BondDistance(Numeric);

impl BondDistance {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_bond_distance: {e}")))?;
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

impl From<BondDistance> for Value {
    fn from(value: BondDistance) -> Self {
        Value::Numeric(value.0)
    }
}

/// `_geom_bond_multiplicity`. The dictionary records the range
/// `0:` (the bond order).
#[derive(Debug, Clone, Copy)]
pub struct BondMultiplicity(Numeric);

impl BondMultiplicity {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_bond_multiplicity: {e}")))?;
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

impl From<BondMultiplicity> for Value {
    fn from(value: BondMultiplicity) -> Self {
        Value::Numeric(value.0)
    }
}
