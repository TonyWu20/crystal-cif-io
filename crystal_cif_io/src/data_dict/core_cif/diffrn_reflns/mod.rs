mod av_r_equivalents;
mod av_sigma_i_over_net_i;
mod av_unet_i_over_net_i;
mod diffrn_reflns;
mod laue_measured_fraction_full;
mod laue_measured_fraction_max;
mod point_group_measured_fraction_full;
mod point_group_measured_fraction_max;
mod point_measured_fraction_full;
mod point_measured_fraction_max;
mod reflns_number;
mod resolution_full;
mod resolution_max;
mod theta_full;
mod theta_max;
mod theta_min;

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};

pub use self::av_r_equivalents::AvREquivalents;
pub use self::av_sigma_i_over_net_i::AvSigmaIOverNetI;
pub use self::av_unet_i_over_net_i::AvUnetIOverNetI;
pub use self::diffrn_reflns::{DiffrnReflns, DiffrnReflnsRow};
pub use self::laue_measured_fraction_full::LaueMeasuredFractionFull;
pub use self::laue_measured_fraction_max::LaueMeasuredFractionMax;
pub use self::point_group_measured_fraction_full::PointGroupMeasuredFractionFull;
pub use self::point_group_measured_fraction_max::PointGroupMeasuredFractionMax;
pub use self::point_measured_fraction_full::PointMeasuredFractionFull;
pub use self::point_measured_fraction_max::PointMeasuredFractionMax;
pub use self::reflns_number::ReflnsNumber;
pub use self::resolution_full::ResolutionFull;
pub use self::resolution_max::ResolutionMax;
pub use self::theta_full::ThetaFull;
pub use self::theta_max::ThetaMax;
pub use self::theta_min::ThetaMin;

