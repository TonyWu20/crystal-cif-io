mod multiplicity;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::multiplicity::Multiplicity;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupWyckoffTerms {
    Coords_xyz(CharString),
    Id(CharString),
    Letter(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Multiplicity(Multiplicity),
    Site_symmetry(CharString),
}

impl CifTerm for SpaceGroupWyckoffTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupWyckoffTerms::Coords_xyz(_) => "coords_xyz",
            SpaceGroupWyckoffTerms::Id(_) => "id",
            SpaceGroupWyckoffTerms::Letter(_) => "letter",
            SpaceGroupWyckoffTerms::Multiplicity(_) => "multiplicity",
            SpaceGroupWyckoffTerms::Site_symmetry(_) => "site_symmetry",
        };
        Tag::new(format!("space_group_wyckoff_{suffix}"))
    }
}

impl SingleValueTerm for SpaceGroupWyckoffTerms {
    fn value(&self) -> Value {
        match self {
            SpaceGroupWyckoffTerms::Coords_xyz(s)
            | SpaceGroupWyckoffTerms::Id(s)
            | SpaceGroupWyckoffTerms::Letter(s)
            | SpaceGroupWyckoffTerms::Site_symmetry(s) => Value::CharString(s.clone()),
            SpaceGroupWyckoffTerms::Multiplicity(v) => Value::from(*v),
        }
    }
}

impl Display for SpaceGroupWyckoffTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
