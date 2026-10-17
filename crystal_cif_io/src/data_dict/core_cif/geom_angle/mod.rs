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
/// The loop-value terms of the `geom_angle` category: one variant per

/// registry loop-value item, in item order.
pub enum GeomAngleLoopItem {
    /// The `_geom_angle` item. The variant holds the column values, one per loop row.
    ///
    /// Angle in degrees defined by the three sites
    /// _geom_angle_atom_site_label_1, *_2 and *_3. The site at *_2 is at
    /// the apex of the angle.
    /// Units: deg (degree of arc).
    Angle(Vec<Numeric>),
    /// The `_geom_angle_atom_site_label_1` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of the three atom sites which define the angle given by
    /// _geom_angle. These must match labels specified as _atom_site_label
    /// in the atom list. Label 2 identifies the site at the apex of the
    /// angle.
    Atom_site_label_1(Vec<CharString>),
    /// The `_geom_angle_atom_site_label_2` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of the three atom sites which define the angle given by
    /// _geom_angle. These must match labels specified as _atom_site_label
    /// in the atom list. Label 2 identifies the site at the apex of the
    /// angle.
    Atom_site_label_2(Vec<CharString>),
    /// The `_geom_angle_atom_site_label_3` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of the three atom sites which define the angle given by
    /// _geom_angle. These must match labels specified as _atom_site_label
    /// in the atom list. Label 2 identifies the site at the apex of the
    /// angle.
    Atom_site_label_3(Vec<CharString>),
    /// The `_geom_angle_publ_flag` item. The variant holds the column values, one per loop row.
    ///
    /// This code signals whether the angle is referred to in a publication
    /// or should be placed in a table of significant angles.
    Publ_flag(Vec<PublFlag>),
    /// The `_geom_angle_site_symmetry_1` item. The variant holds the column values, one per loop row.
    ///
    /// The symmetry code of each atom site as the symmetry-equivalent
    /// position number 'n' and the cell translation number 'klm'. These
    /// numbers are combined to form the code 'n klm' or n_klm. The
    /// character string n_klm is composed as follows: n refers to the
    /// symmetry operation that is applied to the coordinates stored in
    /// _atom_site_fract_x, _atom_site_fract_y and _atom_site_fract_z. It
    /// must match a number given in _space_group_symop_id. k, l and m refer
    /// to the translations that are subsequently applied to the
    /// symmetry-transformed coordinates to generate the atom used in
    /// calculating the angle. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    Site_symmetry_1(Vec<CharString>),
    /// The `_geom_angle_site_symmetry_2` item. The variant holds the column values, one per loop row.
    ///
    /// The symmetry code of each atom site as the symmetry-equivalent
    /// position number 'n' and the cell translation number 'klm'. These
    /// numbers are combined to form the code 'n klm' or n_klm. The
    /// character string n_klm is composed as follows: n refers to the
    /// symmetry operation that is applied to the coordinates stored in
    /// _atom_site_fract_x, _atom_site_fract_y and _atom_site_fract_z. It
    /// must match a number given in _space_group_symop_id. k, l and m refer
    /// to the translations that are subsequently applied to the
    /// symmetry-transformed coordinates to generate the atom used in
    /// calculating the angle. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    Site_symmetry_2(Vec<CharString>),
    /// The `_geom_angle_site_symmetry_3` item. The variant holds the column values, one per loop row.
    ///
    /// The symmetry code of each atom site as the symmetry-equivalent
    /// position number 'n' and the cell translation number 'klm'. These
    /// numbers are combined to form the code 'n klm' or n_klm. The
    /// character string n_klm is composed as follows: n refers to the
    /// symmetry operation that is applied to the coordinates stored in
    /// _atom_site_fract_x, _atom_site_fract_y and _atom_site_fract_z. It
    /// must match a number given in _space_group_symop_id. k, l and m refer
    /// to the translations that are subsequently applied to the
    /// symmetry-transformed coordinates to generate the atom used in
    /// calculating the angle. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
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
/// The single-value terms of the `geom_angle` category: one variant per

/// registry single-value item, in item order.
pub enum GeomAngleTerms {
    /// The shell records a two-value list (`_type.dimension`
    /// `[2]`); the read path does not bound the shape.
    /// The `_geom_angle_distances` item. The variant holds the column values, one per loop row.
    Distances(Vec<Numeric>),
    /// The `_geom_angle_id` item. The variant holds the column values, one per loop row.
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
