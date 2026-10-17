use std::fmt::Display;

use crystal_cif_fmt::grammar::{Integer, Number, Numeric, UnsignedInteger, Value};

#[derive(Debug, Clone, Copy)]
/// The `atom_site` symmetry multiplicity: the value clamped into the
/// 1 to 192 range the dictionary records.
pub struct SymMultiplicity {
    value: u8,
}

impl SymMultiplicity {
    /// Clamps the value into the 1 to 192 range.
    pub fn new(value: u8) -> Self {
        Self {
            value: value.clamp(1, 192),
        }
    }
}

impl Display for SymMultiplicity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<SymMultiplicity> for UnsignedInteger {
    fn from(value: SymMultiplicity) -> Self {
        UnsignedInteger(value.value as u32)
    }
}

impl From<SymMultiplicity> for Integer {
    fn from(value: SymMultiplicity) -> Self {
        Integer::from(UnsignedInteger::from(value))
    }
}

impl From<SymMultiplicity> for Number {
    fn from(value: SymMultiplicity) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<SymMultiplicity> for Numeric {
    fn from(value: SymMultiplicity) -> Self {
        Numeric::new(Number::from(value), None)
    }
}

impl From<SymMultiplicity> for Value {
    fn from(value: SymMultiplicity) -> Self {
        Value::Numeric(Numeric::from(value))
    }
}
