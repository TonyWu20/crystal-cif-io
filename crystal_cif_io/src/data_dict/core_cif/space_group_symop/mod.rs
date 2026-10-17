use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
mod space_group_symop;

use crystal_cif_fmt::grammar::{CharString, Matrix, Tag, Value};

pub use space_group_symop::Symop;

/// The single-value terms of the `space_group_symop` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupSymopTerms {
    /// The `_space_group_symop_operation_description` item. The variant holds the single value.
    Operation_description(CharString),
    /// The `_space_group_symop_r` item. The variant holds the single value.
    R(Matrix),
    /// The `_space_group_symop_rt` item. The variant holds the single value.
    Rt(Matrix),
    /// The `_space_group_symop_seitz_matrix` item. The variant holds the single value.
    Seitz_matrix(Matrix),
    /// The `_space_group_symop_t` item. The variant holds the single value.
    T(Matrix),
}

impl CifTerm for SpaceGroupSymopTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupSymopTerms::Operation_description(_) => "operation_description",
            SpaceGroupSymopTerms::R(_) => "r",
            SpaceGroupSymopTerms::Rt(_) => "rt",
            SpaceGroupSymopTerms::Seitz_matrix(_) => "seitz_matrix",
            SpaceGroupSymopTerms::T(_) => "t",
        };
        Tag::new(format!("space_group_symop_{suffix}"))
    }
}

impl SingleValueTerm for SpaceGroupSymopTerms {
    fn value(&self) -> Value {
        match self {
            SpaceGroupSymopTerms::Operation_description(s) => Value::CharString(s.clone()),
            SpaceGroupSymopTerms::R(m)
            | SpaceGroupSymopTerms::Rt(m)
            | SpaceGroupSymopTerms::Seitz_matrix(m)
            | SpaceGroupSymopTerms::T(m) => matrix::to_value(m),
        }
    }
}

impl Display for SpaceGroupSymopTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `space_group_symop` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupSymopLoopItem {
    /// The `_space_group_symop_id` item. The variant holds the column values, one per loop row.
    ///
    /// An arbitrary identifier that uniquely labels each symmetry operation
    /// in the list. In order for the defaults to work correctly, the
    /// identity operation should have _space_group_symop_id or
    /// _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    Id(Vec<CharString>),
    /// The `_space_group_symop_operation_xyz` item. The variant holds the column values, one per loop row.
    ///
    /// A parsable string giving one of the symmetry operations of the space
    /// group in algebraic form. If W is a matrix representation of the
    /// rotational part of the symmetry operation defined by the positions
    /// and signs of x, y and z, and w is a column of translations defined
    /// by fractions, an equivalent position X' is generated from a given
    /// position X by the equation X' = WX + w (Note: X is used to represent
    /// bold_italics_x in International Tables for Crystallography Vol. A,
    /// Part 5) When a list of symmetry operations is given, it must contain
    /// a complete set of coordinate representatives which generates all the
    /// operations of the space group by the addition of all primitive
    /// translations of the space group. Such representatives are to be
    /// found as the coordinates of the general-equivalent position in
    /// International Tables for Crystallography Vol. A (2002), to which it
    /// is necessary to add any centring translations shown above the
    /// general-equivalent position. That is to say, it is necessary to list
    /// explicitly all the symmetry operations required to generate all the
    /// atoms in the unit cell defined by the setting used. In order for the
    /// defaults to work correctly, the identity operation should have
    /// _space_group_symop_id or _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    Operation_xyz(Vec<CharString>),
}

impl CifTerm for SpaceGroupSymopLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupSymopLoopItem::Id(_) => "id",
            SpaceGroupSymopLoopItem::Operation_xyz(_) => "operation_xyz",
        };
        Tag::new(format!("space_group_symop_{suffix}"))
    }
}

impl LoopValueTerm for SpaceGroupSymopLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            SpaceGroupSymopLoopItem::Id(s) | SpaceGroupSymopLoopItem::Operation_xyz(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
