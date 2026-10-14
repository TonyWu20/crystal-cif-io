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
pub enum GeomHbondLoopItem {
    Angle_dha(Vec<Numeric>),
    Atom_site_label_a(Vec<CharString>),
    Atom_site_label_d(Vec<CharString>),
    Atom_site_label_h(Vec<CharString>),
    Distance_da(Vec<HbondDistance>),
    Distance_dh(Vec<HbondDistance>),
    Distance_ha(Vec<HbondDistance>),
    Publ_flag(Vec<PublFlag>),
    Site_symmetry_a(Vec<CharString>),
    Site_symmetry_d(Vec<CharString>),
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
pub enum GeomHbondTerms {
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
