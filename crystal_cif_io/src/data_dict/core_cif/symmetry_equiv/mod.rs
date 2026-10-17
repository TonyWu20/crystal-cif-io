//! The `symmetry_equiv` category: the loop items that record the
//! equivalent positions of the space group (roadmap T2, DDL1
//! `cif_core.dic`).

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
mod symmetry_equiv;
pub use symmetry_equiv::SymmetryEquiv;

/// The loop items of the `symmetry_equiv` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `symmetry_equiv` category: one variant per

/// registry loop-value item, in item order.
pub enum SymmetryEquivLoopItem {
    /// The `_symmetry_equiv_pos_as_xyz` item. The variant holds the column values, one per loop row.
    ///
    /// Symmetry-equivalent position in the 'xyz' representation. Except for
    /// the space group P1, these data will be repeated in a loop. The
    /// format of the data item is as per International Tables for
    /// Crystallography Vol. A. (2002). All equivalent positions should be
    /// entered, including those for lattice centring and a centre of
    /// symmetry, if present. In order for the defaults to work correctly,
    /// the identity operation should have _space_group_symop_id or
    /// _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    Equiv_pos_as_xyz(Vec<CharString>),
    /// The `_symmetry_equiv_pos_site_id` item. The variant holds the column values, one per loop row.
    ///
    /// A code identifying each entry in the _symmetry_equiv_pos_as_xyz
    /// list. It is normally the sequence number of the entry in that list,
    /// and should be identified with the code 'n' in _geom_*_symmetry_
    /// codes of the form 'n_klm'. In order for the defaults to work
    /// correctly, the identity operation should have _space_group_symop_id
    /// or _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    Equiv_pos_site_id(Vec<Numeric>),
}

impl CifTerm for SymmetryEquivLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SymmetryEquivLoopItem::Equiv_pos_as_xyz(_) => "pos_as_xyz",
            SymmetryEquivLoopItem::Equiv_pos_site_id(_) => "pos_site_id",
        };
        Tag::new(format!("symmetry_equiv_{suffix}"))
    }
}

impl LoopValueTerm for SymmetryEquivLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            SymmetryEquivLoopItem::Equiv_pos_as_xyz(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            SymmetryEquivLoopItem::Equiv_pos_site_id(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for SymmetryEquivLoopItem {
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
        CharString::Unquoted(UnquotedString::new("x,y,z".into()))
    }

    fn n() -> Numeric {
        Numeric::from(1)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<SymmetryEquivLoopItem> = vec![
            SymmetryEquivLoopItem::Equiv_pos_as_xyz(vec![cs()]),
            SymmetryEquivLoopItem::Equiv_pos_site_id(vec![n()]),
        ];
        let expected: Vec<&str> = vec!["symmetry_equiv_pos_as_xyz", "symmetry_equiv_pos_site_id"];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "symmetry_equiv tag drift");
        }
    }
}
