//! The `refine` category of the typed document layer (roadmap D12).
//!
//! The `refine` category struct: the forty-three single-value items
//! and the `refine_diff_density` loop family. `CifCategory` is on the
//! `Refine` group. The range-checked items keep the entry newtypes;
//! the closed sets keep the typed enums.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::refine::{
    Flack, HydrogenTreatment, MatrixType, NonNeg, Rogers, StructureFactorCoef, WeightingScheme,
};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::data_dict::Ineq;
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, UnsignedInteger, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `refine` category struct. One field per registry item.
pub struct Refine {
    /// The `_refine_ls_abs_structure_details` item. The field holds the single value.
    ///
    /// The nature of the absolute structure and how it was determined.
    pub ls_abs_structure_details: Option<CharString>,
    /// The `_refine_ls_abs_structure_Flack` item. The field holds the single value.
    ///
    /// The measure of absolute structure as defined by Flack (1983). For
    /// centrosymmetric structures, the only permitted value, if the data
    /// name is present, is 'inapplicable', represented by '.' . For
    /// noncentrosymmetric structures, the value must lie in the 99.97%
    /// Gaussian confidence interval -3u =&lt; x =&lt; 1 + 3u and a standard
    /// uncertainty (e.s.d.) u must be supplied. The _enumeration_range of
    /// 0.0:1.0 is correctly interpreted as meaning (0.0 - 3u) =&lt; x =&lt; (1.0
    /// + 3u). Ref: Flack, H. D. (1983). Acta Cryst. A39, 876-881.
    pub ls_abs_structure_flack: Option<Flack>,
    /// The `_refine_ls_abs_structure_Rogers` item. The field holds the single value.
    ///
    /// The measure of absolute structure as defined by Rogers (1981). The
    /// value must lie in the 99.97% Gaussian confidence interval -1 -3u =&lt;
    /// \h =&lt; 1 + 3u and a standard uncertainty (e.s.d.) u must be supplied.
    /// The _enumeration_range of -1.0:1.0 is correctly interpreted as
    /// meaning (-1.0 - 3u) =&lt; \h =&lt; (1.0 + 3u). Ref: Rogers, D. (1981).
    /// Acta Cryst. A37, 734-741.
    pub ls_abs_structure_rogers: Option<Rogers>,
    /// The `_refine_ls_d_res_high` item. The field holds the single value.
    ///
    /// The smallest value in angstroms of the interplanar spacings of the
    /// reflections used in the refinement. This is called the highest
    /// resolution.
    /// Units: A (angstrom).
    pub ls_d_res_high: Option<NonNeg>,
    /// The `_refine_ls_d_res_low` item. The field holds the single value.
    ///
    /// The largest value in angstroms of the interplanar spacings of the
    /// reflections used in the refinement. This is called the lowest
    /// resolution.
    /// Units: A (angstrom).
    pub ls_d_res_low: Option<NonNeg>,
    /// The `_refine_ls_extinction_coef` item. The field holds the single value.
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
    pub ls_extinction_coef: Option<Numeric>,
    /// The `_refine_ls_extinction_expression` item. The field holds the single value.
    ///
    /// A description of or reference to the extinction-correction equation
    /// used to apply the data item _refine_ls_extinction_coef. This
    /// information must be sufficient to reproduce the
    /// extinction-correction factors applied to the structure factors.
    pub ls_extinction_expression: Option<CharString>,
    /// The `_refine_ls_extinction_method` item. The field holds the single value.
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
    pub ls_extinction_method: Option<CharString>,
    /// The `_refine_ls_F_calc_details` item. The field holds the single value.
    ///
    /// Details concerning the evaluation of the structure factors using the
    /// expression given in _refine_ls_F_calc_formula.
    pub ls_f_calc_details: Option<CharString>,
    /// The `_refine_ls_F_calc_formula` item. The field holds the single value.
    ///
    /// Analytical expression used to calculate the structure factors.
    pub ls_f_calc_formula: Option<CharString>,
    /// The `_refine_ls_F_calc_precision` item. The field holds the single value.
    ///
    /// This item gives an estimate of the precision resulting from the
    /// numerical approximations made during the evaluation of the structure
    /// factors using the expression given in _refine_ls_F_calc_formula
    /// following the method outlined in _refine_ls_F_calc_details. For
    /// X-ray diffraction the result is given in electrons.
    pub ls_f_calc_precision: Option<NonNeg>,
    /// The `_refine_ls_goodness_of_fit_all` item. The field holds the single value.
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
    pub ls_goodness_of_fit_all: Option<NonNeg>,
    /// The `_refine_ls_goodness_of_fit_gt` item. The field holds the single value.
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
    pub ls_goodness_of_fit_gt: Option<NonNeg>,
    /// The `_refine_ls_goodness_of_fit_obs` item. The field holds the single value.
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
    pub ls_goodness_of_fit_obs: Option<NonNeg>,
    /// The `_refine_ls_goodness_of_fit_ref` item. The field holds the single value.
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
    pub ls_goodness_of_fit_ref: Option<NonNeg>,
    /// The `_refine_ls_hydrogen_treatment` item. The field holds the single value.
    ///
    /// Treatment of hydrogen atoms in the least-squares refinement.
    pub ls_hydrogen_treatment: Option<HydrogenTreatment>,
    /// The `_refine_ls_matrix_type` item. The field holds the single value.
    ///
    /// Type of matrix used to accumulate the least-squares derivatives.
    pub ls_matrix_type: Option<MatrixType>,
    /// The `_refine_ls_number_constraints` item. The field holds the single value.
    ///
    /// The number of constrained (non-refined or dependent) parameters in
    /// the least-squares process. These may be due to symmetry or any other
    /// constraint process (e.g. rigid-body refinement). See also
    /// _atom_site_constraints and _atom_site_refinement_flags. A general
    /// description of constraints may appear in _refine_special_details.
    pub ls_number_constraints: Option<UnsignedInteger>,
    /// The `_refine_ls_number_parameters` item. The field holds the single value.
    ///
    /// The number of parameters refined in the least-squares process. If
    /// possible, this number should include some contribution from the
    /// restrained parameters. The restrained parameters are distinct from
    /// the constrained parameters (where one or more parameters are
    /// linearly dependent on the refined value of another). Least-squares
    /// restraints often depend on geometry or energy considerations and
    /// this makes their direct contribution to this number, and to the
    /// goodness-of-fit calculation, difficult to assess.
    pub ls_number_parameters: Option<UnsignedInteger>,
    /// The `_refine_ls_number_reflns` item. The field holds the single value.
    ///
    /// The number of unique reflections contributing to the least-squares
    /// refinement calculation.
    pub ls_number_reflns: Option<UnsignedInteger>,
    /// The `_refine_ls_number_restraints` item. The field holds the single value.
    ///
    /// The number of restrained parameters. These are parameters which are
    /// not directly dependent on another refined parameter. Restrained
    /// parameters often involve geometry or energy dependencies. See also
    /// _atom_site_constraints and _atom_site_refinement_flags. A general
    /// description of refinement constraints may appear in
    /// _refine_special_details.
    pub ls_number_restraints: Option<UnsignedInteger>,
    /// The `_refine_ls_R_factor_all` item. The field holds the single value.
    ///
    /// Residual factor for all reflections satisfying the resolution limits
    /// established by _refine_ls_d_res_high and _refine_ls_d_res_low. This
    /// is the conventional R factor. See also _refine_ls_wR_factor_
    /// definitions. sum | F(obs) - F(calc) | R = ------------------------
    /// sum | F(obs) | F(obs) = the observed structure-factor amplitudes
    /// F(calc) = the calculated structure-factor amplitudes and the sum is
    /// taken over the specified reflections
    pub ls_r_factor_all: Option<NonNeg>,
    /// The `_refine_ls_R_factor_gt` item. The field holds the single value.
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
    pub ls_r_factor_gt: Option<NonNeg>,
    /// The `_refine_ls_R_factor_obs` item. The field holds the single value.
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
    pub ls_r_factor_obs: Option<NonNeg>,
    /// The `_refine_ls_R_Fsqd_factor` item. The field holds the single value.
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
    pub ls_r_fsqd_factor: Option<NonNeg>,
    /// The `_refine_ls_R_I_factor` item. The field holds the single value.
    ///
    /// Residual factor R(I) for significantly intense reflections
    /// (satisfying _reflns_threshold_expression) and included in the
    /// refinement. This is most often calculated in Rietveld refinements
    /// against powder data, where it is referred to as R~B~ or R~Bragg~.
    /// sum | I(obs) - I(calc) | R(I) = ------------------------ sum |
    /// I(obs) | I(obs) = the net observed intensities I(calc) = the net
    /// calculated intensities and the sum is taken over the specified
    /// reflections
    pub ls_r_i_factor: Option<NonNeg>,
    /// The `_refine_ls_restrained_S_all` item. The field holds the single value.
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
    pub ls_restrained_s_all: Option<NonNeg>,
    /// The `_refine_ls_restrained_S_gt` item. The field holds the single value.
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
    pub ls_restrained_s_gt: Option<NonNeg>,
    /// The `_refine_ls_restrained_S_obs` item. The field holds the single value.
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
    pub ls_restrained_s_obs: Option<NonNeg>,
    /// The `_refine_ls_shift/esd_max` item. The field holds the single value.
    ///
    /// The largest ratio of the final least-squares parameter shift to the
    /// final standard uncertainty (s.u., formerly described as estimated
    /// standard deviation, e.s.d.).
    pub ls_shift_esd_max: Option<NonNeg>,
    /// The `_refine_ls_shift/esd_mean` item. The field holds the single value.
    ///
    /// The average ratio of the final least-squares parameter shift to the
    /// final standard uncertainty (s.u., formerly described as estimated
    /// standard deviation, e.s.d.).
    pub ls_shift_esd_mean: Option<NonNeg>,
    /// The `_refine_ls_shift/su_max` item. The field holds the single value.
    ///
    /// The largest ratio of the final least-squares parameter shift to the
    /// final standard uncertainty.
    pub ls_shift_su_max: Option<Ineq<NonNeg>>,
    /// The `_refine_ls_shift/su_max_lt` item. The field holds the single value.
    ///
    /// An upper limit for the largest ratio of the final least-squares
    /// parameter shift to the final standard uncertainty. This item is used
    /// when the largest value of the shift divided by the final standard
    /// uncertainty is too small to measure.
    pub ls_shift_su_max_lt: Option<NonNeg>,
    /// The `_refine_ls_shift/su_mean` item. The field holds the single value.
    ///
    /// The average ratio of the final least-squares parameter shift to the
    /// final standard uncertainty.
    pub ls_shift_su_mean: Option<NonNeg>,
    /// The `_refine_ls_shift/su_mean_lt` item. The field holds the single value.
    ///
    /// An upper limit for the average ratio of the final least-squares
    /// parameter shift to the final standard uncertainty. This item is used
    /// when the average value of the shift divided by the final standard
    /// uncertainty is too small to measure.
    pub ls_shift_su_mean_lt: Option<NonNeg>,
    /// The `_refine_ls_structure_factor_coef` item. The field holds the single value.
    ///
    /// Structure-factor coefficient |F|, F^2^ or I used in the
    /// least-squares refinement process.
    pub ls_structure_factor_coef: Option<StructureFactorCoef>,
    /// The `_refine_ls_weighting_details` item. The field holds the single value.
    ///
    /// A description of special aspects of the weighting scheme used in the
    /// least-squares refinement. Used to describe the weighting when the
    /// value of _refine_ls_weighting_scheme is specified as 'calc'.
    pub ls_weighting_details: Option<CharString>,
    /// The `_refine_ls_weighting_scheme` item. The field holds the single value.
    ///
    /// The weighting scheme applied in the least-squares process. The
    /// standard code may be followed by a description of the weight (but
    /// see _refine_ls_weighting_details for a preferred approach).
    pub ls_weighting_scheme: Option<WeightingScheme>,
    /// The `_refine_ls_wR_factor_all` item. The field holds the single value.
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
    pub ls_wr_factor_all: Option<NonNeg>,
    /// The `_refine_ls_wR_factor_gt` item. The field holds the single value.
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
    pub ls_wr_factor_gt: Option<NonNeg>,
    /// The `_refine_ls_wR_factor_obs` item. The field holds the single value.
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
    pub ls_wr_factor_obs: Option<NonNeg>,
    /// The `_refine_ls_wR_factor_ref` item. The field holds the single value.
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
    pub ls_wr_factor_ref: Option<NonNeg>,
    /// The `_refine_special_details` item. The field holds the single value.
    ///
    /// Description of special aspects of the refinement process.
    pub special_details: Option<CharString>,
    /// The rows of the `_refine_diff_density_` loop family.
    pub diff_density: Vec<RefineDiffDensity>,
}

