use std::fmt::Display;

use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_diffrn_reflns_Laue_measured_fraction_full`. `new` clamps the value
/// to [0.95, 1.0], matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LaueMeasuredFractionFull(Float);

impl LaueMeasuredFractionFull {
    pub fn new(float: Float) -> Self {
        Self(Float(float.clamp(0.95, 1.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for LaueMeasuredFractionFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<LaueMeasuredFractionFull> for Value {
    fn from(value: LaueMeasuredFractionFull) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for LaueMeasuredFractionFull {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for LaueMeasuredFractionFull {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
