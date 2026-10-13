//! The closed code sets of the `refine` category.

use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The range-check and closed-set error of the `refine` category.
///
/// Follows the style of `CrystalSystemCifError`: a dedicated error
/// type with `Display` and `std::error::Error`.
#[derive(Debug)]
pub struct RefineCifError(String);

impl RefineCifError {
    pub(super) fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for RefineCifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for RefineCifError {}

/// The closed value set of `_refine_ls_hydrogen_treatment`.
///
/// The dictionary records the value `refU` with a capital `U`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HydrogenTreatment {
    Refall,
    Refxyz,
    RefU,
    Noref,
    Constr,
    Hetero,
}

impl HydrogenTreatment {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Refall => "refall",
            Self::Refxyz => "refxyz",
            Self::RefU => "refU",
            Self::Noref => "noref",
            Self::Constr => "constr",
            Self::Hetero => "hetero",
        }
    }
}

impl Display for HydrogenTreatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<HydrogenTreatment> for UnquotedString {
    fn from(value: HydrogenTreatment) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<HydrogenTreatment> for CharString {
    fn from(value: HydrogenTreatment) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<HydrogenTreatment> for Value {
    fn from(value: HydrogenTreatment) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for HydrogenTreatment {
    type Error = RefineCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "refall" => Ok(Self::Refall),
            "refxyz" => Ok(Self::Refxyz),
            "refu" => Ok(Self::RefU),
            "noref" => Ok(Self::Noref),
            "constr" => Ok(Self::Constr),
            "hetero" => Ok(Self::Hetero),
            other => Err(RefineCifError::new(format!(
                "'{other}' is not a value of the _refine_ls_hydrogen_treatment closed set"
            ))),
        }
    }
}

impl TryFrom<String> for HydrogenTreatment {
    type Error = RefineCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// The closed value set of `_refine_ls_matrix_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixType {
    Full,
    Fullcycle,
    Atomblock,
    Userblock,
    Diagonal,
    Sparse,
}

impl MatrixType {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Full => "full",
            Self::Fullcycle => "fullcycle",
            Self::Atomblock => "atomblock",
            Self::Userblock => "userblock",
            Self::Diagonal => "diagonal",
            Self::Sparse => "sparse",
        }
    }
}

impl Display for MatrixType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<MatrixType> for UnquotedString {
    fn from(value: MatrixType) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<MatrixType> for CharString {
    fn from(value: MatrixType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<MatrixType> for Value {
    fn from(value: MatrixType) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for MatrixType {
    type Error = RefineCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "full" => Ok(Self::Full),
            "fullcycle" => Ok(Self::Fullcycle),
            "atomblock" => Ok(Self::Atomblock),
            "userblock" => Ok(Self::Userblock),
            "diagonal" => Ok(Self::Diagonal),
            "sparse" => Ok(Self::Sparse),
            other => Err(RefineCifError::new(format!(
                "'{other}' is not a value of the _refine_ls_matrix_type closed set"
            ))),
        }
    }
}

impl TryFrom<String> for MatrixType {
    type Error = RefineCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// The closed value set of `_refine_ls_structure_factor_coef`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureFactorCoef {
    F,
    Fsqd,
    Inet,
}

impl StructureFactorCoef {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::F => "F",
            Self::Fsqd => "Fsqd",
            Self::Inet => "Inet",
        }
    }
}

impl Display for StructureFactorCoef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<StructureFactorCoef> for UnquotedString {
    fn from(value: StructureFactorCoef) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<StructureFactorCoef> for CharString {
    fn from(value: StructureFactorCoef) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<StructureFactorCoef> for Value {
    fn from(value: StructureFactorCoef) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for StructureFactorCoef {
    type Error = RefineCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "f" => Ok(Self::F),
            "fsqd" => Ok(Self::Fsqd),
            "inet" => Ok(Self::Inet),
            other => Err(RefineCifError::new(format!(
                "'{other}' is not a value of the _refine_ls_structure_factor_coef closed set"
            ))),
        }
    }
}

impl TryFrom<String> for StructureFactorCoef {
    type Error = RefineCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// The closed value set of `_refine_ls_weighting_scheme`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightingScheme {
    Sigma,
    Unit,
    Calc,
}

impl WeightingScheme {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Sigma => "sigma",
            Self::Unit => "unit",
            Self::Calc => "calc",
        }
    }
}

impl Display for WeightingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<WeightingScheme> for UnquotedString {
    fn from(value: WeightingScheme) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<WeightingScheme> for CharString {
    fn from(value: WeightingScheme) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<WeightingScheme> for Value {
    fn from(value: WeightingScheme) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for WeightingScheme {
    type Error = RefineCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "sigma" => Ok(Self::Sigma),
            "unit" => Ok(Self::Unit),
            "calc" => Ok(Self::Calc),
            other => Err(RefineCifError::new(format!(
                "'{other}' is not a value of the _refine_ls_weighting_scheme closed set"
            ))),
        }
    }
}

impl TryFrom<String> for WeightingScheme {
    type Error = RefineCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_sets_round_trip() {
        for value in [
            HydrogenTreatment::Refall,
            HydrogenTreatment::Refxyz,
            HydrogenTreatment::RefU,
            HydrogenTreatment::Noref,
            HydrogenTreatment::Constr,
            HydrogenTreatment::Hetero,
        ] {
            let parsed = HydrogenTreatment::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
        for value in [
            MatrixType::Full,
            MatrixType::Fullcycle,
            MatrixType::Atomblock,
            MatrixType::Userblock,
            MatrixType::Diagonal,
            MatrixType::Sparse,
        ] {
            let parsed = MatrixType::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
        for value in [
            StructureFactorCoef::F,
            StructureFactorCoef::Fsqd,
            StructureFactorCoef::Inet,
        ] {
            let parsed = StructureFactorCoef::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
        for value in [
            WeightingScheme::Sigma,
            WeightingScheme::Unit,
            WeightingScheme::Calc,
        ] {
            let parsed = WeightingScheme::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
    }

    #[test]
    fn closed_sets_accept_case_insensitive_input() {
        assert_eq!(
            HydrogenTreatment::try_from("refU").unwrap(),
            HydrogenTreatment::RefU
        );
        assert_eq!(
            HydrogenTreatment::try_from("REFU").unwrap(),
            HydrogenTreatment::RefU
        );
    }

    #[test]
    fn closed_sets_reject_unknown_values() {
        assert!(HydrogenTreatment::try_from("bogus").is_err());
        assert!(MatrixType::try_from("bogus").is_err());
        assert!(StructureFactorCoef::try_from("bogus").is_err());
        assert!(WeightingScheme::try_from("bogus").is_err());
    }
}
