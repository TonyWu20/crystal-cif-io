mod multiplicity;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::multiplicity::Multiplicity;

/// The single-value terms of the `space_group_wyckoff` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpaceGroupWyckoffTerms {
    /// The `_space_group_wyckoff_coords_xyz` item. The variant holds the single value.
    Coords_xyz(CharString),
    /// The `_space_group_wyckoff_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_space_group_wyckoff_letter` item. The variant holds the single value.
    Letter(CharString),
    /// The `_space_group_wyckoff_multiplicity` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Multiplicity(Multiplicity),
    /// The `_space_group_wyckoff_site_symmetry` item. The variant holds the single value.
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