/// One loop row of the `_refine_diff_density_` loop family: the
/// maximum, minimum, and root-mean-square difference densities.
#[derive(Debug, Clone, Default)]
pub struct RefineDiffDensity {
    /// The `_refine_diff_density_max` item. The field holds the column values, one per loop row.
    ///
    /// The largest and smallest values and the root-mean-square deviation,
    /// in electrons per angstrom cubed, of the final difference electron
    /// density. The *_rms value is measured with respect to the arithmetic
    /// mean density and is derived from summations over each grid point in
    /// the asymmetric unit of the cell. This quantity is useful for
    /// assessing the significance of *_min and *_max values, and also for
    /// defining suitable contour levels.
    /// Units: e/A^3^ (electron per angstrom cubed).
    pub max: Option<Numeric>,
    /// The `_refine_diff_density_min` item. The field holds the column values, one per loop row.
    ///
    /// The largest and smallest values and the root-mean-square deviation,
    /// in electrons per angstrom cubed, of the final difference electron
    /// density. The *_rms value is measured with respect to the arithmetic
    /// mean density and is derived from summations over each grid point in
    /// the asymmetric unit of the cell. This quantity is useful for
    /// assessing the significance of *_min and *_max values, and also for
    /// defining suitable contour levels.
    /// Units: e/A^3^ (electron per angstrom cubed).
    pub min: Option<Numeric>,
    /// The `_refine_diff_density_rms` item. The field holds the column
    /// values, one per loop row.
    ///
    /// The root-mean-square deviation of the final difference electron
    /// density. Units: e/A^3 (electron per angstrom cubed).
    pub rms: Option<Numeric>,
}

