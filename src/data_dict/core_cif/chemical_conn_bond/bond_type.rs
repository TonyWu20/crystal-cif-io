use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_chemical_conn_bond_type`. The codes
/// describe the bond order and character in the connection table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondType {
    /// `sing`: single bond.
    Sing,
    /// `doub`: double bond.
    Doub,
    /// `trip`: triple bond.
    Trip,
    /// `quad`: quadruple bond.
    Quad,
    /// `arom`: aromatic bond.
    Arom,
    /// `poly`: polymeric bond.
    Poly,
    /// `delo`: delocalized double bond.
    Delo,
    /// `pi`: pi bond.
    Pi,
}

impl Display for BondType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            BondType::Sing => "sing",
            BondType::Doub => "doub",
            BondType::Trip => "trip",
            BondType::Quad => "quad",
            BondType::Arom => "arom",
            BondType::Poly => "poly",
            BondType::Delo => "delo",
            BondType::Pi => "pi",
        };
        f.write_str(code)
    }
}

impl From<BondType> for UnquotedString {
    fn from(value: BondType) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<BondType> for CharString {
    fn from(value: BondType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<BondType> for Value {
    fn from(value: BondType) -> Self {
        Value::CharString(CharString::from(value))
    }
}
