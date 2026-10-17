//! Write-time range payloads of the `geom_hbond` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_geom_hbond_distance_DA`, `_geom_hbond_distance_DH`, and
/// `_geom_hbond_distance_HA`. The dictionary records the range
/// `0.0:` for all three (hydrogen-bond distances in angstroms).
#[derive(Debug, Clone, Copy)]
pub struct HbondDistance(Numeric);

impl HbondDistance {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_da(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_hbond_distance_DA: {e}")))?;
        Ok(Self(value))
    }

    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_dh(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_hbond_distance_DH: {e}")))?;
        Ok(Self(value))
    }

    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_ha(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_hbond_distance_HA: {e}")))?;
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

impl From<HbondDistance> for Value {
    fn from(value: HbondDistance) -> Self {
        Value::Numeric(value.0)
    }
}
