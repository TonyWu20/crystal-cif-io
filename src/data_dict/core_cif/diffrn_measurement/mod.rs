mod method_precession;
mod rotation_mode;
mod sample_tracking;
mod specimen_attachment_type;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::method_precession::MethodPrecession;
pub use self::rotation_mode::RotationMode;
pub use self::sample_tracking::SampleTracking;
pub use self::specimen_attachment_type::SpecimenAttachmentType;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnMeasurementTerms {
    Details(CharString),
    Device(CharString),
    Device_details(CharString),
    Device_type(CharString),
    Method(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Method_precession(MethodPrecession),
    /// closed code set recorded in the dictionary _enumeration_set
    Rotation_mode(RotationMode),
    /// closed code set recorded in the dictionary _enumeration_set
    Sample_tracking(SampleTracking),
    Sample_tracking_method(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Specimen_attachment_type(SpecimenAttachmentType),
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
            DiffrnMeasurementTerms::Method_precession(_) => "method_precession",
            DiffrnMeasurementTerms::Rotation_mode(_) => "rotation_mode",
            DiffrnMeasurementTerms::Sample_tracking(_) => "sample_tracking",
            DiffrnMeasurementTerms::Sample_tracking_method(_) => "sample_tracking_method",
            DiffrnMeasurementTerms::Specimen_attachment_type(_) => "specimen_attachment_type",
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
            | DiffrnMeasurementTerms::Sample_tracking_method(s)
            | DiffrnMeasurementTerms::Specimen_support(s) => Value::CharString(s.clone()),
            DiffrnMeasurementTerms::Method_precession(v) => Value::from(*v),
            DiffrnMeasurementTerms::Rotation_mode(m) => Value::from(*m),
            DiffrnMeasurementTerms::Sample_tracking(c) => Value::from(*c),
            DiffrnMeasurementTerms::Specimen_attachment_type(b) => Value::from(*b),
        }
    }
}

impl Display for DiffrnMeasurementTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
