use std::fmt::Display;

use crate::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_diffrn_refln_detect_slit_horiz`. `new` clamps the value
/// to [0.0, 90.0], matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DetectSlitHoriz(Float);

impl DetectSlitHoriz {
    pub fn new(float: Float) -> Self {
        Self(Float(float.clamp(0.0, 90.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for DetectSlitHoriz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DetectSlitHoriz> for Value {
    fn from(value: DetectSlitHoriz) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for DetectSlitHoriz {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for DetectSlitHoriz {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
