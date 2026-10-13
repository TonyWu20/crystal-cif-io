mod angle;

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, UnsignedInteger, Value};

pub use self::angle::CellAngle;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum CellTerms {
    Angle_alpha(CellAngle),
    Angle_beta(CellAngle),
    Angle_gamma(CellAngle),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0.0:`); the plain numeric form is kept.
    Atomic_mass(Numeric),
    Convert_uij_to_betaij(Matrix),
    Convert_uiso_to_uij(Matrix),
    Formula_units_Z(UnsignedInteger),
    Formula_units_z_details(CharString),
    Formula_units_z_prime(Numeric),
    Formula_units_z_prime_details(CharString),
    Length_a(Numeric),
    Length_b(Numeric),
    Length_c(Numeric),
    Measurement_pressure(Numeric),
    Measurement_radiation(CharString),
    Measurement_refln_hkl(Matrix),
    Measurement_reflns_used(Numeric),
    Measurement_temperature(Numeric),
    Measurement_theta_max(Numeric),
    Measurement_theta_min(Numeric),
    Measurement_wavelength(Numeric),
    Metric_tensor(Matrix),
    Orthogonal_matrix(Matrix),
    Reciprocal_angle_alpha(CellAngle),
    Reciprocal_angle_beta(CellAngle),
    Reciprocal_angle_gamma(CellAngle),
    Reciprocal_length_a(Numeric),
    Reciprocal_length_b(Numeric),
    Reciprocal_length_c(Numeric),
    Reciprocal_metric_tensor(Matrix),
    Reciprocal_orthogonal_matrix(Matrix),
    Reciprocal_vector_a(Matrix),
    Reciprocal_vector_b(Matrix),
    Reciprocal_vector_c(Matrix),
    Special_details(CharString),
    Vector_a(Matrix),
    Vector_b(Matrix),
    Vector_c(Matrix),
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
            CellTerms::Atomic_mass(_) => "atomic_mass",
            CellTerms::Convert_uij_to_betaij(_) => "convert_uij_to_betaij",
            CellTerms::Convert_uiso_to_uij(_) => "convert_uiso_to_uij",
            CellTerms::Formula_units_Z(_) => "formula_units_z",
            CellTerms::Formula_units_z_details(_) => "formula_units_z_details",
            CellTerms::Formula_units_z_prime(_) => "formula_units_z_prime",
            CellTerms::Formula_units_z_prime_details(_) => "formula_units_z_prime_details",
            CellTerms::Length_a(_) => "length_a",
            CellTerms::Length_b(_) => "length_b",
            CellTerms::Length_c(_) => "length_c",
            CellTerms::Measurement_pressure(_) => "measurement_pressure",
            CellTerms::Measurement_radiation(_) => "measurement_radiation",
            CellTerms::Measurement_refln_hkl(_) => "measurement_refln_hkl",
            CellTerms::Measurement_reflns_used(_) => "measurement_reflns_used",
            CellTerms::Measurement_temperature(_) => "measurement_temperature",
            CellTerms::Measurement_theta_max(_) => "measurement_theta_max",
            CellTerms::Measurement_theta_min(_) => "measurement_theta_min",
            CellTerms::Measurement_wavelength(_) => "measurement_wavelength",
            CellTerms::Metric_tensor(_) => "metric_tensor",
            CellTerms::Orthogonal_matrix(_) => "orthogonal_matrix",
            CellTerms::Reciprocal_angle_alpha(_) => "reciprocal_angle_alpha",
            CellTerms::Reciprocal_angle_beta(_) => "reciprocal_angle_beta",
            CellTerms::Reciprocal_angle_gamma(_) => "reciprocal_angle_gamma",
            CellTerms::Reciprocal_length_a(_) => "reciprocal_length_a",
            CellTerms::Reciprocal_length_b(_) => "reciprocal_length_b",
            CellTerms::Reciprocal_length_c(_) => "reciprocal_length_c",
            CellTerms::Reciprocal_metric_tensor(_) => "reciprocal_metric_tensor",
            CellTerms::Reciprocal_orthogonal_matrix(_) => "reciprocal_orthogonal_matrix",
            CellTerms::Reciprocal_vector_a(_) => "reciprocal_vector_a",
            CellTerms::Reciprocal_vector_b(_) => "reciprocal_vector_b",
            CellTerms::Reciprocal_vector_c(_) => "reciprocal_vector_c",
            CellTerms::Special_details(_) => "special_details",
            CellTerms::Vector_a(_) => "vector_a",
            CellTerms::Vector_b(_) => "vector_b",
            CellTerms::Vector_c(_) => "vector_c",
            CellTerms::Volume(_) => "volume",
        };
        Tag::new(format!("cell_{name}"))
    }
}

impl SingleValueTerm for CellTerms {
    fn value(&self) -> crystal_cif_fmt::grammar::Value {
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
            CellTerms::Atomic_mass(n) | CellTerms::Formula_units_z_prime(n) => Value::Numeric(*n),
            CellTerms::Convert_uij_to_betaij(m)
            | CellTerms::Convert_uiso_to_uij(m)
            | CellTerms::Measurement_refln_hkl(m)
            | CellTerms::Metric_tensor(m)
            | CellTerms::Orthogonal_matrix(m)
            | CellTerms::Reciprocal_metric_tensor(m)
            | CellTerms::Reciprocal_orthogonal_matrix(m)
            | CellTerms::Reciprocal_vector_a(m)
            | CellTerms::Reciprocal_vector_b(m)
            | CellTerms::Reciprocal_vector_c(m)
            | CellTerms::Vector_a(m)
            | CellTerms::Vector_b(m)
            | CellTerms::Vector_c(m) => matrix::to_value(m),
            CellTerms::Measurement_pressure(v)
            | CellTerms::Measurement_reflns_used(v)
            | CellTerms::Measurement_temperature(v)
            | CellTerms::Measurement_theta_max(v)
            | CellTerms::Measurement_theta_min(v)
            | CellTerms::Measurement_wavelength(v) => Value::Numeric(*v),
            CellTerms::Measurement_radiation(s)
            | CellTerms::Special_details(s)
            | CellTerms::Formula_units_z_details(s)
            | CellTerms::Formula_units_z_prime_details(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for CellTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
