//! The `exptl` category: the experimental details recorded before the
//! intensity measurement and the absorption-correction technique
//! (roadmap T2, DDL1 `cif_core.dic`).
//!
//! The category holds seven single-value items. The dictionary
//! records the two absorption-correction T factors as a loop family.

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Number, Numeric, Tag, Value},
};

mod codes;

pub use codes::AbsorptCorrectionType;

/// The range-check and closed-set error of the `exptl` category.
///
/// Follows the style of `CrystalSystemCifError`: a dedicated error
/// type with `Display` and `std::error::Error`.
#[derive(Debug)]
pub struct ExptlCifError(String);

impl ExptlCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for ExptlCifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ExptlCifError {}

/// Read the value of a `Numeric` as an `f64`.
fn numeric_value(value: &Numeric) -> f64 {
    match value.number() {
        Number::Integer(i) => i.0 as f64,
        Number::Float(f) => f.0,
    }
}

/// The value of `_exptl_absorpt_coefficient_mu`.
///
/// The dictionary range is `0.0:`. The try constructor rejects
/// negative values at write time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbsorptCoefficient(Numeric);

impl AbsorptCoefficient {
    /// Validates the `0.0:` range of the item.
    pub fn try_new(value: Numeric) -> Result<Self, ExptlCifError> {
        let found = numeric_value(&value);
        if found < 0.0 {
            return Err(ExptlCifError::new(format!(
                "_exptl_absorpt_coefficient_mu: {found} is below the 0.0 range"
            )));
        }
        Ok(Self(value))
    }

    /// The held value.
    pub fn numeric(&self) -> &Numeric {
        &self.0
    }
}

impl Display for AbsorptCoefficient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AbsorptCoefficient> for Value {
    fn from(value: AbsorptCoefficient) -> Self {
        Value::Numeric(value.0)
    }
}

/// The value of the transmission-factor items of the `exptl`
/// category.
///
/// The dictionary range is `0.0:1.0`. The try constructor rejects
/// values outside the unit interval at write time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransmissionFactor(Numeric);

impl TransmissionFactor {
    /// Validates the `0.0:1.0` range of the item.
    pub fn try_new(value: Numeric, item: &str) -> Result<Self, ExptlCifError> {
        let found = numeric_value(&value);
        if !(0.0..=1.0).contains(&found) {
            return Err(ExptlCifError::new(format!(
                "_{item}: {found} is outside the 0.0:1.0 range"
            )));
        }
        Ok(Self(value))
    }

    /// The held value.
    pub fn numeric(&self) -> &Numeric {
        &self.0
    }
}

impl Display for TransmissionFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<TransmissionFactor> for Value {
    fn from(value: TransmissionFactor) -> Self {
        Value::Numeric(value.0)
    }
}

/// The value of `_exptl_crystals_number`.
///
/// The dictionary range is `1:`: a positive integer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrystalCount(Numeric);

impl CrystalCount {
    /// Validates the `1:` range of the item. Rejects a non-integer
    /// value and a zero value at write time.
    pub fn try_new(value: Numeric) -> Result<Self, ExptlCifError> {
        let number = value.number();
        let integer = number.as_integer().ok_or_else(|| {
            ExptlCifError::new(
                "_exptl_crystals_number: expected an integer value in the 1: range".to_string(),
            )
        })?;
        if integer.0 < 1 {
            return Err(ExptlCifError::new(format!(
                "_exptl_crystals_number: {integer} is below the 1: range"
            )));
        }
        Ok(Self(value))
    }

    /// The held value.
    pub fn numeric(&self) -> &Numeric {
        &self.0
    }
}

impl Display for CrystalCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CrystalCount> for Value {
    fn from(value: CrystalCount) -> Self {
        Value::Numeric(value.0)
    }
}

/// The single-value items of the `exptl` category.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ExptlTerms {
    Absorpt_coefficient_mu(AbsorptCoefficient),
    Absorpt_correction_type(AbsorptCorrectionType),
    Absorpt_process_details(CharString),
    Crystals_number(CrystalCount),
    Special_details(CharString),
    Transmission_factor_max(TransmissionFactor),
    Transmission_factor_min(TransmissionFactor),
}

