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

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnStandardsTerms {
    /// range constraint recorded in the dictionary _enumeration
    DecayPercent(DecayPercent),
    /// range constraint recorded in the dictionary _enumeration
    Interval_count(IntervalCount),
    /// range constraint recorded in the dictionary _enumeration
    Interval_time(IntervalTime),
    /// range constraint recorded in the dictionary _enumeration
    StandardsNumber(StandardsNumber),
    /// range constraint recorded in the dictionary _enumeration
    Scale_sigma(ScaleSigma),
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
