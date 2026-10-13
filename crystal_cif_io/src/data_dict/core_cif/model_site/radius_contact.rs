use std::fmt::Display;

use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_model_site_radius_contact`. `new` clamps the value
/// to [1.0, +infinity), matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RadiusContact(Float);

impl RadiusContact {
    pub fn new(float: Float) -> Self {
        Self(Float(float.max(1.0)))
    }

    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for RadiusContact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RadiusContact> for Value {
    fn from(value: RadiusContact) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for RadiusContact {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for RadiusContact {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
