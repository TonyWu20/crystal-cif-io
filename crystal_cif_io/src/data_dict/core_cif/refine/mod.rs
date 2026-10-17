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
            Ineq::LessThan(v) => Value::CharString(CharString::Unquoted(UnquotedString::new(
                format!("<{}", v.numeric()),
            ))),
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
/// The single-value terms of the `refine` category: one variant per

/// registry single-value item, in item order.
pub enum RefineTerms {
    /// The closed code set (dynamical, kinematical) recorded in the
    /// shell dictionary; the plain text form is kept.
    /// The `_refine_diffraction_theory` item. The variant holds the column values, one per loop row.
    Diffraction_theory(CharString),
    /// The `_refine_diffraction_theory_details` item. The variant holds the column values, one per loop row.
    Diffraction_theory_details(CharString),
    /// The `_refine_ls_F_calc_details` item. The variant holds the column values, one per loop row.
    ///
    /// Details concerning the evaluation of the structure factors using the
    /// expression given in _refine_ls_F_calc_formula.
    Ls_F_calc_details(CharString),
    /// The `_refine_ls_F_calc_formula` item. The variant holds the column values, one per loop row.
    ///
    /// Analytical expression used to calculate the structure factors.
    Ls_F_calc_formula(CharString),
    /// The `_refine_ls_F_calc_precision` item. The variant holds the column values, one per loop row.
    ///
    /// This item gives an estimate of the precision resulting from the
    /// numerical approximations made during the evaluation of the structure
    /// factors using the expression given in _refine_ls_F_calc_formula
    /// following the method outlined in _refine_ls_F_calc_details. For
    /// X-ray diffraction the result is given in electrons.
    Ls_F_calc_precision(NonNeg),
    /// The `_refine_ls_R_Fsqd_factor` item. The variant holds the column values, one per loop row.
    ///
    /// Residual factor R(Fsqd), calculated on the squared amplitudes of the
    /// observed and calculated structure factors, for significantly intense
    /// reflections (satisfying _reflns_threshold_expression) and included
    /// in the refinement. The reflections also satisfy the resolution
    /// limits established by _refine_ls_d_res_high and
    /// _refine_ls_d_res_low. sum | F(obs)^2^ - F(calc)^2^ | R(Fsqd) =
    /// ------------------------------- sum F(obs)^2^ F(obs)^2^ = squares of
    /// the observed structure-factor amplitudes F(calc)^2^ = squares of the
    /// calculated structure-factor amplitudes and the sum is taken over the
    /// specified reflections
    Ls_R_Fsqd_factor(NonNeg),
    /// The `_refine_ls_R_I_factor` item. The variant holds the column values, one per loop row.
    ///
    /// Residual factor R(I) for significantly intense reflections
    /// (satisfying _reflns_threshold_expression) and included in the
    /// refinement. This is most often calculated in Rietveld refinements
    /// against powder data, where it is referred to as R~B~ or R~Bragg~.
    /// sum | I(obs) - I(calc) | R(I) = ------------------------ sum |
    /// I(obs) | I(obs) = the net observed intensities I(calc) = the net
    /// calculated intensities and the sum is taken over the specified
    /// reflections
    Ls_R_I_factor(NonNeg),
    /// The `_refine_ls_R_factor_all` item. The variant holds the column values, one per loop row.
    ///
    /// Residual factor for all reflections satisfying the resolution limits
    /// established by _refine_ls_d_res_high and _refine_ls_d_res_low. This
    /// is the conventional R factor. See also _refine_ls_wR_factor_
    /// definitions. sum | F(obs) - F(calc) | R = ------------------------
    /// sum | F(obs) | F(obs) = the observed structure-factor amplitudes
    /// F(calc) = the calculated structure-factor amplitudes and the sum is
    /// taken over the specified reflections
    Ls_R_factor_all(NonNeg),
    /// The `_refine_ls_R_factor_gt` item. The variant holds the column values, one per loop row.
    ///
    /// Residual factor for the reflections (with number given by
    /// _reflns_number_gt) judged significantly intense (i.e. satisfying the
    /// threshold specified by _reflns_threshold_expression) and included in
    /// the refinement. The reflections also satisfy the resolution limits
    /// established by _refine_ls_d_res_high and _refine_ls_d_res_low. This
    /// is the conventional R factor. See also _refine_ls_wR_factor_
    /// definitions. sum | F(obs) - F(calc) | R = ------------------------
    /// sum | F(obs) | F(obs) = the observed structure-factor amplitudes
    /// F(calc) = the calculated structure-factor amplitudes and the sum is
    /// taken over the specified reflections
    Ls_R_factor_gt(NonNeg),
    /// The `_refine_ls_R_factor_obs` item. The variant holds the column values, one per loop row.
    ///
    /// Residual factor for the reflections classified as 'observed' (see
    /// _reflns_observed_criterion) and included in the refinement. The
    /// reflections also satisfy the resolution limits established by
    /// _refine_ls_d_res_high and _refine_ls_d_res_low. This is the
    /// conventional R factor. See also _refine_ls_wR_factor_ definitions.
    /// sum | F(obs) - F(calc) | R = ------------------------ sum | F(obs) |
    /// F(obs) = the observed structure-factor amplitudes F(calc) = the
    /// calculated structure-factor amplitudes and the sum is taken over the
    /// specified reflections
    Ls_R_factor_obs(NonNeg),
    /// The `_refine_ls_abs_structure_Flack` item. The variant holds the column values, one per loop row.
    ///
    /// The measure of absolute structure as defined by Flack (1983). For
    /// centrosymmetric structures, the only permitted value, if the data
    /// name is present, is 'inapplicable', represented by '.' . For
    /// noncentrosymmetric structures, the value must lie in the 99.97%
    /// Gaussian confidence interval -3u =&lt; x =&lt; 1 + 3u and a standard
    /// uncertainty (e.s.d.) u must be supplied. The _enumeration_range of
    /// 0.0:1.0 is correctly interpreted as meaning (0.0 - 3u) =&lt; x =&lt; (1.0
    /// + 3u). Ref: Flack, H. D. (1983). Acta Cryst. A39, 876-881.
    Ls_abs_structure_Flack(Flack),
    /// The `_refine_ls_abs_structure_Rogers` item. The variant holds the column values, one per loop row.
    ///
    /// The measure of absolute structure as defined by Rogers (1981). The
    /// value must lie in the 99.97% Gaussian confidence interval -1 -3u =&lt;
    /// \h =&lt; 1 + 3u and a standard uncertainty (e.s.d.) u must be supplied.
    /// The _enumeration_range of -1.0:1.0 is correctly interpreted as
    /// meaning (-1.0 - 3u) =&lt; \h =&lt; (1.0 + 3u). Ref: Rogers, D. (1981).
    /// Acta Cryst. A37, 734-741.
    Ls_abs_structure_Rogers(Rogers),
    /// The `_refine_ls_abs_structure_details` item. The variant holds the column values, one per loop row.
    ///
    /// The nature of the absolute structure and how it was determined.
    Ls_abs_structure_details(CharString),
    /// The `_refine_ls_d_res_high` item. The variant holds the column values, one per loop row.
    ///
    /// The smallest value in angstroms of the interplanar spacings of the
    /// reflections used in the refinement. This is called the highest
    /// resolution.
    /// Units: A (angstrom).
    Ls_d_res_high(NonNeg),
    /// The `_refine_ls_d_res_low` item. The variant holds the column values, one per loop row.
    ///
    /// The largest value in angstroms of the interplanar spacings of the
    /// reflections used in the refinement. This is called the lowest
    /// resolution.
    /// Units: A (angstrom).
    Ls_d_res_low(NonNeg),
    /// The `_refine_ls_extinction_coef` item. The variant holds the column values, one per loop row.
    ///
    /// The extinction coefficient used to calculate the correction factor
    /// applied to the structure-factor data. The nature of the extinction
    /// coefficient is given in the definitions of
    /// _refine_ls_extinction_expression and _refine_ls_extinction_method.
    /// For the 'Zachariasen' method it is the r* value; for the
    /// 'Becker-Coppens type 1 isotropic' method it is the 'g' value and for
    /// 'Becker-Coppens type 2 isotropic' corrections it is the 'rho' value.
    /// Note that the magnitude of these values is usually of the order of
    /// 10000. Ref: Becker, P. J. & Coppens, P. (1974). Acta Cryst. A30,
    /// 129-147, 148-153. Zachariasen, W. H. (1967). Acta Cryst. 23,
    /// 558-564. Larson, A. C. (1967). Acta Cryst. 23, 664-665.
    Ls_extinction_coef(Numeric),
    /// The `_refine_ls_extinction_expression` item. The variant holds the column values, one per loop row.
    ///
    /// A description of or reference to the extinction-correction equation
    /// used to apply the data item _refine_ls_extinction_coef. This
    /// information must be sufficient to reproduce the
    /// extinction-correction factors applied to the structure factors.
    Ls_extinction_expression(CharString),
    /// The `_refine_ls_extinction_method` item. The variant holds the column values, one per loop row.
    ///
    /// A description of the extinction-correction method applied. This
    /// description should include information about the correction method,
    /// either 'Becker-Coppens' or 'Zachariasen'. The latter is sometimes
    /// referred to as the 'Larson' method even though it employs
    /// Zachariasen's formula. The Becker-Coppens procedure is referred to
    /// as 'type 1' when correcting secondary extinction dominated by the
    /// mosaic spread; as 'type 2' when secondary extinction is dominated by
    /// particle size and includes a primary extinction component; and as
    /// 'mixed' when there is a mixture of types 1 and 2. For the
    /// Becker-Coppens method, it is also necessary to set the mosaic
    /// distribution as either 'Gaussian' or 'Lorentzian' and the nature of
    /// the extinction as 'isotropic' or 'anisotropic'. Note that if either
    /// the 'mixed' or 'anisotropic' corrections are applied, the multiple
    /// coefficients cannot be contained in *_extinction_coef and must be
    /// listed in _refine_special_details. Ref: Becker, P. J. & Coppens, P.
    /// (1974). Acta Cryst. A30, 129-147, 148-153. Zachariasen, W. H.
    /// (1967). Acta Cryst. 23, 558-564. Larson, A. C. (1967). Acta Cryst.
    /// 23, 664-665.
    Ls_extinction_method(CharString),
    /// The `_refine_ls_goodness_of_fit_all` item. The variant holds the column values, one per loop row.
    ///
    /// The least-squares goodness-of-fit parameter S for all reflections
    /// after the final cycle of refinement. Ideally, account should be
    /// taken of parameters restrained in the least-squares refinement. See
    /// also _refine_ls_restrained_S_ definitions. { sum { w [ Y(obs) -
    /// Y(calc) ]^2^ } }^1/2^ S = { ----------------------------------- } {
    /// Nref - Nparam } Y(obs) = the observed coefficients (see
    /// _refine_ls_structure_factor_coef) Y(calc) = the calculated
    /// coefficients (see _refine_ls_structure_factor_coef) w = the
    /// least-squares reflection weight [1/(u^2^)] u = the standard
    /// uncertainty Nref = the number of reflections used in the refinement
    /// Nparam = the number of refined parameters and the sum is taken over
    /// the specified reflections
    Ls_goodness_of_fit_all(NonNeg),
    /// The `_refine_ls_goodness_of_fit_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The least-squares goodness-of-fit parameter S for significantly
    /// intense reflections (see _reflns_threshold_expression) after the
    /// final cycle of refinement. Ideally, account should be taken of
    /// parameters restrained in the least-squares refinement. See also
    /// _refine_ls_restrained_S_ definitions. { sum { w [ Y(obs) - Y(calc)
    /// ]^2^ } }^1/2^ S = { ----------------------------------- } { Nref -
    /// Nparam } Y(obs) = the observed coefficients (see
    /// _refine_ls_structure_factor_coef) Y(calc) = the calculated
    /// coefficients (see _refine_ls_structure_factor_coef) w = the
    /// least-squares reflection weight [1/(u^2^)] u = standard uncertainty
    /// Nref = the number of reflections used in the refinement Nparam = the
    /// number of refined parameters and the sum is taken over the specified
    /// reflections
    Ls_goodness_of_fit_gt(NonNeg),
    /// The `_refine_ls_goodness_of_fit_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The least-squares goodness-of-fit parameter S for observed
    /// reflections (see _reflns_observed_criterion) after the final cycle
    /// of refinement. Ideally, account should be taken of parameters
    /// restrained in the least-squares refinement. See also
    /// _refine_ls_restrained_S_ definitions. { sum { w [ Y(obs) - Y(calc)
    /// ]^2^ } }^1/2^ S = { ----------------------------------- } { Nref -
    /// Nparam } Y(obs) = the observed coefficients (see
    /// _refine_ls_structure_factor_coef) Y(calc) = the calculated
    /// coefficients (see _refine_ls_structure_factor_coef) w = the
    /// least-squares reflection weight [1/(u^2^)] u = standard uncertainty
    /// (e.s.d.) Nref = the number of reflections used in the refinement
    /// Nparam = the number of refined parameters and the sum is taken over
    /// the specified reflections
    Ls_goodness_of_fit_obs(NonNeg),
    /// The `_refine_ls_goodness_of_fit_ref` item. The variant holds the column values, one per loop row.
    ///
    /// The least-squares goodness-of-fit parameter S for all reflections
    /// included in the refinement after the final cycle of refinement.
    /// Ideally, account should be taken of parameters restrained in the
    /// least-squares refinement. See also _refine_ls_restrained_S_
    /// definitions. { sum | w | Y(obs) - Y(calc) |^2^ | }^1/2^ S = {
    /// ----------------------------------- } { Nref - Nparam } Y(obs) = the
    /// observed coefficients (see _refine_ls_structure_factor_coef) Y(calc)
    /// = the calculated coefficients (see _refine_ls_structure_factor_coef)
    /// w = the least-squares reflection weight [1/(u^2^)] u = standard
    /// uncertainty Nref = the number of reflections used in the refinement
    /// Nparam = the number of refined parameters and the sum is taken over
    /// the specified reflections
    Ls_goodness_of_fit_ref(NonNeg),
    /// The `_refine_ls_hydrogen_treatment` item. The variant holds the column values, one per loop row.
    ///
    /// Treatment of hydrogen atoms in the least-squares refinement.
    Ls_hydrogen_treatment(HydrogenTreatment),
    /// The `_refine_ls_matrix_type` item. The variant holds the column values, one per loop row.
    ///
    /// Type of matrix used to accumulate the least-squares derivatives.
    Ls_matrix_type(MatrixType),
    /// The `_refine_ls_number_constraints` item. The variant holds the column values, one per loop row.
    ///
    /// The number of constrained (non-refined or dependent) parameters in
    /// the least-squares process. These may be due to symmetry or any other
    /// constraint process (e.g. rigid-body refinement). See also
    /// _atom_site_constraints and _atom_site_refinement_flags. A general
    /// description of constraints may appear in _refine_special_details.
    Ls_number_constraints(UnsignedInteger),
    /// The `_refine_ls_number_parameters` item. The variant holds the column values, one per loop row.
    ///
    /// The number of parameters refined in the least-squares process. If
    /// possible, this number should include some contribution from the
    /// restrained parameters. The restrained parameters are distinct from
    /// the constrained parameters (where one or more parameters are
    /// linearly dependent on the refined value of another). Least-squares
    /// restraints often depend on geometry or energy considerations and
    /// this makes their direct contribution to this number, and to the
    /// goodness-of-fit calculation, difficult to assess.
    Ls_number_parameters(UnsignedInteger),
    /// The `_refine_ls_number_reflns` item. The variant holds the column values, one per loop row.
    ///
    /// The number of unique reflections contributing to the least-squares
    /// refinement calculation.
    Ls_number_reflns(UnsignedInteger),
    /// The `_refine_ls_number_restraints` item. The variant holds the column values, one per loop row.
    ///
    /// The number of restrained parameters. These are parameters which are
    /// not directly dependent on another refined parameter. Restrained
    /// parameters often involve geometry or energy dependencies. See also
    /// _atom_site_constraints and _atom_site_refinement_flags. A general
    /// description of refinement constraints may appear in
    /// _refine_special_details.
    Ls_number_restraints(UnsignedInteger),
    /// The `_refine_ls_restrained_S_all` item. The variant holds the column values, one per loop row.
    ///
    /// The least-squares goodness-of-fit parameter S' for all reflections
    /// after the final cycle of least-squares refinement. This parameter
    /// explicitly includes the restraints applied in the least-squares
    /// process. See also _refine_ls_goodness_of_fit_ definitions. {sum { w
    /// [ Y(obs) - Y(calc) ]^2^ } }^1/2^ { + sum~r~ { w~r~ [ P(calc) -
    /// P(targ) ]^2^ } } S' = {
    /// -------------------------------------------------- } { N~ref~ +
    /// N~restr~ - N~param~ } Y(obs) = the observed coefficients (see
    /// _refine_ls_structure_factor_coef) Y(calc) = the calculated
    /// coefficients (see _refine_ls_structure_factor_coef) w = the
    /// least-squares reflection weight [1/square of standard uncertainty
    /// (e.s.d.)] P(calc) = the calculated restraint values P(targ) = the
    /// target restraint values w~r~ = the restraint weight N~ref~ = the
    /// number of reflections used in the refinement (see
    /// _refine_ls_number_reflns) N~restr~ = the number of restraints (see
    /// _refine_ls_number_restraints) N~param~ = the number of refined
    /// parameters (see _refine_ls_number_parameters) sum is taken over the
    /// specified reflections sum~r~ is taken over the restraints
    Ls_restrained_S_all(NonNeg),
    /// The `_refine_ls_restrained_S_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The least-squares goodness-of-fit parameter S' for significantly
    /// intense reflections (satisfying _reflns_threshold_expression) after
    /// the final cycle of least-squares refinement. This parameter
    /// explicitly includes the restraints applied in the least-squares
    /// process. See also _refine_ls_goodness_of_fit_ definitions. {sum { w
    /// [ Y(obs) - Y(calc) ]^2^ } }^1/2^ { + sum~r~ { w~r~ [ P(calc) -
    /// P(targ) ]^2^ } } S' = {
    /// -------------------------------------------------- } { N~ref~ +
    /// N~restr~ - N~param~ } Y(obs) = the observed coefficients (see
    /// _refine_ls_structure_factor_coef) Y(calc) = the calculated
    /// coefficients (see _refine_ls_structure_factor_coef) w = the
    /// least-squares reflection weight [1/square of standard uncertainty
    /// (e.s.d.)] P(calc) = the calculated restraint values P(targ) = the
    /// target restraint values w~r~ = the restraint weight N~ref~ = the
    /// number of reflections used in the refinement (see
    /// _refine_ls_number_reflns) N~restr~ = the number of restraints (see
    /// _refine_ls_number_restraints) N~param~ = the number of refined
    /// parameters (see _refine_ls_number_parameters) sum is taken over the
    /// specified reflections sum~r~ is taken over the restraints
    Ls_restrained_S_gt(NonNeg),
    /// The `_refine_ls_restrained_S_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The least-squares goodness-of-fit parameter S' for observed
    /// reflections after the final cycle of least-squares refinement. This
    /// parameter explicitly includes the restraints applied in the
    /// least-squares process. See also _refine_ls_goodness_of_fit_
    /// definitions. {sum { w [ Y(obs) - Y(calc) ]^2^ } }^1/2^ { + sum~r~ {
    /// w~r~ [ P(calc) - P(targ) ]^2^ } } S' = {
    /// -------------------------------------------------- } { N~ref~ +
    /// N~restr~ - N~param~ } Y(obs) = the observed coefficients (see
    /// _refine_ls_structure_factor_coef) Y(calc) = the calculated
    /// coefficients (see _refine_ls_structure_factor_coef) w = the
    /// least-squares reflection weight [1/square of standard uncertainty
    /// (e.s.d.)] P(calc) = the calculated restraint values P(targ) = the
    /// target restraint values w~r~ = the restraint weight N~ref~ = the
    /// number of reflections used in the refinement (see
    /// _refine_ls_number_reflns) N~restr~ = the number of restraints (see
    /// _refine_ls_number_restraints) N~param~ = the number of refined
    /// parameters (see _refine_ls_number_parameters) sum is taken over the
    /// specified reflections sum~r~ is taken over the restraints
    Ls_restrained_S_obs(NonNeg),
    /// The `_refine_ls_shift/esd_max` item. The variant holds the column values, one per loop row.
    ///
    /// The largest ratio of the final least-squares parameter shift to the
    /// final standard uncertainty (s.u., formerly described as estimated
    /// standard deviation, e.s.d.).
    Ls_shift_esd_max(NonNeg),
    /// The `_refine_ls_shift/esd_mean` item. The variant holds the column values, one per loop row.
    ///
    /// The average ratio of the final least-squares parameter shift to the
    /// final standard uncertainty (s.u., formerly described as estimated
    /// standard deviation, e.s.d.).
    Ls_shift_esd_mean(NonNeg),
    /// The `_refine_ls_shift/su_max` item. The variant holds the column values, one per loop row.
    ///
    /// The largest ratio of the final least-squares parameter shift to the
    /// final standard uncertainty.
    Ls_shift_su_max(Ineq<NonNeg>),
    /// The `_refine_ls_shift/su_max_lt` item. The variant holds the column values, one per loop row.
    ///
    /// An upper limit for the largest ratio of the final least-squares
    /// parameter shift to the final standard uncertainty. This item is used
    /// when the largest value of the shift divided by the final standard
    /// uncertainty is too small to measure.
    Ls_shift_su_max_lt(NonNeg),
    /// The `_refine_ls_shift/su_mean` item. The variant holds the column values, one per loop row.
    ///
    /// The average ratio of the final least-squares parameter shift to the
    /// final standard uncertainty.
    Ls_shift_su_mean(NonNeg),
    /// The `_refine_ls_shift/su_mean_lt` item. The variant holds the column values, one per loop row.
    ///
    /// An upper limit for the average ratio of the final least-squares
    /// parameter shift to the final standard uncertainty. This item is used
    /// when the average value of the shift divided by the final standard
    /// uncertainty is too small to measure.
    Ls_shift_su_mean_lt(NonNeg),
    /// The `_refine_ls_structure_factor_coef` item. The variant holds the column values, one per loop row.
    ///
    /// Structure-factor coefficient |F|, F^2^ or I used in the
    /// least-squares refinement process.
    Ls_structure_factor_coef(StructureFactorCoef),
    /// The `_refine_ls_wR_factor_all` item. The variant holds the column values, one per loop row.
    ///
    /// Weighted residual factors for all reflections. The reflections also
    /// satisfy the resolution limits established by _refine_ls_d_res_high
    /// and _refine_ls_d_res_low. See also the _refine_ls_R_factor_
    /// definitions. ( sum w [ Y(obs) - Y(calc) ]^2^ )^1/2^ wR = (
    /// ------------------------------ ) ( sum w Y(obs)^2^ ) Y(obs) = the
    /// observed amplitude specified by _refine_ls_structure_factor_coef
    /// Y(calc) = the calculated amplitude specified by
    /// _refine_ls_structure_factor_coef w = the least-squares weight and
    /// the sum is taken over the specified reflections
    Ls_wR_factor_all(NonNeg),
    /// The `_refine_ls_wR_factor_gt` item. The variant holds the column values, one per loop row.
    ///
    /// Weighted residual factors for significantly intense reflections
    /// (satisfying _reflns_threshold_expression) included in the
    /// refinement. The reflections also satisfy the resolution limits
    /// established by _refine_ls_d_res_high and _refine_ls_d_res_low. See
    /// also the _refine_ls_R_factor_ definitions. ( sum w [ Y(obs) -
    /// Y(calc) ]^2^ )^1/2^ wR = ( ------------------------------ ) ( sum w
    /// Y(obs)^2^ ) Y(obs) = the observed amplitude specified by
    /// _refine_ls_structure_factor_coef Y(calc) = the calculated amplitude
    /// specified by _refine_ls_structure_factor_coef w = the least-squares
    /// weight and the sum is taken over the specified reflections
    Ls_wR_factor_gt(NonNeg),
    /// The `_refine_ls_wR_factor_obs` item. The variant holds the column values, one per loop row.
    ///
    /// Weighted residual factors for the reflections classified as
    /// 'observed' (see _reflns_observed_criterion) and included in the
    /// refinement. The reflections also satisfy the resolution limits
    /// established by _refine_ls_d_res_high and _refine_ls_d_res_low. See
    /// also the _refine_ls_R_factor_ definitions. ( sum w [ Y(obs) -
    /// Y(calc) ]^2^ )^1/2^ wR = ( ------------------------------ ) ( sum w
    /// Y(obs)^2^ ) Y(obs) = the observed amplitude specified by
    /// _refine_ls_structure_factor_coef Y(calc) = the calculated amplitude
    /// specified by _refine_ls_structure_factor_coef w = the least-squares
    /// weight and the sum is taken over the specified reflections
    Ls_wR_factor_obs(NonNeg),
    /// The `_refine_ls_wR_factor_ref` item. The variant holds the column values, one per loop row.
    ///
    /// Weighted residual factors for all reflections included in the
    /// refinement. The reflections also satisfy the resolution limits
    /// established by _refine_ls_d_res_high and _refine_ls_d_res_low. See
    /// also the _refine_ls_R_factor_ definitions. ( sum w [ Y(obs) -
    /// Y(calc) ]^2^ )^1/2^ wR = ( ------------------------------ ) ( sum w
    /// Y(obs)^2^ ) Y(obs) = the observed amplitude specified by
    /// _refine_ls_structure_factor_coef Y(calc) = the calculated amplitude
    /// specified by _refine_ls_structure_factor_coef w = the least-squares
    /// weight and the sum is taken over the specified reflections
    Ls_wR_factor_ref(NonNeg),
    /// The `_refine_ls_weighting_details` item. The variant holds the column values, one per loop row.
    ///
    /// A description of special aspects of the weighting scheme used in the
    /// least-squares refinement. Used to describe the weighting when the
    /// value of _refine_ls_weighting_scheme is specified as 'calc'.
    Ls_weighting_details(CharString),
    /// The `_refine_ls_weighting_scheme` item. The variant holds the column values, one per loop row.
    ///
    /// The weighting scheme applied in the least-squares process. The
    /// standard code may be followed by a description of the weight (but
    /// see _refine_ls_weighting_details for a preferred approach).
    Ls_weighting_scheme(WeightingScheme),
    /// The `_refine_special_details` item. The variant holds the column values, one per loop row.
    ///
    /// Description of special aspects of the refinement process.
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
/// The loop-value terms of the `refine` category: one variant per

/// registry loop-value item, in item order.
pub enum RefineLoopItem {
    /// The `_refine_diff_density_max` item. The variant holds the column values, one per loop row.
    ///
    /// The largest and smallest values and the root-mean-square deviation,
    /// in electrons per angstrom cubed, of the final difference electron
    /// density. The *_rms value is measured with respect to the arithmetic
    /// mean density and is derived from summations over each grid point in
    /// the asymmetric unit of the cell. This quantity is useful for
    /// assessing the significance of *_min and *_max values, and also for
    /// defining suitable contour levels.
    /// Units: e/A^3^ (electron per angstrom cubed).
    Diff_density_max(Vec<Numeric>),
    /// The `_refine_diff_density_min` item. The variant holds the column values, one per loop row.
    ///
    /// The largest and smallest values and the root-mean-square deviation,
    /// in electrons per angstrom cubed, of the final difference electron
    /// density. The *_rms value is measured with respect to the arithmetic
    /// mean density and is derived from summations over each grid point in
    /// the asymmetric unit of the cell. This quantity is useful for
    /// assessing the significance of *_min and *_max values, and also for
    /// defining suitable contour levels.
    /// Units: e/A^3^ (electron per angstrom cubed).
    Diff_density_min(Vec<Numeric>),
    /// The `_refine_diff_density_rms` item. The variant holds the column values, one per loop row.
    ///
    /// The largest and smallest values and the root-mean-square deviation,
    /// in electrons per angstrom cubed, of the final difference electron
    /// density. The *_rms value is measured with respect to the arithmetic
    /// mean density and is derived from summations over each grid point in
    /// the asymmetric unit of the cell. This quantity is useful for
    /// assessing the significance of *_min and *_max values, and also for
    /// defining suitable contour levels.
    /// Units: e/A^3^ (electron per angstrom cubed).
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
