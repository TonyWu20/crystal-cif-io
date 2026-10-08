mod angle;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Numeric, Tag, UnsignedInteger, Value},
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
    Measurement_pressure(Numeric),
    Measurement_radiation(CharString),
    Measurement_reflns_used(Numeric),
    Measurement_temperature(Numeric),
    Measurement_theta_max(Numeric),
    Measurement_theta_min(Numeric),
    Measurement_wavelength(Numeric),
    Reciprocal_angle_alpha(CellAngle),
    Reciprocal_angle_beta(CellAngle),
    Reciprocal_angle_gamma(CellAngle),
    Reciprocal_length_a(Numeric),
    Reciprocal_length_b(Numeric),
    Reciprocal_length_c(Numeric),
    Special_details(CharString),
    Volume(Numeric),
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum MeasurementReflnLoopItem {
    Measurement_refln_index_h(Vec<Numeric>),
    Measurement_refln_index_k(Vec<Numeric>),
    Measurement_refln_index_l(Vec<Numeric>),
    Measurement_refln_theta(Vec<Numeric>),
}

impl CifTerm for MeasurementReflnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            MeasurementReflnLoopItem::Measurement_refln_index_h(_) => "measurement_refln_index_h",
            MeasurementReflnLoopItem::Measurement_refln_index_k(_) => "measurement_refln_index_k",
            MeasurementReflnLoopItem::Measurement_refln_index_l(_) => "measurement_refln_index_l",
            MeasurementReflnLoopItem::Measurement_refln_theta(_) => "measurement_refln_theta",
        };
        Tag::new(format!("cell_{suffix}"))
    }
}

impl LoopValueTerm for MeasurementReflnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            MeasurementReflnLoopItem::Measurement_refln_index_h(v)
            | MeasurementReflnLoopItem::Measurement_refln_index_k(v)
            | MeasurementReflnLoopItem::Measurement_refln_index_l(v)
            | MeasurementReflnLoopItem::Measurement_refln_theta(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl CifTerm for CellTerms {
    fn tag(&self) -> Tag {
        let name = match self {
            CellTerms::Angle_alpha(_) => "angle_alpha",
            CellTerms::Angle_beta(_) => "angle_beta",
            CellTerms::Angle_gamma(_) => "angle_gamma",
            CellTerms::Formula_units_Z(_) => "formula_units_z",
            CellTerms::Length_a(_) => "length_a",
            CellTerms::Length_b(_) => "length_b",
            CellTerms::Length_c(_) => "length_c",
            CellTerms::Measurement_pressure(_) => "measurement_pressure",
            CellTerms::Measurement_radiation(_) => "measurement_radiation",
            CellTerms::Measurement_reflns_used(_) => "measurement_reflns_used",
            CellTerms::Measurement_temperature(_) => "measurement_temperature",
            CellTerms::Measurement_theta_max(_) => "measurement_theta_max",
            CellTerms::Measurement_theta_min(_) => "measurement_theta_min",
            CellTerms::Measurement_wavelength(_) => "measurement_wavelength",
            CellTerms::Reciprocal_angle_alpha(_) => "reciprocal_angle_alpha",
            CellTerms::Reciprocal_angle_beta(_) => "reciprocal_angle_beta",
            CellTerms::Reciprocal_angle_gamma(_) => "reciprocal_angle_gamma",
            CellTerms::Reciprocal_length_a(_) => "reciprocal_length_a",
            CellTerms::Reciprocal_length_b(_) => "reciprocal_length_b",
            CellTerms::Reciprocal_length_c(_) => "reciprocal_length_c",
            CellTerms::Special_details(_) => "special_details",
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
            CellTerms::Measurement_pressure(v)
            | CellTerms::Measurement_reflns_used(v)
            | CellTerms::Measurement_temperature(v)
            | CellTerms::Measurement_theta_max(v)
            | CellTerms::Measurement_theta_min(v)
            | CellTerms::Measurement_wavelength(v) => Value::Numeric(*v),
            CellTerms::Measurement_radiation(s) | CellTerms::Special_details(s) => {
                Value::CharString(s.clone())
            }
        }
    }
}

impl Display for CellTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
