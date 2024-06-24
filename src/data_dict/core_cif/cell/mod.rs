use chemrust_core::data::lattice::UnitCellParameters;

use crate::data_dict::{DataLabel, SingleValueSection};

mod angle;

pub use self::angle::CellAngle;

pub type CellDataSection = SingleValueSection<CellItem>;

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

impl DataLabel for CellItem {
    fn category_prefix(&self) -> String {
        "cell".to_string()
    }

    fn tag(&self) -> String {
        match self {
            CellItem::Angle_alpha(_) => "angle_alpha".to_string(),
            CellItem::Angle_beta(_) => "angle_beta".to_string(),
            CellItem::Angle_gamma(_) => "angle_gamma".to_string(),
            CellItem::Formula_units_Z(_) => "formula_units_Z".to_string(),
            CellItem::Length_a(_) => "length_a".to_string(),
            CellItem::Length_b(_) => "length_b".to_string(),
            CellItem::Length_c(_) => "length_c".to_string(),
            CellItem::Measurement_pressure => todo!(),
            CellItem::Measurement_radiation => todo!(),
            CellItem::Measurement_reflns_used => todo!(),
            CellItem::Measurement_temperature => todo!(),
            CellItem::Measurement_theta_max => todo!(),
            CellItem::Measurement_theta_min => todo!(),
            CellItem::Measurement_wavelength => todo!(),
            CellItem::Reciprocal_angle_alpha(_) => "reciprocal_angle_alpha".to_string(),
            CellItem::Reciprocal_angle_beta(_) => "reciprocal_angle_beta".to_string(),
            CellItem::Reciprocal_angle_gamma(_) => "reciprocal_angle_gamma".to_string(),
            CellItem::Reciprocal_length_a(_) => "reciprocal_length_a".to_string(),
            CellItem::Reciprocal_length_b(_) => "reciprocal_length_b".to_string(),
            CellItem::Reciprocal_length_c(_) => "reciprocal_length_c".to_string(),
            CellItem::Special_details => todo!(),
            CellItem::Volume(_) => "volume".to_string(),
        }
    }

    fn value_string(&self) -> String {
        match self {
            CellItem::Angle_alpha(angle)
            | CellItem::Angle_beta(angle)
            | CellItem::Angle_gamma(angle)
            | CellItem::Reciprocal_angle_alpha(angle)
            | CellItem::Reciprocal_angle_beta(angle)
            | CellItem::Reciprocal_angle_gamma(angle) => format!("{angle:.4}"),
            CellItem::Formula_units_Z(z) => format!("{z}"),
            CellItem::Length_a(f)
            | CellItem::Length_b(f)
            | CellItem::Length_c(f)
            | CellItem::Reciprocal_length_a(f)
            | CellItem::Reciprocal_length_b(f)
            | CellItem::Reciprocal_length_c(f)
            | CellItem::Volume(f) => format!("{f:.4}"),
            CellItem::Measurement_pressure => todo!(),
            CellItem::Measurement_radiation => todo!(),
            CellItem::Measurement_reflns_used => todo!(),
            CellItem::Measurement_temperature => todo!(),
            CellItem::Measurement_theta_max => todo!(),
            CellItem::Measurement_theta_min => todo!(),
            CellItem::Measurement_wavelength => todo!(),
            CellItem::Special_details => todo!(),
        }
    }
}

impl<T: UnitCellParameters> From<&T> for CellDataSection {
    fn from(value: &T) -> Self {
        Self::init_builder()
            .add_entry(CellItem::Length_a(value.length_a()))
            .add_entry(CellItem::Length_b(value.length_b()))
            .add_entry(CellItem::Length_c(value.length_c()))
            .add_entry(CellItem::Angle_alpha(CellAngle::new(
                value.angle_alpha().to_degrees(),
            )))
            .add_entry(CellItem::Angle_beta(CellAngle::new(
                value.angle_beta().to_degrees(),
            )))
            .add_entry(CellItem::Angle_gamma(CellAngle::new(
                value.angle_gamma().to_degrees(),
            )))
            .finish()
    }
}
