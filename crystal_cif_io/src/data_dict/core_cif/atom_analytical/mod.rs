mod analyte_mass_percent;
mod chemical_species_mass_percent;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::analyte_mass_percent::AnalyteMassPercent;
pub use self::chemical_species_mass_percent::ChemicalSpeciesMassPercent;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomAnalyticalTerms {
    Analyte(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Analyte_mass_percent(AnalyteMassPercent),
    Chemical_species(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Chemical_species_mass_percent(ChemicalSpeciesMassPercent),
    Id(CharString),
    Meas_id(CharString),
}

impl CifTerm for AtomAnalyticalTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomAnalyticalTerms::Analyte(_) => "analyte",
            AtomAnalyticalTerms::Analyte_mass_percent(_) => "analyte_mass_percent",
            AtomAnalyticalTerms::Chemical_species(_) => "chemical_species",
            AtomAnalyticalTerms::Chemical_species_mass_percent(_) => {
                "chemical_species_mass_percent"
            }
            AtomAnalyticalTerms::Id(_) => "id",
            AtomAnalyticalTerms::Meas_id(_) => "meas_id",
        };
        Tag::new(format!("atom_analytical_{suffix}"))
    }
}

impl SingleValueTerm for AtomAnalyticalTerms {
    fn value(&self) -> Value {
        match self {
            AtomAnalyticalTerms::Analyte(s)
            | AtomAnalyticalTerms::Chemical_species(s)
            | AtomAnalyticalTerms::Id(s)
            | AtomAnalyticalTerms::Meas_id(s) => Value::CharString(s.clone()),
            AtomAnalyticalTerms::Analyte_mass_percent(v) => Value::from(*v),
            AtomAnalyticalTerms::Chemical_species_mass_percent(m) => Value::from(*m),
        }
    }
}

impl Display for AtomAnalyticalTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
