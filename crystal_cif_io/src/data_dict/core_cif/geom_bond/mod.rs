//! The `geom_bond` category (8 items, all loop columns).
//!
//! One loop lists the bonds of the special list: the two site
//! labels, the distance, the bond order, the publication flag, the
//! site symmetry of the two sites, and the valence.

mod geom_bond;
mod ranges;

pub use geom_bond::{GeomBond, GeomBondRow};
pub use ranges::{BondDistance, BondMultiplicity};

use std::fmt::Display;

use crate::data_dict::core_cif::geom::PublFlag;
use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Loop columns of the `geom_bond` category (8 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `geom_bond` category: one variant per

/// registry loop-value item, in item order.
pub enum GeomBondLoopItem {
    /// The `_geom_bond_atom_site_label_1` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of two atom sites that form a bond. These must match
    /// labels specified as _atom_site_label in the atom list.
    Atom_site_label_1(Vec<CharString>),
    /// The `_geom_bond_atom_site_label_2` item. The variant holds the column values, one per loop row.
    ///
    /// The labels of two atom sites that form a bond. These must match
    /// labels specified as _atom_site_label in the atom list.
    Atom_site_label_2(Vec<CharString>),
    /// The `_geom_bond_distance` item. The variant holds the column values, one per loop row.
    ///
    /// The intramolecular bond distance in angstroms.
    /// Units: A (angstrom).
    Distance(Vec<BondDistance>),
    /// The `_geom_bond_multiplicity` item. The variant holds the column values, one per loop row.
    ///
    /// The number of times the given bond appears in the environment of the
    /// atoms labelled _geom_bond_atom_site_label_1. In cases where the full
    /// list of bonds is given, one of the series of equivalent bonds may be
    /// assigned the appropriate multiplicity while the others are assigned
    /// a value of 0.
    Multiplicity(Vec<BondMultiplicity>),
    /// The `_geom_bond_publ_flag` item. The variant holds the column values, one per loop row.
    ///
    /// This code signals whether the bond distance is referred to in a
    /// publication or should be placed in a list of significant bond
    /// distances.
    Publ_flag(Vec<PublFlag>),
    /// The `_geom_bond_site_symmetry_1` item. The variant holds the column values, one per loop row.
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
    /// calculating the bond. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    Site_symmetry_1(Vec<CharString>),
    /// The `_geom_bond_site_symmetry_2` item. The variant holds the column values, one per loop row.
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
    /// calculating the bond. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    Site_symmetry_2(Vec<CharString>),
    /// The `_geom_bond_valence` item. The variant holds the column values, one per loop row.
    ///
    /// The bond valence calculated from _geom_bond_distance.
    Valence(Vec<Numeric>),
}

impl CifTerm for GeomBondLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomBondLoopItem::Atom_site_label_1(_) => "atom_site_label_1",
            GeomBondLoopItem::Atom_site_label_2(_) => "atom_site_label_2",
            GeomBondLoopItem::Distance(_) => "distance",
            GeomBondLoopItem::Multiplicity(_) => "multiplicity",
            GeomBondLoopItem::Publ_flag(_) => "publ_flag",
            GeomBondLoopItem::Site_symmetry_1(_) => "site_symmetry_1",
            GeomBondLoopItem::Site_symmetry_2(_) => "site_symmetry_2",
            GeomBondLoopItem::Valence(_) => "valence",
        };
        Tag::new(format!("geom_bond_{suffix}"))
    }
}

impl LoopValueTerm for GeomBondLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            GeomBondLoopItem::Atom_site_label_1(v)
            | GeomBondLoopItem::Atom_site_label_2(v)
            | GeomBondLoopItem::Site_symmetry_1(v)
            | GeomBondLoopItem::Site_symmetry_2(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Valence(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Distance(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Multiplicity(v) => v.iter().cloned().map(Value::from).collect(),
            GeomBondLoopItem::Publ_flag(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

// The T6 single-value item of the `geom_bond` category.

/// The single-value item of the `geom_bond` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `geom_bond` category: one variant per

/// registry single-value item, in item order.
pub enum GeomBondTerms {
    /// The `_geom_bond_id` item. The variant holds the column values, one per loop row.
    Id(CharString),
}

impl CifTerm for GeomBondTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            GeomBondTerms::Id(_) => "id",
        };
        Tag::new(format!("geom_bond_{suffix}"))
    }
}

impl SingleValueTerm for GeomBondTerms {
    fn value(&self) -> Value {
        match self {
            GeomBondTerms::Id(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for GeomBondTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for GeomBondLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
