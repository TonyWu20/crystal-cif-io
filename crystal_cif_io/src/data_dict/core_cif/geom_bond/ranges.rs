//! Write-time range payloads of the `geom_bond` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_geom_bond_distance`. The dictionary records the range `0.0:`
/// (bond length in angstroms).
#[derive(Debug, Clone, Copy)]
pub struct BondDistance(Numeric);

impl BondDistance {
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_bond_distance: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
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
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_bond_multiplicity: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
    }
}

impl From<BondMultiplicity> for Value {
    fn from(value: BondMultiplicity) -> Self {
        Value::Numeric(value.0)
    }
}
