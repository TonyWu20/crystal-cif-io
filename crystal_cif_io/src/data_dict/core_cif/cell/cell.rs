//! The `cell` category of the typed document layer (roadmap D12).
//!
//! The `cell` category struct. One `Option` field per registry item. The
//! angles keep the clamped `CellAngle` newtype; the matrices keep the
//! `Matrix` payload of the dictionary shape.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::cell::CellAngle;
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::data_dict::matrix;
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, UnsignedInteger, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `cell` category struct. One field per registry item.
pub struct Cell {
    /// The `_cell_angle_alpha` item. The field holds the column values, one per loop row.
    ///
    /// Unit-cell angles of the reported structure in degrees. The values of
    /// _refln_index_h, *_k, *_l must correspond to the cell defined by
    /// these values and _cell_length_a, *_b and *_c. The values of
    /// _diffrn_refln_index_h, *_k, *_l may not correspond to these values
    /// if a cell transformation took place following the measurement of the
    /// diffraction intensities. See also _diffrn_reflns_transf_matrix_.
    /// Units: deg (degree of arc).
    pub angle_alpha: Option<CellAngle>,
    /// The `_cell_angle_beta` item. The field holds the column values, one per loop row.
    ///
    /// Unit-cell angles of the reported structure in degrees. The values of
    /// _refln_index_h, *_k, *_l must correspond to the cell defined by
    /// these values and _cell_length_a, *_b and *_c. The values of
    /// _diffrn_refln_index_h, *_k, *_l may not correspond to these values
    /// if a cell transformation took place following the measurement of the
    /// diffraction intensities. See also _diffrn_reflns_transf_matrix_.
    /// Units: deg (degree of arc).
    pub angle_beta: Option<CellAngle>,
    /// The `_cell_angle_gamma` item. The field holds the column values, one per loop row.
    ///
    /// Unit-cell angles of the reported structure in degrees. The values of
    /// _refln_index_h, *_k, *_l must correspond to the cell defined by
    /// these values and _cell_length_a, *_b and *_c. The values of
    /// _diffrn_refln_index_h, *_k, *_l may not correspond to these values
    /// if a cell transformation took place following the measurement of the
    /// diffraction intensities. See also _diffrn_reflns_transf_matrix_.
    /// Units: deg (degree of arc).
    pub angle_gamma: Option<CellAngle>,
    /// The `_cell_atomic_mass` item. The field holds the single value.
    pub atomic_mass: Option<Numeric>,
    /// The `_cell_convert_Uij_to_betaij` item. The field holds the single value.
    pub convert_uij_to_betaij: Option<Matrix>,
    /// The `_cell_convert_Uiso_to_Uij` item. The field holds the single value.
    pub convert_uiso_to_uij: Option<Matrix>,
    /// The `_cell_formula_units_Z` item. The field holds the single value.
    ///
    /// The number of the formula units in the unit cell as specified by
    /// _chemical_formula_structural, _chemical_formula_moiety or
    /// _chemical_formula_sum.
    pub formula_units_z: Option<UnsignedInteger>,
    /// The `_cell_formula_units_Z_details` item. The field holds the single value.
    pub formula_units_z_details: Option<CharString>,
    /// The `_cell_formula_units_Z_prime` item. The field holds the single value.
    pub formula_units_z_prime: Option<Numeric>,
    /// The `_cell_formula_units_Z_prime_details` item. The field holds the single value.
    pub formula_units_z_prime_details: Option<CharString>,
    /// The `_cell_length_a` item. The field holds the column values, one per loop row.
    ///
    /// Unit-cell lengths in angstroms corresponding to the structure
    /// reported. The values of _refln_index_h, *_k, *_l must correspond to
    /// the cell defined by these values and _cell_angle_ values. The values
    /// of _diffrn_refln_index_h, *_k, *_l may not correspond to these
    /// values if a cell transformation took place following the measurement
    /// of the diffraction intensities. See also
    /// _diffrn_reflns_transf_matrix_.
    /// Units: A (angstrom).
    pub length_a: Option<Numeric>,
    /// The `_cell_length_b` item. The field holds the column values, one per loop row.
    ///
    /// Unit-cell lengths in angstroms corresponding to the structure
    /// reported. The values of _refln_index_h, *_k, *_l must correspond to
    /// the cell defined by these values and _cell_angle_ values. The values
    /// of _diffrn_refln_index_h, *_k, *_l may not correspond to these
    /// values if a cell transformation took place following the measurement
    /// of the diffraction intensities. See also
    /// _diffrn_reflns_transf_matrix_.
    /// Units: A (angstrom).
    pub length_b: Option<Numeric>,
    /// The `_cell_length_c` item. The field holds the column values, one per loop row.
    ///
    /// Unit-cell lengths in angstroms corresponding to the structure
    /// reported. The values of _refln_index_h, *_k, *_l must correspond to
    /// the cell defined by these values and _cell_angle_ values. The values
    /// of _diffrn_refln_index_h, *_k, *_l may not correspond to these
    /// values if a cell transformation took place following the measurement
    /// of the diffraction intensities. See also
    /// _diffrn_reflns_transf_matrix_.
    /// Units: A (angstrom).
    pub length_c: Option<Numeric>,
    /// The `_cell_measurement_pressure` item. The field holds the single value.
    ///
    /// The pressure in kilopascals at which the unit-cell parameters were
    /// measured (not the pressure at which the sample was synthesized).
    /// Units: kPa (kilopascal).
    pub measurement_pressure: Option<Numeric>,
    /// The `_cell_measurement_radiation` item. The field holds the single value.
    ///
    /// Description of the radiation used to measure the unit-cell data. See
    /// also _cell_measurement_wavelength.
    pub measurement_radiation: Option<CharString>,
    /// The `_cell_measurement_reflns_used` item. The field holds the single value.
    ///
    /// The total number of reflections used to determine the unit cell.
    /// These reflections may be specified as _cell_measurement_refln_ data
    /// items.
    pub measurement_reflns_used: Option<Numeric>,
    /// The `_cell_measurement_temperature` item. The field holds the single value.
    ///
    /// The temperature in kelvins at which the unit-cell parameters were
    /// measured (not the temperature of synthesis).
    /// Units: K (kelvin).
    pub measurement_temperature: Option<Numeric>,
    /// The `_cell_measurement_theta_max` item. The field holds the column values, one per loop row.
    ///
    /// The maximum and minimum theta angles of reflections used to measure
    /// the unit cell in degrees.
    /// Units: deg (degree of arc).
    pub measurement_theta_max: Option<Numeric>,
    /// The `_cell_measurement_theta_min` item. The field holds the column values, one per loop row.
    ///
    /// The maximum and minimum theta angles of reflections used to measure
    /// the unit cell in degrees.
    /// Units: deg (degree of arc).
    pub measurement_theta_min: Option<Numeric>,
    /// The `_cell_measurement_wavelength` item. The field holds the single value.
    ///
    /// The wavelength in angstroms of the radiation used to measure the
    /// unit cell. If this is not specified, the wavelength is assumed to be
    /// the same as that given in _diffrn_radiation_wavelength.
    /// Units: A (angstrom).
    pub measurement_wavelength: Option<Numeric>,
    /// The `_cell_metric_tensor` item. The field holds the single value.
    pub metric_tensor: Option<Matrix>,
    /// The `_cell_orthogonal_matrix` item. The field holds the single value.
    pub orthogonal_matrix: Option<Matrix>,
    /// The `_cell_reciprocal_angle_alpha` item. The field holds the column values, one per loop row.
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
    pub reciprocal_angle_alpha: Option<CellAngle>,
    /// The `_cell_reciprocal_angle_beta` item. The field holds the column values, one per loop row.
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
    pub reciprocal_angle_beta: Option<CellAngle>,
    /// The `_cell_reciprocal_angle_gamma` item. The field holds the column values, one per loop row.
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
    pub reciprocal_angle_gamma: Option<CellAngle>,
    /// The `_cell_reciprocal_length_a` item. The field holds the column values, one per loop row.
    ///
    /// The reciprocal-cell lengths in inverse angstroms. These are related
    /// to the real cell by: recip-a = b*c*sin(alpha)/V recip-b =
    /// c*a*sin(beta)/V recip-c = a*b*sin(gamma)/V where V is the cell
    /// volume. Ref: Buerger, M. J. (1942). X-ray Crystallography, p. 360.
    /// New York: John Wiley & Sons Inc.
    /// Units: A^-1^ (reciprocal angstrom).
    pub reciprocal_length_a: Option<Numeric>,
    /// The `_cell_reciprocal_length_b` item. The field holds the column values, one per loop row.
    ///
    /// The reciprocal-cell lengths in inverse angstroms. These are related
    /// to the real cell by: recip-a = b*c*sin(alpha)/V recip-b =
    /// c*a*sin(beta)/V recip-c = a*b*sin(gamma)/V where V is the cell
    /// volume. Ref: Buerger, M. J. (1942). X-ray Crystallography, p. 360.
    /// New York: John Wiley & Sons Inc.
    /// Units: A^-1^ (reciprocal angstrom).
    pub reciprocal_length_b: Option<Numeric>,
    /// The `_cell_reciprocal_length_c` item. The field holds the column values, one per loop row.
    ///
    /// The reciprocal-cell lengths in inverse angstroms. These are related
    /// to the real cell by: recip-a = b*c*sin(alpha)/V recip-b =
    /// c*a*sin(beta)/V recip-c = a*b*sin(gamma)/V where V is the cell
    /// volume. Ref: Buerger, M. J. (1942). X-ray Crystallography, p. 360.
    /// New York: John Wiley & Sons Inc.
    /// Units: A^-1^ (reciprocal angstrom).
    pub reciprocal_length_c: Option<Numeric>,
    /// The `_cell_reciprocal_metric_tensor` item. The field holds the single value.
    pub reciprocal_metric_tensor: Option<Matrix>,
    /// The `_cell_reciprocal_orthogonal_matrix` item. The field holds the single value.
    pub reciprocal_orthogonal_matrix: Option<Matrix>,
    /// The `_cell_reciprocal_vector_a` item. The field holds the single value.
    pub reciprocal_vector_a: Option<Matrix>,
    /// The `_cell_reciprocal_vector_b` item. The field holds the single value.
    pub reciprocal_vector_b: Option<Matrix>,
    /// The `_cell_reciprocal_vector_c` item. The field holds the single value.
    pub reciprocal_vector_c: Option<Matrix>,
    /// The `_cell_special_details` item. The field holds the single value.
    ///
    /// A description of special aspects of the cell choice, noting possible
    /// alternative settings.
    pub special_details: Option<CharString>,
    /// The `_cell_vector_a` item. The field holds the single value.
    pub vector_a: Option<Matrix>,
    /// The `_cell_vector_b` item. The field holds the single value.
    pub vector_b: Option<Matrix>,
    /// The `_cell_vector_c` item. The field holds the single value.
    pub vector_c: Option<Matrix>,
    /// The `_cell_volume` item. The field holds the single value.
    ///
    /// Cell volume V in angstroms cubed. V = a b c [1 - cos^2^(alpha) -
    /// cos^2^(beta) - cos^2^(gamma) + 2 cos(alpha) cos(beta) cos(gamma) ]
    /// ^1/2^ a = _cell_length_a b = _cell_length_b c = _cell_length_c alpha
    /// = _cell_angle_alpha beta = _cell_angle_beta gamma =
    /// _cell_angle_gamma
    /// Units: A^3^ (angstrom cubed).
    pub volume: Option<Numeric>,
}

