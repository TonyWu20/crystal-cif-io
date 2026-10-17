use std::fmt::Display;

use crate::grammar::SyntacticUnit;

pub use self::{float::Float, integer::Integer, unsigned_integer::UnsignedInteger};

use super::CIFValue;

mod float;
mod integer;
mod unsigned_integer;

/// A numeric value without standard uncertainty: an `Integer` or a
/// `Float`, as written in the CIF text.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
    /// An integer value (`42`, not `42.0`).
    Integer(Integer),
    /// A floating point value.
    Float(Float),
}

impl Number {
    /// The integer payload, when the value is an integer.
    pub fn as_integer(&self) -> Option<&Integer> {
        if let Self::Integer(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// The float payload, when the value is a float.
    pub fn as_float(&self) -> Option<&Float> {
        if let Self::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl CIFValue for Number {}

impl SyntacticUnit for Number {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            Number::Integer(i) => format!("{i}"),
            // The f64 Display is the shortest round-trip form. The
            // old `{:<6.3}` width truncated write-path values.
            Number::Float(f) => f.0.to_string(),
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Integer(Integer(value))
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

/// A numeric value with an optional standard uncertainty:
/// `7.01(3)` is `7.01` with the uncertainty digits `3`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Numeric {
    number: Number,
    std_uncertainty: Option<UnsignedInteger>,
}

impl Numeric {
    /// Build a numeric value from its number and uncertainty digits.
    pub fn new(number: Number, std_uncertainty: Option<UnsignedInteger>) -> Self {
        Self {
            number,
            std_uncertainty,
        }
    }

    /// The value without the uncertainty digits.
    pub fn number(&self) -> Number {
        self.number
    }

    /// The uncertainty digits, when the value carries them.
    pub fn std_uncertainty(&self) -> Option<UnsignedInteger> {
        self.std_uncertainty
    }
}

impl SyntacticUnit for Numeric {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self.std_uncertainty {
            Some(c) => format!("{}({})", self.number, c),
            None => format!("{}", self.number),
        }
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl From<f64> for Numeric {
    fn from(value: f64) -> Self {
        Numeric::new(Number::Float(Float(value)), None)
    }
}

impl From<f32> for Numeric {
    fn from(value: f32) -> Self {
        Numeric::new(Number::Float(Float(value as f64)), None)
    }
}

impl From<i32> for Numeric {
    fn from(value: i32) -> Self {
        Numeric::new(Number::from(value), None)
    }
}

impl From<Numeric> for crate::Value {
    fn from(value: Numeric) -> Self {
        crate::Value::Numeric(value)
    }
}
