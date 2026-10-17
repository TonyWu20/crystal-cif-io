mod ambient_pressure;
mod ambient_pressure_gt;
mod ambient_pressure_lt;
mod ambient_temp_gt;
mod ambient_temp_lt;
mod ambient_temperature;
mod ambient_temperature_gt;
mod ambient_temperature_lt;
mod diffrn;
mod flux_density;
mod measured_fraction_theta_full;
mod measured_fraction_theta_max;
mod precession_semi_angle;
mod total_dose;
mod total_exposure_time;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::ambient_pressure::AmbientPressure;
pub use self::ambient_pressure_gt::AmbientPressureGt;
pub use self::ambient_pressure_lt::AmbientPressureLt;
pub use self::ambient_temp_gt::AmbientTempGt;
pub use self::ambient_temp_lt::AmbientTempLt;
pub use self::ambient_temperature::AmbientTemperature;
pub use self::ambient_temperature_gt::AmbientTemperatureGt;
pub use self::ambient_temperature_lt::AmbientTemperatureLt;
pub use self::diffrn::{Diffrn, DiffrnRow};
pub use self::flux_density::FluxDensity;
pub use self::measured_fraction_theta_full::MeasuredFractionThetaFull;
pub use self::measured_fraction_theta_max::MeasuredFractionThetaMax;
pub use self::precession_semi_angle::PrecessionSemiAngle;
pub use self::total_dose::TotalDose;
pub use self::total_exposure_time::TotalExposureTime;

/// The single-value terms of the `diffrn` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnTerms {
    /// The `_diffrn_ambient_environment` item. The variant holds the single value.
    ///
    /// The gas or liquid surrounding the sample, if not air.
    Ambient_environment(CharString),
    /// The `_diffrn_ambient_pressure` item. The variant holds the single value.
    ///
    /// The mean hydrostatic pressure in kilopascals at which the
    /// intensities were measured.
    /// Units: kPa (kilopascal).
    /// range constraint recorded in the dictionary _enumeration
    Ambient_pressure(AmbientPressure),
    /// The `_diffrn_ambient_temp_details` item. The variant holds the single value.
    Ambient_temp_details(CharString),
    /// The `_diffrn_ambient_temp_gt` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temp_gt(AmbientTempGt),
    /// The `_diffrn_ambient_temp_lt` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temp_lt(AmbientTempLt),
    /// The `_diffrn_ambient_temperature` item. The variant holds the single value.
    ///
    /// The mean temperature in kelvins at which the intensities were
    /// measured.
    /// Units: K (kelvin).
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temperature(AmbientTemperature),
    /// The `_diffrn_crystal_support` item. The variant holds the single value.
    Crystal_support(CharString),
    /// The `_diffrn_crystal_treatment` item. The variant holds the single value.
    ///
    /// Remarks about how the crystal was treated prior to the intensity
    /// measurements. Particularly relevant when intensities were measured
    /// at low temperature.
    Crystal_treatment(CharString),
    /// The `_diffrn_flux_density` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Flux_density(FluxDensity),
    /// The `_diffrn_measured_fraction_theta_full` item. The variant holds the single value.
    ///
    /// Fraction of unique (symmetry-independent) reflections measured out
    /// to _diffrn_reflns_theta_full.
    /// range constraint recorded in the dictionary _enumeration
    Measured_fraction_theta_full(MeasuredFractionThetaFull),
    /// The `_diffrn_measured_fraction_theta_max` item. The variant holds the single value.
    ///
    /// Fraction of unique (symmetry-independent) reflections measured out
    /// to _diffrn_reflns_theta_max.
    /// range constraint recorded in the dictionary _enumeration
    Measured_fraction_theta_max(MeasuredFractionThetaMax),
    /// The `_diffrn_precession_semi_angle` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Precession_semi_angle(PrecessionSemiAngle),
    /// The `_diffrn_special_details` item. The variant holds the single value.
    ///
    /// Special details of the intensity-measurement process. Should include
    /// information about source instability, crystal motion, degradation
    /// and so on.
    Special_details(CharString),
    /// The `_diffrn_symmetry_description` item. The variant holds the single value.
    ///
    /// Observed diffraction point symmetry, systematic absences and
    /// possible space group(s) or superspace group(s) compatible with
    /// these.
    Symmetry_description(CharString),
    /// The `_diffrn_total_dose` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Total_dose(TotalDose),
    /// The `_diffrn_total_exposure_time` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Total_exposure_time(TotalExposureTime),
}

