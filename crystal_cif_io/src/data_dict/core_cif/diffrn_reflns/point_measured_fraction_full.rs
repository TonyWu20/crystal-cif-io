use std::fmt::Display;

use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_diffrn_reflns_point_measured_fraction_full`. `new` clamps the value
/// to [0.95, 1.0], matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PointMeasuredFractionFull(Float);

impl PointMeasuredFractionFull {
    pub fn new(float: Float) -> Self {
        Self(Float(float.clamp(0.95, 1.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for PointMeasuredFractionFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PointMeasuredFractionFull> for Value {
    fn from(value: PointMeasuredFractionFull) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for PointMeasuredFractionFull {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for PointMeasuredFractionFull {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