impl CifTerm for ExptlTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ExptlTerms::Absorpt_coefficient_mu(_) => "absorpt_coefficient_mu",
            ExptlTerms::Absorpt_correction_type(_) => "absorpt_correction_type",
            ExptlTerms::Absorpt_process_details(_) => "absorpt_process_details",
            ExptlTerms::Crystals_number(_) => "crystals_number",
            ExptlTerms::Special_details(_) => "special_details",
            ExptlTerms::Transmission_factor_max(_) => "transmission_factor_max",
            ExptlTerms::Transmission_factor_min(_) => "transmission_factor_min",
        };
        Tag::new(format!("exptl_{suffix}"))
    }
}

impl SingleValueTerm for ExptlTerms {
    fn value(&self) -> Value {
        match self {
            ExptlTerms::Absorpt_coefficient_mu(v) => (*v).into(),
            ExptlTerms::Absorpt_correction_type(c) => (*c).into(),
            ExptlTerms::Absorpt_process_details(s) | ExptlTerms::Special_details(s) => {
                Value::CharString(s.clone())
            }
            ExptlTerms::Crystals_number(v) => (*v).into(),
            ExptlTerms::Transmission_factor_max(v) | ExptlTerms::Transmission_factor_min(v) => {
                (*v).into()
            }
        }
    }
}

impl Display for ExptlTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop items of the `exptl` category.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ExptlLoopItem {
    Absorpt_correction_T_max(Vec<Numeric>),
    Absorpt_correction_T_min(Vec<Numeric>),
}

impl CifTerm for ExptlLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ExptlLoopItem::Absorpt_correction_T_max(_) => "absorpt_correction_T_max",
            ExptlLoopItem::Absorpt_correction_T_min(_) => "absorpt_correction_T_min",
        };
        Tag::new(format!("exptl_{suffix}"))
    }
}

impl LoopValueTerm for ExptlLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ExptlLoopItem::Absorpt_correction_T_max(v)
            | ExptlLoopItem::Absorpt_correction_T_min(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for ExptlLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn char_value() -> CharString {
        CharString::Unquoted(crate::grammar::UnquotedString::new(".".into()))
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let terms: Vec<ExptlTerms> = vec![
            ExptlTerms::Absorpt_coefficient_mu(
                AbsorptCoefficient::try_new(Numeric::from(0.962)).unwrap(),
            ),
            ExptlTerms::Absorpt_correction_type(AbsorptCorrectionType::None),
            ExptlTerms::Absorpt_process_details(char_value()),
            ExptlTerms::Crystals_number(CrystalCount::try_new(Numeric::from(1)).unwrap()),
            ExptlTerms::Special_details(char_value()),
            ExptlTerms::Transmission_factor_max(
                TransmissionFactor::try_new(Numeric::from(0.997), "exptl_transmission_factor_max")
                    .unwrap(),
            ),
            ExptlTerms::Transmission_factor_min(
                TransmissionFactor::try_new(Numeric::from(0.929), "exptl_transmission_factor_min")
                    .unwrap(),
            ),
        ];
        let expected: Vec<&str> = vec![
            "exptl_absorpt_coefficient_mu",
            "exptl_absorpt_correction_type",
            "exptl_absorpt_process_details",
            "exptl_crystals_number",
            "exptl_special_details",
            "exptl_transmission_factor_max",
            "exptl_transmission_factor_min",
        ];
        for (term, tag) in terms.iter().zip(expected) {
            assert_eq!(term.tag().as_ref(), tag, "ExptlTerms tag drift");
        }
    }

    #[test]
    fn loop_item_tags_match_the_inventory() {
        let item = ExptlLoopItem::Absorpt_correction_T_max(vec![Numeric::from(0.997)]);
        assert_eq!(item.tag().as_ref(), "exptl_absorpt_correction_T_max");
        let item = ExptlLoopItem::Absorpt_correction_T_min(vec![Numeric::from(0.929)]);
        assert_eq!(item.tag().as_ref(), "exptl_absorpt_correction_T_min");
    }

    #[test]
    fn range_checks_reject_out_of_range_values() {
        assert!(AbsorptCoefficient::try_new(Numeric::from(-0.1)).is_err());
        assert!(
            TransmissionFactor::try_new(Numeric::from(1.5), "exptl_transmission_factor_max")
                .is_err()
        );
        assert!(CrystalCount::try_new(Numeric::from(0)).is_err());
        assert!(CrystalCount::try_new(Numeric::from(0.5)).is_err());
    }
}
