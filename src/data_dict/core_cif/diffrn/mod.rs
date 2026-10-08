mod ambient_pressure;
mod ambient_pressure_gt;
mod ambient_pressure_lt;
mod ambient_temperature;
mod ambient_temperature_gt;
mod ambient_temperature_lt;
mod measured_fraction_theta_full;
mod measured_fraction_theta_max;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::ambient_pressure::AmbientPressure;
pub use self::ambient_pressure_gt::AmbientPressureGt;
pub use self::ambient_pressure_lt::AmbientPressureLt;
pub use self::ambient_temperature::AmbientTemperature;
pub use self::ambient_temperature_gt::AmbientTemperatureGt;
pub use self::ambient_temperature_lt::AmbientTemperatureLt;
pub use self::measured_fraction_theta_full::MeasuredFractionThetaFull;
pub use self::measured_fraction_theta_max::MeasuredFractionThetaMax;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnTerms {
    Ambient_environment(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Ambient_pressure(AmbientPressure),
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temperature(AmbientTemperature),
    Crystal_treatment(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Measured_fraction_theta_full(MeasuredFractionThetaFull),
    /// range constraint recorded in the dictionary _enumeration
    Measured_fraction_theta_max(MeasuredFractionThetaMax),
    Special_details(CharString),
    Symmetry_description(CharString),
}

impl CifTerm for DiffrnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnTerms::Ambient_environment(_) => "ambient_environment",
            DiffrnTerms::Ambient_pressure(_) => "ambient_pressure",
            DiffrnTerms::Ambient_temperature(_) => "ambient_temperature",
            DiffrnTerms::Crystal_treatment(_) => "crystal_treatment",
            DiffrnTerms::Measured_fraction_theta_full(_) => "measured_fraction_theta_full",
            DiffrnTerms::Measured_fraction_theta_max(_) => "measured_fraction_theta_max",
            DiffrnTerms::Special_details(_) => "special_details",
            DiffrnTerms::Symmetry_description(_) => "symmetry_description",
        };
        Tag::new(format!("diffrn_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnTerms::Ambient_environment(s)
            | DiffrnTerms::Crystal_treatment(s)
            | DiffrnTerms::Special_details(s)
            | DiffrnTerms::Symmetry_description(s) => Value::CharString(s.clone()),
            DiffrnTerms::Ambient_pressure(v) => Value::from(*v),
            DiffrnTerms::Ambient_temperature(m) => Value::from(*m),
            DiffrnTerms::Measured_fraction_theta_full(c) => Value::from(*c),
            DiffrnTerms::Measured_fraction_theta_max(b) => Value::from(*b),
        }
    }
}

impl Display for DiffrnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnLoopItem {
    /// range constraint recorded in the dictionary _enumeration
    Ambient_pressure_gt(Vec<AmbientPressureGt>),
    /// range constraint recorded in the dictionary _enumeration
    Ambient_pressure_lt(Vec<AmbientPressureLt>),
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temperature_gt(Vec<AmbientTemperatureGt>),
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temperature_lt(Vec<AmbientTemperatureLt>),
}

impl CifTerm for DiffrnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnLoopItem::Ambient_pressure_gt(_) => "ambient_pressure_gt",
            DiffrnLoopItem::Ambient_pressure_lt(_) => "ambient_pressure_lt",
            DiffrnLoopItem::Ambient_temperature_gt(_) => "ambient_temperature_gt",
            DiffrnLoopItem::Ambient_temperature_lt(_) => "ambient_temperature_lt",
        };
        Tag::new(format!("diffrn_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnLoopItem::Ambient_pressure_gt(v) => v.iter().cloned().map(Value::from).collect(),
            DiffrnLoopItem::Ambient_pressure_lt(m) => m.iter().cloned().map(Value::from).collect(),
            DiffrnLoopItem::Ambient_temperature_gt(c) => {
                c.iter().cloned().map(Value::from).collect()
            }
            DiffrnLoopItem::Ambient_temperature_lt(b) => {
                b.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
