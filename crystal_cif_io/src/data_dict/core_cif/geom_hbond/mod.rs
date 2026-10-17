//! The `geom_hbond` category (11 items, all loop columns).
//!
//! One loop lists the hydrogen bonds of the special list: the DHA
//! angle, the three site labels (donor, hydrogen, acceptor), the
//! three distances, the publication flag, and the site symmetry of
//! the three sites.

mod geom_hbond;
mod ranges;

pub use geom_hbond::{GeomHbond, GeomHbondRow};
pub use ranges::HbondDistance;

use std::fmt::Display;

use crate::data_dict::core_cif::geom::PublFlag;
use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Loop columns of the `geom_hbond` category (11 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `geom_hbond` category: one variant per

/// registry loop-value item, in item order.
pub enum GeomHbondLoopItem {
    /// The `_geom_hbond_angle_DHA` item. The variant holds the column values, one per loop row.
    ///
    /// Angle in degrees defined by the three sites
    /// _geom_hbond_atom_site_label_D, *_H and *_A. The site at *_H (the
    /// hydrogen atom participating in the interaction) is at the apex of
    /// the angle.
    /// Units: deg (degree of arc).
    Angle_dha(Vec<Numeric>),
    /// The `_geom_hbond_atom_site_label_A` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of three atom sites (respectively, the donor atom,
    /// hydrogen atom and acceptor atom) participating in a hydrogen bond.
    /// These must match labels specified as _atom_site_label in the atom
    /// list.
    Atom_site_label_a(Vec<CharString>),
    /// The `_geom_hbond_atom_site_label_D` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of three atom sites (respectively, the donor atom,
    /// hydrogen atom and acceptor atom) participating in a hydrogen bond.
    /// These must match labels specified as _atom_site_label in the atom
    /// list.
    Atom_site_label_d(Vec<CharString>),
    /// The `_geom_hbond_atom_site_label_H` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of three atom sites (respectively, the donor atom,
    /// hydrogen atom and acceptor atom) participating in a hydrogen bond.
    /// These must match labels specified as _atom_site_label in the atom
    /// list.
    Atom_site_label_h(Vec<CharString>),
    /// The `_geom_hbond_distance_DA` item. The variant holds the column values, one per loop row.
    ///
    /// Distances in angstroms between the donor and hydrogen (*_DH),
    /// hydrogen and acceptor (*_HA) and donor and acceptor (*_DA) sites in
    /// a hydrogen bond.
    /// Units: A (angstrom).
    Distance_da(Vec<HbondDistance>),
    /// The `_geom_hbond_distance_DH` item. The variant holds the column values, one per loop row.
    ///
    /// Distances in angstroms between the donor and hydrogen (*_DH),
    /// hydrogen and acceptor (*_HA) and donor and acceptor (*_DA) sites in
    /// a hydrogen bond.
    /// Units: A (angstrom).
    Distance_dh(Vec<HbondDistance>),
    /// The `_geom_hbond_distance_HA` item. The variant holds the column values, one per loop row.
    ///
    /// Distances in angstroms between the donor and hydrogen (*_DH),
    /// hydrogen and acceptor (*_HA) and donor and acceptor (*_DA) sites in
    /// a hydrogen bond.
    /// Units: A (angstrom).
    Distance_ha(Vec<HbondDistance>),
    /// The `_geom_hbond_publ_flag` item. The variant holds the column values, one per loop row.
    ///
    /// This code signals whether the hydrogen-bond information is referred
    /// to in a publication or should be placed in a table of significant
    /// hydrogen-bond geometry.
    Publ_flag(Vec<PublFlag>),
    /// The `_geom_hbond_site_symmetry_A` item. The variant holds the column values, one per loop row.
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
    /// calculating the hydrogen bond. These translations (x,y,z) are
    /// related to (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By
    /// adding 5 to the translations, the use of negative numbers is
    /// avoided.
    Site_symmetry_a(Vec<CharString>),
    /// The `_geom_hbond_site_symmetry_D` item. The variant holds the column values, one per loop row.
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
    /// calculating the hydrogen bond. These translations (x,y,z) are
    /// related to (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By
    /// adding 5 to the translations, the use of negative numbers is
    /// avoided.
    Site_symmetry_d(Vec<CharString>),
    /// The `_geom_hbond_site_symmetry_H` item. The variant holds the column values, one per loop row.
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
    /// calculating the hydrogen bond. These translations (x,y,z) are
    /// related to (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By
    /// adding 5 to the translations, the use of negative numbers is
    /// avoided.
    Site_symmetry_h(Vec<CharString>),
}

impl CifTerm for GeomHbondLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomHbondLoopItem::Angle_dha(_) => "angle_DHA",
            GeomHbondLoopItem::Atom_site_label_a(_) => "atom_site_label_A",
            GeomHbondLoopItem::Atom_site_label_d(_) => "atom_site_label_D",
            GeomHbondLoopItem::Atom_site_label_h(_) => "atom_site_label_H",
            GeomHbondLoopItem::Distance_da(_) => "distance_DA",
            GeomHbondLoopItem::Distance_dh(_) => "distance_DH",
            GeomHbondLoopItem::Distance_ha(_) => "distance_HA",
            GeomHbondLoopItem::Publ_flag(_) => "publ_flag",
            GeomHbondLoopItem::Site_symmetry_a(_) => "site_symmetry_A",
            GeomHbondLoopItem::Site_symmetry_d(_) => "site_symmetry_D",
            GeomHbondLoopItem::Site_symmetry_h(_) => "site_symmetry_H",
        };
        Tag::new(format!("geom_hbond_{suffix}"))
    }
}

impl LoopValueTerm for GeomHbondLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            GeomHbondLoopItem::Angle_dha(v) => v.iter().cloned().map(Value::from).collect(),
            GeomHbondLoopItem::Atom_site_label_a(v)
            | GeomHbondLoopItem::Atom_site_label_d(v)
            | GeomHbondLoopItem::Atom_site_label_h(v)
            | GeomHbondLoopItem::Site_symmetry_a(v)
            | GeomHbondLoopItem::Site_symmetry_d(v)
            | GeomHbondLoopItem::Site_symmetry_h(v) => v.iter().cloned().map(Value::from).collect(),
            GeomHbondLoopItem::Distance_da(v)
            | GeomHbondLoopItem::Distance_dh(v)
            | GeomHbondLoopItem::Distance_ha(v) => v.iter().cloned().map(Value::from).collect(),
            GeomHbondLoopItem::Publ_flag(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

// The T6 single-value item of the `geom_hbond` category.

/// The single-value item of the `geom_hbond` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `geom_hbond` category: one variant per

/// registry single-value item, in item order.
pub enum GeomHbondTerms {
    /// The `_geom_hbond_id` item. The variant holds the column values, one per loop row.
    Id(CharString),
}

impl CifTerm for GeomHbondTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomHbondTerms::Id(_) => "id",
        };
        Tag::new(format!("geom_hbond_{suffix}"))
    }
}

impl SingleValueTerm for GeomHbondTerms {
    fn value(&self) -> Value {
        match self {
            GeomHbondTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomHbondTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for GeomHbondLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
