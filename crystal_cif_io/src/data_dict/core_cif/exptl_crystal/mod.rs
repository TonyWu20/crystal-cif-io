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

pub use codes::{ColourModifier, ColourPrimary, Lustre};

/// The loop items of the `exptl_crystal` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ExptlCrystalLoopItem {
    F_000(Vec<Numeric>),
    Colour(Vec<CharString>),
    Colour_lustre(Vec<Lustre>),
    Colour_modifier(Vec<ColourModifier>),
    Colour_primary(Vec<ColourPrimary>),
    Density_diffrn(Vec<Numeric>),
    Density_meas(Vec<Numeric>),
    Density_meas_gt(Vec<Numeric>),
    Density_meas_lt(Vec<Numeric>),
    Density_meas_temp(Vec<Numeric>),
    Density_meas_temp_gt(Vec<Numeric>),
    Density_meas_temp_lt(Vec<Numeric>),
    Density_method(Vec<CharString>),
    Description(Vec<CharString>),
    Id(Vec<CharString>),
    Preparation(Vec<CharString>),
    Pressure_history(Vec<CharString>),
    Recrystallization_method(Vec<CharString>),
    Size_length(Vec<Numeric>),
    Size_max(Vec<Numeric>),
    Size_mid(Vec<Numeric>),
    Size_min(Vec<Numeric>),
    Size_rad(Vec<Numeric>),
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
pub enum ExptlCrystalTerms {
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0.`); the plain numeric form is kept.
    Mosaic_block_size(Numeric),
    Mosaic_method(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0.`); the plain numeric form is kept.
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
///
/// Follows the style of `CrystalSystemCifError`: a dedicated error
/// type with `Display` and `std::error::Error`.
#[derive(Debug)]
pub struct ExptlCrystalCifError(String);

impl ExptlCrystalCifError {
    fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for ExptlCrystalCifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ExptlCrystalCifError {}
