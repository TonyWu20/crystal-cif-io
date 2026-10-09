mod temperature;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Numeric, Tag, Value},
};

pub use self::temperature::Temperature;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomAnalyticalMassLossTerms {
    Id(CharString),
    Meas_id(CharString),
    Percent(Numeric),
    Special_details(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Temperature(Temperature),
}

impl CifTerm for AtomAnalyticalMassLossTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomAnalyticalMassLossTerms::Id(_) => "id",
            AtomAnalyticalMassLossTerms::Meas_id(_) => "meas_id",
            AtomAnalyticalMassLossTerms::Percent(_) => "percent",
            AtomAnalyticalMassLossTerms::Special_details(_) => "special_details",
            AtomAnalyticalMassLossTerms::Temperature(_) => "temperature",
        };
        Tag::new(format!("atom_analytical_mass_loss_{suffix}"))
    }
}

impl SingleValueTerm for AtomAnalyticalMassLossTerms {
    fn value(&self) -> Value {
        match self {
            AtomAnalyticalMassLossTerms::Id(s)
            | AtomAnalyticalMassLossTerms::Meas_id(s)
            | AtomAnalyticalMassLossTerms::Special_details(s) => Value::CharString(s.clone()),
            AtomAnalyticalMassLossTerms::Percent(n) => Value::Numeric(*n),
            AtomAnalyticalMassLossTerms::Temperature(v) => Value::from(*v),
        }
    }
}

impl Display for AtomAnalyticalMassLossTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
