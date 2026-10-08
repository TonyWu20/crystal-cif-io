//! The `reflns` category: the summary items of the measured
//! reflection set (roadmap T2, DDL1 `cif_core.dic`).
//!
//! Nine items are single-value: the Friedel coverage and fraction
//! items, the three reflection counts, and three open `char`
//! items. Eight items are loop columns: the resolution and index
//! limit items.

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Number, Numeric, Tag, UnsignedInteger, Value},
};

/// The range-check and closed-set error of the `reflns` category.
///
/// Follows the style of `CrystalSystemCifError`: a dedicated error
/// type with `Display` and `std::error::Error`.
#[derive(Debug)]
pub struct ReflnsCifError(String);

impl ReflnsCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for ReflnsCifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ReflnsCifError {}

/// Read the value of a `Numeric` as an `f64`.
fn numeric_value(value: &Numeric) -> f64 {
    match value.number() {
        Number::Integer(i) => i.0 as f64,
        Number::Float(f) => f.0,
    }
}

/// The value of the Friedel items of the `reflns` category.
///
/// The dictionary range is `0.0:1.0`. The try constructor rejects
/// values outside the unit interval at write time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FriedelFraction(Numeric);

impl FriedelFraction {
    /// Validates the `0.0:1.0` range of the item.
    pub fn try_new(value: Numeric, item: &str) -> Result<Self, ReflnsCifError> {
        let found = numeric_value(&value);
        if !(0.0..=1.0).contains(&found) {
            return Err(ReflnsCifError::new(format!(
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

impl Display for FriedelFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<FriedelFraction> for Value {
    fn from(value: FriedelFraction) -> Self {
        Value::Numeric(value.0)
    }
}

/// The single-value items of the `reflns` category.
///
/// The three count items carry the `0:` range by construction: an
/// `UnsignedInteger` value is always non-negative.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ReflnsTerms {
    Friedel_coverage(FriedelFraction),
    Friedel_fraction_full(FriedelFraction),
    Friedel_fraction_max(FriedelFraction),
    Number_gt(UnsignedInteger),
    Number_observed(UnsignedInteger),
    Number_total(UnsignedInteger),
    Observed_criterion(CharString),
    Special_details(CharString),
    Threshold_expression(CharString),
}

impl CifTerm for ReflnsTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsTerms::Friedel_coverage(_) => "Friedel_coverage",
            ReflnsTerms::Friedel_fraction_full(_) => "Friedel_fraction_full",
            ReflnsTerms::Friedel_fraction_max(_) => "Friedel_fraction_max",
            ReflnsTerms::Number_gt(_) => "number_gt",
            ReflnsTerms::Number_observed(_) => "number_observed",
            ReflnsTerms::Number_total(_) => "number_total",
            ReflnsTerms::Observed_criterion(_) => "observed_criterion",
            ReflnsTerms::Special_details(_) => "special_details",
            ReflnsTerms::Threshold_expression(_) => "threshold_expression",
        };
        Tag::new(format!("reflns_{suffix}"))
    }
}

impl SingleValueTerm for ReflnsTerms {
    fn value(&self) -> Value {
        match self {
            ReflnsTerms::Friedel_coverage(v)
            | ReflnsTerms::Friedel_fraction_full(v)
            | ReflnsTerms::Friedel_fraction_max(v) => (*v).into(),
            ReflnsTerms::Number_gt(v)
            | ReflnsTerms::Number_observed(v)
            | ReflnsTerms::Number_total(v) => (*v).into(),
            ReflnsTerms::Observed_criterion(s)
            | ReflnsTerms::Special_details(s)
            | ReflnsTerms::Threshold_expression(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for ReflnsTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop items of the `reflns` category.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ReflnsLoopItem {
    D_resolution_high(Vec<Numeric>),
    D_resolution_low(Vec<Numeric>),
    Limit_h_max(Vec<Numeric>),
    Limit_h_min(Vec<Numeric>),
    Limit_k_max(Vec<Numeric>),
    Limit_k_min(Vec<Numeric>),
    Limit_l_max(Vec<Numeric>),
    Limit_l_min(Vec<Numeric>),
}

impl CifTerm for ReflnsLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsLoopItem::D_resolution_high(_) => "d_resolution_high",
            ReflnsLoopItem::D_resolution_low(_) => "d_resolution_low",
            ReflnsLoopItem::Limit_h_max(_) => "limit_h_max",
            ReflnsLoopItem::Limit_h_min(_) => "limit_h_min",
            ReflnsLoopItem::Limit_k_max(_) => "limit_k_max",
            ReflnsLoopItem::Limit_k_min(_) => "limit_k_min",
            ReflnsLoopItem::Limit_l_max(_) => "limit_l_max",
            ReflnsLoopItem::Limit_l_min(_) => "limit_l_min",
        };
        Tag::new(format!("reflns_{suffix}"))
    }
}

impl LoopValueTerm for ReflnsLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ReflnsLoopItem::D_resolution_high(v)
            | ReflnsLoopItem::D_resolution_low(v)
            | ReflnsLoopItem::Limit_h_max(v)
            | ReflnsLoopItem::Limit_h_min(v)
            | ReflnsLoopItem::Limit_k_max(v)
            | ReflnsLoopItem::Limit_k_min(v)
            | ReflnsLoopItem::Limit_l_max(v)
            | ReflnsLoopItem::Limit_l_min(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

impl Display for ReflnsLoopItem {
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
    use crate::grammar::UnquotedString;

    fn cs() -> CharString {
        CharString::Unquoted(UnquotedString::new("I > 2sigma(I)".into()))
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let terms: Vec<ReflnsTerms> = vec![
            ReflnsTerms::Friedel_coverage(
                FriedelFraction::try_new(Numeric::from(1.0), "reflns_Friedel_coverage").unwrap(),
            ),
            ReflnsTerms::Friedel_fraction_full(
                FriedelFraction::try_new(Numeric::from(1.0), "reflns_Friedel_fraction_full")
                    .unwrap(),
            ),
            ReflnsTerms::Friedel_fraction_max(
                FriedelFraction::try_new(Numeric::from(1.0), "reflns_Friedel_fraction_max")
                    .unwrap(),
            ),
            ReflnsTerms::Number_gt(UnsignedInteger(1286)),
            ReflnsTerms::Number_observed(UnsignedInteger(1286)),
            ReflnsTerms::Number_total(UnsignedInteger(1421)),
            ReflnsTerms::Observed_criterion(cs()),
            ReflnsTerms::Special_details(cs()),
            ReflnsTerms::Threshold_expression(cs()),
        ];
        let expected: Vec<&str> = vec![
            "reflns_Friedel_coverage",
            "reflns_Friedel_fraction_full",
            "reflns_Friedel_fraction_max",
            "reflns_number_gt",
            "reflns_number_observed",
            "reflns_number_total",
            "reflns_observed_criterion",
            "reflns_special_details",
            "reflns_threshold_expression",
        ];
        for (term, tag) in terms.iter().zip(expected) {
            assert_eq!(term.tag().as_ref(), tag, "ReflnsTerms tag drift");
        }

        let items: Vec<ReflnsLoopItem> = vec![
            ReflnsLoopItem::D_resolution_high(vec![Numeric::from(0.81)]),
            ReflnsLoopItem::D_resolution_low(vec![Numeric::from(1.39)]),
            ReflnsLoopItem::Limit_h_max(vec![Numeric::from(9)]),
            ReflnsLoopItem::Limit_h_min(vec![Numeric::from(-9)]),
            ReflnsLoopItem::Limit_k_max(vec![Numeric::from(9)]),
            ReflnsLoopItem::Limit_k_min(vec![Numeric::from(-9)]),
            ReflnsLoopItem::Limit_l_max(vec![Numeric::from(12)]),
            ReflnsLoopItem::Limit_l_min(vec![Numeric::from(-12)]),
        ];
        let expected: Vec<&str> = vec![
            "reflns_d_resolution_high",
            "reflns_d_resolution_low",
            "reflns_limit_h_max",
            "reflns_limit_h_min",
            "reflns_limit_k_max",
            "reflns_limit_k_min",
            "reflns_limit_l_max",
            "reflns_limit_l_min",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "ReflnsLoopItem tag drift");
        }
    }

    #[test]
    fn friedel_range_check_rejects_out_of_range_values() {
        assert!(FriedelFraction::try_new(Numeric::from(1.5), "reflns_Friedel_coverage").is_err());
        assert!(FriedelFraction::try_new(Numeric::from(-0.1), "reflns_Friedel_coverage").is_err());
    }
}
