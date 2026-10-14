//! The `symmetry` category: the single-value items that record the
//! space-group name, the cell setting, and the International Tables
//! number (roadmap T2, DDL1 `cif_core.dic`).

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

mod codes;
mod symmetry;
pub use symmetry::Symmetry;

pub use codes::CellSetting;

/// The range-check and closed-set error of the `symmetry` category.
///
/// Follows the style of `CrystalSystemCifError`: a dedicated error
/// type with `Display` and `std::error::Error`.
#[derive(Debug)]
pub struct SymmetryCifError(String);

impl SymmetryCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for SymmetryCifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SymmetryCifError {}

/// The value of `_symmetry_Int_Tables_number`.
///
/// The dictionary range is `1:230`. The try constructor rejects a
/// non-integer value and a value outside the range at write time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntTablesNumber(Numeric);

impl IntTablesNumber {
    /// Validates the `1:230` range of the item.
    pub fn try_new(value: Numeric) -> Result<Self, SymmetryCifError> {
        let number = value.number();
        let integer = number.as_integer().ok_or_else(|| {
            SymmetryCifError::new(
                "_symmetry_Int_Tables_number: expected an integer value in the 1:230 range"
                    .to_string(),
            )
        })?;
        let found = integer.0;
        if !(1..=230).contains(&found) {
            return Err(SymmetryCifError::new(format!(
                "_symmetry_Int_Tables_number: {integer} is outside the 1:230 range"
            )));
        }
        Ok(Self(value))
    }

    /// The held value.
    pub fn numeric(&self) -> &Numeric {
        &self.0
    }

    /// The read-path constructor. The recorded value stores as
    /// written; the range guard applies at the write.
    pub fn from_numeric(value: Numeric) -> Self {
        Self(value)
    }
}

impl Display for IntTablesNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<IntTablesNumber> for Value {
    fn from(value: IntTablesNumber) -> Self {
        Value::Numeric(value.0)
    }
}

/// The single-value items of the `symmetry` category.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SymmetryTerms {
    Int_Tables_number(IntTablesNumber),
    Cell_setting(CellSetting),
    Space_group_name_H_M(CharString),
    Space_group_name_Hall(CharString),
}

impl CifTerm for SymmetryTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SymmetryTerms::Int_Tables_number(_) => "Int_Tables_number",
            SymmetryTerms::Cell_setting(_) => "cell_setting",
            SymmetryTerms::Space_group_name_H_M(_) => "space_group_name_H-M",
            SymmetryTerms::Space_group_name_Hall(_) => "space_group_name_Hall",
        };
        Tag::new(format!("symmetry_{suffix}"))
    }
}

impl SingleValueTerm for SymmetryTerms {
    fn value(&self) -> Value {
        match self {
            SymmetryTerms::Int_Tables_number(v) => (*v).into(),
            SymmetryTerms::Cell_setting(c) => (*c).into(),
            SymmetryTerms::Space_group_name_H_M(s) | SymmetryTerms::Space_group_name_Hall(s) => {
                Value::CharString(s.clone())
            }
        }
    }
}

impl Display for SymmetryTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::grammar::UnquotedString;

    fn cs() -> CharString {
        CharString::Unquoted(UnquotedString::new("P 42/1 c".into()))
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let terms: Vec<SymmetryTerms> = vec![
            SymmetryTerms::Int_Tables_number(IntTablesNumber::try_new(Numeric::from(79)).unwrap()),
            SymmetryTerms::Cell_setting(CellSetting::Tetragonal),
            SymmetryTerms::Space_group_name_H_M(cs()),
            SymmetryTerms::Space_group_name_Hall(CharString::Unquoted(UnquotedString::new(
                "-P 4a 2c".into(),
            ))),
        ];
        let expected: Vec<&str> = vec![
            "symmetry_Int_Tables_number",
            "symmetry_cell_setting",
            "symmetry_space_group_name_H-M",
            "symmetry_space_group_name_Hall",
        ];
        for (term, tag) in terms.iter().zip(expected) {
            assert_eq!(term.tag().as_ref(), tag, "SymmetryTerms tag drift");
        }
    }

    #[test]
    fn int_tables_range_check_rejects_out_of_range_values() {
        assert!(IntTablesNumber::try_new(Numeric::from(0)).is_err());
        assert!(IntTablesNumber::try_new(Numeric::from(231)).is_err());
        assert!(IntTablesNumber::try_new(Numeric::from(1.5)).is_err());
        assert!(IntTablesNumber::try_new(Numeric::from(79)).is_ok());
    }
}
