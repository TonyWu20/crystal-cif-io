//! Write-time range payloads of the `chemical_conn_atom` category.

use crate::data_dict::core_cif::range_check::{check_range, RangeError};
use crystal_cif_fmt::grammar::{Numeric, Value};

/// `_chemical_conn_atom_NCA` and `_chemical_conn_atom_NH`. The
/// dictionary records the range `0:` (the number of neighbouring
/// atoms, and the hydrogen count of the atom type).
#[derive(Debug, Clone, Copy)]
pub struct ConnAtomCount(Numeric);

impl ConnAtomCount {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_nca(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("chemical_conn_atom_NCA: {e}")))?;
        Ok(Self(value))
    }

    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_nh(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), None)
            .map_err(|e| RangeError::new(format!("chemical_conn_atom_NH: {e}")))?;
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

impl From<ConnAtomCount> for Value {
    fn from(value: ConnAtomCount) -> Self {
        Value::Numeric(value.0)
    }
}

/// `_chemical_conn_atom_charge`. The dictionary records the range
/// `-6:6` (the atom charge).
#[derive(Debug, Clone, Copy)]
pub struct ConnAtomCharge(Numeric);

impl ConnAtomCharge {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(-6.0), Some(6.0))
            .map_err(|e| RangeError::new(format!("chemical_conn_atom_charge: {e}")))?;
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

impl From<ConnAtomCharge> for Value {
    fn from(value: ConnAtomCharge) -> Self {
        Value::Numeric(value.0)
    }
}

/// `_chemical_conn_atom_display_x` and `_chemical_conn_atom_display_y`.
/// The dictionary records the range `0.0:1.0` for both (the
/// coordinates of the atom in the display drawing).
#[derive(Debug, Clone, Copy)]
pub struct ConnAtomDisplay(Numeric);

impl ConnAtomDisplay {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_x(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), Some(1.0))
            .map_err(|e| RangeError::new(format!("chemical_conn_atom_display_x: {e}")))?;
        Ok(Self(value))
    }

    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new_y(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(0.0), Some(1.0))
            .map_err(|e| RangeError::new(format!("chemical_conn_atom_display_y: {e}")))?;
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

impl From<ConnAtomDisplay> for Value {
    fn from(value: ConnAtomDisplay) -> Self {
        Value::Numeric(value.0)
    }
}

/// `_chemical_conn_atom_number`. The dictionary records the range
/// `1:` (the 1-based index of the atom in the connection loop).
#[derive(Debug, Clone, Copy)]
pub struct ConnAtomNumber(Numeric);

impl ConnAtomNumber {
    /// Builds the value and checks the dictionary range. An out-of-range
    /// value rejects with a [`RangeError`].
    pub fn try_new(value: Numeric) -> Result<Self, RangeError> {
        check_range(&value, Some(1.0), None)
            .map_err(|e| RangeError::new(format!("chemical_conn_atom_number: {e}")))?;
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

impl From<ConnAtomNumber> for Value {
    fn from(value: ConnAtomNumber) -> Self {
        Value::Numeric(value.0)
    }
}