/// The single-value terms of the `diffrn_reflns` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsTerms {
    /// The `_diffrn_reflns_Laue_measured_fraction_full` item. The variant holds the single value.
    ///
    /// Fraction of Laue unique reflections (symmetry-independent in the
    /// Laue group) measured out to the resolution given in
    /// _diffrn_reflns_resolution_full or _diffrn_reflns_theta_full. The
    /// Laue group always contains a centre of symmetry so that the
    /// reflection h,k,l is always equivalent to the reflection -h,-k,-l
    /// even in space groups without a centre of symmetry. This number
    /// should not be less than 0.95, since it represents the fraction of
    /// reflections measured in the part of the diffraction pattern that is
    /// essentially complete.
    /// range constraint recorded in the dictionary _enumeration
    Laue_measured_fraction_full(LaueMeasuredFractionFull),
    /// The `_diffrn_reflns_Laue_measured_fraction_max` item. The variant holds the single value.
    ///
    /// Fraction of Laue unique reflections (symmetry-independent in the
    /// Laue group) measured out to the resolution given in
    /// _diffrn_reflns_resolution_max or _diffrn_reflns_theta_max. The Laue
    /// group always contains a centre of symmetry so that the reflection
    /// h,k,l is always equivalent to the reflection -h,-k,-l even in space
    /// groups without a centre of symmetry.
    /// range constraint recorded in the dictionary _enumeration
    Laue_measured_fraction_max(LaueMeasuredFractionMax),
    /// The `_diffrn_reflns_av_R_equivalents` item. The variant holds the single value.
    ///
    /// The residual [sum av|del(I)| / sum |av(I)|] for symmetry-equivalent
    /// reflections used to calculate the average intensity av(I). The
    /// av|del(I)| term is the average absolute difference between av(I) and
    /// the individual symmetry-equivalent intensities.
    /// range constraint recorded in the dictionary _enumeration
    Av_R_equivalents(AvREquivalents),
    /// The `_diffrn_reflns_av_sigmaI/netI` item. The variant holds the single value.
    ///
    /// Measure [sum |u(net I)|/sum|net I|] for all measured reflections.
    /// range constraint recorded in the dictionary _enumeration
    AvSigmaIOverNetI(AvSigmaIOverNetI),
    /// The `_diffrn_reflns_av_unetI/netI` item. The variant holds the single value.
    ///
    /// Measure [sum |u(net I)|/sum|net I|] for all measured reflections.
    /// range constraint recorded in the dictionary _enumeration
    AvUnetIOverNetI(AvUnetIOverNetI),
    /// The `_diffrn_reflns_limit_max` item. The variant holds the single value.
    Limit_max(Matrix),
    /// The `_diffrn_reflns_limit_min` item. The variant holds the single value.
    Limit_min(Matrix),
    /// The `_diffrn_reflns_number` item. The variant holds the single value.
    ///
    /// The total number of measured intensities, excluding reflections that
    /// are classed as systematically absent arising from translational
    /// symmetry in the crystal unit cell.
    /// range constraint recorded in the dictionary _enumeration
    ReflnsNumber(ReflnsNumber),
    /// The `_diffrn_reflns_point_group_measured_fraction_full` item. The variant holds the single value.
    ///
    /// Fraction of crystal point-group unique reflections (i.e.
    /// symmetry-independent in the crystal point group) measured out to the
    /// resolution given in _diffrn_reflns_resolution_full or
    /// _diffrn_reflns_theta_full. For space groups that do not contain a
    /// centre of symmetry the reflections h,k,l and -h,-k,-l are
    /// independent. This number should not be less than 0.95, since it
    /// represents the fraction of reflections measured in the part of the
    /// diffraction pattern that is essentially complete.
    /// range constraint recorded in the dictionary _enumeration
    Point_group_measured_fraction_full(PointGroupMeasuredFractionFull),
    /// The `_diffrn_reflns_point_group_measured_fraction_max` item. The variant holds the single value.
    ///
    /// Fraction of crystal point-group unique reflections (i.e.
    /// symmetry-independent in the crystal point group) measured out to the
    /// resolution given in _diffrn_reflns_resolution_max or
    /// _diffrn_reflns_theta_max. For space groups that do not contain a
    /// centre of symmetry the reflections h,k,l and -h,-k,-l are
    /// independent.
    /// range constraint recorded in the dictionary _enumeration
    Point_group_measured_fraction_max(PointGroupMeasuredFractionMax),
    /// The `_diffrn_reflns_point_measured_fraction_full` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Point_measured_fraction_full(PointMeasuredFractionFull),
    /// The `_diffrn_reflns_point_measured_fraction_max` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Point_measured_fraction_max(PointMeasuredFractionMax),
    /// The `_diffrn_reflns_reduction_process` item. The variant holds the single value.
    ///
    /// A description of the process used to reduce the intensities into
    /// structure-factor magnitudes.
    Reduction_process(CharString),
    /// The `_diffrn_reflns_resolution_full` item. The variant holds the single value.
    ///
    /// The resolution in reciprocal angstroms at which the measured
    /// reflection count is close to complete. The fraction of unique
    /// reflections measured out to this angle is given by
    /// _diffrn_measured_fraction_theta_full.
    /// Units: A^-1^ (reciprocal angstrom).
    /// range constraint recorded in the dictionary _enumeration
    Resolution_full(ResolutionFull),
    /// The `_diffrn_reflns_resolution_max` item. The variant holds the single value.
    ///
    /// Maximum resolution in reciprocal angstroms of the measured
    /// diffraction pattern. The fraction of unique reflections measured out
    /// to this angle is given by _diffrn_measured_fraction_theta_max
    /// Units: A^-1^ (reciprocal angstrom).
    /// range constraint recorded in the dictionary _enumeration
    Resolution_max(ResolutionMax),
    /// The `_diffrn_reflns_theta_full` item. The variant holds the single value.
    ///
    /// The theta angle (in degrees) at which the measured reflection count
    /// is close to complete. The fraction of unique reflections measured
    /// out to this angle is given by _diffrn_measured_fraction_theta_full.
    /// Units: deg (degree of arc).
    /// range constraint recorded in the dictionary _enumeration
    Theta_full(ThetaFull),
    /// The `_diffrn_reflns_theta_max` item. The variant holds the single value.
    ///
    /// Maximum theta angle in degrees for the measured intensities. The
    /// fraction of unique reflections measured out to this angle is given
    /// by _diffrn_measured_fraction_theta_max
    /// Units: deg (degree of arc).
    /// range constraint recorded in the dictionary _enumeration
    Theta_max(ThetaMax),
    /// The `_diffrn_reflns_theta_min` item. The variant holds the single value.
    ///
    /// Minimum theta angle in degrees for the measured intensities.
    /// Units: deg (degree of arc).
    /// range constraint recorded in the dictionary _enumeration
    Theta_min(ThetaMin),
}

