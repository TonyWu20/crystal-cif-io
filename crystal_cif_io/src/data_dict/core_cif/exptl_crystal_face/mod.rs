//! The `exptl_crystal_face` category: the loop items that describe
//! a crystal face by its Miller index, its diffractometer
//! orientation angles, and its perpendicular distance (roadmap T2).

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Matrix, Numeric, Tag, Value};
mod exptl_crystal_face;
pub use exptl_crystal_face::ExptlCrystalFace;

/// The loop items of the `exptl_crystal_face` category. Each variant
/// holds one column of the loop. The dictionary records all eight
/// items as `numb` values.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `exptl_crystal_face` category: one variant per

/// registry loop-value item, in item order.
pub enum ExptlCrystalFaceLoopItem {
    /// The `_exptl_crystal_face_diffr_chi` item. The variant holds the column values, one per loop row.
    ///
    /// The goniometer angle settings in degrees when the perpendicular to
    /// the specified crystal face is aligned along a specified direction
    /// (e.g. the bisector of the incident and reflected beams in an optical
    /// goniometer).
    /// Units: deg (degree of arc).
    Diffr_chi(Vec<Numeric>),
    /// The `_exptl_crystal_face_diffr_kappa` item. The variant holds the column values, one per loop row.
    ///
    /// The goniometer angle settings in degrees when the perpendicular to
    /// the specified crystal face is aligned along a specified direction
    /// (e.g. the bisector of the incident and reflected beams in an optical
    /// goniometer).
    /// Units: deg (degree of arc).
    Diffr_kappa(Vec<Numeric>),
    /// The `_exptl_crystal_face_diffr_phi` item. The variant holds the column values, one per loop row.
    ///
    /// The goniometer angle settings in degrees when the perpendicular to
    /// the specified crystal face is aligned along a specified direction
    /// (e.g. the bisector of the incident and reflected beams in an optical
    /// goniometer).
    /// Units: deg (degree of arc).
    Diffr_phi(Vec<Numeric>),
    /// The `_exptl_crystal_face_diffr_psi` item. The variant holds the column values, one per loop row.
    ///
    /// The goniometer angle settings in degrees when the perpendicular to
    /// the specified crystal face is aligned along a specified direction
    /// (e.g. the bisector of the incident and reflected beams in an optical
    /// goniometer).
    /// Units: deg (degree of arc).
    Diffr_psi(Vec<Numeric>),
    /// The `_exptl_crystal_face_index_h` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of the crystal face associated with the value
    /// _exptl_crystal_face_perp_dist.
    Index_h(Vec<Numeric>),
    /// The `_exptl_crystal_face_index_k` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of the crystal face associated with the value
    /// _exptl_crystal_face_perp_dist.
    Index_k(Vec<Numeric>),
    /// The `_exptl_crystal_face_index_l` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of the crystal face associated with the value
    /// _exptl_crystal_face_perp_dist.
    Index_l(Vec<Numeric>),
    /// The `_exptl_crystal_face_perp_dist` item. The variant holds the column values, one per loop row.
    ///
    /// The perpendicular distance in millimetres from the face to the
    /// centre of rotation of the crystal.
    /// Units: mm (millimetre).
    Perp_dist(Vec<Numeric>),
}

impl CifTerm for ExptlCrystalFaceLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ExptlCrystalFaceLoopItem::Diffr_chi(_) => "diffr_chi",
            ExptlCrystalFaceLoopItem::Diffr_kappa(_) => "diffr_kappa",
            ExptlCrystalFaceLoopItem::Diffr_phi(_) => "diffr_phi",
            ExptlCrystalFaceLoopItem::Diffr_psi(_) => "diffr_psi",
            ExptlCrystalFaceLoopItem::Index_h(_) => "index_h",
            ExptlCrystalFaceLoopItem::Index_k(_) => "index_k",
            ExptlCrystalFaceLoopItem::Index_l(_) => "index_l",
            ExptlCrystalFaceLoopItem::Perp_dist(_) => "perp_dist",
        };
        Tag::new(format!("exptl_crystal_face_{suffix}"))
    }
}

impl LoopValueTerm for ExptlCrystalFaceLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ExptlCrystalFaceLoopItem::Diffr_chi(v)
            | ExptlCrystalFaceLoopItem::Diffr_kappa(v)
            | ExptlCrystalFaceLoopItem::Diffr_phi(v)
            | ExptlCrystalFaceLoopItem::Diffr_psi(v)
            | ExptlCrystalFaceLoopItem::Index_h(v)
            | ExptlCrystalFaceLoopItem::Index_k(v)
            | ExptlCrystalFaceLoopItem::Index_l(v)
            | ExptlCrystalFaceLoopItem::Perp_dist(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

// The T6 single-value item of the `exptl_crystal_face` category.

/// The single-value item of the `exptl_crystal_face` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `exptl_crystal_face` category: one variant per

/// registry single-value item, in item order.
pub enum ExptlCrystalFaceTerms {
    /// The shell records a 3-vector (`_type.dimension` `[3]`);
    /// the `Matrix` payload holds the row list.
    /// The `_exptl_crystal_face_hkl` item. The variant holds the column values, one per loop row.
    Hkl(Matrix),
}

impl CifTerm for ExptlCrystalFaceTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ExptlCrystalFaceTerms::Hkl(_) => "hkl",
        };
        Tag::new(format!("exptl_crystal_face_{suffix}"))
    }
}

impl SingleValueTerm for ExptlCrystalFaceTerms {
    fn value(&self) -> Value {
        match self {
            ExptlCrystalFaceTerms::Hkl(m) => matrix::to_value(m),
        }
    }
}

impl Display for ExptlCrystalFaceTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for ExptlCrystalFaceLoopItem {
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

    fn n() -> Numeric {
        Numeric::from(1)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<ExptlCrystalFaceLoopItem> = vec![
            ExptlCrystalFaceLoopItem::Diffr_chi(vec![n()]),
            ExptlCrystalFaceLoopItem::Diffr_kappa(vec![n()]),
            ExptlCrystalFaceLoopItem::Diffr_phi(vec![n()]),
            ExptlCrystalFaceLoopItem::Diffr_psi(vec![n()]),
            ExptlCrystalFaceLoopItem::Index_h(vec![n()]),
            ExptlCrystalFaceLoopItem::Index_k(vec![n()]),
            ExptlCrystalFaceLoopItem::Index_l(vec![n()]),
            ExptlCrystalFaceLoopItem::Perp_dist(vec![n()]),
        ];
        let expected: Vec<&str> = vec![
            "exptl_crystal_face_diffr_chi",
            "exptl_crystal_face_diffr_kappa",
            "exptl_crystal_face_diffr_phi",
            "exptl_crystal_face_diffr_psi",
            "exptl_crystal_face_index_h",
            "exptl_crystal_face_index_k",
            "exptl_crystal_face_index_l",
            "exptl_crystal_face_perp_dist",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "exptl_crystal_face tag drift");
        }
    }
}
