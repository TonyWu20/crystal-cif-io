mod diffrn_measurement;
mod method_precession;
mod rotation_mode;
mod sample_tracking;
mod specimen_attachment_type;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::diffrn_measurement::DiffrnMeasurement;
pub use self::method_precession::MethodPrecession;
pub use self::rotation_mode::RotationMode;
pub use self::sample_tracking::SampleTracking;
pub use self::specimen_attachment_type::SpecimenAttachmentType;

/// The single-value terms of the `diffrn_measurement` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnMeasurementTerms {
    /// The `_diffrn_measurement_details` item. The variant holds the single value.
    ///
    /// A description of special aspects of the intensity measurement.
    Details(CharString),
    /// The `_diffrn_measurement_device` item. The variant holds the single value.
    ///
    /// The general class of goniometer or device used to support and orient
    /// the specimen.
    Device(CharString),
    /// The `_diffrn_measurement_device_details` item. The variant holds the single value.
    ///
    /// A description of special aspects of the device used to measure the
    /// diffraction intensities.
    Device_details(CharString),
    /// The `_diffrn_measurement_device_type` item. The variant holds the single value.
    ///
    /// The make, model or name of the measurement device (goniometer) used.
    Device_type(CharString),
    /// The `_diffrn_measurement_method` item. The variant holds the single value.
    ///
    /// Method used to measure the intensities.
    Method(CharString),
    /// The `_diffrn_measurement_method_precession` item. The variant holds the single value.
    /// closed code set recorded in the dictionary _enumeration_set
    Method_precession(MethodPrecession),
    /// The `_diffrn_measurement_rotation_mode` item. The variant holds the single value.
    /// closed code set recorded in the dictionary _enumeration_set
    Rotation_mode(RotationMode),
    /// The `_diffrn_measurement_sample_tracking` item. The variant holds the single value.
    /// closed code set recorded in the dictionary _enumeration_set
    Sample_tracking(SampleTracking),
    /// The `_diffrn_measurement_sample_tracking_method` item. The variant holds the single value.
    Sample_tracking_method(CharString),
    /// The `_diffrn_measurement_specimen_attachment_type` item. The variant holds the single value.
    /// closed code set recorded in the dictionary _enumeration_set
    Specimen_attachment_type(SpecimenAttachmentType),
    /// The `_diffrn_measurement_specimen_support` item. The variant holds the single value.
    ///
    /// The physical device used to support the crystal during data
    /// collection.
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
