//! The `atom_sites` category (29 items).
//!
//! Single-value items describe how the structure was solved and
//! carry the coordinate-frame transform axes. The loop items are
//! the numbered transform-matrix families: the Cartesian and
//! fractional transform matrices (nine numbered items each) and
//! the two transform vectors (three numbered items each). The
//! numbered items stay plain items; the optional 3 x 3 matrix
//! sugar is out of scope for this wave (roadmap section 12.3,
//! decision log T6).

mod atom_sites;
mod solution_method;

pub use atom_sites::{AtomSites, AtomSitesRow};
pub use solution_method::SolutionMethod;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Single-value items of the `atom_sites` category.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomSitesItem {
    /// The `_atom_sites_Cartn_transform_axes` item. The variant holds the single value.
    Cartn_transform_axes(CharString),
    /// The `_atom_sites_solution_hydrogens` item. The variant holds the single value.
    Solution_hydrogens(SolutionMethod),
    /// The `_atom_sites_solution_primary` item. The variant holds the single value.
    Solution_primary(SolutionMethod),
    /// The `_atom_sites_solution_secondary` item. The variant holds the single value.
    Solution_secondary(SolutionMethod),
    /// The `_atom_sites_special_details` item. The variant holds the single value.
    Special_details(CharString),
}

impl CifTerm for AtomSitesItem {
    fn tag(&self) -> Tag {
        let name = match self {
            AtomSitesItem::Cartn_transform_axes(_) => "Cartn_transform_axes",
            AtomSitesItem::Solution_hydrogens(_) => "solution_hydrogens",
            AtomSitesItem::Solution_primary(_) => "solution_primary",
            AtomSitesItem::Solution_secondary(_) => "solution_secondary",
            AtomSitesItem::Special_details(_) => "special_details",
        };
        Tag::new(format!("atom_sites_{name}"))
    }
}

impl SingleValueTerm for AtomSitesItem {
    fn value(&self) -> Value {
        match self {
            AtomSitesItem::Cartn_transform_axes(s) | AtomSitesItem::Special_details(s) => {
                Value::CharString(s.clone())
            }
            AtomSitesItem::Solution_hydrogens(m)
            | AtomSitesItem::Solution_primary(m)
            | AtomSitesItem::Solution_secondary(m) => Value::from(*m),
        }
    }
}

impl Display for AtomSitesItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The numbered transform-matrix families of the `atom_sites`
/// category (24 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `atom_sites` category: one variant per

