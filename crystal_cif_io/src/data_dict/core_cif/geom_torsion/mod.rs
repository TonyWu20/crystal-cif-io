//! The `geom_torsion` category (10 items, all loop columns).
//!
//! One loop lists the torsion angles of the special list: the
//! value, the four site labels, the publication flag, and the site
//! symmetry of the four sites. The value column carries the
//! category name itself (`_geom_torsion`), not a suffixed name.

mod geom_torsion;

pub use geom_torsion::{GeomTorsion, GeomTorsionRow};

use std::fmt::Display;

use crate::data_dict::core_cif::geom::PublFlag;
use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Loop columns of the `geom_torsion` category (10 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomTorsionLoopItem {
    Torsion(Vec<Numeric>),
    Atom_site_label_1(Vec<CharString>),
    Atom_site_label_2(Vec<CharString>),
    Atom_site_label_3(Vec<CharString>),
    Atom_site_label_4(Vec<CharString>),
    Publ_flag(Vec<PublFlag>),
    Site_symmetry_1(Vec<CharString>),
    Site_symmetry_2(Vec<CharString>),
    Site_symmetry_3(Vec<CharString>),
    Site_symmetry_4(Vec<CharString>),
}

impl CifTerm for GeomTorsionLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomTorsionLoopItem::Atom_site_label_1(_) => "atom_site_label_1",
            GeomTorsionLoopItem::Atom_site_label_2(_) => "atom_site_label_2",
            GeomTorsionLoopItem::Atom_site_label_3(_) => "atom_site_label_3",
            GeomTorsionLoopItem::Atom_site_label_4(_) => "atom_site_label_4",
            GeomTorsionLoopItem::Publ_flag(_) => "publ_flag",
            GeomTorsionLoopItem::Site_symmetry_1(_) => "site_symmetry_1",
            GeomTorsionLoopItem::Site_symmetry_2(_) => "site_symmetry_2",
            GeomTorsionLoopItem::Site_symmetry_3(_) => "site_symmetry_3",
            GeomTorsionLoopItem::Site_symmetry_4(_) => "site_symmetry_4",
            // The value column is named after the category itself.
            GeomTorsionLoopItem::Torsion(_) => "",
        };
        if suffix.is_empty() {
            Tag::new("geom_torsion".to_string())
        } else {
            Tag::new(format!("geom_torsion_{suffix}"))
        }
    }
}

impl LoopValueTerm for GeomTorsionLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            GeomTorsionLoopItem::Torsion(v) => v.iter().cloned().map(Value::from).collect(),
            GeomTorsionLoopItem::Atom_site_label_1(v)
            | GeomTorsionLoopItem::Atom_site_label_2(v)
            | GeomTorsionLoopItem::Atom_site_label_3(v)
            | GeomTorsionLoopItem::Atom_site_label_4(v)
            | GeomTorsionLoopItem::Site_symmetry_1(v)
            | GeomTorsionLoopItem::Site_symmetry_2(v)
            | GeomTorsionLoopItem::Site_symmetry_3(v)
            | GeomTorsionLoopItem::Site_symmetry_4(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            GeomTorsionLoopItem::Publ_flag(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

// The T6 single-value items of the `geom_torsion` category.

/// The single-value items of the `geom_torsion` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomTorsionTerms {
    /// The shell records a three-value list (`_type.dimension`
    /// `[3]`); the read path does not bound the shape.
    Distances(Vec<Numeric>),
    Id(CharString),
}

impl CifTerm for GeomTorsionTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomTorsionTerms::Distances(_) => "distances",
            GeomTorsionTerms::Id(_) => "id",
        };
        Tag::new(format!("geom_torsion_{suffix}"))
    }
}

impl SingleValueTerm for GeomTorsionTerms {
    fn value(&self) -> Value {
        match self {
            GeomTorsionTerms::Distances(v) => {
                Value::List(v.iter().cloned().map(Value::from).collect())
            }
            GeomTorsionTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomTorsionTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for GeomTorsionLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
