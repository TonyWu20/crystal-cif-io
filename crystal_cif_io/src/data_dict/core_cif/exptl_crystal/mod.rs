//! The `exptl_crystal` category: the loop items that describe the
//! crystal used in the experiment (roadmap T2, DDL1 `cif_core.dic`).
//!
//! All 24 items are loop columns, one row per crystal. The three
//! colour items carry closed code sets; the rest are `numb` or
//! open `char` values.

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

mod codes;
mod exptl_crystal;
pub use exptl_crystal::ExptlCrystal;

pub use codes::{ColourModifier, ColourPrimary, Lustre};

/// The loop items of the `exptl_crystal` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `exptl_crystal` category: one variant per

/// registry loop-value item, in item order.
pub enum ExptlCrystalLoopItem {
    /// The `_exptl_crystal_F_000` item. The variant holds the column values, one per loop row.
    ///
    /// The expression for a structure factor evaluated in the zeroth-order
    /// case h = k = l = 0, F(000). This may contain dispersion
    /// contributions and is calculated as F(000) = [ (sum f~r~)^2^ + (sum
    /// f~i~)^2^ ]^1/2^ f~r~ = real part of the scattering factors at theta
    /// = 0 f~i~ = imaginary part of the scattering factors at theta = 0 the
    /// sum is taken over each atom in the unit cell For X-rays,
    /// non-dispersive F(000) is a positive number and counts the effective
    /// number of electrons in the unit cell; for neutrons, non-dispersive
    /// F(000) (which may be negative) counts the total nuclear scattering
    /// power in the unit cell. See
    /// `http://reference.iucr.org/dictionary/F(000`)
    F_000(Vec<Numeric>),
    /// The `_exptl_crystal_colour` item. The variant holds the column values, one per loop row.
    ///
    /// The colour of the crystal.
    Colour(Vec<CharString>),
    /// The `_exptl_crystal_colour_lustre` item. The variant holds the column values, one per loop row.
    ///
    /// The enumeration list of standardized names developed for the
    /// International Centre for Diffraction Data. The colour of a crystal
    /// is given by the combination of _exptl_crystal_colour_modifier with
    /// _exptl_crystal_colour_primary, as in 'dark-green' or
    /// 'bluish-violet', if necessary combined with
    /// _exptl_crystal_colour_lustre, as in 'metallic-green'. Note that
    /// 'dull' and 'clear' should no longer be used.
    Colour_lustre(Vec<Lustre>),
    /// The `_exptl_crystal_colour_modifier` item. The variant holds the column values, one per loop row.
    ///
    /// The enumeration list of standardized names developed for the
    /// International Centre for Diffraction Data. The colour of a crystal
    /// is given by the combination of _exptl_crystal_colour_modifier with
    /// _exptl_crystal_colour_primary, as in 'dark-green' or
    /// 'bluish-violet', if necessary combined with
    /// _exptl_crystal_colour_lustre, as in 'metallic-green'.
    Colour_modifier(Vec<ColourModifier>),
    /// The `_exptl_crystal_colour_primary` item. The variant holds the column values, one per loop row.
    ///
    /// The enumeration list of standardized names developed for the
    /// International Centre for Diffraction Data. The colour of a crystal
    /// is given by the combination of _exptl_crystal_colour_modifier with
    /// _exptl_crystal_colour_primary, as in 'dark-green' or
    /// 'bluish-violet', if necessary combined with
    /// _exptl_crystal_colour_lustre, as in 'metallic-green'.
    Colour_primary(Vec<ColourPrimary>),
    /// The `_exptl_crystal_density_diffrn` item. The variant holds the column values, one per loop row.
    ///
    /// Density values calculated from the crystal cell and contents. The
    /// units are megagrams per cubic metre (grams per cubic centimetre).
    /// Units: Mg/m^3^ (megagram per metre cubed).
    Density_diffrn(Vec<Numeric>),
    /// The `_exptl_crystal_density_meas` item. The variant holds the column values, one per loop row.
    ///
    /// Density values measured using standard chemical and physical
    /// methods. The units are megagrams per cubic metre (grams per cubic
    /// centimetre).
    /// Units: Mg/m^3^ (megagram per metre cubed).
    Density_meas(Vec<Numeric>),
    /// The `_exptl_crystal_density_meas_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The value above which the density measured using standard chemical
    /// and physical methods lies. The units are megagrams per cubic metre
    /// (grams per cubic centimetre). _exptl_crystal_density_meas_gt and
    /// _exptl_crystal_density_meas_lt should not be used to report new
    /// experimental work, for which _exptl_crystal_density_meas should be
    /// used. These items are intended for use in reporting information in
    /// existing databases and archives which would be misleading if
    /// reported under _exptl_crystal_density_meas.
    /// Units: Mg/m^3^ (megagram per metre cubed).
    Density_meas_gt(Vec<Numeric>),
    /// The `_exptl_crystal_density_meas_lt` item. The variant holds the column values, one per loop row.
    ///
    /// The value below which the density measured using standard chemical
    /// and physical methods lies. The units are megagrams per cubic metre
    /// (grams per cubic centimetre). _exptl_crystal_density_meas_gt and
    /// _exptl_crystal_density_meas_lt should not be used to report new
    /// experimental work, for which _exptl_crystal_density_meas should be
    /// used. These items are intended for use in reporting information in
    /// existing databases and archives which would be misleading if
    /// reported under _exptl_crystal_density_meas.
    /// Units: Mg/m^3^ (megagram per metre cubed).
    Density_meas_lt(Vec<Numeric>),
    /// The `_exptl_crystal_density_meas_temp` item. The variant holds the column values, one per loop row.
    ///
    /// Temperature in kelvins at which _exptl_crystal_density_meas was
    /// determined.
    /// Units: K (kelvin).
    Density_meas_temp(Vec<Numeric>),
    /// The `_exptl_crystal_density_meas_temp_gt` item. The variant holds the column values, one per loop row.
    ///
    /// Temperature in kelvins above which _exptl_crystal_density_meas was
    /// determined. _exptl_crystal_density_meas_temp_gt and
    /// _exptl_crystal_density_meas_temp_lt should not be used for reporting
    /// new work, for which the correct temperature of measurement should be
    /// given. These items are intended for use in reporting information
    /// stored in databases or archives which would be misleading if
    /// reported under _exptl_crystal_density_meas_temp.
    /// Units: K (kelvin).
    Density_meas_temp_gt(Vec<Numeric>),
    /// The `_exptl_crystal_density_meas_temp_lt` item. The variant holds the column values, one per loop row.
    ///
    /// Temperature in kelvins below which _exptl_crystal_density_meas was
    /// determined. _exptl_crystal_density_meas_temp_gt and
    /// _exptl_crystal_density_meas_temp_lt should not be used for reporting
    /// new work, for which the correct temperature of measurement should be
    /// given. These items are intended for use in reporting information
    /// stored in databases or archives which would be misleading if
    /// reported under _exptl_crystal_density_meas_temp.
    /// Units: K (kelvin).
    Density_meas_temp_lt(Vec<Numeric>),
    /// The `_exptl_crystal_density_method` item. The variant holds the column values, one per loop row.
    ///
    /// The method used to measure _exptl_crystal_density_meas.
    Density_method(Vec<CharString>),
    /// The `_exptl_crystal_description` item. The variant holds the column values, one per loop row.
    ///
    /// A description of the quality and habit of the crystal. The crystal
    /// dimensions should not normally be reported here; use instead
    /// _exptl_crystal_size_ for the gross dimensions of the crystal and
    /// _exptl_crystal_face_ to describe the relationship between individual
    /// faces.
    Description(Vec<CharString>),
    /// The `_exptl_crystal_id` item. The variant holds the column values, one per loop row.
    ///
    /// Code identifying each crystal if multiple crystals are used. It is
    /// used to link with _diffrn_refln_crystal_id in the intensity
    /// measurements and with _refln_crystal_id in the _refln_ list.
    Id(Vec<CharString>),
    /// The `_exptl_crystal_preparation` item. The variant holds the column values, one per loop row.
    ///
    /// Details of crystal growth and preparation of the crystal (e.g.
    /// mounting) prior to the intensity measurements.
    Preparation(Vec<CharString>),
    /// The `_exptl_crystal_pressure_history` item. The variant holds the column values, one per loop row.
    ///
    /// Relevant details concerning the pressure history of the sample.
    Pressure_history(Vec<CharString>),
    /// The `_exptl_crystal_recrystallization_method` item. The variant holds the column values, one per loop row.
    ///
    /// Describes the method used to recrystallize the sample. Sufficient
    /// details should be given for the procedure to be repeated. The
    /// temperature or temperatures should be given as well as details of
    /// the solvent, flux or carrier gas with concentrations or pressures
    /// and ambient atmosphere.
    Recrystallization_method(Vec<CharString>),
    /// The `_exptl_crystal_size_length` item. The variant holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    Size_length(Vec<Numeric>),
    /// The `_exptl_crystal_size_max` item. The variant holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    Size_max(Vec<Numeric>),
    /// The `_exptl_crystal_size_mid` item. The variant holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    Size_mid(Vec<Numeric>),
    /// The `_exptl_crystal_size_min` item. The variant holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    Size_min(Vec<Numeric>),
    /// The `_exptl_crystal_size_rad` item. The variant holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    Size_rad(Vec<Numeric>),
    /// The `_exptl_crystal_thermal_history` item. The variant holds the column values, one per loop row.
    ///
    /// Relevant details concerning the thermal history of the sample.
    Thermal_history(Vec<CharString>),
}

