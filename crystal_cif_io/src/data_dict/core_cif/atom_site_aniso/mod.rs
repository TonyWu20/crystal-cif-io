use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomSiteAnisoTerms {
    Atom_site_tensor_beta(Matrix),
    Beta_11(Numeric),
    Beta_12(Numeric),
    Beta_13(Numeric),
    Beta_22(Numeric),
    Beta_23(Numeric),
    Beta_33(Numeric),
    Matrix_b(Matrix),
    Matrix_u(Matrix),
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
