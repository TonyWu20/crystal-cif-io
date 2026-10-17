mod diffrn_orient_matrix;
use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};
pub use diffrn_orient_matrix::{DiffrnOrientMatrix, DiffrnOrientMatrixRow};

/// The single-value terms of the `diffrn_orient_matrix` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientMatrixTerms {
    /// The `_diffrn_orient_matrix_type` item. The variant holds the single value.
    ///
    /// A description of the orientation matrix type and how it should be
    /// applied to define the orientation of the crystal precisely with
    /// respect to the diffractometer axes.
    Type(CharString),
    /// The `_diffrn_orient_matrix_ubij` item. The variant holds the single value.
    Ubij(Matrix),
}

impl CifTerm for DiffrnOrientMatrixTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientMatrixTerms::Type(_) => "type",
            DiffrnOrientMatrixTerms::Ubij(_) => "ubij",
        };
        Tag::new(format!("diffrn_orient_matrix_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnOrientMatrixTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnOrientMatrixTerms::Type(s) => Value::CharString(s.clone()),
            DiffrnOrientMatrixTerms::Ubij(m) => matrix::to_value(m),
        }
    }
}

impl Display for DiffrnOrientMatrixTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `diffrn_orient_matrix` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientMatrixLoopItem {
    /// The `_diffrn_orient_matrix_UB_11` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_11(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_12` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_12(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_13` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_13(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_21` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_21(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_22` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_22(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_23` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_23(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_31` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_31(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_32` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_32(Vec<Numeric>),
    /// The `_diffrn_orient_matrix_UB_33` item. The variant holds the column values, one per loop row.
    ///
    /// The elements of the diffractometer orientation matrix. These define
    /// the dimensions of the reciprocal cell and its orientation to the
    /// local diffractometer axes. See _diffrn_orient_matrix_type.
    UB_33(Vec<Numeric>),
}

impl CifTerm for DiffrnOrientMatrixLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientMatrixLoopItem::UB_11(_) => "ub_11",
            DiffrnOrientMatrixLoopItem::UB_12(_) => "ub_12",
            DiffrnOrientMatrixLoopItem::UB_13(_) => "ub_13",
            DiffrnOrientMatrixLoopItem::UB_21(_) => "ub_21",
            DiffrnOrientMatrixLoopItem::UB_22(_) => "ub_22",
            DiffrnOrientMatrixLoopItem::UB_23(_) => "ub_23",
            DiffrnOrientMatrixLoopItem::UB_31(_) => "ub_31",
            DiffrnOrientMatrixLoopItem::UB_32(_) => "ub_32",
            DiffrnOrientMatrixLoopItem::UB_33(_) => "ub_33",
        };
        Tag::new(format!("diffrn_orient_matrix_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnOrientMatrixLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnOrientMatrixLoopItem::UB_11(n)
            | DiffrnOrientMatrixLoopItem::UB_12(n)
            | DiffrnOrientMatrixLoopItem::UB_13(n)
            | DiffrnOrientMatrixLoopItem::UB_21(n)
            | DiffrnOrientMatrixLoopItem::UB_22(n)
            | DiffrnOrientMatrixLoopItem::UB_23(n)
            | DiffrnOrientMatrixLoopItem::UB_31(n)
            | DiffrnOrientMatrixLoopItem::UB_32(n)
            | DiffrnOrientMatrixLoopItem::UB_33(n) => n.iter().cloned().map(Value::from).collect(),
        }
    }
}