impl CifTerm for ExptlCrystalLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ExptlCrystalLoopItem::F_000(_) => "F_000",
            ExptlCrystalLoopItem::Colour(_) => "colour",
            ExptlCrystalLoopItem::Colour_lustre(_) => "colour_lustre",
            ExptlCrystalLoopItem::Colour_modifier(_) => "colour_modifier",
            ExptlCrystalLoopItem::Colour_primary(_) => "colour_primary",
            ExptlCrystalLoopItem::Density_diffrn(_) => "density_diffrn",
            ExptlCrystalLoopItem::Density_meas(_) => "density_meas",
            ExptlCrystalLoopItem::Density_meas_gt(_) => "density_meas_gt",
            ExptlCrystalLoopItem::Density_meas_lt(_) => "density_meas_lt",
            ExptlCrystalLoopItem::Density_meas_temp(_) => "density_meas_temp",
            ExptlCrystalLoopItem::Density_meas_temp_gt(_) => "density_meas_temp_gt",
            ExptlCrystalLoopItem::Density_meas_temp_lt(_) => "density_meas_temp_lt",
            ExptlCrystalLoopItem::Density_method(_) => "density_method",
            ExptlCrystalLoopItem::Description(_) => "description",
            ExptlCrystalLoopItem::Id(_) => "id",
            ExptlCrystalLoopItem::Preparation(_) => "preparation",
            ExptlCrystalLoopItem::Pressure_history(_) => "pressure_history",
            ExptlCrystalLoopItem::Recrystallization_method(_) => "recrystallization_method",
            ExptlCrystalLoopItem::Size_length(_) => "size_length",
            ExptlCrystalLoopItem::Size_max(_) => "size_max",
            ExptlCrystalLoopItem::Size_mid(_) => "size_mid",
            ExptlCrystalLoopItem::Size_min(_) => "size_min",
            ExptlCrystalLoopItem::Size_rad(_) => "size_rad",
            ExptlCrystalLoopItem::Thermal_history(_) => "thermal_history",
        };
        Tag::new(format!("exptl_crystal_{suffix}"))
    }
}

