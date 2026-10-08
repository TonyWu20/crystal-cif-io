mod av_r_equivalents;
mod av_sigma_i_over_net_i;
mod av_unet_i_over_net_i;
mod laue_measured_fraction_full;
mod laue_measured_fraction_max;
mod point_group_measured_fraction_full;
mod point_group_measured_fraction_max;
mod reflns_number;
mod resolution_full;
mod resolution_max;
mod theta_full;
mod theta_max;
mod theta_min;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Numeric, Tag, Value},
};

pub use self::av_r_equivalents::AvREquivalents;
pub use self::av_sigma_i_over_net_i::AvSigmaIOverNetI;
pub use self::av_unet_i_over_net_i::AvUnetIOverNetI;
pub use self::laue_measured_fraction_full::LaueMeasuredFractionFull;
pub use self::laue_measured_fraction_max::LaueMeasuredFractionMax;
pub use self::point_group_measured_fraction_full::PointGroupMeasuredFractionFull;
pub use self::point_group_measured_fraction_max::PointGroupMeasuredFractionMax;
pub use self::reflns_number::ReflnsNumber;
pub use self::resolution_full::ResolutionFull;
pub use self::resolution_max::ResolutionMax;
pub use self::theta_full::ThetaFull;
pub use self::theta_max::ThetaMax;
pub use self::theta_min::ThetaMin;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsTerms {
    /// range constraint recorded in the dictionary _enumeration
    Laue_measured_fraction_full(LaueMeasuredFractionFull),
    /// range constraint recorded in the dictionary _enumeration
    Laue_measured_fraction_max(LaueMeasuredFractionMax),
    /// range constraint recorded in the dictionary _enumeration
    Av_R_equivalents(AvREquivalents),
    /// range constraint recorded in the dictionary _enumeration
    AvSigmaIOverNetI(AvSigmaIOverNetI),
    /// range constraint recorded in the dictionary _enumeration
    AvUnetIOverNetI(AvUnetIOverNetI),
    /// range constraint recorded in the dictionary _enumeration
    ReflnsNumber(ReflnsNumber),
    /// range constraint recorded in the dictionary _enumeration
    Point_group_measured_fraction_full(PointGroupMeasuredFractionFull),
    /// range constraint recorded in the dictionary _enumeration
    Point_group_measured_fraction_max(PointGroupMeasuredFractionMax),
    Reduction_process(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Resolution_full(ResolutionFull),
    /// range constraint recorded in the dictionary _enumeration
    Resolution_max(ResolutionMax),
    /// range constraint recorded in the dictionary _enumeration
    Theta_full(ThetaFull),
    /// range constraint recorded in the dictionary _enumeration
    Theta_max(ThetaMax),
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
            DiffrnReflnsTerms::ReflnsNumber(_) => "number",
            DiffrnReflnsTerms::Point_group_measured_fraction_full(_) => {
                "point_group_measured_fraction_full"
            }
            DiffrnReflnsTerms::Point_group_measured_fraction_max(_) => {
                "point_group_measured_fraction_max"
            }
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
            DiffrnReflnsTerms::ReflnsNumber(u) => Value::from(*u),
            DiffrnReflnsTerms::Point_group_measured_fraction_full(w) => Value::from(*w),
            DiffrnReflnsTerms::Point_group_measured_fraction_max(x) => Value::from(*x),
            DiffrnReflnsTerms::Reduction_process(s) => Value::CharString(s.clone()),
            DiffrnReflnsTerms::Resolution_full(y) => Value::from(*y),
            DiffrnReflnsTerms::Resolution_max(z) => Value::from(*z),
            DiffrnReflnsTerms::Theta_full(q) => Value::from(*q),
            DiffrnReflnsTerms::Theta_max(r) => Value::from(*r),
            DiffrnReflnsTerms::Theta_min(f) => Value::from(*f),
        }
    }
}

impl Display for DiffrnReflnsTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsLoopItem {
    Limit_h_max(Vec<Numeric>),
    Limit_h_min(Vec<Numeric>),
    Limit_k_max(Vec<Numeric>),
    Limit_k_min(Vec<Numeric>),
    Limit_l_max(Vec<Numeric>),
    Limit_l_min(Vec<Numeric>),
    Transf_matrix_11(Vec<Numeric>),
    Transf_matrix_12(Vec<Numeric>),
    Transf_matrix_13(Vec<Numeric>),
    Transf_matrix_21(Vec<Numeric>),
    Transf_matrix_22(Vec<Numeric>),
    Transf_matrix_23(Vec<Numeric>),
    Transf_matrix_31(Vec<Numeric>),
    Transf_matrix_32(Vec<Numeric>),
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
