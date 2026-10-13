use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A CIF `<Float>` value.
///
/// Stored as `f64` (the winnow parser stored `f32` and lost
/// precision on values with more than about six significant digits;
/// the chumsky refactor stores `f64`, per `docs/ground-truth.md`
/// known gap 5).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float(pub f64);

impl Display for Float {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <f64 as Display>::fmt(&self.0, fmt)
    }
}

impl std::ops::Deref for Float {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SyntacticUnit for Float {
    type FormatOutput = f64;

    fn formatted_output(&self) -> Self::FormatOutput {
        self.0
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(value as f64)
    }
}

impl From<Float> for crate::Number {
    fn from(value: Float) -> Self {
        crate::Number::Float(value)
    }
}

impl From<Float> for crate::Numeric {
    fn from(value: Float) -> Self {
        Self::new(crate::Number::from(value), None)
    }
}

impl From<Float> for crate::Value {
    fn from(value: Float) -> Self {
        crate::Value::Numeric(crate::Numeric::from(value))
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        value.0
    }
}
