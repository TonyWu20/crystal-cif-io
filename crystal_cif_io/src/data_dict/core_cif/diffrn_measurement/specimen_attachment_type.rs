use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_measurement_specimen_attachment_type`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SpecimenAttachmentType {
    /// `epoxy`
    Epoxy,
    /// `oil`
    Oil,
    /// `grease`
    Grease,
    /// `frozen_liquid`
    Frozen_liquid,
    /// `other`
    Other,
    /// `ice`
    Ice,
    /// `mechanical`
    Mechanical,
}

impl Display for SpecimenAttachmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Epoxy => "epoxy",
            Self::Oil => "oil",
            Self::Grease => "grease",
            Self::Frozen_liquid => "frozen_liquid",
            Self::Other => "other",
            Self::Ice => "ice",
            Self::Mechanical => "mechanical",
        };
        f.write_str(code)
    }
}

impl From<SpecimenAttachmentType> for UnquotedString {
    fn from(value: SpecimenAttachmentType) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<SpecimenAttachmentType> for CharString {
    fn from(value: SpecimenAttachmentType) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<SpecimenAttachmentType> for Value {
    fn from(value: SpecimenAttachmentType) -> Self {
        Value::CharString(CharString::from(value))
    }
}
