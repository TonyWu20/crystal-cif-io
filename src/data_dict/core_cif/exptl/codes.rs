//! The closed code sets of the `exptl` category.

use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

use super::ExptlCifError;

/// The closed value set of `_exptl_absorpt_correction_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsorptCorrectionType {
    Analytical,
    Cylinder,
    Empirical,
    Gaussian,
    Integration,
    MultiScan,
    None,
    Numerical,
    PsiScan,
    Refdelf,
    Sphere,
}

impl AbsorptCorrectionType {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Analytical => "analytical",
            Self::Cylinder => "cylinder",
            Self::Empirical => "empirical",
            Self::Gaussian => "gaussian",
            Self::Integration => "integration",
            Self::MultiScan => "multi-scan",
            Self::None => "none",
            Self::Numerical => "numerical",
            Self::PsiScan => "psi-scan",
            Self::Refdelf => "refdelf",
            Self::Sphere => "sphere",
        }
    }
}

impl Display for AbsorptCorrectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<AbsorptCorrectionType> for UnquotedString {
    fn from(value: AbsorptCorrectionType) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<AbsorptCorrectionType> for CharString {
    fn from(value: AbsorptCorrectionType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<AbsorptCorrectionType> for Value {
    fn from(value: AbsorptCorrectionType) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for AbsorptCorrectionType {
    type Error = ExptlCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "analytical" => Ok(Self::Analytical),
            "cylinder" => Ok(Self::Cylinder),
            "empirical" => Ok(Self::Empirical),
            "gaussian" => Ok(Self::Gaussian),
            "integration" => Ok(Self::Integration),
            "multi-scan" => Ok(Self::MultiScan),
            "none" => Ok(Self::None),
            "numerical" => Ok(Self::Numerical),
            "psi-scan" => Ok(Self::PsiScan),
            "refdelf" => Ok(Self::Refdelf),
            "sphere" => Ok(Self::Sphere),
            other => Err(ExptlCifError::new(format!(
                "'{other}' is not a value of the _exptl_absorpt_correction_type closed set"
            ))),
        }
    }
}

impl TryFrom<String> for AbsorptCorrectionType {
    type Error = ExptlCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_set_round_trip() {
        for value in [
            AbsorptCorrectionType::Analytical,
            AbsorptCorrectionType::Cylinder,
            AbsorptCorrectionType::Empirical,
            AbsorptCorrectionType::Gaussian,
            AbsorptCorrectionType::Integration,
            AbsorptCorrectionType::MultiScan,
            AbsorptCorrectionType::None,
            AbsorptCorrectionType::Numerical,
            AbsorptCorrectionType::PsiScan,
            AbsorptCorrectionType::Refdelf,
            AbsorptCorrectionType::Sphere,
        ] {
            let spelled = value.dictionary_value().to_string();
            let parsed = AbsorptCorrectionType::try_from(spelled.as_str()).unwrap();
            assert_eq!(value, parsed);
            let cif_value: Value = value.into();
            assert_eq!(
                cif_value.as_char_string().unwrap().as_ref(),
                value.dictionary_value()
            );
        }
    }

    #[test]
    fn closed_set_rejects_unknown_values() {
        assert!(AbsorptCorrectionType::try_from("bogus").is_err());
        assert!(AbsorptCorrectionType::try_from(String::from("bogus")).is_err());
    }
}
