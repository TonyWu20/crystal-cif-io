mod decay_percent;
mod diffrn_standards;
mod interval_count;
mod interval_time;
mod scale_sigma;
mod scale_u;
mod standards_number;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Tag, Value};

pub use self::decay_percent::DecayPercent;
pub use self::diffrn_standards::DiffrnStandards;
pub use self::interval_count::IntervalCount;
pub use self::interval_time::IntervalTime;
pub use self::scale_sigma::ScaleSigma;
pub use self::scale_u::ScaleU;
pub use self::standards_number::StandardsNumber;

/// The single-value terms of the `diffrn_standards` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnStandardsTerms {
    /// The `_diffrn_standards_decay_%` item. The variant holds the single value.
    ///
    /// The percentage decrease in the mean intensity of the set of standard
    /// reflections measured at the start of the measurement process and at
    /// the finish. This value usually affords a measure of the overall
    /// decay in crystal quality during the diffraction measurement process.
    /// Negative values are used in exceptional instances where the final
    /// intensities are greater than the initial ones. If no measurable
    /// decay has occurred, the standard uncertainty should be quoted to
    /// indicate the maximum possible value the decay might have. A range of
    /// 3 standard uncertainties is considered possible. Thus 0.0(1) would
    /// indicate a decay of less than 0.3% or an enhancement of less than
    /// 0.3%.
    /// range constraint recorded in the dictionary _enumeration
    DecayPercent(DecayPercent),
    /// The `_diffrn_standards_interval_count` item. The variant holds the single value.
    ///
    /// The number of reflection intensities between the measurements of
    /// standard reflection intensities.
    /// range constraint recorded in the dictionary _enumeration
    Interval_count(IntervalCount),
    /// The `_diffrn_standards_interval_time` item. The variant holds the single value.
    ///
    /// The time in minutes between the measurements of standard reflection
    /// intensities.
    /// Units: min (minute).
    /// range constraint recorded in the dictionary _enumeration
    Interval_time(IntervalTime),
    /// The `_diffrn_standards_number` item. The variant holds the single value.
    ///
    /// The number of unique standard reflections used during the
    /// measurement of the diffraction intensities.
    /// range constraint recorded in the dictionary _enumeration
    StandardsNumber(StandardsNumber),
    /// The `_diffrn_standards_scale_sigma` item. The variant holds the single value.
    ///
    /// The standard uncertainty (e.s.d.) of the individual mean standard
    /// scales applied to the intensity data.
    /// range constraint recorded in the dictionary _enumeration
    Scale_sigma(ScaleSigma),
    /// The `_diffrn_standards_scale_u` item. The variant holds the single value.
    ///
    /// The standard uncertainty of the individual mean standard scales
    /// applied to the intensity data.
    /// range constraint recorded in the dictionary _enumeration
    Scale_u(ScaleU),
}

impl CifTerm for DiffrnStandardsTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnStandardsTerms::DecayPercent(_) => "decay_%",
            DiffrnStandardsTerms::Interval_count(_) => "interval_count",
            DiffrnStandardsTerms::Interval_time(_) => "interval_time",
            DiffrnStandardsTerms::StandardsNumber(_) => "number",
            DiffrnStandardsTerms::Scale_sigma(_) => "scale_sigma",
            DiffrnStandardsTerms::Scale_u(_) => "scale_u",
        };
        Tag::new(format!("diffrn_standards_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnStandardsTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnStandardsTerms::DecayPercent(v) => Value::from(*v),
            DiffrnStandardsTerms::Interval_count(m) => Value::from(*m),
            DiffrnStandardsTerms::Interval_time(c) => Value::from(*c),
            DiffrnStandardsTerms::StandardsNumber(b) => Value::from(*b),
            DiffrnStandardsTerms::Scale_sigma(t) => Value::from(*t),
            DiffrnStandardsTerms::Scale_u(u) => Value::from(*u),
        }
    }
}

impl Display for DiffrnStandardsTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
