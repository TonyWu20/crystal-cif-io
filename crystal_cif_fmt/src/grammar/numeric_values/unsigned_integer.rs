use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A CIF `<UnsignedInteger>` value (the digits of a standard
/// uncertainty).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsignedInteger(pub u32);

impl std::ops::Deref for UnsignedInteger {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SyntacticUnit for UnsignedInteger {
    type FormatOutput = u32;

    fn formatted_output(&self) -> Self::FormatOutput {
        self.0
    }
}

impl Display for UnsignedInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl From<UnsignedInteger> for super::integer::Integer {
    fn from(value: UnsignedInteger) -> Self {
        Self(value.0 as i32)
    }
}

impl From<UnsignedInteger> for crate::Value {
    fn from(value: UnsignedInteger) -> Self {
        crate::Value::Numeric(crate::Numeric::new(
            crate::Number::Integer(value.into()),
            None,
        ))
    }
}
