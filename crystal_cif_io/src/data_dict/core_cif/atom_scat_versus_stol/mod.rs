mod scat_value;
mod stol_value;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::scat_value::ScatValue;
pub use self::stol_value::StolValue;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomScatVersusStolTerms {
    Atom_type(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Scat_value(ScatValue),
    /// range constraint recorded in the dictionary _enumeration
    Stol_value(StolValue),
}

impl CifTerm for AtomScatVersusStolTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomScatVersusStolTerms::Atom_type(_) => "atom_type",
            AtomScatVersusStolTerms::Scat_value(_) => "scat_value",
            AtomScatVersusStolTerms::Stol_value(_) => "stol_value",
        };
        Tag::new(format!("atom_scat_versus_stol_{suffix}"))
    }
}

impl SingleValueTerm for AtomScatVersusStolTerms {
    fn value(&self) -> Value {
        match self {
            AtomScatVersusStolTerms::Atom_type(s) => Value::CharString(s.clone()),
            AtomScatVersusStolTerms::Scat_value(v) => Value::from(*v),
            AtomScatVersusStolTerms::Stol_value(m) => Value::from(*m),
        }
    }
}

impl Display for AtomScatVersusStolTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
