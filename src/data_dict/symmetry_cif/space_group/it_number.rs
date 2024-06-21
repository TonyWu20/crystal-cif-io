use std::fmt::Display;

use crate::data_dict::DataLabel;

use super::SpaceGroup;

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

impl DataLabel for ITNumber {
    type C = SpaceGroup;

    fn tag(&self) -> String {
        "IT_number".to_string()
    }
}

impl Display for ITNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
