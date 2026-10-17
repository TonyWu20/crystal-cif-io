//! The `reflns_scale` category: the loop items that record the
//! scale group and its measured intensities (roadmap T2, DDL1
//! `cif_core.dic`).

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
mod reflns_scale;
pub use reflns_scale::ReflnsScale;

/// The loop items of the `reflns_scale` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `reflns_scale` category: one variant per

/// registry loop-value item, in item order.
pub enum ReflnsScaleLoopItem {
    /// The `_reflns_scale_group_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying a scale _reflns_scale_meas_. These are linked
    /// to the _refln_ list by the _refln_scale_group_code. These codes need
    /// not correspond to those in the _diffrn_scale_ list.
    Group_code(Vec<CharString>),
    /// The `_reflns_scale_meas_F` item. The variant holds the column values, one per loop row.
    ///
    /// Scales associated with _reflns_scale_group_code.
    Meas_F(Vec<Numeric>),
    /// The `_reflns_scale_meas_F_squared` item. The variant holds the column values, one per loop row.
    ///
    /// Scales associated with _reflns_scale_group_code.
    Meas_F_squared(Vec<Numeric>),
    /// The `_reflns_scale_meas_intensity` item. The variant holds the column values, one per loop row.
    ///
    /// Scales associated with _reflns_scale_group_code.
    Meas_intensity(Vec<Numeric>),
}

impl CifTerm for ReflnsScaleLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsScaleLoopItem::Group_code(_) => "group_code",
            ReflnsScaleLoopItem::Meas_F(_) => "meas_F",
            ReflnsScaleLoopItem::Meas_F_squared(_) => "meas_F_squared",
            ReflnsScaleLoopItem::Meas_intensity(_) => "meas_intensity",
        };
        Tag::new(format!("reflns_scale_{suffix}"))
    }
}

impl LoopValueTerm for ReflnsScaleLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ReflnsScaleLoopItem::Group_code(v) => v.iter().cloned().map(Value::from).collect(),
            ReflnsScaleLoopItem::Meas_F(v)
            | ReflnsScaleLoopItem::Meas_F_squared(v)
            | ReflnsScaleLoopItem::Meas_intensity(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for ReflnsScaleLoopItem {
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
        CharString::Unquoted(UnquotedString::new("1".into()))
    }

    fn n() -> Numeric {
        Numeric::from(1.5)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<ReflnsScaleLoopItem> = vec![
            ReflnsScaleLoopItem::Group_code(vec![cs()]),
            ReflnsScaleLoopItem::Meas_F(vec![n()]),
            ReflnsScaleLoopItem::Meas_F_squared(vec![n()]),
            ReflnsScaleLoopItem::Meas_intensity(vec![n()]),
        ];
        let expected: Vec<&str> = vec![
            "reflns_scale_group_code",
            "reflns_scale_meas_F",
            "reflns_scale_meas_F_squared",
            "reflns_scale_meas_intensity",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "reflns_scale tag drift");
        }
    }
}