impl CifCategory for Cell {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.angle_alpha {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_angle_alpha".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.angle_beta {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_angle_beta".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.angle_gamma {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_angle_gamma".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.atomic_mass {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_atomic_mass".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.convert_uij_to_betaij {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_convert_uij_to_betaij".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.convert_uiso_to_uij {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_convert_uiso_to_uij".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.formula_units_z {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_formula_units_z".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.formula_units_z_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_formula_units_z_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.formula_units_z_prime {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_formula_units_z_prime".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.formula_units_z_prime_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_formula_units_z_prime_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.length_a {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_length_a".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.length_b {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_length_b".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.length_c {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_length_c".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.measurement_pressure {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_measurement_pressure".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.measurement_radiation {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_measurement_radiation".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.measurement_reflns_used {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_measurement_reflns_used".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.measurement_temperature {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_measurement_temperature".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.measurement_theta_max {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_measurement_theta_max".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.measurement_theta_min {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_measurement_theta_min".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.measurement_wavelength {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_measurement_wavelength".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.metric_tensor {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_metric_tensor".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.orthogonal_matrix {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_orthogonal_matrix".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_angle_alpha {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_angle_alpha".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_angle_beta {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_angle_beta".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_angle_gamma {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_angle_gamma".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_length_a {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_length_a".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_length_b {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_length_b".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_length_c {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_length_c".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_metric_tensor {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_metric_tensor".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_orthogonal_matrix {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_orthogonal_matrix".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_vector_a {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_vector_a".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_vector_b {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_vector_b".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.reciprocal_vector_c {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_reciprocal_vector_c".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_special_details".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.vector_a {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_vector_a".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.vector_b {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_vector_b".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.vector_c {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_vector_c".to_string()),
                    matrix::to_value(v),
                )),
            )));
        }
        if let Some(v) = &self.volume {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("cell_volume".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        out
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "cell_angle_alpha" => {
                        match convert::read_cell_angle("cell_angle_alpha", value) {
                            Ok(v) => self.angle_alpha = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_angle_beta" => match convert::read_cell_angle("cell_angle_beta", value) {
                        Ok(v) => self.angle_beta = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_angle_gamma" => {
                        match convert::read_cell_angle("cell_angle_gamma", value) {
                            Ok(v) => self.angle_gamma = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_atomic_mass" => match convert::read_numeric("cell_atomic_mass", value) {
                        Ok(v) => self.atomic_mass = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_convert_uij_to_betaij" => {
                        match convert::read_matrix("cell_convert_uij_to_betaij", value, (3, 3)) {
                            Ok(v) => self.convert_uij_to_betaij = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_convert_uiso_to_uij" => {
                        match convert::read_matrix("cell_convert_uiso_to_uij", value, (3, 3)) {
                            Ok(v) => self.convert_uiso_to_uij = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_formula_units_z" => {
                        match convert::read_unsigned("cell_formula_units_z", value) {
                            Ok(v) => self.formula_units_z = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_formula_units_z_details" => {
                        match convert::read_char("cell_formula_units_z_details", value) {
                            Ok(v) => self.formula_units_z_details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_formula_units_z_prime" => {
                        match convert::read_numeric("cell_formula_units_z_prime", value) {
                            Ok(v) => self.formula_units_z_prime = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_formula_units_z_prime_details" => {
                        match convert::read_char("cell_formula_units_z_prime_details", value) {
                            Ok(v) => self.formula_units_z_prime_details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_length_a" => match convert::read_numeric("cell_length_a", value) {
                        Ok(v) => self.length_a = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_length_b" => match convert::read_numeric("cell_length_b", value) {
                        Ok(v) => self.length_b = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_length_c" => match convert::read_numeric("cell_length_c", value) {
                        Ok(v) => self.length_c = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_measurement_pressure" => {
                        match convert::read_numeric("cell_measurement_pressure", value) {
                            Ok(v) => self.measurement_pressure = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_measurement_radiation" => {
                        match convert::read_char("cell_measurement_radiation", value) {
                            Ok(v) => self.measurement_radiation = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_measurement_reflns_used" => {
                        match convert::read_numeric("cell_measurement_reflns_used", value) {
                            Ok(v) => self.measurement_reflns_used = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_measurement_temperature" => {
                        match convert::read_numeric("cell_measurement_temperature", value) {
                            Ok(v) => self.measurement_temperature = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_measurement_theta_max" => {
                        match convert::read_numeric("cell_measurement_theta_max", value) {
                            Ok(v) => self.measurement_theta_max = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_measurement_theta_min" => {
                        match convert::read_numeric("cell_measurement_theta_min", value) {
                            Ok(v) => self.measurement_theta_min = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_measurement_wavelength" => {
                        match convert::read_numeric("cell_measurement_wavelength", value) {
                            Ok(v) => self.measurement_wavelength = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_metric_tensor" => {
                        match convert::read_matrix("cell_metric_tensor", value, (3, 3)) {
                            Ok(v) => self.metric_tensor = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_orthogonal_matrix" => {
                        match convert::read_matrix("cell_orthogonal_matrix", value, (3, 3)) {
                            Ok(v) => self.orthogonal_matrix = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_angle_alpha" => {
                        match convert::read_cell_angle("cell_reciprocal_angle_alpha", value) {
                            Ok(v) => self.reciprocal_angle_alpha = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_angle_beta" => {
                        match convert::read_cell_angle("cell_reciprocal_angle_beta", value) {
                            Ok(v) => self.reciprocal_angle_beta = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_angle_gamma" => {
                        match convert::read_cell_angle("cell_reciprocal_angle_gamma", value) {
                            Ok(v) => self.reciprocal_angle_gamma = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_length_a" => {
                        match convert::read_numeric("cell_reciprocal_length_a", value) {
                            Ok(v) => self.reciprocal_length_a = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_length_b" => {
                        match convert::read_numeric("cell_reciprocal_length_b", value) {
                            Ok(v) => self.reciprocal_length_b = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_length_c" => {
                        match convert::read_numeric("cell_reciprocal_length_c", value) {
                            Ok(v) => self.reciprocal_length_c = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_metric_tensor" => {
                        match convert::read_matrix("cell_reciprocal_metric_tensor", value, (3, 3)) {
                            Ok(v) => self.reciprocal_metric_tensor = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_orthogonal_matrix" => {
                        match convert::read_matrix(
                            "cell_reciprocal_orthogonal_matrix",
                            value,
                            (3, 3),
                        ) {
                            Ok(v) => self.reciprocal_orthogonal_matrix = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_vector_a" => {
                        match convert::read_matrix("cell_reciprocal_vector_a", value, (3, 1)) {
                            Ok(v) => self.reciprocal_vector_a = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_vector_b" => {
                        match convert::read_matrix("cell_reciprocal_vector_b", value, (3, 1)) {
                            Ok(v) => self.reciprocal_vector_b = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_reciprocal_vector_c" => {
                        match convert::read_matrix("cell_reciprocal_vector_c", value, (3, 1)) {
                            Ok(v) => self.reciprocal_vector_c = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_special_details" => {
                        match convert::read_char("cell_special_details", value) {
                            Ok(v) => self.special_details = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "cell_vector_a" => match convert::read_matrix("cell_vector_a", value, (3, 1)) {
                        Ok(v) => self.vector_a = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_vector_b" => match convert::read_matrix("cell_vector_b", value, (3, 1)) {
                        Ok(v) => self.vector_b = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_vector_c" => match convert::read_matrix("cell_vector_c", value, (3, 1)) {
                        Ok(v) => self.vector_c = v,
                        Err(e) => errors.push(e),
                    },
                    "cell_volume" => match convert::read_numeric("cell_volume", value) {
                        Ok(v) => self.volume = v,
                        Err(e) => errors.push(e),
                    },
                    _ => residue.push(m.clone()),
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}
