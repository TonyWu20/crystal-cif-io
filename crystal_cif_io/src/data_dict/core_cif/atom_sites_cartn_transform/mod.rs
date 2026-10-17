use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Matrix, Tag, Value};

/// The single-value terms of the `atom_sites_cartn_transform` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomSitesCartnTransformTerms {
    /// The `_atom_sites_cartn_transform_matrix` item. The variant holds the single value.
    Matrix(Matrix),
    /// The `_atom_sites_cartn_transform_vector` item. The variant holds the single value.
    Vector(Matrix),
}

impl CifTerm for AtomSitesCartnTransformTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomSitesCartnTransformTerms::Matrix(_) => "matrix",
            AtomSitesCartnTransformTerms::Vector(_) => "vector",
        };
        Tag::new(format!("atom_sites_cartn_transform_{suffix}"))
    }
}

impl SingleValueTerm for AtomSitesCartnTransformTerms {
    fn value(&self) -> Value {
        match self {
            AtomSitesCartnTransformTerms::Matrix(m) | AtomSitesCartnTransformTerms::Vector(m) => {
                matrix::to_value(m)
            }
        }
    }
}

impl Display for AtomSitesCartnTransformTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