impl CifCategory for Refine {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.ls_abs_structure_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_abs_structure_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_abs_structure_flack {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_abs_structure_Flack".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_abs_structure_rogers {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_abs_structure_Rogers".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_d_res_high {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_d_res_high".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_d_res_low {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_d_res_low".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_extinction_coef {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_extinction_coef".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_extinction_expression {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_extinction_expression".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_extinction_method {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_extinction_method".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_f_calc_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_F_calc_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_f_calc_formula {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_F_calc_formula".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_f_calc_precision {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_F_calc_precision".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_goodness_of_fit_ref {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_goodness_of_fit_ref".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_hydrogen_treatment {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_hydrogen_treatment".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_matrix_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_matrix_type".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_constraints {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_constraints".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_parameters {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_parameters".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_reflns {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_reflns".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_number_restraints {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_number_restraints".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_factor_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_factor_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_factor_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_factor_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_factor_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_factor_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_fsqd_factor {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_Fsqd_factor".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_r_i_factor {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_R_I_factor".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_restrained_s_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_restrained_S_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_restrained_s_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_restrained_S_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_restrained_s_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_restrained_S_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_esd_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/esd_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_esd_mean {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/esd_mean".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_max".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_max_lt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_max_lt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_mean {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_mean".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_shift_su_mean_lt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_shift/su_mean_lt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_structure_factor_coef {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_structure_factor_coef".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_weighting_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_weighting_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.ls_weighting_scheme {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_weighting_scheme".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_all {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_all".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_gt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_gt".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_obs {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_obs".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.ls_wr_factor_ref {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_ls_wR_factor_ref".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("refine_special_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if !self.diff_density.is_empty() {
            out.extend(diff_density_to_loop_members(&self.diff_density));
        }
        out
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::SingleValue(sv)) => {
                    let tag = sv.tag().as_ref().to_lowercase();
                    let value = sv.value();
                    match tag.as_str() {
                        "refine_ls_abs_structure_details" => {
                            match convert::read_char("refine_ls_abs_structure_details", value) {
                                Ok(v) => self.ls_abs_structure_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_abs_structure_flack" => {
                            match convert::read_flack("refine_ls_abs_structure_flack", value) {
                                Ok(v) => self.ls_abs_structure_flack = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_abs_structure_rogers" => {
                            match convert::read_rogers("refine_ls_abs_structure_rogers", value) {
                                Ok(v) => self.ls_abs_structure_rogers = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_d_res_high" => {
                            match convert::read_non_neg("refine_ls_d_res_high", value) {
                                Ok(v) => self.ls_d_res_high = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_d_res_low" => {
                            match convert::read_non_neg("refine_ls_d_res_low", value) {
                                Ok(v) => self.ls_d_res_low = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_extinction_coef" => {
                            match convert::read_numeric("refine_ls_extinction_coef", value) {
                                Ok(v) => self.ls_extinction_coef = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_extinction_expression" => {
                            match convert::read_char("refine_ls_extinction_expression", value) {
                                Ok(v) => self.ls_extinction_expression = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_extinction_method" => {
                            match convert::read_char("refine_ls_extinction_method", value) {
                                Ok(v) => self.ls_extinction_method = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_f_calc_details" => {
                            match convert::read_char("refine_ls_f_calc_details", value) {
                                Ok(v) => self.ls_f_calc_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_f_calc_formula" => {
                            match convert::read_char("refine_ls_f_calc_formula", value) {
                                Ok(v) => self.ls_f_calc_formula = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_f_calc_precision" => {
                            match convert::read_non_neg("refine_ls_f_calc_precision", value) {
                                Ok(v) => self.ls_f_calc_precision = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_all" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_all", value) {
                                Ok(v) => self.ls_goodness_of_fit_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_gt" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_gt", value) {
                                Ok(v) => self.ls_goodness_of_fit_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_obs" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_obs", value) {
                                Ok(v) => self.ls_goodness_of_fit_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_goodness_of_fit_ref" => {
                            match convert::read_non_neg("refine_ls_goodness_of_fit_ref", value) {
                                Ok(v) => self.ls_goodness_of_fit_ref = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_hydrogen_treatment" => {
                            match convert::read_closed_set(
                                "refine_ls_hydrogen_treatment",
                                value,
                                |c| HydrogenTreatment::try_from(c).ok(),
                                "HydrogenTreatment",
                            ) {
                                Ok(v) => self.ls_hydrogen_treatment = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_matrix_type" => {
                            match convert::read_closed_set(
                                "refine_ls_matrix_type",
                                value,
                                |c| MatrixType::try_from(c).ok(),
                                "MatrixType",
                            ) {
                                Ok(v) => self.ls_matrix_type = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_constraints" => {
                            match convert::read_unsigned("refine_ls_number_constraints", value) {
                                Ok(v) => self.ls_number_constraints = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_parameters" => {
                            match convert::read_unsigned("refine_ls_number_parameters", value) {
                                Ok(v) => self.ls_number_parameters = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_reflns" => {
                            match convert::read_unsigned("refine_ls_number_reflns", value) {
                                Ok(v) => self.ls_number_reflns = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_number_restraints" => {
                            match convert::read_unsigned("refine_ls_number_restraints", value) {
                                Ok(v) => self.ls_number_restraints = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_factor_all" => {
                            match convert::read_non_neg("refine_ls_r_factor_all", value) {
                                Ok(v) => self.ls_r_factor_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_factor_gt" => {
                            match convert::read_non_neg("refine_ls_r_factor_gt", value) {
                                Ok(v) => self.ls_r_factor_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_factor_obs" => {
                            match convert::read_non_neg("refine_ls_r_factor_obs", value) {
                                Ok(v) => self.ls_r_factor_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_fsqd_factor" => {
                            match convert::read_non_neg("refine_ls_r_fsqd_factor", value) {
                                Ok(v) => self.ls_r_fsqd_factor = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_r_i_factor" => {
                            match convert::read_non_neg("refine_ls_r_i_factor", value) {
                                Ok(v) => self.ls_r_i_factor = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_restrained_s_all" => {
                            match convert::read_non_neg("refine_ls_restrained_s_all", value) {
                                Ok(v) => self.ls_restrained_s_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_restrained_s_gt" => {
                            match convert::read_non_neg("refine_ls_restrained_s_gt", value) {
                                Ok(v) => self.ls_restrained_s_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_restrained_s_obs" => {
                            match convert::read_non_neg("refine_ls_restrained_s_obs", value) {
                                Ok(v) => self.ls_restrained_s_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/esd_max" => {
                            match convert::read_non_neg("refine_ls_shift/esd_max", value) {
                                Ok(v) => self.ls_shift_esd_max = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/esd_mean" => {
                            match convert::read_non_neg("refine_ls_shift/esd_mean", value) {
                                Ok(v) => self.ls_shift_esd_mean = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_max" => {
                            // A real-data file may record the value as a
                            // "<N" assertion (the COD). The assertion is a
                            // less-than statement. It records in the payload
                            // of this field (roadmap D12l, option B).
                            if let Value::CharString(c) = value {
                                let num = c.content().strip_prefix('<');
                                if let Some(n) = num.and_then(|t| t.parse::<f64>().ok()) {
                                    self.ls_shift_su_max = Some(Ineq::LessThan(
                                        NonNeg::from_numeric(Numeric::from(n)),
                                    ));
                                    continue;
                                }
                            }
                            match convert::read_non_neg("refine_ls_shift/su_max", value) {
                                Ok(v) => self.ls_shift_su_max = v.map(Ineq::Value),
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_max_lt" => {
                            match convert::read_non_neg("refine_ls_shift/su_max_lt", value) {
                                Ok(v) => self.ls_shift_su_max_lt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_mean" => {
                            match convert::read_non_neg("refine_ls_shift/su_mean", value) {
                                Ok(v) => self.ls_shift_su_mean = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_shift/su_mean_lt" => {
                            match convert::read_non_neg("refine_ls_shift/su_mean_lt", value) {
                                Ok(v) => self.ls_shift_su_mean_lt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_structure_factor_coef" => {
                            match convert::read_closed_set(
                                "refine_ls_structure_factor_coef",
                                value,
                                |c| StructureFactorCoef::try_from(c).ok(),
                                "StructureFactorCoef",
                            ) {
                                Ok(v) => self.ls_structure_factor_coef = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_weighting_details" => {
                            match convert::read_char("refine_ls_weighting_details", value) {
                                Ok(v) => self.ls_weighting_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_weighting_scheme" => {
                            match convert::read_closed_set(
                                "refine_ls_weighting_scheme",
                                value,
                                |c| WeightingScheme::try_from(c).ok(),
                                "WeightingScheme",
                            ) {
                                Ok(v) => self.ls_weighting_scheme = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_all" => {
                            match convert::read_non_neg("refine_ls_wr_factor_all", value) {
                                Ok(v) => self.ls_wr_factor_all = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_gt" => {
                            match convert::read_non_neg("refine_ls_wr_factor_gt", value) {
                                Ok(v) => self.ls_wr_factor_gt = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_obs" => {
                            match convert::read_non_neg("refine_ls_wr_factor_obs", value) {
                                Ok(v) => self.ls_wr_factor_obs = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_ls_wr_factor_ref" => {
                            match convert::read_non_neg("refine_ls_wr_factor_ref", value) {
                                Ok(v) => self.ls_wr_factor_ref = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "refine_special_details" => {
                            match convert::read_char_text("refine_special_details", value) {
                                Ok(v) => self.special_details = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop belongs to this category only when it carries one
                    // of the loop-family column tags. A sibling loop of
                    // another category stays in the residue.
                    let diff_density = [
                        "refine_diff_density_max",
                        "refine_diff_density_min",
                        "refine_diff_density_rms",
                    ];
                    let is_ours = cols.iter().any(|c| {
                        let t = c.tag().as_ref().to_lowercase();
                        diff_density.contains(&t.as_str())
                    });
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        if cols.iter().any(|c| {
                            diff_density.contains(&c.tag().as_ref().to_lowercase().as_str())
                        }) {
                            self.diff_density = diff_density_rows_from_columns(cols, &mut errors);
                        }
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn diff_density_to_loop_members(rows: &[RefineDiffDensity]) -> Vec<DataBlockMember> {
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.max.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.max {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("refine_diff_density_max".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.min.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.min {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("refine_diff_density_min".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.rms.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.rms {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("refine_diff_density_rms".to_string()),
            values,
        ));
    }
    if columns.is_empty() {
        return Vec::new();
    }
    vec![DataBlockMember::DataItems(DataItems::MultiValues(
        LoopColumns::new(columns),
    ))]
}

fn diff_density_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<RefineDiffDensity> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![RefineDiffDensity::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_diff_density_max")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_diff_density_max", v) {
                Ok(val) => rows[i].max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_diff_density_min")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_diff_density_min", v) {
                Ok(val) => rows[i].min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "refine_diff_density_rms")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("refine_diff_density_rms", v) {
                Ok(val) => rows[i].rms = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::parse_cif;
    use crystal_cif_fmt::{DataBlockMember, DataItems};

    /// The single member of the tag, with the value as recorded.
    fn members_for(value: &str) -> Vec<DataBlockMember> {
        let text = format!("data_test\n_refine_ls_shift/su_max {value}\n");
        let doc = parse_cif(&text).expect("the snippet parses");
        let block = &doc.data_blocks().expect("the block is present")[0];
        assert_eq!(block.members().len(), 1, "the snippet holds one member");
        block.members().to_vec()
    }

    /// The emitted member of the written value.
    fn emitted(member: &DataBlockMember) -> (&Tag, &Value) {
        match member {
            DataBlockMember::DataItems(DataItems::SingleValue(sv)) => (sv.tag(), sv.value()),
            other => panic!("the emitted member is a single value: {other:?}"),
        }
    }

    #[test]
    fn the_less_than_assertion_records_in_the_payload() {
        let mut refine = Refine::default();
        let (residue, errors) = refine.from_members(&members_for("<0.001"));
        assert!(residue.is_empty(), "the tag is consumed: {residue:?}");
        assert!(errors.is_empty(), "no mismatch: {errors:?}");
        let expected = Ineq::LessThan(NonNeg::from_numeric(Numeric::from(0.001)));
        assert_eq!(refine.ls_shift_su_max, Some(expected));
        assert_eq!(
            refine.ls_shift_su_max_lt, None,
            "the plain assertion stays in the payload"
        );
        let member = &refine.to_members()[0];
        let (tag, value) = emitted(member);
        assert_eq!(tag.as_ref(), "refine_ls_shift/su_max");
        assert_eq!(
            format!("{value}"),
            "<0.001",
            "the assertion re-emits as written"
        );
    }

    #[test]
    fn the_plain_value_records_in_the_payload() {
        let mut refine = Refine::default();
        let (residue, errors) = refine.from_members(&members_for("0.001"));
        assert!(residue.is_empty(), "the tag is consumed: {residue:?}");
        assert!(errors.is_empty(), "no mismatch: {errors:?}");
        let expected = Ineq::Value(NonNeg::from_numeric(Numeric::from(0.001)));
        assert_eq!(refine.ls_shift_su_max, Some(expected));
        assert_eq!(refine.ls_shift_su_max_lt, None);
    }
}
