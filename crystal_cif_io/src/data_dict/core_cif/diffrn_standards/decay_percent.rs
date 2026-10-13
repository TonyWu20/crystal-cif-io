use std::fmt::Display;

use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_diffrn_standards_decay_%`. `new` clamps the value
/// to (-infinity, 100.0], matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DecayPercent(Float);

impl DecayPercent {
    pub fn new(float: Float) -> Self {
        Self(Float(float.min(100.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for DecayPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DecayPercent> for Value {
    fn from(value: DecayPercent) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for DecayPercent {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for DecayPercent {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
