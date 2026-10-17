mod area_resol_mean;
mod diffrn_detector;
mod diffrn_radiation_detector_dtime;
mod dtime;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::area_resol_mean::AreaResolMean;
pub use self::diffrn_detector::DiffrnDetector;
pub use self::diffrn_radiation_detector_dtime::DiffrnRadiationDetectorDtime;
pub use self::dtime::Dtime;

/// The single-value terms of the `diffrn_detector` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnDetectorTerms {
    /// The `_diffrn_detector_area_resol_mean` item. The variant holds the single value.
    ///
    /// The resolution of an area detector, in pixels/mm.
    /// Units: pixel/mm (pixel per millimetre).
    /// range constraint recorded in the dictionary _enumeration
    Area_resol_mean(AreaResolMean),
    /// The `_diffrn_detector_details` item. The variant holds the single value.
    ///
    /// A description of special aspects of the radiation detector.
    Details(CharString),
    /// The `_diffrn_detector` item. The variant holds the single value.
    ///
    /// The general class of the radiation detector.
    Diffrn_detector(CharString),
    /// The `_diffrn_radiation_detector` item. The variant holds the single value.
    ///
    /// The detector used to measure the diffraction intensities.
    Diffrn_radiation_detector(CharString),
    /// The `_diffrn_radiation_detector_dtime` item. The variant holds the single value.
    ///
    /// The deadtime in microseconds of the detector used to measure the
    /// diffraction intensities.
    /// Units: \ms (microsecond).
    /// range constraint recorded in the dictionary _enumeration
    Diffrn_radiation_detector_dtime(DiffrnRadiationDetectorDtime),
    /// The `_diffrn_detector_dtime` item. The variant holds the single value.
    ///
    /// The deadtime in microseconds of the detector used to measure the
    /// diffraction intensities.
    /// Units: \ms (microsecond).
    /// range constraint recorded in the dictionary _enumeration
    Dtime(Dtime),
    /// The `_diffrn_detector_type` item. The variant holds the single value.
    ///
    /// The make, model or name of the detector device used.
    Type(CharString),
}

impl CifTerm for DiffrnDetectorTerms {
    fn tag(&self) -> Tag {
        let tag = match self {
            DiffrnDetectorTerms::Area_resol_mean(_) => "diffrn_detector_area_resol_mean",
            DiffrnDetectorTerms::Details(_) => "diffrn_detector_details",
            DiffrnDetectorTerms::Diffrn_detector(_) => "diffrn_detector",
            DiffrnDetectorTerms::Diffrn_radiation_detector(_) => "diffrn_radiation_detector",
            DiffrnDetectorTerms::Diffrn_radiation_detector_dtime(_) => {
                "diffrn_radiation_detector_dtime"
            }
            DiffrnDetectorTerms::Dtime(_) => "diffrn_detector_dtime",
            DiffrnDetectorTerms::Type(_) => "diffrn_detector_type",
        };
        Tag::new(tag.to_string())
    }
}

impl SingleValueTerm for DiffrnDetectorTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnDetectorTerms::Area_resol_mean(v) => Value::from(*v),
            DiffrnDetectorTerms::Details(s)
            | DiffrnDetectorTerms::Diffrn_detector(s)
            | DiffrnDetectorTerms::Diffrn_radiation_detector(s)
            | DiffrnDetectorTerms::Type(s) => Value::CharString(s.clone()),
            DiffrnDetectorTerms::Diffrn_radiation_detector_dtime(m) => Value::from(*m),
            DiffrnDetectorTerms::Dtime(c) => Value::from(*c),
        }
    }
}

impl Display for DiffrnDetectorTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
