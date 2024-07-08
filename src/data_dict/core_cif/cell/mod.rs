use chemrust_core::data::lattice::UnitCellParameters;

mod angle;

use crate::{data_dict::CifTerm, grammar::Tag};

pub use self::angle::CellAngle;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CellItem {
    Angle_alpha(CellAngle),
    Angle_beta(CellAngle),
    Angle_gamma(CellAngle),
    Formula_units_Z(usize),
    Length_a(f64),
    Length_b(f64),
    Length_c(f64),
    Measurement_pressure,
    Measurement_radiation,
    Measurement_reflns_used,
    Measurement_temperature,
    Measurement_theta_max,
    Measurement_theta_min,
    Measurement_wavelength,
    Reciprocal_angle_alpha(CellAngle),
    Reciprocal_angle_beta(CellAngle),
    Reciprocal_angle_gamma(CellAngle),
    Reciprocal_length_a(f64),
    Reciprocal_length_b(f64),
    Reciprocal_length_c(f64),
    Special_details,
    Volume(f64),
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum MeasurementReflnLoopItem {
    Measurement_refln_index_h,
    Measurement_refln_index_k,
    Measurement_refln_index_l,
    Measurement_refln_theta,
}