impl CifTerm for DiffrnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnTerms::Ambient_environment(_) => "ambient_environment",
            DiffrnTerms::Ambient_pressure(_) => "ambient_pressure",
            DiffrnTerms::Ambient_temp_details(_) => "ambient_temp_details",
            DiffrnTerms::Ambient_temp_gt(_) => "ambient_temp_gt",
            DiffrnTerms::Ambient_temp_lt(_) => "ambient_temp_lt",
            DiffrnTerms::Ambient_temperature(_) => "ambient_temperature",
            DiffrnTerms::Crystal_support(_) => "crystal_support",
            DiffrnTerms::Crystal_treatment(_) => "crystal_treatment",
            DiffrnTerms::Flux_density(_) => "flux_density",
            DiffrnTerms::Measured_fraction_theta_full(_) => "measured_fraction_theta_full",
            DiffrnTerms::Measured_fraction_theta_max(_) => "measured_fraction_theta_max",
            DiffrnTerms::Precession_semi_angle(_) => "precession_semi_angle",
            DiffrnTerms::Special_details(_) => "special_details",
            DiffrnTerms::Symmetry_description(_) => "symmetry_description",
            DiffrnTerms::Total_dose(_) => "total_dose",
            DiffrnTerms::Total_exposure_time(_) => "total_exposure_time",
        };
        Tag::new(format!("diffrn_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnTerms::Ambient_environment(s)
            | DiffrnTerms::Ambient_temp_details(s)
            | DiffrnTerms::Crystal_support(s)
            | DiffrnTerms::Crystal_treatment(s)
            | DiffrnTerms::Special_details(s)
            | DiffrnTerms::Symmetry_description(s) => Value::CharString(s.clone()),
            DiffrnTerms::Ambient_pressure(v) => Value::from(*v),
            DiffrnTerms::Ambient_temp_gt(m) => Value::from(*m),
            DiffrnTerms::Ambient_temp_lt(c) => Value::from(*c),
            DiffrnTerms::Ambient_temperature(b) => Value::from(*b),
            DiffrnTerms::Flux_density(t) => Value::from(*t),
            DiffrnTerms::Measured_fraction_theta_full(u) => Value::from(*u),
            DiffrnTerms::Measured_fraction_theta_max(w) => Value::from(*w),
            DiffrnTerms::Precession_semi_angle(x) => Value::from(*x),
            DiffrnTerms::Total_dose(y) => Value::from(*y),
            DiffrnTerms::Total_exposure_time(z) => Value::from(*z),
        }
    }
}

impl Display for DiffrnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `diffrn` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnLoopItem {
    /// The `_diffrn_ambient_pressure_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The mean hydrostatic pressure in kilopascals above which (*_gt) or
    /// below which (*_lt) the intensities were measured. These items allow
    /// for a pressure range to be given. _diffrn_ambient_pressure should
    /// always be used in preference to these items whenever possible.
    /// Units: kPa (kilopascal).
    /// range constraint recorded in the dictionary _enumeration
    Ambient_pressure_gt(Vec<AmbientPressureGt>),
    /// The `_diffrn_ambient_pressure_lt` item. The variant holds the column values, one per loop row.
    ///
    /// The mean hydrostatic pressure in kilopascals above which (*_gt) or
    /// below which (*_lt) the intensities were measured. These items allow
    /// for a pressure range to be given. _diffrn_ambient_pressure should
    /// always be used in preference to these items whenever possible.
    /// Units: kPa (kilopascal).
    /// range constraint recorded in the dictionary _enumeration
    Ambient_pressure_lt(Vec<AmbientPressureLt>),
    /// The `_diffrn_ambient_temperature_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The mean temperature in kelvins above which (*_gt) or below which
    /// (*_lt) the intensities were measured. These items allow a range of
    /// temperatures to be given. _diffrn_ambient_temperature should always
    /// be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temperature_gt(Vec<AmbientTemperatureGt>),
    /// The `_diffrn_ambient_temperature_lt` item. The variant holds the column values, one per loop row.
    ///
    /// The mean temperature in kelvins above which (*_gt) or below which
    /// (*_lt) the intensities were measured. These items allow a range of
    /// temperatures to be given. _diffrn_ambient_temperature should always
    /// be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    /// range constraint recorded in the dictionary _enumeration
    Ambient_temperature_lt(Vec<AmbientTemperatureLt>),
}

impl CifTerm for DiffrnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnLoopItem::Ambient_pressure_gt(_) => "ambient_pressure_gt",
            DiffrnLoopItem::Ambient_pressure_lt(_) => "ambient_pressure_lt",
            DiffrnLoopItem::Ambient_temperature_gt(_) => "ambient_temperature_gt",
            DiffrnLoopItem::Ambient_temperature_lt(_) => "ambient_temperature_lt",
        };
        Tag::new(format!("diffrn_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnLoopItem::Ambient_pressure_gt(v) => v.iter().cloned().map(Value::from).collect(),
            DiffrnLoopItem::Ambient_pressure_lt(m) => m.iter().cloned().map(Value::from).collect(),
            DiffrnLoopItem::Ambient_temperature_gt(c) => {
                c.iter().cloned().map(Value::from).collect()
            }
            DiffrnLoopItem::Ambient_temperature_lt(b) => {
                b.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