impl CifTerm for DiffrnReflnsTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnReflnsTerms::Laue_measured_fraction_full(_) => "laue_measured_fraction_full",
            DiffrnReflnsTerms::Laue_measured_fraction_max(_) => "laue_measured_fraction_max",
            DiffrnReflnsTerms::Av_R_equivalents(_) => "av_r_equivalents",
            DiffrnReflnsTerms::AvSigmaIOverNetI(_) => "av_sigmai/neti",
            DiffrnReflnsTerms::AvUnetIOverNetI(_) => "av_uneti/neti",
            DiffrnReflnsTerms::Limit_max(_) => "limit_max",
            DiffrnReflnsTerms::Limit_min(_) => "limit_min",
            DiffrnReflnsTerms::ReflnsNumber(_) => "number",
            DiffrnReflnsTerms::Point_group_measured_fraction_full(_) => {
                "point_group_measured_fraction_full"
            }
            DiffrnReflnsTerms::Point_group_measured_fraction_max(_) => {
                "point_group_measured_fraction_max"
            }
            DiffrnReflnsTerms::Point_measured_fraction_full(_) => "point_measured_fraction_full",
            DiffrnReflnsTerms::Point_measured_fraction_max(_) => "point_measured_fraction_max",
            DiffrnReflnsTerms::Reduction_process(_) => "reduction_process",
            DiffrnReflnsTerms::Resolution_full(_) => "resolution_full",
            DiffrnReflnsTerms::Resolution_max(_) => "resolution_max",
            DiffrnReflnsTerms::Theta_full(_) => "theta_full",
            DiffrnReflnsTerms::Theta_max(_) => "theta_max",
            DiffrnReflnsTerms::Theta_min(_) => "theta_min",
        };
        Tag::new(format!("diffrn_reflns_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnReflnsTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnReflnsTerms::Laue_measured_fraction_full(v) => Value::from(*v),
            DiffrnReflnsTerms::Laue_measured_fraction_max(m) => Value::from(*m),
            DiffrnReflnsTerms::Av_R_equivalents(c) => Value::from(*c),
            DiffrnReflnsTerms::AvSigmaIOverNetI(b) => Value::from(*b),
            DiffrnReflnsTerms::AvUnetIOverNetI(t) => Value::from(*t),
            DiffrnReflnsTerms::Limit_max(m) | DiffrnReflnsTerms::Limit_min(m) => {
                matrix::to_value(m)
            }
            DiffrnReflnsTerms::ReflnsNumber(u) => Value::from(*u),
            DiffrnReflnsTerms::Point_group_measured_fraction_full(w) => Value::from(*w),
            DiffrnReflnsTerms::Point_group_measured_fraction_max(x) => Value::from(*x),
            DiffrnReflnsTerms::Point_measured_fraction_full(y) => Value::from(*y),
            DiffrnReflnsTerms::Point_measured_fraction_max(z) => Value::from(*z),
            DiffrnReflnsTerms::Reduction_process(s) => Value::CharString(s.clone()),
            DiffrnReflnsTerms::Resolution_full(q) => Value::from(*q),
            DiffrnReflnsTerms::Resolution_max(r) => Value::from(*r),
            DiffrnReflnsTerms::Theta_full(f) => Value::from(*f),
            DiffrnReflnsTerms::Theta_max(g) => Value::from(*g),
            DiffrnReflnsTerms::Theta_min(h) => Value::from(*h),
        }
    }
}

