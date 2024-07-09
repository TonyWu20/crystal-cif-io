mod angle;
#[cfg(feature = "chemrust-core")]
pub mod chemrust_impl;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{Numeric, Tag, UnsignedInteger, Value},
};

pub use self::angle::CellAngle;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CellTerms {
    Angle_alpha(CellAngle),
    Angle_beta(CellAngle),
    Angle_gamma(CellAngle),
    Formula_units_Z(UnsignedInteger),
    Length_a(Numeric),
    Length_b(Numeric),
    Length_c(Numeric),
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
    Reciprocal_length_a(Numeric),
    Reciprocal_length_b(Numeric),
    Reciprocal_length_c(Numeric),
    Special_details,
    Volume(Numeric),
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum MeasurementReflnLoopItem {
    Measurement_refln_index_h,
    Measurement_refln_index_k,
    Measurement_refln_index_l,
    Measurement_refln_theta,
}

impl CifTerm for CellTerms {
    fn category_prefix(&self) -> String {
        "cell".to_string()
    }

    fn tag(&self) -> Tag {
        let name = match self {
            CellTerms::Angle_alpha(_) => "angle_alpha",
            CellTerms::Angle_beta(_) => "angle_beta",
            CellTerms::Angle_gamma(_) => "angle_gamma",
            CellTerms::Formula_units_Z(_) => "formula_units_z",
            CellTerms::Length_a(_) => "length_a",
            CellTerms::Length_b(_) => "length_b",
            CellTerms::Length_c(_) => "length_c",
            CellTerms::Measurement_pressure => "measurement_pressure",
            CellTerms::Measurement_radiation => "measurement_radiation",
            CellTerms::Measurement_reflns_used => "measurement_reflns_used",
            CellTerms::Measurement_temperature => "measurement_temperature",
            CellTerms::Measurement_theta_max => "measurement_theta_max",
            CellTerms::Measurement_theta_min => "measurement_theta_min",
            CellTerms::Measurement_wavelength => "measurement_wavelength",
            CellTerms::Reciprocal_angle_alpha(_) => "reciprocal_angle_alpha",
            CellTerms::Reciprocal_angle_beta(_) => "reciprocal_angle_beta",
            CellTerms::Reciprocal_angle_gamma(_) => "reciprocal_angle_gamma",
            CellTerms::Reciprocal_length_a(_) => "reciprocal_length_a",
            CellTerms::Reciprocal_length_b(_) => "reciprocal_length_b",
            CellTerms::Reciprocal_length_c(_) => "reciprocal_length_c",
            CellTerms::Special_details => "special_details",
            CellTerms::Volume(_) => "volume",
        };
        Tag::new(format!("cell_{name}"))
    }
}

impl SingleValueTerm for CellTerms {
    fn value(&self) -> crate::grammar::Value {
        match self {
            CellTerms::Angle_alpha(a)
            | CellTerms::Angle_beta(a)
            | CellTerms::Angle_gamma(a)
            | CellTerms::Reciprocal_angle_alpha(a)
            | CellTerms::Reciprocal_angle_beta(a)
            | CellTerms::Reciprocal_angle_gamma(a) => (*a).into(),

            CellTerms::Formula_units_Z(z) => (*z).into(),
            CellTerms::Length_a(numeric)
            | CellTerms::Length_b(numeric)
            | CellTerms::Length_c(numeric)
            | CellTerms::Reciprocal_length_a(numeric)
            | CellTerms::Reciprocal_length_b(numeric)
            | CellTerms::Reciprocal_length_c(numeric)
            | CellTerms::Volume(numeric) => Value::Numeric(*numeric),
            CellTerms::Measurement_pressure => todo!(),
            CellTerms::Measurement_radiation => todo!(),
            CellTerms::Measurement_reflns_used => todo!(),
            CellTerms::Measurement_temperature => todo!(),
            CellTerms::Measurement_theta_max => todo!(),
            CellTerms::Measurement_theta_min => todo!(),
            CellTerms::Measurement_wavelength => todo!(),
            CellTerms::Special_details => todo!(),
        }
    }
}
