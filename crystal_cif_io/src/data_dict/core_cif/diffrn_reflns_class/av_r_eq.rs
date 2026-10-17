use std::fmt::Display;

use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_diffrn_reflns_class_av_R_eq`. `new` clamps the value
/// to [0.0, +infinity), matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AvREq(Float);

impl AvREq {
    /// Build the guarded value: the input clamped to the dictionary range.
    pub fn new(float: Float) -> Self {
        Self(Float(float.max(0.0)))
    }

    /// The recorded value, stored as written.
    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for AvREq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AvREq> for Value {
    fn from(value: AvREq) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for AvREq {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for AvREq {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
