use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_source_device`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Device {
    /// `tube`
    Tube,
    /// `nuclear`
    Nuclear,
    /// `spallation`
    Spallation,
    /// `elect-micro`
    Elect_micro,
    /// `rot-anode`
    Rot_anode,
    /// `synch`
    Synch,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Tube => "tube",
            Self::Nuclear => "nuclear",
            Self::Spallation => "spallation",
            Self::Elect_micro => "elect-micro",
            Self::Rot_anode => "rot-anode",
            Self::Synch => "synch",
        };
        f.write_str(code)
    }
}

impl From<Device> for UnquotedString {
    fn from(value: Device) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Device> for CharString {
    fn from(value: Device) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Device> for Value {
    fn from(value: Device) -> Self {
        Value::CharString(CharString::from(value))
    }
}
