//! The closed code sets of the `symmetry` category.

use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

use super::SymmetryCifError;

/// The closed value set of `_symmetry_cell_setting`.
///
/// The set names the eight crystal systems, including
/// `rhombohedral`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellSetting {
    /// `triclinic`
    Triclinic,
    /// `monoclinic`
    Monoclinic,
    /// `orthorhombic`
    Orthorhombic,
    /// `tetragonal`
    Tetragonal,
    /// `rhombohedral`
    Rhombohedral,
    /// `trigonal`
    Trigonal,
    /// `hexagonal`
    Hexagonal,
    /// `cubic`
    Cubic,
}

impl CellSetting {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Triclinic => "triclinic",
            Self::Monoclinic => "monoclinic",
            Self::Orthorhombic => "orthorhombic",
            Self::Tetragonal => "tetragonal",
            Self::Rhombohedral => "rhombohedral",
            Self::Trigonal => "trigonal",
            Self::Hexagonal => "hexagonal",
            Self::Cubic => "cubic",
        }
    }
}

impl Display for CellSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<CellSetting> for UnquotedString {
    fn from(value: CellSetting) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<CellSetting> for CharString {
    fn from(value: CellSetting) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<CellSetting> for Value {
    fn from(value: CellSetting) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for CellSetting {
    type Error = SymmetryCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "triclinic" => Ok(Self::Triclinic),
            "monoclinic" => Ok(Self::Monoclinic),
            "orthorhombic" => Ok(Self::Orthorhombic),
            "tetragonal" => Ok(Self::Tetragonal),
            "rhombohedral" => Ok(Self::Rhombohedral),
            "trigonal" => Ok(Self::Trigonal),
            "hexagonal" => Ok(Self::Hexagonal),
            "cubic" => Ok(Self::Cubic),
            other => Err(SymmetryCifError::new(format!(
                "'{other}' is not a value of the _symmetry_cell_setting closed set"
            ))),
        }
    }
}

impl TryFrom<String> for CellSetting {
    type Error = SymmetryCifError;

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
            CellSetting::Triclinic,
            CellSetting::Monoclinic,
            CellSetting::Orthorhombic,
            CellSetting::Tetragonal,
            CellSetting::Rhombohedral,
            CellSetting::Trigonal,
            CellSetting::Hexagonal,
            CellSetting::Cubic,
        ] {
            let parsed = CellSetting::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
    }

    #[test]
    fn closed_set_rejects_unknown_values() {
        assert!(CellSetting::try_from("square").is_err());
    }
}
