use std::fmt::Display;

use crate::grammar::SyntacticUnit;

/// A CIF `<Integer>` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer(pub i32);

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        <i32 as Display>::fmt(&self.0, f)
    }
}

impl std::ops::Deref for Integer {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SyntacticUnit for Integer {
    type FormatOutput = i32;

    fn formatted_output(&self) -> Self::FormatOutput {
        self.0
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer(value as i32)
    }
}
