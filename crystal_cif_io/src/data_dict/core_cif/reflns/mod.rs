//! The `reflns` category: the summary items of the measured
//! reflection set (roadmap T2, DDL1 `cif_core.dic`).
//!
//! Nine items are single-value: the Friedel coverage and fraction
//! items, the three reflection counts, and three open `char`
//! items. Eight items are loop columns: the resolution and index
//! limit items.

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Number, Numeric, Tag, UnsignedInteger, Value};
mod reflns;
pub use reflns::{Reflns, ReflnsDResolution, ReflnsLimit};

/// The range-check and closed-set error of the `reflns` category.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ReflnsCifError(String);

impl ReflnsCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}

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

    /// The read-path constructor. The recorded value stores as
    /// written; the range guard applies at the write.
    pub fn from_numeric(value: Numeric) -> Self {
        Self(value)
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
/// The single-value terms of the `reflns` category: one variant per

/// registry single-value item, in item order.
pub enum ReflnsTerms {
    /// The closed code set (yes, y, no, n) recorded in the shell
    /// dictionary; the plain text form is kept.
    /// The `_reflns_apply_dispersion_to_Fcalc` item. The variant holds the column values, one per loop row.
    Apply_dispersion_to_fcalc(CharString),
    /// The `_reflns_Friedel_coverage` item. The variant holds the column values, one per loop row.
    ///
    /// The proportion of Friedel-related reflections present in the number
    /// of 'independent' reflections specified by the item
    /// _reflns_number_total. This proportion is calculated as the ratio:
    /// [N(crystal class) - N(Laue symmetry)] / N(Laue symmetry) where,
    /// working from the _diffrn_refln_ list, N(crystal class) is the number
    /// of reflections obtained on averaging under the symmetry of the
    /// crystal class N(Laue symmetry) is the number of reflections obtained
    /// on averaging under the Laue symmetry. Examples: (a) For
    /// centrosymmetric structures, _reflns_Friedel_coverage is necessarily
    /// equal to 0.0 as the crystal class is identical to the Laue symmetry.
    /// (b) For whole-sphere data for a crystal in the space group P1,
    /// _reflns_Friedel_coverage is equal to 1.0, as no reflection h k l is
    /// equivalent to -h -k -l in the crystal class and all Friedel pairs {h
    /// k l; -h -k -l} have been measured. (c) For whole-sphere data in
    /// space group Pmm2, _reflns_Friedel_coverage will be &lt; 1.0 because
    /// although reflections h k l and -h -k -l are not equivalent when h k
    /// l indices are nonzero, they are when l=0. (d) For a crystal in the
    /// space group Pmm2, measurements of the two inequivalent octants h >=
    /// 0, k >=0, l lead to the same value as in (c), whereas measurements
    /// of the two equivalent octants h >= 0, k, l >= 0 will lead to a value
    /// of zero for _reflns_Friedel_coverage.
    Friedel_coverage(FriedelFraction),
    /// The `_reflns_Friedel_fraction_full` item. The variant holds the column values, one per loop row.
    ///
    /// The number of Friedel pairs measured out to
    /// _diffrn_reflns_theta_full. divided by the number theoretically
    /// possible (ignoring reflections in centric projections and systematic
    /// absences throughout). In contrast to _reflns_Friedel_coverage this
    /// can take values in the full range 0 to 1 for any non-centrosymmetric
    /// space group, and so one can see at a glance how completely the
    /// Friedel pairs have been measured. For centrosymmetric space groups
    /// the value would be 0/0 and so would be given as '.'.
    Friedel_fraction_full(FriedelFraction),
    /// The `_reflns_Friedel_fraction_max` item. The variant holds the column values, one per loop row.
    ///
    /// The number of Friedel pairs measured out to
    /// _diffrn_reflns_theta_max. divided by the number theoretically
    /// possible (ignoring reflections in centric projections and systematic
    /// absences throughout). In contrast to _reflns_Friedel_coverage this
    /// can take values in the full range 0 to 1 for any non-centrosymmetric
    /// space group, and so one can see at a glance how completely the
    /// Friedel pairs have been measured. For centrosymmetric space groups
    /// the value would be 0/0 and so would be given as '.'.
    Friedel_fraction_max(FriedelFraction),
    /// The `_reflns_limit_max` item. The variant holds the column values, one per loop row.
    Limit_max(Matrix),
    /// The `_reflns_limit_min` item. The variant holds the column values, one per loop row.
    Limit_min(Matrix),
    /// The `_reflns_number_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The number of reflections in the _refln_ list (not the
    /// _diffrn_refln_ list) that are significantly intense, satisfying the
    /// criterion specified by _reflns_threshold_expression. This may
    /// include Friedel-equivalent reflections (i.e. those which are
    /// symmetry-equivalent under the Laue symmetry but inequivalent under
    /// the crystal class) according to the nature of the structure and the
    /// procedures used. Special characteristics of the reflections included
    /// in the _refln_ list should be given in the item
    /// _reflns_special_details.
    Number_gt(UnsignedInteger),
    /// The `_reflns_number_observed` item. The variant holds the column values, one per loop row.
    ///
    /// The number of 'observed' reflections in the _refln_ list (not the
    /// _diffrn_refln_ list). The observed reflections satisfy the threshold
    /// criterion specified by _reflns_threshold_expression (or the
    /// deprecated item _reflns_observed_criterion). They may include
    /// Friedel-equivalent reflections according to the nature of the
    /// structure and the procedures used. Special characteristics of the
    /// reflections included in the _refln_ list should be given in the item
    /// _reflns_special_details.
    Number_observed(UnsignedInteger),
    /// The `_reflns_number_total` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of reflections in the _refln_ list (not the
    /// _diffrn_refln_ list). This may include Friedel-equivalent
    /// reflections (i.e. those which are symmetry-equivalent under the Laue
    /// symmetry but inequivalent under the crystal class) according to the
    /// nature of the structure and the procedures used. Special
    /// characteristics of the reflections included in the _refln_ list
    /// should be given in the item _reflns_special_details.
    Number_total(UnsignedInteger),
    /// The `_reflns_observed_criterion` item. The variant holds the column values, one per loop row.
    ///
    /// The criterion used to classify a reflection as 'observed'. This
    /// criterion is usually expressed in terms of a sigma(I) or sigma(F)
    /// threshold.
    Observed_criterion(CharString),
    /// The `_reflns_special_details` item. The variant holds the column values, one per loop row.
    ///
    /// Description of the properties of the reported reflection list that
    /// are not given in other data items. In particular, this should
    /// include information about the averaging (or not) of
    /// symmetry-equivalent reflections including Friedel pairs.
    Special_details(CharString),
    /// The `_reflns_threshold_expression` item. The variant holds the column values, one per loop row.
    ///
    /// The threshold, usually based on multiples of u(I), u(F^2^) or u(F),
    /// that serves to identify significantly intense reflections, the
    /// number of which is given by _reflns_number_gt. These reflections are
    /// used in the calculation of _refine_ls_R_factor_gt.
    Threshold_expression(CharString),
}

impl CifTerm for ReflnsTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsTerms::Apply_dispersion_to_fcalc(_) => "apply_dispersion_to_fcalc",
            ReflnsTerms::Friedel_coverage(_) => "Friedel_coverage",
            ReflnsTerms::Friedel_fraction_full(_) => "Friedel_fraction_full",
            ReflnsTerms::Friedel_fraction_max(_) => "Friedel_fraction_max",
            ReflnsTerms::Limit_max(_) => "limit_max",
            ReflnsTerms::Limit_min(_) => "limit_min",
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
            ReflnsTerms::Apply_dispersion_to_fcalc(s) => Value::CharString(s.clone()),
            ReflnsTerms::Friedel_coverage(v)
            | ReflnsTerms::Friedel_fraction_full(v)
            | ReflnsTerms::Friedel_fraction_max(v) => (*v).into(),
            ReflnsTerms::Limit_max(m) | ReflnsTerms::Limit_min(m) => matrix::to_value(m),
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
/// The loop-value terms of the `reflns` category: one variant per

/// registry loop-value item, in item order.
pub enum ReflnsLoopItem {
    /// The `_reflns_d_resolution_high` item. The variant holds the column values, one per loop row.
    ///
    /// The highest and lowest resolution in angstroms for the reflections.
    /// These are the smallest and largest d values.
    /// Units: A (angstrom).
    D_resolution_high(Vec<Numeric>),
    /// The `_reflns_d_resolution_low` item. The variant holds the column values, one per loop row.
    ///
    /// The highest and lowest resolution in angstroms for the reflections.
    /// These are the smallest and largest d values.
    /// Units: A (angstrom).
    D_resolution_low(Vec<Numeric>),
    /// The `_reflns_limit_h_max` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices limits for the reported reflections. These need not
    /// be the same as the _diffrn_reflns_limit_ values.
    Limit_h_max(Vec<Numeric>),
    /// The `_reflns_limit_h_min` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices limits for the reported reflections. These need not
    /// be the same as the _diffrn_reflns_limit_ values.
    Limit_h_min(Vec<Numeric>),
    /// The `_reflns_limit_k_max` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices limits for the reported reflections. These need not
    /// be the same as the _diffrn_reflns_limit_ values.
    Limit_k_max(Vec<Numeric>),
    /// The `_reflns_limit_k_min` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices limits for the reported reflections. These need not
    /// be the same as the _diffrn_reflns_limit_ values.
    Limit_k_min(Vec<Numeric>),
    /// The `_reflns_limit_l_max` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices limits for the reported reflections. These need not
    /// be the same as the _diffrn_reflns_limit_ values.
    Limit_l_max(Vec<Numeric>),
    /// The `_reflns_limit_l_min` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices limits for the reported reflections. These need not
    /// be the same as the _diffrn_reflns_limit_ values.
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
    use crystal_cif_fmt::grammar::UnquotedString;

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
