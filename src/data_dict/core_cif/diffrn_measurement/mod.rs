use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnMeasurementTerms {
    Details(CharString),
    Device(CharString),
    Device_details(CharString),
    Device_type(CharString),
    Method(CharString),
    Specimen_support(CharString),
}

impl CifTerm for DiffrnMeasurementTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnMeasurementTerms::Details(_) => "details",
            DiffrnMeasurementTerms::Device(_) => "device",
            DiffrnMeasurementTerms::Device_details(_) => "device_details",
            DiffrnMeasurementTerms::Device_type(_) => "device_type",
            DiffrnMeasurementTerms::Method(_) => "method",
            DiffrnMeasurementTerms::Specimen_support(_) => "specimen_support",
        };
        Tag::new(format!("diffrn_measurement_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnMeasurementTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnMeasurementTerms::Details(s)
            | DiffrnMeasurementTerms::Device(s)
            | DiffrnMeasurementTerms::Device_details(s)
            | DiffrnMeasurementTerms::Device_type(s)
            | DiffrnMeasurementTerms::Method(s)
            | DiffrnMeasurementTerms::Specimen_support(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for DiffrnMeasurementTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
