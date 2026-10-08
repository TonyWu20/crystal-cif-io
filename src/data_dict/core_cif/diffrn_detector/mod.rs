mod area_resol_mean;
mod diffrn_radiation_detector_dtime;
mod dtime;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::area_resol_mean::AreaResolMean;
pub use self::diffrn_radiation_detector_dtime::DiffrnRadiationDetectorDtime;
pub use self::dtime::Dtime;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnDetectorTerms {
    /// range constraint recorded in the dictionary _enumeration
    Area_resol_mean(AreaResolMean),
    Details(CharString),
    Diffrn_detector(CharString),
    Diffrn_radiation_detector(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Diffrn_radiation_detector_dtime(DiffrnRadiationDetectorDtime),
    /// range constraint recorded in the dictionary _enumeration
    Dtime(Dtime),
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
