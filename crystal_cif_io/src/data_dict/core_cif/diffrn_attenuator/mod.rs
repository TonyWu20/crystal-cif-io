mod diffrn_attenuator;
mod scale;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::diffrn_attenuator::DiffrnAttenuator;
pub use self::scale::Scale;

/// The loop-value terms of the `diffrn_attenuator` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnAttenuatorLoopItem {
    /// The `_diffrn_attenuator_code` item. The variant holds the column values, one per loop row.
    ///
    /// A code associated with a particular attenuator setting. This code is
    /// referenced by the _diffrn_refln_attenuator_code which is stored with
    /// the intensities. See _diffrn_attenuator_scale.
    Code(Vec<CharString>),
    /// The `_diffrn_attenuator_material` item. The variant holds the column values, one per loop row.
    ///
    /// Material from which the attenuator is made.
    Material(Vec<CharString>),
    /// The `_diffrn_attenuator_scale` item. The variant holds the column values, one per loop row.
    ///
    /// The scale factor applied when an intensity measurement is reduced by
    /// an attenuator identified by _diffrn_attenuator_code. The measured
    /// intensity must be multiplied by this scale to convert it to the same
    /// scale as unattenuated intensities.
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
