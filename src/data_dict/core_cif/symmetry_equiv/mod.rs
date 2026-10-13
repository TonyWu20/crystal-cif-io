//! The `symmetry_equiv` category: the loop items that record the
//! equivalent positions of the space group (roadmap T2, DDL1
//! `cif_core.dic`).

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Numeric, Tag, Value},
};

/// The loop items of the `symmetry_equiv` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SymmetryEquivLoopItem {
    Equiv_pos_as_xyz(Vec<CharString>),
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
    use crate::grammar::UnquotedString;

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
