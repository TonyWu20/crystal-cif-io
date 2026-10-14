mod diffrn_radiation;
mod filter_edge;
mod illumination_mode;
mod inhomogeneity;
mod polarisn_norm;
mod polarisn_ratio;
mod probe;
mod xray_symbol;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::diffrn_radiation::DiffrnRadiation;
pub use self::filter_edge::FilterEdge;
pub use self::illumination_mode::IlluminationMode;
pub use self::inhomogeneity::Inhomogeneity;
pub use self::polarisn_norm::PolarisnNorm;
pub use self::polarisn_ratio::PolarisnRatio;
pub use self::probe::Probe;
pub use self::xray_symbol::XraySymbol;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnRadiationTerms {
    Collimation(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Filter_edge(FilterEdge),
    /// closed code set recorded in the dictionary _enumeration_set
    Illumination_mode(IlluminationMode),
    /// range constraint recorded in the dictionary _enumeration
    Inhomogeneity(Inhomogeneity),
    Monochromator(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Polarisn_norm(PolarisnNorm),
    /// range constraint recorded in the dictionary _enumeration
    Polarisn_ratio(PolarisnRatio),
    /// closed code set recorded in the dictionary _enumeration
    Probe(Probe),
    Type(CharString),
    /// closed code set recorded in the dictionary _enumeration
    Xray_symbol(XraySymbol),
}

impl CifTerm for DiffrnRadiationTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnRadiationTerms::Collimation(_) => "collimation",
            DiffrnRadiationTerms::Filter_edge(_) => "filter_edge",
            DiffrnRadiationTerms::Illumination_mode(_) => "illumination_mode",
            DiffrnRadiationTerms::Inhomogeneity(_) => "inhomogeneity",
            DiffrnRadiationTerms::Monochromator(_) => "monochromator",
            DiffrnRadiationTerms::Polarisn_norm(_) => "polarisn_norm",
            DiffrnRadiationTerms::Polarisn_ratio(_) => "polarisn_ratio",
            DiffrnRadiationTerms::Probe(_) => "probe",
            DiffrnRadiationTerms::Type(_) => "type",
            DiffrnRadiationTerms::Xray_symbol(_) => "xray_symbol",
        };
        Tag::new(format!("diffrn_radiation_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnRadiationTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnRadiationTerms::Collimation(s)
            | DiffrnRadiationTerms::Monochromator(s)
            | DiffrnRadiationTerms::Type(s) => Value::CharString(s.clone()),
            DiffrnRadiationTerms::Filter_edge(v) => Value::from(*v),
            DiffrnRadiationTerms::Illumination_mode(m) => Value::from(*m),
            DiffrnRadiationTerms::Inhomogeneity(c) => Value::from(*c),
            DiffrnRadiationTerms::Polarisn_norm(b) => Value::from(*b),
            DiffrnRadiationTerms::Polarisn_ratio(t) => Value::from(*t),
            DiffrnRadiationTerms::Probe(u) => Value::from(*u),
            DiffrnRadiationTerms::Xray_symbol(w) => Value::from(*w),
        }
    }
}

impl Display for DiffrnRadiationTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
