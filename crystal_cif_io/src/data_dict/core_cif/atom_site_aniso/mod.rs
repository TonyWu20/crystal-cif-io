use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};

/// The single-value terms of the `atom_site_aniso` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomSiteAnisoTerms {
    /// The `_atom_site.tensor_beta` item. The variant holds the single value.
    Atom_site_tensor_beta(Matrix),
    /// The `_atom_site_aniso_beta_11` item. The variant holds the single value.
    Beta_11(Numeric),
    /// The `_atom_site_aniso_beta_12` item. The variant holds the single value.
    Beta_12(Numeric),
    /// The `_atom_site_aniso_beta_13` item. The variant holds the single value.
    Beta_13(Numeric),
    /// The `_atom_site_aniso_beta_22` item. The variant holds the single value.
    Beta_22(Numeric),
    /// The `_atom_site_aniso_beta_23` item. The variant holds the single value.
    Beta_23(Numeric),
    /// The `_atom_site_aniso_beta_33` item. The variant holds the single value.
    Beta_33(Numeric),
    /// The `_atom_site_aniso_matrix_b` item. The variant holds the single value.
    Matrix_b(Matrix),
    /// The `_atom_site_aniso_matrix_u` item. The variant holds the single value.
    Matrix_u(Matrix),
    /// The `_atom_site_aniso_symmform` item. The variant holds the single value.
    Symmform(CharString),
}

impl CifTerm for AtomSiteAnisoTerms {
    fn tag(&self) -> Tag {
        let tag = match self {
            AtomSiteAnisoTerms::Atom_site_tensor_beta(_) => "atom_site.tensor_beta",
            AtomSiteAnisoTerms::Beta_11(_) => "atom_site_aniso_beta_11",
            AtomSiteAnisoTerms::Beta_12(_) => "atom_site_aniso_beta_12",
            AtomSiteAnisoTerms::Beta_13(_) => "atom_site_aniso_beta_13",
            AtomSiteAnisoTerms::Beta_22(_) => "atom_site_aniso_beta_22",
            AtomSiteAnisoTerms::Beta_23(_) => "atom_site_aniso_beta_23",
            AtomSiteAnisoTerms::Beta_33(_) => "atom_site_aniso_beta_33",
            AtomSiteAnisoTerms::Matrix_b(_) => "atom_site_aniso_matrix_b",
            AtomSiteAnisoTerms::Matrix_u(_) => "atom_site_aniso_matrix_u",
            AtomSiteAnisoTerms::Symmform(_) => "atom_site_aniso_symmform",
        };
        Tag::new(tag.to_string())
    }
}

impl SingleValueTerm for AtomSiteAnisoTerms {
    fn value(&self) -> Value {
        match self {
            AtomSiteAnisoTerms::Atom_site_tensor_beta(m)
            | AtomSiteAnisoTerms::Matrix_b(m)
            | AtomSiteAnisoTerms::Matrix_u(m) => matrix::to_value(m),
            AtomSiteAnisoTerms::Beta_11(n)
            | AtomSiteAnisoTerms::Beta_12(n)
            | AtomSiteAnisoTerms::Beta_13(n)
            | AtomSiteAnisoTerms::Beta_22(n)
            | AtomSiteAnisoTerms::Beta_23(n)
            | AtomSiteAnisoTerms::Beta_33(n) => Value::Numeric(*n),
            AtomSiteAnisoTerms::Symmform(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for AtomSiteAnisoTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
