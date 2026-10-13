//! The closed code sets of the `refln` category.

use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed-set error of the `refln` category.
///
/// Follows the style of `CrystalSystemCifError`: a dedicated error
/// type with `Display` and `std::error::Error`.
#[derive(Debug)]
pub struct ReflnCifError(String);

impl ReflnCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for ReflnCifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ReflnCifError {}

/// The closed value set of `_refln_include_status`.
///
/// The dictionary records the single value `o` (observed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncludeStatus {
    Observed,
}

impl IncludeStatus {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Observed => "o",
        }
    }
}

impl Display for IncludeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<IncludeStatus> for UnquotedString {
    fn from(value: IncludeStatus) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<IncludeStatus> for CharString {
    fn from(value: IncludeStatus) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<IncludeStatus> for Value {
    fn from(value: IncludeStatus) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for IncludeStatus {
    type Error = ReflnCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "o" => Ok(Self::Observed),
            other => Err(ReflnCifError::new(format!(
                "'{other}' is not a value of the _refln_include_status closed set"
            ))),
        }
    }
}

impl TryFrom<String> for IncludeStatus {
    type Error = ReflnCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// The closed value set of `_refln_observed_status`.
///
/// The dictionary records the single value `o` (observed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservedStatus {
    Observed,
}

impl ObservedStatus {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Observed => "o",
        }
    }
}

impl Display for ObservedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<ObservedStatus> for UnquotedString {
    fn from(value: ObservedStatus) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<ObservedStatus> for CharString {
    fn from(value: ObservedStatus) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<ObservedStatus> for Value {
    fn from(value: ObservedStatus) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for ObservedStatus {
    type Error = ReflnCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "o" => Ok(Self::Observed),
            other => Err(ReflnCifError::new(format!(
                "'{other}' is not a value of the _refln_observed_status closed set"
            ))),
        }
    }
}

impl TryFrom<String> for ObservedStatus {
    type Error = ReflnCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// The closed value set of `_refln_refinement_status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefinementStatus {
    Included,
    Excluded,
    External,
}

impl RefinementStatus {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Included => "incl",
            Self::Excluded => "excl",
            Self::External => "extn",
        }
    }
}

impl Display for RefinementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<RefinementStatus> for UnquotedString {
    fn from(value: RefinementStatus) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<RefinementStatus> for CharString {
    fn from(value: RefinementStatus) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<RefinementStatus> for Value {
    fn from(value: RefinementStatus) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for RefinementStatus {
    type Error = ReflnCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "incl" => Ok(Self::Included),
            "excl" => Ok(Self::Excluded),
            "extn" => Ok(Self::External),
            other => Err(ReflnCifError::new(format!(
                "'{other}' is not a value of the _refln_refinement_status closed set"
            ))),
        }
    }
}

impl TryFrom<String> for RefinementStatus {
    type Error = ReflnCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_sets_round_trip() {
        {
            let value = IncludeStatus::Observed;
            let parsed = IncludeStatus::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
        {
            let value = ObservedStatus::Observed;
            let parsed = ObservedStatus::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
        for value in [
            RefinementStatus::Included,
            RefinementStatus::Excluded,
            RefinementStatus::External,
        ] {
            let parsed = RefinementStatus::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
    }

    #[test]
    fn closed_sets_reject_unknown_values() {
        assert!(IncludeStatus::try_from("x").is_err());
        assert!(ObservedStatus::try_from("x").is_err());
        assert!(RefinementStatus::try_from("x").is_err());
    }
}
