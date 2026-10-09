use std::fmt::Display;

use crate::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_function_symkey`. `new` clamps the value
/// to [1.0, 192.0], matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Symkey(Float);

impl Symkey {
    pub fn new(float: Float) -> Self {
        Self(Float(float.clamp(1.0, 192.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for Symkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Symkey> for Value {
    fn from(value: Symkey) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for Symkey {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for Symkey {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
