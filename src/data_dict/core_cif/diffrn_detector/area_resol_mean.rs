use std::fmt::Display;

use crate::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_diffrn_detector_area_resol_mean`. `new` clamps the value
/// to [0.0, +infinity), matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AreaResolMean(Float);

impl AreaResolMean {
    pub fn new(float: Float) -> Self {
        Self(Float(float.max(0.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for AreaResolMean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AreaResolMean> for Value {
    fn from(value: AreaResolMean) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for AreaResolMean {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for AreaResolMean {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