impl LoopValueTerm for ExptlCrystalLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ExptlCrystalLoopItem::F_000(v)
            | ExptlCrystalLoopItem::Density_diffrn(v)
            | ExptlCrystalLoopItem::Density_meas(v)
            | ExptlCrystalLoopItem::Density_meas_gt(v)
            | ExptlCrystalLoopItem::Density_meas_lt(v)
            | ExptlCrystalLoopItem::Density_meas_temp(v)
            | ExptlCrystalLoopItem::Density_meas_temp_gt(v)
            | ExptlCrystalLoopItem::Density_meas_temp_lt(v)
            | ExptlCrystalLoopItem::Size_length(v)
            | ExptlCrystalLoopItem::Size_max(v)
            | ExptlCrystalLoopItem::Size_mid(v)
            | ExptlCrystalLoopItem::Size_min(v)
            | ExptlCrystalLoopItem::Size_rad(v) => v.iter().cloned().map(Value::from).collect(),
            ExptlCrystalLoopItem::Colour(v)
            | ExptlCrystalLoopItem::Density_method(v)
            | ExptlCrystalLoopItem::Description(v)
            | ExptlCrystalLoopItem::Id(v)
            | ExptlCrystalLoopItem::Preparation(v)
            | ExptlCrystalLoopItem::Pressure_history(v)
            | ExptlCrystalLoopItem::Recrystallization_method(v)
            | ExptlCrystalLoopItem::Thermal_history(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            ExptlCrystalLoopItem::Colour_lustre(v) => v.iter().cloned().map(Value::from).collect(),
            ExptlCrystalLoopItem::Colour_modifier(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            ExptlCrystalLoopItem::Colour_primary(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

// The T6 single-value items of the `exptl_crystal` category.

/// The single-value items of the `exptl_crystal` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `exptl_crystal` category: one variant per

/// registry single-value item, in item order.
pub enum ExptlCrystalTerms {
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0.`); the plain numeric form is kept.
    /// The `_exptl_crystal_mosaic_block_size` item. The variant holds the column values, one per loop row.
    Mosaic_block_size(Numeric),
    /// The `_exptl_crystal_mosaic_method` item. The variant holds the column values, one per loop row.
    Mosaic_method(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0.`); the plain numeric form is kept.
    /// The `_exptl_crystal_mosaicity` item. The variant holds the column values, one per loop row.
    Mosaicity(Numeric),
}

impl CifTerm for ExptlCrystalTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ExptlCrystalTerms::Mosaic_block_size(_) => "mosaic_block_size",
            ExptlCrystalTerms::Mosaic_method(_) => "mosaic_method",
            ExptlCrystalTerms::Mosaicity(_) => "mosaicity",
        };
        Tag::new(format!("exptl_crystal_{suffix}"))
    }
}

