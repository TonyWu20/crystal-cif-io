//! Write-time range payloads of the `geom_contact` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crate::grammar::{Numeric, Value};

/// `_geom_contact_distance`. The dictionary records the range
/// `0.0:` (contact distance in angstroms).
#[derive(Debug, Clone, Copy)]
pub struct ContactDistance(Numeric);

impl ContactDistance {
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("geom_contact_distance: {e}")))?;
        Ok(Self(value))
    }

    pub fn value(&self) -> Numeric {
        self.0
    }
}

impl From<ContactDistance> for Value {
    fn from(value: ContactDistance) -> Self {
        Value::Numeric(value.0)
    }
}