impl Display for DiffrnReflnsTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `diffrn_reflns` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsLoopItem {
    /// The `_diffrn_reflns_limit_h_max` item. The variant holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    Limit_h_max(Vec<Numeric>),
    /// The `_diffrn_reflns_limit_h_min` item. The variant holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    Limit_h_min(Vec<Numeric>),
    /// The `_diffrn_reflns_limit_k_max` item. The variant holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    Limit_k_max(Vec<Numeric>),
    /// The `_diffrn_reflns_limit_k_min` item. The variant holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    Limit_k_min(Vec<Numeric>),
    /// The `_diffrn_reflns_limit_l_max` item. The variant holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    Limit_l_max(Vec<Numeric>),
    /// The `_diffrn_reflns_limit_l_min` item. The variant holds the column values, one per loop row.
    ///
    /// The limits on the Miller indices of the intensities specified by
    /// _diffrn_refln_index_h, *_k, *_l.
    Limit_l_min(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_11` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_11(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_12` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_12(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_13` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_13(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_21` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_21(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_22` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_22(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_23` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_23(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_31` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_31(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_32` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_32(Vec<Numeric>),
    /// The `_diffrn_reflns_transf_matrix_33` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of the matrix used to transform the diffraction reflection
    /// indices _diffrn_refln_index_h, *_k, *_l into the _refln_index_h,
    /// *_k, *_l indices. |11 12 13| (h k l) diffraction |21 22 23| = (h' k'
    /// l') |31 32 33|
    Transf_matrix_33(Vec<Numeric>),
}

impl CifTerm for DiffrnReflnsLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnReflnsLoopItem::Limit_h_max(_) => "limit_h_max",
            DiffrnReflnsLoopItem::Limit_h_min(_) => "limit_h_min",
            DiffrnReflnsLoopItem::Limit_k_max(_) => "limit_k_max",
            DiffrnReflnsLoopItem::Limit_k_min(_) => "limit_k_min",
            DiffrnReflnsLoopItem::Limit_l_max(_) => "limit_l_max",
            DiffrnReflnsLoopItem::Limit_l_min(_) => "limit_l_min",
            DiffrnReflnsLoopItem::Transf_matrix_11(_) => "transf_matrix_11",
            DiffrnReflnsLoopItem::Transf_matrix_12(_) => "transf_matrix_12",
            DiffrnReflnsLoopItem::Transf_matrix_13(_) => "transf_matrix_13",
            DiffrnReflnsLoopItem::Transf_matrix_21(_) => "transf_matrix_21",
            DiffrnReflnsLoopItem::Transf_matrix_22(_) => "transf_matrix_22",
            DiffrnReflnsLoopItem::Transf_matrix_23(_) => "transf_matrix_23",
            DiffrnReflnsLoopItem::Transf_matrix_31(_) => "transf_matrix_31",
            DiffrnReflnsLoopItem::Transf_matrix_32(_) => "transf_matrix_32",
            DiffrnReflnsLoopItem::Transf_matrix_33(_) => "transf_matrix_33",
        };
        Tag::new(format!("diffrn_reflns_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnReflnsLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnReflnsLoopItem::Limit_h_max(n)
            | DiffrnReflnsLoopItem::Limit_h_min(n)
            | DiffrnReflnsLoopItem::Limit_k_max(n)
            | DiffrnReflnsLoopItem::Limit_k_min(n)
            | DiffrnReflnsLoopItem::Limit_l_max(n)
            | DiffrnReflnsLoopItem::Limit_l_min(n)
            | DiffrnReflnsLoopItem::Transf_matrix_11(n)
            | DiffrnReflnsLoopItem::Transf_matrix_12(n)
            | DiffrnReflnsLoopItem::Transf_matrix_13(n)
            | DiffrnReflnsLoopItem::Transf_matrix_21(n)
            | DiffrnReflnsLoopItem::Transf_matrix_22(n)
            | DiffrnReflnsLoopItem::Transf_matrix_23(n)
            | DiffrnReflnsLoopItem::Transf_matrix_31(n)
            | DiffrnReflnsLoopItem::Transf_matrix_32(n)
            | DiffrnReflnsLoopItem::Transf_matrix_33(n) => {
                n.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
