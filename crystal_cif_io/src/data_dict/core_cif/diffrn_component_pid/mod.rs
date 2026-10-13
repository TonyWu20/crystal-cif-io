mod framework;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::framework::Framework;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnComponentPidTerms {
    Details(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Framework(Framework),
    Pid(CharString),
}

impl CifTerm for DiffrnComponentPidTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnComponentPidTerms::Details(_) => "details",
            DiffrnComponentPidTerms::Framework(_) => "framework",
            DiffrnComponentPidTerms::Pid(_) => "pid",
        };
        Tag::new(format!("diffrn_component_pid_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnComponentPidTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnComponentPidTerms::Details(s) | DiffrnComponentPidTerms::Pid(s) => {
                Value::CharString(s.clone())
            }
            DiffrnComponentPidTerms::Framework(v) => Value::from(*v),
        }
    }
}

impl Display for DiffrnComponentPidTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
