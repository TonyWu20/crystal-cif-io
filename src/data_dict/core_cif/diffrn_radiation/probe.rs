use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_radiation_probe`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Probe {
    /// `x-ray`
    X_ray,
    /// `neutron`
    Neutron,
    /// `electron`
    Electron,
    /// `gamma`
    Gamma,
}

impl Display for Probe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::X_ray => "x-ray",
            Self::Neutron => "neutron",
            Self::Electron => "electron",
            Self::Gamma => "gamma",
        };
        f.write_str(code)
    }
}

impl From<Probe> for UnquotedString {
    fn from(value: Probe) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Probe> for CharString {
    fn from(value: Probe) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Probe> for Value {
    fn from(value: Probe) -> Self {
        Value::CharString(CharString::from(value))
    }
}
