//! Write-time range payloads of the `geom_hbond` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crate::grammar::{Numeric, Value};

/// `_geom_hbond_distance_DA`, `_geom_hbond_distance_DH`, and
/// `_geom_hbond_distance_HA`. The dictionary records the range
/// `0.0:` for all three (hydrogen-bond distances in angstroms).
#[derive(Debug, Clone, Copy)]
pub struct HbondDistance(Numeric);

impl HbondDistance {
    pub fn try_new_da(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_hbond_distance_DA: {e}")))?;
        Ok(Self(value))
    }

    pub fn try_new_dh(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_hbond_distance_DH: {e}")))?;
        Ok(Self(value))
    }

    pub fn try_new_ha(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_hbond_distance_HA: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
    }
}

impl From<HbondDistance> for Value {
    fn from(value: HbondDistance) -> Self {
        Value::Numeric(value.0)
    }
}
