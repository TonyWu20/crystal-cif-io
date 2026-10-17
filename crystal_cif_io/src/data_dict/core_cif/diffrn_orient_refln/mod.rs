mod diffrn_orient_refln;
use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Matrix, Numeric, Tag, Value};
pub use diffrn_orient_refln::DiffrnOrientRefln;

/// The single-value terms of the `diffrn_orient_refln` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientReflnTerms {
    /// The `_diffrn_orient_refln_hkl` item. The variant holds the single value.
    Hkl(Matrix),
}

impl CifTerm for DiffrnOrientReflnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientReflnTerms::Hkl(_) => "hkl",
        };
        Tag::new(format!("diffrn_orient_refln_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnOrientReflnTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnOrientReflnTerms::Hkl(m) => matrix::to_value(m),
        }
    }
}

impl Display for DiffrnOrientReflnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `diffrn_orient_refln` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientReflnLoopItem {
    /// The `_diffrn_orient_refln_angle_chi` item. The variant holds the column values, one per loop row.
    ///
    /// Diffractometer angles of a reflection used to define the orientation
    /// matrix in degrees. See _diffrn_orient_matrix_UB_ and
    /// _diffrn_orient_refln_index_h, *_k and *_l.
    /// Units: deg (degree of arc).
    Angle_chi(Vec<Numeric>),
    /// The `_diffrn_orient_refln_angle_kappa` item. The variant holds the column values, one per loop row.
    ///
    /// Diffractometer angles of a reflection used to define the orientation
    /// matrix in degrees. See _diffrn_orient_matrix_UB_ and
    /// _diffrn_orient_refln_index_h, *_k and *_l.
    /// Units: deg (degree of arc).
    Angle_kappa(Vec<Numeric>),
    /// The `_diffrn_orient_refln_angle_omega` item. The variant holds the column values, one per loop row.
    ///
    /// Diffractometer angles of a reflection used to define the orientation
    /// matrix in degrees. See _diffrn_orient_matrix_UB_ and
    /// _diffrn_orient_refln_index_h, *_k and *_l.
    /// Units: deg (degree of arc).
    Angle_omega(Vec<Numeric>),
    /// The `_diffrn_orient_refln_angle_phi` item. The variant holds the column values, one per loop row.
    ///
    /// Diffractometer angles of a reflection used to define the orientation
    /// matrix in degrees. See _diffrn_orient_matrix_UB_ and
    /// _diffrn_orient_refln_index_h, *_k and *_l.
    /// Units: deg (degree of arc).
    Angle_phi(Vec<Numeric>),
    /// The `_diffrn_orient_refln_angle_psi` item. The variant holds the column values, one per loop row.
    ///
    /// Diffractometer angles of a reflection used to define the orientation
    /// matrix in degrees. See _diffrn_orient_matrix_UB_ and
    /// _diffrn_orient_refln_index_h, *_k and *_l.
    /// Units: deg (degree of arc).
    Angle_psi(Vec<Numeric>),
    /// The `_diffrn_orient_refln_angle_theta` item. The variant holds the column values, one per loop row.
    ///
    /// Diffractometer angles of a reflection used to define the orientation
    /// matrix in degrees. See _diffrn_orient_matrix_UB_ and
    /// _diffrn_orient_refln_index_h, *_k and *_l.
    /// Units: deg (degree of arc).
    Angle_theta(Vec<Numeric>),
    /// The `_diffrn_orient_refln_index_h` item. The variant holds the column values, one per loop row.
    ///
    /// The indices of a reflection used to define the orientation matrix.
    /// See _diffrn_orient_matrix_.
    Index_h(Vec<Numeric>),
    /// The `_diffrn_orient_refln_index_k` item. The variant holds the column values, one per loop row.
    ///
    /// The indices of a reflection used to define the orientation matrix.
    /// See _diffrn_orient_matrix_.
    Index_k(Vec<Numeric>),
    /// The `_diffrn_orient_refln_index_l` item. The variant holds the column values, one per loop row.
    ///
    /// The indices of a reflection used to define the orientation matrix.
    /// See _diffrn_orient_matrix_.
    Index_l(Vec<Numeric>),
}

impl CifTerm for DiffrnOrientReflnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientReflnLoopItem::Angle_chi(_) => "angle_chi",
            DiffrnOrientReflnLoopItem::Angle_kappa(_) => "angle_kappa",
            DiffrnOrientReflnLoopItem::Angle_omega(_) => "angle_omega",
            DiffrnOrientReflnLoopItem::Angle_phi(_) => "angle_phi",
            DiffrnOrientReflnLoopItem::Angle_psi(_) => "angle_psi",
            DiffrnOrientReflnLoopItem::Angle_theta(_) => "angle_theta",
            DiffrnOrientReflnLoopItem::Index_h(_) => "index_h",
            DiffrnOrientReflnLoopItem::Index_k(_) => "index_k",
            DiffrnOrientReflnLoopItem::Index_l(_) => "index_l",
        };
        Tag::new(format!("diffrn_orient_refln_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnOrientReflnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnOrientReflnLoopItem::Angle_chi(n)
            | DiffrnOrientReflnLoopItem::Angle_kappa(n)
            | DiffrnOrientReflnLoopItem::Angle_omega(n)
            | DiffrnOrientReflnLoopItem::Angle_phi(n)
            | DiffrnOrientReflnLoopItem::Angle_psi(n)
            | DiffrnOrientReflnLoopItem::Angle_theta(n)
            | DiffrnOrientReflnLoopItem::Index_h(n)
            | DiffrnOrientReflnLoopItem::Index_k(n)
            | DiffrnOrientReflnLoopItem::Index_l(n) => n.iter().cloned().map(Value::from).collect(),
        }
    }
}
