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

mod solution_method;

pub use solution_method::SolutionMethod;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Numeric, Tag, Value},
};

/// Single-value items of the `atom_sites` category.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomSitesItem {
    Cartn_transform_axes(CharString),
    Solution_hydrogens(SolutionMethod),
    Solution_primary(SolutionMethod),
    Solution_secondary(SolutionMethod),
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
pub enum AtomSitesLoopItem {
    Cartn_tran_matrix_11(Vec<Numeric>),
    Cartn_tran_matrix_12(Vec<Numeric>),
    Cartn_tran_matrix_13(Vec<Numeric>),
    Cartn_tran_matrix_21(Vec<Numeric>),
    Cartn_tran_matrix_22(Vec<Numeric>),
    Cartn_tran_matrix_23(Vec<Numeric>),
    Cartn_tran_matrix_31(Vec<Numeric>),
    Cartn_tran_matrix_32(Vec<Numeric>),
    Cartn_tran_matrix_33(Vec<Numeric>),
    Cartn_tran_vector_1(Vec<Numeric>),
    Cartn_tran_vector_2(Vec<Numeric>),
    Cartn_tran_vector_3(Vec<Numeric>),
    Fract_tran_matrix_11(Vec<Numeric>),
    Fract_tran_matrix_12(Vec<Numeric>),
    Fract_tran_matrix_13(Vec<Numeric>),
    Fract_tran_matrix_21(Vec<Numeric>),
    Fract_tran_matrix_22(Vec<Numeric>),
    Fract_tran_matrix_23(Vec<Numeric>),
    Fract_tran_matrix_31(Vec<Numeric>),
    Fract_tran_matrix_32(Vec<Numeric>),
    Fract_tran_matrix_33(Vec<Numeric>),
    Fract_tran_vector_1(Vec<Numeric>),
    Fract_tran_vector_2(Vec<Numeric>),
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
