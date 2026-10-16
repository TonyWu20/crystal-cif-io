use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_radiation_wavelength_xray_symbol`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum XraySymbol {
    /// `K-L~3~`
    K_L_3,
    /// `K-L~2~`
    K_L_2,
    /// `K-M~3~`
    K_M_3,
    /// `K-L~2,3~`
    K_L_2_3,
}

impl Display for XraySymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::K_L_3 => "K-L~3~",
            Self::K_L_2 => "K-L~2~",
            Self::K_M_3 => "K-M~3~",
            Self::K_L_2_3 => "K-L~2,3~",
        };
        f.write_str(code)
    }
}

impl From<XraySymbol> for UnquotedString {
    fn from(value: XraySymbol) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<XraySymbol> for CharString {
    fn from(value: XraySymbol) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<XraySymbol> for Value {
    fn from(value: XraySymbol) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl XraySymbol {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "K-L~3~" => Some(Self::K_L_3),
            "K-L~2~" => Some(Self::K_L_2),
            "K-M~3~" => Some(Self::K_M_3),
            "K-L~2,3~" => Some(Self::K_L_2_3),
            _ => None,
        }
    }
}
