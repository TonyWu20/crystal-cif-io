//! Write-time range checks for dictionary `numb` items.
//!
//! The dictionary records a numeric range on some `numb` items
//! (for example `_atom_type_oxidation_number` is `-8:8`). The
//! per-item typed payloads in the category modules call
//! [`check_range`] inside their `try_new` constructor, in the style
//! of `CrystalSystemCifError` (a plain error type, no `todo!`).
//! Out-of-range values fail at construction instead of reaching the
//! write path.
//!
//! A dictionary range like `0.0:1.0` is a closed interval. An open
//! side is recorded as a missing bound: `0:` means a minimum of
//! zero with no maximum.

use crystal_cif_fmt::grammar::{Number, Numeric};

/// The error returned by a `try_new` range check.
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct RangeError {
    message: String,
}

impl RangeError {
    /// Build an error with a caller-supplied message.
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

/// Check a `Numeric` value against a dictionary range.
///
/// A bound of `None` means open on that side. The error message
/// records the offending value and the violated bound. The caller
/// prefixes the message with the item name.
pub fn check_range(value: &Numeric, min: Option<f64>, max: Option<f64>) -> Result<(), RangeError> {
    let number = number_as_f64(&value.number());
    if let Some(min) = min {
        if number < min {
            return Err(RangeError::new(format!(
                "value {number} is below the dictionary minimum {min}"
            )));
        }
    }
    if let Some(max) = max {
        if number > max {
            return Err(RangeError::new(format!(
                "value {number} exceeds the dictionary maximum {max}"
            )));
        }
    }
    Ok(())
}

/// Read a `Number` as `f64` for a range check.
pub fn number_as_f64(number: &Number) -> f64 {
    match number {
        Number::Integer(i) => i.0 as f64,
        Number::Float(f) => f.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::grammar::{Float, Integer};

    fn numeric(f64_value: f64) -> Numeric {
        Numeric::new(Number::Float(Float(f64_value)), None)
    }

    #[test]
    fn accepts_values_inside_the_range() {
        let value = numeric(0.5);
        assert!(check_range(&value, Some(0.0), Some(1.0)).is_ok());
    }

    #[test]
    fn accepts_the_closed_bounds() {
        let low = numeric(0.0);
        let high = numeric(1.0);
        assert!(check_range(&low, Some(0.0), Some(1.0)).is_ok());
        assert!(check_range(&high, Some(0.0), Some(1.0)).is_ok());
    }

    #[test]
    fn rejects_values_outside_the_range() {
        let low = numeric(-0.1);
        let high = numeric(1.1);
        assert!(check_range(&low, Some(0.0), Some(1.0)).is_err());
        assert!(check_range(&high, Some(0.0), Some(1.0)).is_err());
    }

    #[test]
    fn open_sides_pass() {
        let value = numeric(123.0);
        assert!(check_range(&value, Some(0.0), None).is_ok());
        assert!(check_range(&value, None, Some(1.0)).is_err());
    }

    #[test]
    fn integer_numbers_take_the_range_check() {
        let value = Numeric::new(Number::Integer(Integer(42)), None);
        assert!(check_range(&value, None, Some(100.0)).is_ok());
        assert!(check_range(&value, Some(43.0), None).is_err());
    }
}
