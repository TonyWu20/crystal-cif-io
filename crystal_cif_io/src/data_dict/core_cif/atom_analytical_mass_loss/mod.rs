mod temperature;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

pub use self::temperature::Temperature;

/// The single-value terms of the `atom_analytical_mass_loss` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomAnalyticalMassLossTerms {
    /// The `_atom_analytical_mass_loss_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_atom_analytical_mass_loss_meas_id` item. The variant holds the single value.
    Meas_id(CharString),
    /// The `_atom_analytical_mass_loss_percent` item. The variant holds the single value.
    Percent(Numeric),
    /// The `_atom_analytical_mass_loss_special_details` item. The variant holds the single value.
    Special_details(CharString),
    /// The `_atom_analytical_mass_loss_temperature` item. The variant holds the single value.
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
