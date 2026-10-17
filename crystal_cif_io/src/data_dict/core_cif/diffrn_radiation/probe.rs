use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

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

impl Probe {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "x-ray" => Some(Self::X_ray),
            "neutron" => Some(Self::Neutron),
            "electron" => Some(Self::Electron),
            "gamma" => Some(Self::Gamma),
            _ => None,
        }
    }
}
