//! Write-time range payloads of the `chemical_formula` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_chemical_formula_weight` and `_chemical_formula_weight_meas`.
/// The dictionary records the range `1.0:` (molar mass in g/mol,
/// the measured value in the companion item).
#[derive(Debug, Clone, Copy)]
pub struct FormulaWeight(Numeric);

impl FormulaWeight {
    pub fn try_new(item: &str, value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(1.0), None)
            .map_err(|e| RangeError::new(format!("chemical_formula_{item}: {e}")))?;
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

impl From<FormulaWeight> for Value {
    fn from(value: FormulaWeight) -> Self {
        Value::Numeric(value.0)
    }
}
