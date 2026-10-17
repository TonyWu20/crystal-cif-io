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

/// The single-value terms of the `diffrn_radiation` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnRadiationTerms {
    /// The `_diffrn_radiation_collimation` item. The variant holds the single value.
    ///
    /// The collimation or focusing applied to the radiation.
    Collimation(CharString),
    /// The `_diffrn_radiation_filter_edge` item. The variant holds the single value.
    ///
    /// Absorption edge in angstroms of the radiation filter used.
    /// Units: A (angstrom).
    /// range constraint recorded in the dictionary _enumeration
    Filter_edge(FilterEdge),
    /// The `_diffrn_radiation_illumination_mode` item. The variant holds the single value.
    /// closed code set recorded in the dictionary _enumeration_set
    Illumination_mode(IlluminationMode),
    /// The `_diffrn_radiation_inhomogeneity` item. The variant holds the single value.
    ///
    /// Half-width in millimetres of the incident beam in the direction
    /// perpendicular to the diffraction plane.
    /// Units: mm (millimetre).
    /// range constraint recorded in the dictionary _enumeration
    Inhomogeneity(Inhomogeneity),
    /// The `_diffrn_radiation_monochromator` item. The variant holds the single value.
    ///
    /// The method used to obtain monochromatic radiation. If a mono-
    /// chromator crystal is used, the material and the indices of the Bragg
    /// reflection are specified.
    Monochromator(CharString),
    /// The `_diffrn_radiation_polarisn_norm` item. The variant holds the single value.
    ///
    /// The angle in degrees, as viewed from the specimen, between the
    /// perpendicular component of the polarization and the diffraction
    /// plane. See _diffrn_radiation_polarisn_ratio.
    /// Units: deg (degree of arc).
    /// range constraint recorded in the dictionary _enumeration
    Polarisn_norm(PolarisnNorm),
    /// The `_diffrn_radiation_polarisn_ratio` item. The variant holds the single value.
    ///
    /// Polarization ratio of the diffraction beam incident on the crystal.
    /// It is the ratio of the perpendicularly polarized to the parallel
    /// polarized components of the radiation. The perpendicular component
    /// forms an angle of _diffrn_radiation_polarisn_norm to the normal to
    /// the diffraction plane of the sample (i.e. the plane containing the
    /// incident and reflected beams).
    /// range constraint recorded in the dictionary _enumeration
    Polarisn_ratio(PolarisnRatio),
    /// The `_diffrn_radiation_probe` item. The variant holds the single value.
    ///
    /// The nature of the radiation used (i.e. the name of the subatomic
    /// particle or the region of the electromagnetic spectrum). It is
    /// strongly recommended that this information be given, so that the
    /// probe radiation can be simply determined.
    /// closed code set recorded in the dictionary _enumeration
    Probe(Probe),
    /// The `_diffrn_radiation_type` item. The variant holds the single value.
    ///
    /// The type of the radiation. This is used to give a more detailed
    /// description than _diffrn_radiation_probe and is typically a
    /// description of the X-ray wavelength in Siegbahn notation.
    Type(CharString),
    /// The `_diffrn_radiation_xray_symbol` item. The variant holds the single value.
    ///
    /// The IUPAC symbol for the X-ray wavelength for the probe radiation.
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
