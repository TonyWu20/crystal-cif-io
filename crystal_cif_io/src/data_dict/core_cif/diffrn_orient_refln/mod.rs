use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Matrix, Numeric, Tag, Value};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientReflnTerms {
    Hkl(Matrix),
}

impl CifTerm for DiffrnOrientReflnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientReflnTerms::Hkl(_) => "hkl",
        };
        Tag::new(format!("diffrn_orient_refln_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnOrientReflnTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnOrientReflnTerms::Hkl(m) => matrix::to_value(m),
        }
    }
}

impl Display for DiffrnOrientReflnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnOrientReflnLoopItem {
    Angle_chi(Vec<Numeric>),
    Angle_kappa(Vec<Numeric>),
    Angle_omega(Vec<Numeric>),
    Angle_phi(Vec<Numeric>),
    Angle_psi(Vec<Numeric>),
    Angle_theta(Vec<Numeric>),
    Index_h(Vec<Numeric>),
    Index_k(Vec<Numeric>),
    Index_l(Vec<Numeric>),
}

impl CifTerm for DiffrnOrientReflnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnOrientReflnLoopItem::Angle_chi(_) => "angle_chi",
            DiffrnOrientReflnLoopItem::Angle_kappa(_) => "angle_kappa",
            DiffrnOrientReflnLoopItem::Angle_omega(_) => "angle_omega",
            DiffrnOrientReflnLoopItem::Angle_phi(_) => "angle_phi",
            DiffrnOrientReflnLoopItem::Angle_psi(_) => "angle_psi",
            DiffrnOrientReflnLoopItem::Angle_theta(_) => "angle_theta",
            DiffrnOrientReflnLoopItem::Index_h(_) => "index_h",
            DiffrnOrientReflnLoopItem::Index_k(_) => "index_k",
            DiffrnOrientReflnLoopItem::Index_l(_) => "index_l",
        };
        Tag::new(format!("diffrn_orient_refln_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnOrientReflnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnOrientReflnLoopItem::Angle_chi(n)
            | DiffrnOrientReflnLoopItem::Angle_kappa(n)
            | DiffrnOrientReflnLoopItem::Angle_omega(n)
            | DiffrnOrientReflnLoopItem::Angle_phi(n)
            | DiffrnOrientReflnLoopItem::Angle_psi(n)
            | DiffrnOrientReflnLoopItem::Angle_theta(n)
            | DiffrnOrientReflnLoopItem::Index_h(n)
            | DiffrnOrientReflnLoopItem::Index_k(n)
            | DiffrnOrientReflnLoopItem::Index_l(n) => n.iter().cloned().map(Value::from).collect(),
        }
    }
}
