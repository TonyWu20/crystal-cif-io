use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_measurement_rotation_mode`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RotationMode {
    /// `rotation`
    Rotation,
    /// `stepwise`
    Stepwise,
}

impl Display for RotationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Rotation => "rotation",
            Self::Stepwise => "stepwise",
        };
        f.write_str(code)
    }
}

impl From<RotationMode> for UnquotedString {
    fn from(value: RotationMode) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<RotationMode> for CharString {
    fn from(value: RotationMode) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<RotationMode> for Value {
    fn from(value: RotationMode) -> Self {
        Value::CharString(CharString::from(value))
    }
}
