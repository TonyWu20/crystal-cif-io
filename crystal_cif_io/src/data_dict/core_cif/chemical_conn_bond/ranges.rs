//! Write-time range payloads of the `chemical_conn_bond` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_chemical_conn_bond_atom_1` and `_chemical_conn_bond_atom_2`.
/// The dictionary records the range `1:` for both (the 1-based atom
/// index from the `_chemical_conn_atom` loop).
#[derive(Debug, Clone, Copy)]
pub struct BondAtomNumber(Numeric);

impl BondAtomNumber {
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(1.0), None)
            .map_err(|e| RangeError::new(format!("chemical_conn_bond_atom_1 / _atom_2: {e}")))?;
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

impl From<BondAtomNumber> for Value {
    fn from(value: BondAtomNumber) -> Self {
        Value::Numeric(value.0)
    }
}
