mod diffrn_attenuator;
mod scale;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::scale::Scale;
pub use self::diffrn_attenuator::DiffrnAttenuator;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnAttenuatorLoopItem {
    Code(Vec<CharString>),
    Material(Vec<CharString>),
    /// range constraint recorded in the dictionary _enumeration
    Scale(Vec<Scale>),
}

impl CifTerm for DiffrnAttenuatorLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnAttenuatorLoopItem::Code(_) => "code",
            DiffrnAttenuatorLoopItem::Material(_) => "material",
            DiffrnAttenuatorLoopItem::Scale(_) => "scale",
        };
        Tag::new(format!("diffrn_attenuator_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnAttenuatorLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnAttenuatorLoopItem::Code(s) | DiffrnAttenuatorLoopItem::Material(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
            DiffrnAttenuatorLoopItem::Scale(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}
