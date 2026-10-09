use std::fmt::Display;

use crate::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_model_site_radius_bond`. `new` clamps the value
/// to [0.1, +infinity), matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RadiusBond(Float);

impl RadiusBond {
    pub fn new(float: Float) -> Self {
        Self(Float(float.max(0.1)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for RadiusBond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RadiusBond> for Value {
    fn from(value: RadiusBond) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for RadiusBond {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for RadiusBond {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
