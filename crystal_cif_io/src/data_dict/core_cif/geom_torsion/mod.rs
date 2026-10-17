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
/// The loop-value terms of the `geom_torsion` category: one variant per

/// registry loop-value item, in item order.
pub enum GeomTorsionLoopItem {
    /// The `_geom_torsion` item. The variant holds the column values, one per loop row.
    ///
    /// The torsion angle in degrees bounded by the four atom sites
    /// identified by the _geom_torsion_atom_site_label_ codes. These must
    /// match labels specified as _atom_site_label in the atom list. The
    /// torsion-angle definition should be that of Klyne and Prelog. Ref:
    /// Klyne, W. & Prelog, V. (1960). Experientia, 16, 521-523.
    /// Units: deg (degree of arc).
    Torsion(Vec<Numeric>),
    /// The `_geom_torsion_atom_site_label_1` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    Atom_site_label_1(Vec<CharString>),
    /// The `_geom_torsion_atom_site_label_2` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    Atom_site_label_2(Vec<CharString>),
    /// The `_geom_torsion_atom_site_label_3` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    Atom_site_label_3(Vec<CharString>),
    /// The `_geom_torsion_atom_site_label_4` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    Atom_site_label_4(Vec<CharString>),
    /// The `_geom_torsion_publ_flag` item. The variant holds the column values, one per loop row.
    ///
    /// This code signals whether the torsion angle is referred to in a
    /// publication or should be placed in a table of significant torsion
    /// angles.
    Publ_flag(Vec<PublFlag>),
    /// The `_geom_torsion_site_symmetry_1` item. The variant holds the column values, one per loop row.
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
    /// The `_geom_torsion_site_symmetry_2` item. The variant holds the column values, one per loop row.
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
    /// The `_geom_torsion_site_symmetry_3` item. The variant holds the column values, one per loop row.
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
    /// The `_geom_torsion_site_symmetry_4` item. The variant holds the column values, one per loop row.
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
/// The single-value terms of the `geom_torsion` category: one variant per

/// registry single-value item, in item order.
pub enum GeomTorsionTerms {
    /// The shell records a three-value list (`_type.dimension`
    /// `[3]`); the read path does not bound the shape.
    /// The `_geom_torsion_distances` item. The variant holds the column values, one per loop row.
    Distances(Vec<Numeric>),
    /// The `_geom_torsion_id` item. The variant holds the column values, one per loop row.
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
