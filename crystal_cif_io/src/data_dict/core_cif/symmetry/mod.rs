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
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct SymmetryCifError(String);

impl SymmetryCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}

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
/// The single-value terms of the `symmetry` category: one variant per

/// registry single-value item, in item order.
pub enum SymmetryTerms {
    /// The `_symmetry_Int_Tables_number` item. The variant holds the column values, one per loop row.
    ///
    /// Space-group number from International Tables for Crystallography
    /// Vol. A (2002).
    Int_Tables_number(IntTablesNumber),
    /// The `_symmetry_cell_setting` item. The variant holds the column values, one per loop row.
    ///
    /// The cell settings for this space-group symmetry.
    Cell_setting(CellSetting),
    /// The `_symmetry_space_group_name_H-M` item. The variant holds the column values, one per loop row.
    ///
    /// Hermann-Mauguin space-group symbol. Note that the Hermann-Mauguin
    /// symbol does not necessarily contain complete information about the
    /// symmetry and the space-group origin. If used, always supply the FULL
    /// symbol from International Tables for Crystallography Vol. A (2002)
    /// and indicate the origin and the setting if it is not implicit. If
    /// there is any doubt that the equivalent positions can be uniquely
    /// deduced from this symbol, specify the _symmetry_equiv_pos_as_xyz or
    /// *_Hall data items as well. Leave spaces between symbols referring to
    /// different axes.
    Space_group_name_H_M(CharString),
    /// The `_symmetry_space_group_name_Hall` item. The variant holds the column values, one per loop row.
    ///
    /// Space-group symbol as described by Hall. This symbol gives the
    /// space-group setting explicitly. Leave spaces between the separate
    /// components of the symbol. Ref: Hall, S. R. (1981). Acta Cryst. A37,
    /// 517-525; erratum (1981), A37, 921.
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
