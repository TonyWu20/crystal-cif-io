use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The displacement-parameter type of an `atom_site` row: the
/// aniso and iso arms of the `_atom_site_adp_type` enumeration,
/// plus the real-data `betaani` code.
#[derive(Debug, Clone, Copy)]
pub enum AdpType {
    /// The `Uani` value: anisotropic U displacement parameters.
    Uani,
    /// The `Uiso` value: an isotropic U displacement parameter.
    Uiso,
    /// The `Uovl` value: anisotropic U for an overlapping atom.
    Uovl,
    /// The `Umpe` value: anisotropic U for a model-peak position.
    Umpe,
    /// The `Bani` value: anisotropic B displacement parameters.
    Bani,
    /// The `Biso` value: an isotropic B displacement parameter.
    Biso,
    /// The `Bovl` value: anisotropic B for an overlapping atom.
    Bovl,
    /// The `betaani` value: anisotropic beta displacement parameters.
    Betaani,
}

impl Display for AdpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::Betaani => "betaani".to_string(),
            other => format!("{other:?}"),
        };
        write!(f, "{}", content)
    }
}

impl From<AdpType> for UnquotedString {
    fn from(value: AdpType) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<AdpType> for CharString {
    fn from(value: AdpType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<AdpType> for Value {
    fn from(value: AdpType) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl AdpType {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "Uani" => Some(Self::Uani),
            "Uiso" => Some(Self::Uiso),
            "Uovl" => Some(Self::Uovl),
            "Umpe" => Some(Self::Umpe),
            "Bani" => Some(Self::Bani),
            "Biso" => Some(Self::Biso),
            "Bovl" => Some(Self::Bovl),
            "betaani" => Some(Self::Betaani),
            _ => None,
        }
    }
}
