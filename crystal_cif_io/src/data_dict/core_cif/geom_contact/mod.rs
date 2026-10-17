//! The `geom_contact` category (6 items, all loop columns).
//!
//! One loop lists the intermolecular contacts of the special list:
//! the two site labels, the distance, the publication flag, and the
//! site symmetry of the two sites.

mod geom_contact;
mod ranges;

pub use geom_contact::{GeomContact, GeomContactRow};
pub use ranges::ContactDistance;

use std::fmt::Display;

use crate::data_dict::core_cif::geom::PublFlag;
use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// Loop columns of the `geom_contact` category (6 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `geom_contact` category: one variant per

/// registry loop-value item, in item order.
pub enum GeomContactLoopItem {
    /// The `_geom_contact_atom_site_label_1` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of two atom sites that are within contact distance. The
    /// labels must match _atom_site_label codes in the atom list.
    Atom_site_label_1(Vec<CharString>),
    /// The `_geom_contact_atom_site_label_2` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of two atom sites that are within contact distance. The
    /// labels must match _atom_site_label codes in the atom list.
    Atom_site_label_2(Vec<CharString>),
    /// The `_geom_contact_distance` item. The variant holds the column values, one per loop row.
    ///
    /// The interatomic contact distance in angstroms.
    /// Units: A (angstrom).
    Distance(Vec<ContactDistance>),
    /// The `_geom_contact_publ_flag` item. The variant holds the column values, one per loop row.
    ///
    /// This code signals whether the contact distance is referred to in a
    /// publication or should be placed in a list of significant contact
    /// distances.
    Publ_flag(Vec<PublFlag>),
    /// The `_geom_contact_site_symmetry_1` item. The variant holds the column values, one per loop row.
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
    /// calculating the contact. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    Site_symmetry_1(Vec<CharString>),
    /// The `_geom_contact_site_symmetry_2` item. The variant holds the column values, one per loop row.
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
    /// calculating the contact. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    Site_symmetry_2(Vec<CharString>),
}

impl CifTerm for GeomContactLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomContactLoopItem::Atom_site_label_1(_) => "atom_site_label_1",
            GeomContactLoopItem::Atom_site_label_2(_) => "atom_site_label_2",
            GeomContactLoopItem::Distance(_) => "distance",
            GeomContactLoopItem::Publ_flag(_) => "publ_flag",
            GeomContactLoopItem::Site_symmetry_1(_) => "site_symmetry_1",
            GeomContactLoopItem::Site_symmetry_2(_) => "site_symmetry_2",
        };
        Tag::new(format!("geom_contact_{suffix}"))
    }
}

impl LoopValueTerm for GeomContactLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            GeomContactLoopItem::Atom_site_label_1(v)
            | GeomContactLoopItem::Atom_site_label_2(v)
            | GeomContactLoopItem::Site_symmetry_1(v)
            | GeomContactLoopItem::Site_symmetry_2(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            GeomContactLoopItem::Distance(v) => v.iter().cloned().map(Value::from).collect(),
            GeomContactLoopItem::Publ_flag(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

// The T6 single-value item of the `geom_contact` category.

/// The single-value item of the `geom_contact` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `geom_contact` category: one variant per

/// registry single-value item, in item order.
pub enum GeomContactTerms {
    /// The `_geom_contact_id` item. The variant holds the column values, one per loop row.
    Id(CharString),
}

impl CifTerm for GeomContactTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomContactTerms::Id(_) => "id",
        };
        Tag::new(format!("geom_contact_{suffix}"))
    }
}

impl SingleValueTerm for GeomContactTerms {
    fn value(&self) -> Value {
        match self {
            GeomContactTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomContactTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for GeomContactLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
