use std::fmt::Display;

use crate::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_diffrn_reflns_resolution_max`. `new` clamps the value
/// to [0.0, +infinity), matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ResolutionMax(Float);

impl ResolutionMax {
    pub fn new(float: Float) -> Self {
        Self(Float(float.max(0.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for ResolutionMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ResolutionMax> for Value {
    fn from(value: ResolutionMax) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for ResolutionMax {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for ResolutionMax {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
