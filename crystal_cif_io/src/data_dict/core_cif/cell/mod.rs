mod angle;
mod cell;

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, UnsignedInteger, Value};

pub use self::angle::CellAngle;
pub use cell::Cell;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `cell` category: one variant per

/// registry single-value item, in item order.
pub enum CellTerms {
    /// The `_cell_angle_alpha` item. The variant holds the column values, one per loop row.
    ///
    /// Unit-cell angles of the reported structure in degrees. The values of
    /// _refln_index_h, *_k, *_l must correspond to the cell defined by
    /// these values and _cell_length_a, *_b and *_c. The values of
    /// _diffrn_refln_index_h, *_k, *_l may not correspond to these values
    /// if a cell transformation took place following the measurement of the
    /// diffraction intensities. See also _diffrn_reflns_transf_matrix_.
    /// Units: deg (degree of arc).
    Angle_alpha(CellAngle),
    /// The `_cell_angle_beta` item. The variant holds the column values, one per loop row.
    ///
    /// Unit-cell angles of the reported structure in degrees. The values of
    /// _refln_index_h, *_k, *_l must correspond to the cell defined by
    /// these values and _cell_length_a, *_b and *_c. The values of
    /// _diffrn_refln_index_h, *_k, *_l may not correspond to these values
    /// if a cell transformation took place following the measurement of the
    /// diffraction intensities. See also _diffrn_reflns_transf_matrix_.
    /// Units: deg (degree of arc).
    Angle_beta(CellAngle),
    /// The `_cell_angle_gamma` item. The variant holds the column values, one per loop row.
    ///
    /// Unit-cell angles of the reported structure in degrees. The values of
    /// _refln_index_h, *_k, *_l must correspond to the cell defined by
    /// these values and _cell_length_a, *_b and *_c. The values of
    /// _diffrn_refln_index_h, *_k, *_l may not correspond to these values
    /// if a cell transformation took place following the measurement of the
    /// diffraction intensities. See also _diffrn_reflns_transf_matrix_.
    /// Units: deg (degree of arc).
    Angle_gamma(CellAngle),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0.0:`); the plain numeric form is kept.
    /// The `_cell_atomic_mass` item. The variant holds the column values, one per loop row.
    Atomic_mass(Numeric),
    /// The `_cell_convert_Uij_to_betaij` item. The variant holds the column values, one per loop row.
    Convert_uij_to_betaij(Matrix),
    /// The `_cell_convert_Uiso_to_Uij` item. The variant holds the column values, one per loop row.
    Convert_uiso_to_uij(Matrix),
    /// The `_cell_formula_units_Z` item. The variant holds the column values, one per loop row.
    ///
    /// The number of the formula units in the unit cell as specified by
    /// _chemical_formula_structural, _chemical_formula_moiety or
    /// _chemical_formula_sum.
    Formula_units_Z(UnsignedInteger),
    /// The `_cell_formula_units_Z_details` item. The variant holds the column values, one per loop row.
    Formula_units_z_details(CharString),
    /// The `_cell_formula_units_Z_prime` item. The variant holds the column values, one per loop row.
    Formula_units_z_prime(Numeric),
    /// The `_cell_formula_units_Z_prime_details` item. The variant holds the column values, one per loop row.
    Formula_units_z_prime_details(CharString),
    /// The `_cell_length_a` item. The variant holds the column values, one per loop row.
    ///
    /// Unit-cell lengths in angstroms corresponding to the structure
    /// reported. The values of _refln_index_h, *_k, *_l must correspond to
    /// the cell defined by these values and _cell_angle_ values. The values
    /// of _diffrn_refln_index_h, *_k, *_l may not correspond to these
    /// values if a cell transformation took place following the measurement
    /// of the diffraction intensities. See also
    /// _diffrn_reflns_transf_matrix_.
    /// Units: A (angstrom).
    Length_a(Numeric),
    /// The `_cell_length_b` item. The variant holds the column values, one per loop row.
    ///
    /// Unit-cell lengths in angstroms corresponding to the structure
    /// reported. The values of _refln_index_h, *_k, *_l must correspond to
    /// the cell defined by these values and _cell_angle_ values. The values
    /// of _diffrn_refln_index_h, *_k, *_l may not correspond to these
    /// values if a cell transformation took place following the measurement
    /// of the diffraction intensities. See also
    /// _diffrn_reflns_transf_matrix_.
    /// Units: A (angstrom).
    Length_b(Numeric),
    /// The `_cell_length_c` item. The variant holds the column values, one per loop row.
    ///
    /// Unit-cell lengths in angstroms corresponding to the structure
    /// reported. The values of _refln_index_h, *_k, *_l must correspond to
    /// the cell defined by these values and _cell_angle_ values. The values
    /// of _diffrn_refln_index_h, *_k, *_l may not correspond to these
    /// values if a cell transformation took place following the measurement
    /// of the diffraction intensities. See also
    /// _diffrn_reflns_transf_matrix_.
    /// Units: A (angstrom).
    Length_c(Numeric),
    /// The `_cell_measurement_pressure` item. The variant holds the column values, one per loop row.
    ///
    /// The pressure in kilopascals at which the unit-cell parameters were
    /// measured (not the pressure at which the sample was synthesized).
    /// Units: kPa (kilopascal).
    Measurement_pressure(Numeric),
    /// The `_cell_measurement_radiation` item. The variant holds the column values, one per loop row.
    ///
    /// Description of the radiation used to measure the unit-cell data. See
    /// also _cell_measurement_wavelength.
    Measurement_radiation(CharString),
    /// The `_cell_measurement_refln_hkl` item. The variant holds the column values, one per loop row.
    Measurement_refln_hkl(Matrix),
    /// The `_cell_measurement_reflns_used` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of reflections used to determine the unit cell.
    /// These reflections may be specified as _cell_measurement_refln_ data
    /// items.
    Measurement_reflns_used(Numeric),
    /// The `_cell_measurement_temperature` item. The variant holds the column values, one per loop row.
    ///
    /// The temperature in kelvins at which the unit-cell parameters were
    /// measured (not the temperature of synthesis).
    /// Units: K (kelvin).
    Measurement_temperature(Numeric),
    /// The `_cell_measurement_theta_max` item. The variant holds the column values, one per loop row.
    ///
    /// The maximum and minimum theta angles of reflections used to measure
    /// the unit cell in degrees.
    /// Units: deg (degree of arc).
    Measurement_theta_max(Numeric),
    /// The `_cell_measurement_theta_min` item. The variant holds the column values, one per loop row.
    ///
    /// The maximum and minimum theta angles of reflections used to measure
    /// the unit cell in degrees.
    /// Units: deg (degree of arc).
    Measurement_theta_min(Numeric),
    /// The `_cell_measurement_wavelength` item. The variant holds the column values, one per loop row.
    ///
    /// The wavelength in angstroms of the radiation used to measure the
    /// unit cell. If this is not specified, the wavelength is assumed to be
    /// the same as that given in _diffrn_radiation_wavelength.
    /// Units: A (angstrom).
    Measurement_wavelength(Numeric),
    /// The `_cell_metric_tensor` item. The variant holds the column values, one per loop row.
    Metric_tensor(Matrix),
    /// The `_cell_orthogonal_matrix` item. The variant holds the column values, one per loop row.
    Orthogonal_matrix(Matrix),
    /// The `_cell_reciprocal_angle_alpha` item. The variant holds the column values, one per loop row.
    ///
    /// The angles defining the reciprocal cell in degrees. These are
    /// related to those in the real cell by: cos(recip-alpha) =
    /// [cos(beta)*cos(gamma) - cos(alpha)]/[sin(beta)*sin(gamma)]
    /// cos(recip-beta) = [cos(gamma)*cos(alpha) -
    /// cos(beta)]/[sin(gamma)*sin(alpha)] cos(recip-gamma) =
    /// [cos(alpha)*cos(beta) - cos(gamma)]/[sin(alpha)*sin(beta)] Ref:
    /// Buerger, M. J. (1942). X-ray Crystallography, p. 360. New York: John
    /// Wiley & Sons Inc.
    /// Units: deg (degree of arc).
    Reciprocal_angle_alpha(CellAngle),
    /// The `_cell_reciprocal_angle_beta` item. The variant holds the column values, one per loop row.
    ///
    /// The angles defining the reciprocal cell in degrees. These are
    /// related to those in the real cell by: cos(recip-alpha) =
    /// [cos(beta)*cos(gamma) - cos(alpha)]/[sin(beta)*sin(gamma)]
    /// cos(recip-beta) = [cos(gamma)*cos(alpha) -
    /// cos(beta)]/[sin(gamma)*sin(alpha)] cos(recip-gamma) =
    /// [cos(alpha)*cos(beta) - cos(gamma)]/[sin(alpha)*sin(beta)] Ref:
    /// Buerger, M. J. (1942). X-ray Crystallography, p. 360. New York: John
    /// Wiley & Sons Inc.
    /// Units: deg (degree of arc).
    Reciprocal_angle_beta(CellAngle),
    /// The `_cell_reciprocal_angle_gamma` item. The variant holds the column values, one per loop row.
    ///
    /// The angles defining the reciprocal cell in degrees. These are
    /// related to those in the real cell by: cos(recip-alpha) =
    /// [cos(beta)*cos(gamma) - cos(alpha)]/[sin(beta)*sin(gamma)]
    /// cos(recip-beta) = [cos(gamma)*cos(alpha) -
    /// cos(beta)]/[sin(gamma)*sin(alpha)] cos(recip-gamma) =
    /// [cos(alpha)*cos(beta) - cos(gamma)]/[sin(alpha)*sin(beta)] Ref:
    /// Buerger, M. J. (1942). X-ray Crystallography, p. 360. New York: John
    /// Wiley & Sons Inc.
    /// Units: deg (degree of arc).
    Reciprocal_angle_gamma(CellAngle),
    /// The `_cell_reciprocal_length_a` item. The variant holds the column values, one per loop row.
    ///
    /// The reciprocal-cell lengths in inverse angstroms. These are related
    /// to the real cell by: recip-a = b*c*sin(alpha)/V recip-b =
    /// c*a*sin(beta)/V recip-c = a*b*sin(gamma)/V where V is the cell
    /// volume. Ref: Buerger, M. J. (1942). X-ray Crystallography, p. 360.
    /// New York: John Wiley & Sons Inc.
    /// Units: A^-1^ (reciprocal angstrom).
    Reciprocal_length_a(Numeric),
    /// The `_cell_reciprocal_length_b` item. The variant holds the column values, one per loop row.
    ///
    /// The reciprocal-cell lengths in inverse angstroms. These are related
    /// to the real cell by: recip-a = b*c*sin(alpha)/V recip-b =
    /// c*a*sin(beta)/V recip-c = a*b*sin(gamma)/V where V is the cell
    /// volume. Ref: Buerger, M. J. (1942). X-ray Crystallography, p. 360.
    /// New York: John Wiley & Sons Inc.
    /// Units: A^-1^ (reciprocal angstrom).
    Reciprocal_length_b(Numeric),
    /// The `_cell_reciprocal_length_c` item. The variant holds the column values, one per loop row.
    ///
    /// The reciprocal-cell lengths in inverse angstroms. These are related
    /// to the real cell by: recip-a = b*c*sin(alpha)/V recip-b =
    /// c*a*sin(beta)/V recip-c = a*b*sin(gamma)/V where V is the cell
    /// volume. Ref: Buerger, M. J. (1942). X-ray Crystallography, p. 360.
    /// New York: John Wiley & Sons Inc.
    /// Units: A^-1^ (reciprocal angstrom).
    Reciprocal_length_c(Numeric),
    /// The `_cell_reciprocal_metric_tensor` item. The variant holds the column values, one per loop row.
    Reciprocal_metric_tensor(Matrix),
    /// The `_cell_reciprocal_orthogonal_matrix` item. The variant holds the column values, one per loop row.
    Reciprocal_orthogonal_matrix(Matrix),
    /// The `_cell_reciprocal_vector_a` item. The variant holds the column values, one per loop row.
    Reciprocal_vector_a(Matrix),
    /// The `_cell_reciprocal_vector_b` item. The variant holds the column values, one per loop row.
    Reciprocal_vector_b(Matrix),
    /// The `_cell_reciprocal_vector_c` item. The variant holds the column values, one per loop row.
    Reciprocal_vector_c(Matrix),
    /// The `_cell_special_details` item. The variant holds the column values, one per loop row.
    ///
    /// A description of special aspects of the cell choice, noting possible
    /// alternative settings.
    Special_details(CharString),
    /// The `_cell_vector_a` item. The variant holds the column values, one per loop row.
    Vector_a(Matrix),
    /// The `_cell_vector_b` item. The variant holds the column values, one per loop row.
    Vector_b(Matrix),
    /// The `_cell_vector_c` item. The variant holds the column values, one per loop row.
    Vector_c(Matrix),
    /// The `_cell_volume` item. The variant holds the column values, one per loop row.
    ///
    /// Cell volume V in angstroms cubed. V = a b c [1 - cos^2^(alpha) -
    /// cos^2^(beta) - cos^2^(gamma) + 2 cos(alpha) cos(beta) cos(gamma) ]
    /// ^1/2^ a = _cell_length_a b = _cell_length_b c = _cell_length_c alpha
    /// = _cell_angle_alpha beta = _cell_angle_beta gamma =
    /// _cell_angle_gamma
    /// Units: A^3^ (angstrom cubed).
    Volume(Numeric),
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `cell` category: one variant per

/// registry loop-value item, in item order.
pub enum MeasurementReflnLoopItem {
    /// The `_cell_measurement_refln_index_h` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of a reflection used for measurement of the unit
    /// cell.
    Measurement_refln_index_h(Vec<Numeric>),
    /// The `_cell_measurement_refln_index_k` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of a reflection used for measurement of the unit
    /// cell.
    Measurement_refln_index_k(Vec<Numeric>),
    /// The `_cell_measurement_refln_index_l` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of a reflection used for measurement of the unit
    /// cell.
    Measurement_refln_index_l(Vec<Numeric>),
    /// The `_cell_measurement_refln_theta` item. The variant holds the column values, one per loop row.
    ///
    /// Theta angle in degrees for the reflection used for measurement of
    /// the unit cell with the indices _cell_measurement_refln_index_.
    /// Units: deg (degree of arc).
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
