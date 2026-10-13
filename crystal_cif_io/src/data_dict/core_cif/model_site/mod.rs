mod index;
mod mole_index;
mod radius_bond;
mod radius_contact;

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Tag, Value};

pub use self::index::Index;
pub use self::mole_index::MoleIndex;
pub use self::radius_bond::RadiusBond;
pub use self::radius_contact::RadiusContact;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ModelSiteTerms {
    /// list or array payload; the shape is recorded in the shell
    /// _type.dimension
    Adp_eigenvalues(Matrix),
    /// list or array payload; the shape is recorded in the shell
    /// _type.dimension
    Adp_eigenvectors(Matrix),
    Adp_matrix_beta(Matrix),
    Cartn_xyz(Matrix),
    Display_colour(CharString),
    Fract_xyz(Matrix),
    Id(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Index(Index),
    Label(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Mole_index(MoleIndex),
    /// range constraint recorded in the dictionary _enumeration
    Radius_bond(RadiusBond),
    /// range constraint recorded in the dictionary _enumeration
    Radius_contact(RadiusContact),
    Symop(CharString),
    Type_symbol(CharString),
}

impl CifTerm for ModelSiteTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ModelSiteTerms::Adp_eigenvalues(_) => "adp_eigenvalues",
            ModelSiteTerms::Adp_eigenvectors(_) => "adp_eigenvectors",
            ModelSiteTerms::Adp_matrix_beta(_) => "adp_matrix_beta",
            ModelSiteTerms::Cartn_xyz(_) => "cartn_xyz",
            ModelSiteTerms::Display_colour(_) => "display_colour",
            ModelSiteTerms::Fract_xyz(_) => "fract_xyz",
            ModelSiteTerms::Id(_) => "id",
            ModelSiteTerms::Index(_) => "index",
            ModelSiteTerms::Label(_) => "label",
            ModelSiteTerms::Mole_index(_) => "mole_index",
            ModelSiteTerms::Radius_bond(_) => "radius_bond",
            ModelSiteTerms::Radius_contact(_) => "radius_contact",
            ModelSiteTerms::Symop(_) => "symop",
            ModelSiteTerms::Type_symbol(_) => "type_symbol",
        };
        Tag::new(format!("model_site_{suffix}"))
    }
}

impl SingleValueTerm for ModelSiteTerms {
    fn value(&self) -> Value {
        match self {
            ModelSiteTerms::Adp_eigenvalues(m)
            | ModelSiteTerms::Adp_eigenvectors(m)
            | ModelSiteTerms::Adp_matrix_beta(m)
            | ModelSiteTerms::Cartn_xyz(m)
            | ModelSiteTerms::Fract_xyz(m) => matrix::to_value(m),
            ModelSiteTerms::Display_colour(s)
            | ModelSiteTerms::Id(s)
            | ModelSiteTerms::Label(s)
            | ModelSiteTerms::Symop(s)
            | ModelSiteTerms::Type_symbol(s) => Value::CharString(s.clone()),
            ModelSiteTerms::Index(v) => Value::from(*v),
            ModelSiteTerms::Mole_index(m) => Value::from(*m),
            ModelSiteTerms::Radius_bond(c) => Value::from(*c),
            ModelSiteTerms::Radius_contact(b) => Value::from(*b),
        }
    }
}

impl Display for ModelSiteTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
