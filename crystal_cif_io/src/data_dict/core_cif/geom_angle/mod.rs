//! The `geom_angle` category (8 items, all loop columns).
//!
//! One loop lists the angles of the special list: the value, the
//! three site labels, the publication flag, and the site symmetry
//! of the three sites. The value column carries the category name
//! itself (`_geom_angle`), not a suffixed name.

mod geom_angle;

pub use geom_angle::{GeomAngle, GeomAngleRow};

use std::fmt::Display;

use crate::data_dict::core_cif::geom::PublFlag;
use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Loop columns of the `geom_angle` category (8 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomAngleLoopItem {
    Angle(Vec<Numeric>),
    Atom_site_label_1(Vec<CharString>),
    Atom_site_label_2(Vec<CharString>),
    Atom_site_label_3(Vec<CharString>),
    Publ_flag(Vec<PublFlag>),
    Site_symmetry_1(Vec<CharString>),
    Site_symmetry_2(Vec<CharString>),
    Site_symmetry_3(Vec<CharString>),
}

impl CifTerm for GeomAngleLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomAngleLoopItem::Atom_site_label_1(_) => "atom_site_label_1",
            GeomAngleLoopItem::Atom_site_label_2(_) => "atom_site_label_2",
            GeomAngleLoopItem::Atom_site_label_3(_) => "atom_site_label_3",
            GeomAngleLoopItem::Publ_flag(_) => "publ_flag",
            GeomAngleLoopItem::Site_symmetry_1(_) => "site_symmetry_1",
            GeomAngleLoopItem::Site_symmetry_2(_) => "site_symmetry_2",
            GeomAngleLoopItem::Site_symmetry_3(_) => "site_symmetry_3",
            // The value column is named after the category itself.
            GeomAngleLoopItem::Angle(_) => "",
        };
        if suffix.is_empty() {
            Tag::new("geom_angle".to_string())
        } else {
            Tag::new(format!("geom_angle_{suffix}"))
        }
    }
}

impl LoopValueTerm for GeomAngleLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            GeomAngleLoopItem::Angle(v) => v.iter().cloned().map(Value::from).collect(),
            GeomAngleLoopItem::Atom_site_label_1(v)
            | GeomAngleLoopItem::Atom_site_label_2(v)
            | GeomAngleLoopItem::Atom_site_label_3(v)
            | GeomAngleLoopItem::Site_symmetry_1(v)
            | GeomAngleLoopItem::Site_symmetry_2(v)
            | GeomAngleLoopItem::Site_symmetry_3(v) => v.iter().cloned().map(Value::from).collect(),
            GeomAngleLoopItem::Publ_flag(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

// The T6 single-value items of the `geom_angle` category.

/// The single-value items of the `geom_angle` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomAngleTerms {
    /// The shell records a two-value list (`_type.dimension`
    /// `[2]`); the read path does not bound the shape.
    Distances(Vec<Numeric>),
    Id(CharString),
}

impl CifTerm for GeomAngleTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomAngleTerms::Distances(_) => "distances",
            GeomAngleTerms::Id(_) => "id",
        };
        Tag::new(format!("geom_angle_{suffix}"))
    }
}

impl SingleValueTerm for GeomAngleTerms {
    fn value(&self) -> Value {
        match self {
            GeomAngleTerms::Distances(v) => {
                Value::List(v.iter().cloned().map(Value::from).collect())
            }
            GeomAngleTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomAngleTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for GeomAngleLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