/// registry loop-value item, in item order.
pub enum AtomSitesLoopItem {
    /// The `_atom_sites_Cartn_tran_matrix_11` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_11(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_12` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_12(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_13` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_13(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_21` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_21(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_22` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_22(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_23` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_23(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_31` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_31(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_32` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_32(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_matrix_33` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    Cartn_tran_matrix_33(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_vector_1` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// fractional coordinates in the ATOM_SITE category to Cartesian
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33|
    /// z | 3 |
    Cartn_tran_vector_1(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_vector_2` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// fractional coordinates in the ATOM_SITE category to Cartesian
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33|
    /// z | 3 |
    Cartn_tran_vector_2(Vec<Numeric>),
    /// The `_atom_sites_Cartn_tran_vector_3` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// fractional coordinates in the ATOM_SITE category to Cartesian
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33|
    /// z | 3 |
    Cartn_tran_vector_3(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_11` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_11(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_12` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_12(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_13` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_13(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_21` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_21(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_22` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_22(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_23` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_23(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_31` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_31(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_32` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_32(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_matrix_33` item. The variant holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    Fract_tran_matrix_33(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_vector_1` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// Cartesian coordinates in the ATOM_SITE category to fractional
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33|
    /// z | 3 |
    Fract_tran_vector_1(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_vector_2` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// Cartesian coordinates in the ATOM_SITE category to fractional
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33|
    /// z | 3 |
    Fract_tran_vector_2(Vec<Numeric>),
    /// The `_atom_sites_fract_tran_vector_3` item. The variant holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// Cartesian coordinates in the ATOM_SITE category to fractional
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33|
    /// z | 3 |
    Fract_tran_vector_3(Vec<Numeric>),
}

impl CifTerm for AtomSitesLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomSitesLoopItem::Cartn_tran_matrix_11(_) => "Cartn_tran_matrix_11",
            AtomSitesLoopItem::Cartn_tran_matrix_12(_) => "Cartn_tran_matrix_12",
            AtomSitesLoopItem::Cartn_tran_matrix_13(_) => "Cartn_tran_matrix_13",
            AtomSitesLoopItem::Cartn_tran_matrix_21(_) => "Cartn_tran_matrix_21",
            AtomSitesLoopItem::Cartn_tran_matrix_22(_) => "Cartn_tran_matrix_22",
            AtomSitesLoopItem::Cartn_tran_matrix_23(_) => "Cartn_tran_matrix_23",
            AtomSitesLoopItem::Cartn_tran_matrix_31(_) => "Cartn_tran_matrix_31",
            AtomSitesLoopItem::Cartn_tran_matrix_32(_) => "Cartn_tran_matrix_32",
            AtomSitesLoopItem::Cartn_tran_matrix_33(_) => "Cartn_tran_matrix_33",
            AtomSitesLoopItem::Cartn_tran_vector_1(_) => "Cartn_tran_vector_1",
            AtomSitesLoopItem::Cartn_tran_vector_2(_) => "Cartn_tran_vector_2",
            AtomSitesLoopItem::Cartn_tran_vector_3(_) => "Cartn_tran_vector_3",
            AtomSitesLoopItem::Fract_tran_matrix_11(_) => "fract_tran_matrix_11",
            AtomSitesLoopItem::Fract_tran_matrix_12(_) => "fract_tran_matrix_12",
            AtomSitesLoopItem::Fract_tran_matrix_13(_) => "fract_tran_matrix_13",
            AtomSitesLoopItem::Fract_tran_matrix_21(_) => "fract_tran_matrix_21",
            AtomSitesLoopItem::Fract_tran_matrix_22(_) => "fract_tran_matrix_22",
            AtomSitesLoopItem::Fract_tran_matrix_23(_) => "fract_tran_matrix_23",
            AtomSitesLoopItem::Fract_tran_matrix_31(_) => "fract_tran_matrix_31",
            AtomSitesLoopItem::Fract_tran_matrix_32(_) => "fract_tran_matrix_32",
            AtomSitesLoopItem::Fract_tran_matrix_33(_) => "fract_tran_matrix_33",
            AtomSitesLoopItem::Fract_tran_vector_1(_) => "fract_tran_vector_1",
            AtomSitesLoopItem::Fract_tran_vector_2(_) => "fract_tran_vector_2",
            AtomSitesLoopItem::Fract_tran_vector_3(_) => "fract_tran_vector_3",
        };
        Tag::new(format!("atom_sites_{suffix}"))
    }
}

impl LoopValueTerm for AtomSitesLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AtomSitesLoopItem::Cartn_tran_matrix_11(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_12(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_13(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_21(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_22(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_23(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_31(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_32(v)
            | AtomSitesLoopItem::Cartn_tran_matrix_33(v)
            | AtomSitesLoopItem::Cartn_tran_vector_1(v)
            | AtomSitesLoopItem::Cartn_tran_vector_2(v)
            | AtomSitesLoopItem::Cartn_tran_vector_3(v)
            | AtomSitesLoopItem::Fract_tran_matrix_11(v)
            | AtomSitesLoopItem::Fract_tran_matrix_12(v)
            | AtomSitesLoopItem::Fract_tran_matrix_13(v)
            | AtomSitesLoopItem::Fract_tran_matrix_21(v)
            | AtomSitesLoopItem::Fract_tran_matrix_22(v)
            | AtomSitesLoopItem::Fract_tran_matrix_23(v)
            | AtomSitesLoopItem::Fract_tran_matrix_31(v)
            | AtomSitesLoopItem::Fract_tran_matrix_32(v)
            | AtomSitesLoopItem::Fract_tran_matrix_33(v)
            | AtomSitesLoopItem::Fract_tran_vector_1(v)
            | AtomSitesLoopItem::Fract_tran_vector_2(v)
            | AtomSitesLoopItem::Fract_tran_vector_3(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for AtomSitesLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