impl SingleValueTerm for ExptlCrystalTerms {
    fn value(&self) -> Value {
        match self {
            ExptlCrystalTerms::Mosaic_block_size(n) | ExptlCrystalTerms::Mosaicity(n) => {
                Value::Numeric(*n)
            }
            ExptlCrystalTerms::Mosaic_method(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for ExptlCrystalTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for ExptlCrystalLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::grammar::UnquotedString;

    fn cs() -> CharString {
        CharString::Unquoted(UnquotedString::new("colourless".into()))
    }

    fn n() -> Numeric {
        Numeric::from(0.1)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<ExptlCrystalLoopItem> = vec![
            ExptlCrystalLoopItem::F_000(vec![n()]),
            ExptlCrystalLoopItem::Colour(vec![cs()]),
            ExptlCrystalLoopItem::Colour_lustre(vec![Lustre::Clear]),
            ExptlCrystalLoopItem::Colour_modifier(vec![ColourModifier::Light]),
            ExptlCrystalLoopItem::Colour_primary(vec![ColourPrimary::Colourless]),
            ExptlCrystalLoopItem::Density_diffrn(vec![n()]),
            ExptlCrystalLoopItem::Density_meas(vec![n()]),
            ExptlCrystalLoopItem::Density_meas_gt(vec![n()]),
            ExptlCrystalLoopItem::Density_meas_lt(vec![n()]),
            ExptlCrystalLoopItem::Density_meas_temp(vec![n()]),
            ExptlCrystalLoopItem::Density_meas_temp_gt(vec![n()]),
            ExptlCrystalLoopItem::Density_meas_temp_lt(vec![n()]),
            ExptlCrystalLoopItem::Density_method(vec![cs()]),
            ExptlCrystalLoopItem::Description(vec![cs()]),
            ExptlCrystalLoopItem::Id(vec![cs()]),
            ExptlCrystalLoopItem::Preparation(vec![cs()]),
            ExptlCrystalLoopItem::Pressure_history(vec![cs()]),
            ExptlCrystalLoopItem::Recrystallization_method(vec![cs()]),
            ExptlCrystalLoopItem::Size_length(vec![n()]),
            ExptlCrystalLoopItem::Size_max(vec![n()]),
            ExptlCrystalLoopItem::Size_mid(vec![n()]),
            ExptlCrystalLoopItem::Size_min(vec![n()]),
            ExptlCrystalLoopItem::Size_rad(vec![n()]),
            ExptlCrystalLoopItem::Thermal_history(vec![cs()]),
        ];
        let expected: Vec<&str> = vec![
            "exptl_crystal_F_000",
            "exptl_crystal_colour",
            "exptl_crystal_colour_lustre",
            "exptl_crystal_colour_modifier",
            "exptl_crystal_colour_primary",
            "exptl_crystal_density_diffrn",
            "exptl_crystal_density_meas",
            "exptl_crystal_density_meas_gt",
            "exptl_crystal_density_meas_lt",
            "exptl_crystal_density_meas_temp",
            "exptl_crystal_density_meas_temp_gt",
            "exptl_crystal_density_meas_temp_lt",
            "exptl_crystal_density_method",
            "exptl_crystal_description",
            "exptl_crystal_id",
            "exptl_crystal_preparation",
            "exptl_crystal_pressure_history",
            "exptl_crystal_recrystallization_method",
            "exptl_crystal_size_length",
            "exptl_crystal_size_max",
            "exptl_crystal_size_mid",
            "exptl_crystal_size_min",
            "exptl_crystal_size_rad",
            "exptl_crystal_thermal_history",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "exptl_crystal tag drift");
        }
    }
}

/// The closed-set error of the `exptl_crystal` category.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ExptlCrystalCifError(String);

impl ExptlCrystalCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}
