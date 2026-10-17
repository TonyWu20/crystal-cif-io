use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Tag, Value};

/// The single-value terms of the `atom_sites_fract_transform` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomSitesFractTransformTerms {
    /// The `_atom_sites_fract_transform_axes` item. The variant holds the single value.
    Axes(CharString),
    /// The `_atom_sites_fract_transform_matrix` item. The variant holds the single value.
    Matrix(Matrix),
    /// The `_atom_sites_fract_transform_vector` item. The variant holds the single value.
    Vector(Matrix),
}

impl CifTerm for AtomSitesFractTransformTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomSitesFractTransformTerms::Axes(_) => "axes",
            AtomSitesFractTransformTerms::Matrix(_) => "matrix",
            AtomSitesFractTransformTerms::Vector(_) => "vector",
        };
        Tag::new(format!("atom_sites_fract_transform_{suffix}"))
    }
}

impl SingleValueTerm for AtomSitesFractTransformTerms {
    fn value(&self) -> Value {
        match self {
            AtomSitesFractTransformTerms::Axes(s) => Value::CharString(s.clone()),
            AtomSitesFractTransformTerms::Matrix(m) | AtomSitesFractTransformTerms::Vector(m) => {
                matrix::to_value(m)
            }
        }
    }
}

impl Display for AtomSitesFractTransformTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
