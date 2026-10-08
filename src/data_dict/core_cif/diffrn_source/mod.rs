mod current;
mod power;
mod take_off_angle;
mod target;
mod voltage;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::current::Current;
pub use self::power::Power;
pub use self::take_off_angle::TakeOffAngle;
pub use self::target::Target;
pub use self::voltage::Voltage;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnSourceTerms {
    /// range constraint recorded in the dictionary _enumeration
    Current(Current),
    Details(CharString),
    Diffrn_radiation_source(CharString),
    Diffrn_source(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Power(Power),
    Size(CharString),
    /// range constraint recorded in the dictionary _enumeration
    TakeOffAngle(TakeOffAngle),
    /// closed code set recorded in the dictionary _enumeration
    Target(Target),
    Type(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Voltage(Voltage),
}

impl CifTerm for DiffrnSourceTerms {
    fn tag(&self) -> Tag {
        let tag = match self {
            DiffrnSourceTerms::Current(_) => "diffrn_source_current",
            DiffrnSourceTerms::Details(_) => "diffrn_source_details",
            DiffrnSourceTerms::Diffrn_radiation_source(_) => "diffrn_radiation_source",
            DiffrnSourceTerms::Diffrn_source(_) => "diffrn_source",
            DiffrnSourceTerms::Power(_) => "diffrn_source_power",
            DiffrnSourceTerms::Size(_) => "diffrn_source_size",
            DiffrnSourceTerms::TakeOffAngle(_) => "diffrn_source_take-off_angle",
            DiffrnSourceTerms::Target(_) => "diffrn_source_target",
            DiffrnSourceTerms::Type(_) => "diffrn_source_type",
            DiffrnSourceTerms::Voltage(_) => "diffrn_source_voltage",
        };
        Tag::new(tag.to_string())
    }
}

impl SingleValueTerm for DiffrnSourceTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnSourceTerms::Current(v) => Value::from(*v),
            DiffrnSourceTerms::Details(s)
            | DiffrnSourceTerms::Diffrn_radiation_source(s)
            | DiffrnSourceTerms::Diffrn_source(s)
            | DiffrnSourceTerms::Size(s)
            | DiffrnSourceTerms::Type(s) => Value::CharString(s.clone()),
            DiffrnSourceTerms::Power(m) => Value::from(*m),
            DiffrnSourceTerms::TakeOffAngle(c) => Value::from(*c),
            DiffrnSourceTerms::Target(b) => Value::from(*b),
            DiffrnSourceTerms::Voltage(t) => Value::from(*t),
        }
    }
}

impl Display for DiffrnSourceTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
