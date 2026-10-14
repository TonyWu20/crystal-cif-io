//! The `refine` category: the single-value items that record the
//! least-squares refinement details and the three looped
//! difference-density items (roadmap T2, DDL1 `cif_core.dic`).
//!
//! The 43 single-value items split into open `char` items, four
//! closed code sets, count items with the `0:` range, and the
//! range-checked factors. The three difference-density items are
//! loop columns.

use std::fmt::Display;

use crate::data_dict::{CifTerm, Ineq, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{
    CharString, Number, Numeric, Tag, UnquotedString, UnsignedInteger, Value,
};

mod codes;
mod refine;
pub use refine::{Refine, RefineDiffDensity};

pub use codes::{
    HydrogenTreatment, MatrixType, RefineCifError, StructureFactorCoef, WeightingScheme,
};

/// Read the value of a `Numeric` as an `f64`.
fn numeric_value(value: &Numeric) -> f64 {
    match value.number() {
        Number::Integer(i) => i.0 as f64,
        Number::Float(f) => f.0,
    }
}

/// A value in the dictionary range `0.0:` of the `refine` category.
///
/// The try constructor rejects negative values at write time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonNeg(Numeric);

impl NonNeg {
    /// Validates the `0.0:` range of the item.
    pub fn try_new(value: Numeric, item: &str) -> Result<Self, RefineCifError> {
        let found = numeric_value(&value);
        if found < 0.0 {
            return Err(RefineCifError::new(format!(
                "_{item}: {found} is below the 0.0 range"
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

impl Display for NonNeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<NonNeg> for Value {
    fn from(value: NonNeg) -> Self {
        Value::Numeric(value.0)
    }
}

impl From<Ineq<NonNeg>> for Value {
    fn from(value: Ineq<NonNeg>) -> Self {
        match value {
            Ineq::Value(v) => Value::Numeric(v.0),
            Ineq::LessThan(v) => Value::CharString(CharString::Unquoted(
                UnquotedString::new(format!("<{}", v.numeric())),
            )),
        }
    }
}

/// The value of `_refine_ls_abs_structure_Flack`.
///
/// The dictionary range is `0.0:1.0`. The try constructor rejects
/// values outside the unit interval at write time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flack(Numeric);

impl Flack {
    /// Validates the `0.0:1.0` range of the item.
    pub fn try_new(value: Numeric) -> Result<Self, RefineCifError> {
        let found = numeric_value(&value);
        if !(0.0..=1.0).contains(&found) {
            return Err(RefineCifError::new(format!(
                "_refine_ls_abs_structure_Flack: {found} is outside the 0.0:1.0 range"
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

impl Display for Flack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Flack> for Value {
    fn from(value: Flack) -> Self {
        Value::Numeric(value.0)
    }
}

/// The value of `_refine_ls_abs_structure_Rogers`.
///
/// The dictionary range is `-1.0:1.0`. The try constructor rejects
/// values outside the signed unit interval at write time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rogers(Numeric);

impl Rogers {
    /// Validates the `-1.0:1.0` range of the item.
    pub fn try_new(value: Numeric) -> Result<Self, RefineCifError> {
        let found = numeric_value(&value);
        if !(-1.0..=1.0).contains(&found) {
            return Err(RefineCifError::new(format!(
                "_refine_ls_abs_structure_Rogers: {found} is outside the -1.0:1.0 range"
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

impl Display for Rogers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Rogers> for Value {
    fn from(value: Rogers) -> Self {
        Value::Numeric(value.0)
    }
}

/// The single-value items of the `refine` category.
///
/// The four count items carry the `0:` range by construction: an
/// `UnsignedInteger` value is always non-negative.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RefineTerms {
    /// The closed code set (dynamical, kinematical) recorded in the
    /// shell dictionary; the plain text form is kept.
    Diffraction_theory(CharString),
    Diffraction_theory_details(CharString),
    Ls_F_calc_details(CharString),
    Ls_F_calc_formula(CharString),
    Ls_F_calc_precision(NonNeg),
    Ls_R_Fsqd_factor(NonNeg),
    Ls_R_I_factor(NonNeg),
    Ls_R_factor_all(NonNeg),
    Ls_R_factor_gt(NonNeg),
    Ls_R_factor_obs(NonNeg),
    Ls_abs_structure_Flack(Flack),
    Ls_abs_structure_Rogers(Rogers),
    Ls_abs_structure_details(CharString),
    Ls_d_res_high(NonNeg),
    Ls_d_res_low(NonNeg),
    Ls_extinction_coef(Numeric),
    Ls_extinction_expression(CharString),
    Ls_extinction_method(CharString),
    Ls_goodness_of_fit_all(NonNeg),
    Ls_goodness_of_fit_gt(NonNeg),
    Ls_goodness_of_fit_obs(NonNeg),
    Ls_goodness_of_fit_ref(NonNeg),
    Ls_hydrogen_treatment(HydrogenTreatment),
    Ls_matrix_type(MatrixType),
    Ls_number_constraints(UnsignedInteger),
    Ls_number_parameters(UnsignedInteger),
    Ls_number_reflns(UnsignedInteger),
    Ls_number_restraints(UnsignedInteger),
    Ls_restrained_S_all(NonNeg),
    Ls_restrained_S_gt(NonNeg),
    Ls_restrained_S_obs(NonNeg),
    Ls_shift_esd_max(NonNeg),
    Ls_shift_esd_mean(NonNeg),
    Ls_shift_su_max(Ineq<NonNeg>),
    Ls_shift_su_max_lt(NonNeg),
    Ls_shift_su_mean(NonNeg),
    Ls_shift_su_mean_lt(NonNeg),
    Ls_structure_factor_coef(StructureFactorCoef),
    Ls_wR_factor_all(NonNeg),
    Ls_wR_factor_gt(NonNeg),
    Ls_wR_factor_obs(NonNeg),
    Ls_wR_factor_ref(NonNeg),
    Ls_weighting_details(CharString),
    Ls_weighting_scheme(WeightingScheme),
    Special_details(CharString),
}

impl CifTerm for RefineTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            RefineTerms::Diffraction_theory(_) => "diffraction_theory",
            RefineTerms::Diffraction_theory_details(_) => "diffraction_theory_details",
            RefineTerms::Ls_F_calc_details(_) => "ls_F_calc_details",
            RefineTerms::Ls_F_calc_formula(_) => "ls_F_calc_formula",
            RefineTerms::Ls_F_calc_precision(_) => "ls_F_calc_precision",
            RefineTerms::Ls_R_Fsqd_factor(_) => "ls_R_Fsqd_factor",
            RefineTerms::Ls_R_I_factor(_) => "ls_R_I_factor",
            RefineTerms::Ls_R_factor_all(_) => "ls_R_factor_all",
            RefineTerms::Ls_R_factor_gt(_) => "ls_R_factor_gt",
            RefineTerms::Ls_R_factor_obs(_) => "ls_R_factor_obs",
            RefineTerms::Ls_abs_structure_Flack(_) => "ls_abs_structure_Flack",
            RefineTerms::Ls_abs_structure_Rogers(_) => "ls_abs_structure_Rogers",
            RefineTerms::Ls_abs_structure_details(_) => "ls_abs_structure_details",
            RefineTerms::Ls_d_res_high(_) => "ls_d_res_high",
            RefineTerms::Ls_d_res_low(_) => "ls_d_res_low",
            RefineTerms::Ls_extinction_coef(_) => "ls_extinction_coef",
            RefineTerms::Ls_extinction_expression(_) => "ls_extinction_expression",
            RefineTerms::Ls_extinction_method(_) => "ls_extinction_method",
            RefineTerms::Ls_goodness_of_fit_all(_) => "ls_goodness_of_fit_all",
            RefineTerms::Ls_goodness_of_fit_gt(_) => "ls_goodness_of_fit_gt",
            RefineTerms::Ls_goodness_of_fit_obs(_) => "ls_goodness_of_fit_obs",
            RefineTerms::Ls_goodness_of_fit_ref(_) => "ls_goodness_of_fit_ref",
            RefineTerms::Ls_hydrogen_treatment(_) => "ls_hydrogen_treatment",
            RefineTerms::Ls_matrix_type(_) => "ls_matrix_type",
            RefineTerms::Ls_number_constraints(_) => "ls_number_constraints",
            RefineTerms::Ls_number_parameters(_) => "ls_number_parameters",
            RefineTerms::Ls_number_reflns(_) => "ls_number_reflns",
            RefineTerms::Ls_number_restraints(_) => "ls_number_restraints",
            RefineTerms::Ls_restrained_S_all(_) => "ls_restrained_S_all",
            RefineTerms::Ls_restrained_S_gt(_) => "ls_restrained_S_gt",
            RefineTerms::Ls_restrained_S_obs(_) => "ls_restrained_S_obs",
            RefineTerms::Ls_shift_esd_max(_) => "ls_shift/esd_max",
            RefineTerms::Ls_shift_esd_mean(_) => "ls_shift/esd_mean",
            RefineTerms::Ls_shift_su_max(_) => "ls_shift/su_max",
            RefineTerms::Ls_shift_su_max_lt(_) => "ls_shift/su_max_lt",
            RefineTerms::Ls_shift_su_mean(_) => "ls_shift/su_mean",
            RefineTerms::Ls_shift_su_mean_lt(_) => "ls_shift/su_mean_lt",
            RefineTerms::Ls_structure_factor_coef(_) => "ls_structure_factor_coef",
            RefineTerms::Ls_wR_factor_all(_) => "ls_wR_factor_all",
            RefineTerms::Ls_wR_factor_gt(_) => "ls_wR_factor_gt",
            RefineTerms::Ls_wR_factor_obs(_) => "ls_wR_factor_obs",
            RefineTerms::Ls_wR_factor_ref(_) => "ls_wR_factor_ref",
            RefineTerms::Ls_weighting_details(_) => "ls_weighting_details",
            RefineTerms::Ls_weighting_scheme(_) => "ls_weighting_scheme",
            RefineTerms::Special_details(_) => "special_details",
        };
        Tag::new(format!("refine_{suffix}"))
    }
}

impl SingleValueTerm for RefineTerms {
    fn value(&self) -> Value {
        match self {
            RefineTerms::Diffraction_theory(s)
            | RefineTerms::Diffraction_theory_details(s)
            | RefineTerms::Ls_F_calc_details(s)
            | RefineTerms::Ls_F_calc_formula(s)
            | RefineTerms::Ls_abs_structure_details(s)
            | RefineTerms::Ls_extinction_expression(s)
            | RefineTerms::Ls_extinction_method(s)
            | RefineTerms::Ls_weighting_details(s)
            | RefineTerms::Special_details(s) => Value::CharString(s.clone()),
            RefineTerms::Ls_hydrogen_treatment(c) => (*c).into(),
            RefineTerms::Ls_matrix_type(c) => (*c).into(),
            RefineTerms::Ls_structure_factor_coef(c) => (*c).into(),
            RefineTerms::Ls_weighting_scheme(c) => (*c).into(),
            RefineTerms::Ls_extinction_coef(n) => Value::Numeric(*n),
            RefineTerms::Ls_F_calc_precision(v)
            | RefineTerms::Ls_R_Fsqd_factor(v)
            | RefineTerms::Ls_R_I_factor(v)
            | RefineTerms::Ls_R_factor_all(v)
            | RefineTerms::Ls_R_factor_gt(v)
            | RefineTerms::Ls_R_factor_obs(v)
            | RefineTerms::Ls_d_res_high(v)
            | RefineTerms::Ls_d_res_low(v)
            | RefineTerms::Ls_goodness_of_fit_all(v)
            | RefineTerms::Ls_goodness_of_fit_gt(v)
            | RefineTerms::Ls_goodness_of_fit_obs(v)
            | RefineTerms::Ls_goodness_of_fit_ref(v)
            | RefineTerms::Ls_restrained_S_all(v)
            | RefineTerms::Ls_restrained_S_gt(v)
            | RefineTerms::Ls_restrained_S_obs(v)
            | RefineTerms::Ls_shift_esd_max(v)
            | RefineTerms::Ls_shift_esd_mean(v)
            | RefineTerms::Ls_shift_su_max_lt(v)
            | RefineTerms::Ls_shift_su_mean(v)
            | RefineTerms::Ls_shift_su_mean_lt(v)
            | RefineTerms::Ls_wR_factor_all(v)
            | RefineTerms::Ls_wR_factor_gt(v)
            | RefineTerms::Ls_wR_factor_obs(v)
            | RefineTerms::Ls_wR_factor_ref(v) => (*v).into(),
            RefineTerms::Ls_shift_su_max(v) => (*v).into(),
            RefineTerms::Ls_abs_structure_Flack(v) => (*v).into(),
            RefineTerms::Ls_abs_structure_Rogers(v) => (*v).into(),
            RefineTerms::Ls_number_constraints(v)
            | RefineTerms::Ls_number_parameters(v)
            | RefineTerms::Ls_number_reflns(v)
            | RefineTerms::Ls_number_restraints(v) => (*v).into(),
        }
    }
}

impl Display for RefineTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop items of the `refine` category.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RefineLoopItem {
    Diff_density_max(Vec<Numeric>),
    Diff_density_min(Vec<Numeric>),
    Diff_density_rms(Vec<Numeric>),
}

impl CifTerm for RefineLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            RefineLoopItem::Diff_density_max(_) => "diff_density_max",
            RefineLoopItem::Diff_density_min(_) => "diff_density_min",
            RefineLoopItem::Diff_density_rms(_) => "diff_density_rms",
        };
        Tag::new(format!("refine_{suffix}"))
    }
}

impl LoopValueTerm for RefineLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            RefineLoopItem::Diff_density_max(v)
            | RefineLoopItem::Diff_density_min(v)
            | RefineLoopItem::Diff_density_rms(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

impl Display for RefineLoopItem {
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
        CharString::Unquoted(UnquotedString::new(String::from(".")))
    }

    fn non_neg(item: &str) -> NonNeg {
        NonNeg::try_new(Numeric::from(0.05), item).unwrap()
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let terms: Vec<RefineTerms> = vec![
            RefineTerms::Ls_F_calc_details(cs()),
            RefineTerms::Ls_F_calc_formula(cs()),
            RefineTerms::Ls_F_calc_precision(non_neg("refine_ls_F_calc_precision")),
            RefineTerms::Ls_R_Fsqd_factor(non_neg("refine_ls_R_Fsqd_factor")),
            RefineTerms::Ls_R_I_factor(non_neg("refine_ls_R_I_factor")),
            RefineTerms::Ls_R_factor_all(non_neg("refine_ls_R_factor_all")),
            RefineTerms::Ls_R_factor_gt(non_neg("refine_ls_R_factor_gt")),
            RefineTerms::Ls_R_factor_obs(non_neg("refine_ls_R_factor_obs")),
            RefineTerms::Ls_abs_structure_Flack(Flack::try_new(Numeric::from(0.0)).unwrap()),
            RefineTerms::Ls_abs_structure_Rogers(Rogers::try_new(Numeric::from(0.0)).unwrap()),
            RefineTerms::Ls_abs_structure_details(cs()),
            RefineTerms::Ls_d_res_high(non_neg("refine_ls_d_res_high")),
            RefineTerms::Ls_d_res_low(non_neg("refine_ls_d_res_low")),
            RefineTerms::Ls_extinction_coef(Numeric::from(0.0)),
            RefineTerms::Ls_extinction_expression(cs()),
            RefineTerms::Ls_extinction_method(cs()),
            RefineTerms::Ls_goodness_of_fit_all(non_neg("refine_ls_goodness_of_fit_all")),
            RefineTerms::Ls_goodness_of_fit_gt(non_neg("refine_ls_goodness_of_fit_gt")),
            RefineTerms::Ls_goodness_of_fit_obs(non_neg("refine_ls_goodness_of_fit_obs")),
            RefineTerms::Ls_goodness_of_fit_ref(non_neg("refine_ls_goodness_of_fit_ref")),
            RefineTerms::Ls_hydrogen_treatment(HydrogenTreatment::Constr),
            RefineTerms::Ls_matrix_type(MatrixType::Full),
            RefineTerms::Ls_number_constraints(UnsignedInteger(0)),
            RefineTerms::Ls_number_parameters(UnsignedInteger(87)),
            RefineTerms::Ls_number_reflns(UnsignedInteger(1286)),
            RefineTerms::Ls_number_restraints(UnsignedInteger(0)),
            RefineTerms::Ls_restrained_S_all(non_neg("refine_ls_restrained_S_all")),
            RefineTerms::Ls_restrained_S_gt(non_neg("refine_ls_restrained_S_gt")),
            RefineTerms::Ls_restrained_S_obs(non_neg("refine_ls_restrained_S_obs")),
            RefineTerms::Ls_shift_esd_max(non_neg("refine_ls_shift/esd_max")),
            RefineTerms::Ls_shift_esd_mean(non_neg("refine_ls_shift/esd_mean")),
            RefineTerms::Ls_shift_su_max(Ineq::Value(non_neg("refine_ls_shift/su_max"))),
            RefineTerms::Ls_shift_su_max_lt(non_neg("refine_ls_shift/su_max_lt")),
            RefineTerms::Ls_shift_su_mean(non_neg("refine_ls_shift/su_mean")),
            RefineTerms::Ls_shift_su_mean_lt(non_neg("refine_ls_shift/su_mean_lt")),
            RefineTerms::Ls_structure_factor_coef(StructureFactorCoef::F),
            RefineTerms::Ls_wR_factor_all(non_neg("refine_ls_wR_factor_all")),
            RefineTerms::Ls_wR_factor_gt(non_neg("refine_ls_wR_factor_gt")),
            RefineTerms::Ls_wR_factor_obs(non_neg("refine_ls_wR_factor_obs")),
            RefineTerms::Ls_wR_factor_ref(non_neg("refine_ls_wR_factor_ref")),
            RefineTerms::Ls_weighting_details(cs()),
            RefineTerms::Ls_weighting_scheme(WeightingScheme::Sigma),
            RefineTerms::Special_details(cs()),
        ];
        let expected: Vec<&str> = vec![
            "refine_ls_F_calc_details",
            "refine_ls_F_calc_formula",
            "refine_ls_F_calc_precision",
            "refine_ls_R_Fsqd_factor",
            "refine_ls_R_I_factor",
            "refine_ls_R_factor_all",
            "refine_ls_R_factor_gt",
            "refine_ls_R_factor_obs",
            "refine_ls_abs_structure_Flack",
            "refine_ls_abs_structure_Rogers",
            "refine_ls_abs_structure_details",
            "refine_ls_d_res_high",
            "refine_ls_d_res_low",
            "refine_ls_extinction_coef",
            "refine_ls_extinction_expression",
            "refine_ls_extinction_method",
            "refine_ls_goodness_of_fit_all",
            "refine_ls_goodness_of_fit_gt",
            "refine_ls_goodness_of_fit_obs",
            "refine_ls_goodness_of_fit_ref",
            "refine_ls_hydrogen_treatment",
            "refine_ls_matrix_type",
            "refine_ls_number_constraints",
            "refine_ls_number_parameters",
            "refine_ls_number_reflns",
            "refine_ls_number_restraints",
            "refine_ls_restrained_S_all",
            "refine_ls_restrained_S_gt",
            "refine_ls_restrained_S_obs",
            "refine_ls_shift/esd_max",
            "refine_ls_shift/esd_mean",
            "refine_ls_shift/su_max",
            "refine_ls_shift/su_max_lt",
            "refine_ls_shift/su_mean",
            "refine_ls_shift/su_mean_lt",
            "refine_ls_structure_factor_coef",
            "refine_ls_wR_factor_all",
            "refine_ls_wR_factor_gt",
            "refine_ls_wR_factor_obs",
            "refine_ls_wR_factor_ref",
            "refine_ls_weighting_details",
            "refine_ls_weighting_scheme",
            "refine_special_details",
        ];
        assert_eq!(terms.len(), expected.len(), "RefineTerms variant count");
        for (term, tag) in terms.iter().zip(expected) {
            assert_eq!(term.tag().as_ref(), tag, "RefineTerms tag drift");
        }

        let items: Vec<RefineLoopItem> = vec![
            RefineLoopItem::Diff_density_max(vec![Numeric::from(0.3)]),
            RefineLoopItem::Diff_density_min(vec![Numeric::from(-0.4)]),
            RefineLoopItem::Diff_density_rms(vec![Numeric::from(0.05)]),
        ];
        let expected: Vec<&str> = vec![
            "refine_diff_density_max",
            "refine_diff_density_min",
            "refine_diff_density_rms",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "RefineLoopItem tag drift");
        }
    }

    #[test]
    fn range_checks_reject_out_of_range_values() {
        assert!(NonNeg::try_new(Numeric::from(-0.1), "refine_ls_R_factor_gt").is_err());
        assert!(Flack::try_new(Numeric::from(1.5)).is_err());
        assert!(Rogers::try_new(Numeric::from(-1.5)).is_err());
        assert!(Rogers::try_new(Numeric::from(1.5)).is_err());
        assert!(Rogers::try_new(Numeric::from(0.5)).is_ok());
    }
}
