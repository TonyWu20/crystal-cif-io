use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_publ_requested_category`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RequestedCategory {
    /// `AD`
    AD,
    /// `CI`
    CI,
    /// `CM`
    CM,
    /// `CO`
    CO,
    /// `EI`
    EI,
    /// `EM`
    EM,
    /// `EO`
    EO,
    /// `FA`
    FA,
    /// `FI`
    FI,
    /// `FM`
    FM,
    /// `FO`
    FO,
    /// `GI`
    GI,
    /// `GM`
    GM,
    /// `GO`
    GO,
    /// `HI`
    HI,
    /// `HM`
    HM,
    /// `HO`
    HO,
    /// `QI`
    QI,
    /// `QM`
    QM,
    /// `QO`
    QO,
    /// `SC`
    SC,
}

impl Display for RequestedCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::AD => "AD",
            Self::CI => "CI",
            Self::CM => "CM",
            Self::CO => "CO",
            Self::EI => "EI",
            Self::EM => "EM",
            Self::EO => "EO",
            Self::FA => "FA",
            Self::FI => "FI",
            Self::FM => "FM",
            Self::FO => "FO",
            Self::GI => "GI",
            Self::GM => "GM",
            Self::GO => "GO",
            Self::HI => "HI",
            Self::HM => "HM",
            Self::HO => "HO",
            Self::QI => "QI",
            Self::QM => "QM",
            Self::QO => "QO",
            Self::SC => "SC",
        };
        f.write_str(code)
    }
}

impl From<RequestedCategory> for UnquotedString {
    fn from(value: RequestedCategory) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<RequestedCategory> for CharString {
    fn from(value: RequestedCategory) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<RequestedCategory> for Value {
    fn from(value: RequestedCategory) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl RequestedCategory {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "AD" => Some(Self::AD),
            "CI" => Some(Self::CI),
            "CM" => Some(Self::CM),
            "CO" => Some(Self::CO),
            "EI" => Some(Self::EI),
            "EM" => Some(Self::EM),
            "EO" => Some(Self::EO),
            "FA" => Some(Self::FA),
            "FI" => Some(Self::FI),
            "FM" => Some(Self::FM),
            "FO" => Some(Self::FO),
            "GI" => Some(Self::GI),
            "GM" => Some(Self::GM),
            "GO" => Some(Self::GO),
            "HI" => Some(Self::HI),
            "HM" => Some(Self::HM),
            "HO" => Some(Self::HO),
            "QI" => Some(Self::QI),
            "QM" => Some(Self::QM),
            "QO" => Some(Self::QO),
            "SC" => Some(Self::SC),
            _ => None,
        }
    }
}
