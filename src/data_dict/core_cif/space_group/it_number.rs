use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ITNumber(u8);

impl Default for ITNumber {
    fn default() -> Self {
        Self(1_u8)
    }
}

impl ITNumber {
    pub fn new(number: u8) -> Self {
        Self(number.clamp(1, 230))
    }
}

impl Display for ITNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for ITNumber {
    fn from(value: i32) -> Self {
        Self(value as u8)
    }
}

impl From<ITNumber> for u64 {
    fn from(value: ITNumber) -> Self {
        value.0 as u64
    }
}
