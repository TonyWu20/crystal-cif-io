mod number_reflns_gt;
mod sample_thickness;
mod shift_over_su_max;
mod shift_over_su_mean;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Numeric, Tag, Value},
};

pub use self::number_reflns_gt::NumberReflnsGt;
pub use self::sample_thickness::SampleThickness;
pub use self::shift_over_su_max::ShiftOverSuMax;
pub use self::shift_over_su_mean::ShiftOverSuMean;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RefineLsTerms {
    Abs_structure_z_score(Numeric),
    /// range constraint recorded in the dictionary _enumeration
    Number_reflns_gt(NumberReflnsGt),
    Sample_shape_details(CharString),
    Sample_shape_expression(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Sample_thickness(SampleThickness),
    /// range constraint recorded in the dictionary _enumeration
    Shift_over_su_max(ShiftOverSuMax),
    /// range constraint recorded in the dictionary _enumeration
    Shift_over_su_mean(ShiftOverSuMean),
}

impl CifTerm for RefineLsTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            RefineLsTerms::Abs_structure_z_score(_) => "abs_structure_z_score",
            RefineLsTerms::Number_reflns_gt(_) => "number_reflns_gt",
            RefineLsTerms::Sample_shape_details(_) => "sample_shape_details",
            RefineLsTerms::Sample_shape_expression(_) => "sample_shape_expression",
            RefineLsTerms::Sample_thickness(_) => "sample_thickness",
            RefineLsTerms::Shift_over_su_max(_) => "shift_over_su_max",
            RefineLsTerms::Shift_over_su_mean(_) => "shift_over_su_mean",
        };
        Tag::new(format!("refine_ls_{suffix}"))
    }
}

impl SingleValueTerm for RefineLsTerms {
    fn value(&self) -> Value {
        match self {
            RefineLsTerms::Abs_structure_z_score(n) => Value::Numeric(*n),
            RefineLsTerms::Number_reflns_gt(v) => Value::from(*v),
            RefineLsTerms::Sample_shape_details(s) | RefineLsTerms::Sample_shape_expression(s) => {
                Value::CharString(s.clone())
            }
            RefineLsTerms::Sample_thickness(m) => Value::from(*m),
            RefineLsTerms::Shift_over_su_max(c) => Value::from(*c),
            RefineLsTerms::Shift_over_su_mean(b) => Value::from(*b),
        }
    }
}

impl Display for RefineLsTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
