mod diffrn_scale_group;
mod i_net;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::diffrn_scale_group::DiffrnScaleGroup;
pub use self::i_net::INet;

/// The loop-value terms of the `diffrn_scale_group` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnScaleGroupLoopItem {
    /// The `_diffrn_scale_group_I_net` item. The variant holds the column values, one per loop row.
    ///
    /// The scale for a specific measurement group which is to be multiplied
    /// with the net intensity to place all intensities in the
    /// _diffrn_refln_ or _refln_ list on a common scale.
    /// range constraint recorded in the dictionary _enumeration
    I_net(Vec<INet>),
    /// The `_diffrn_scale_group_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying a specific measurement group (e.g. for
    /// multi-film or multi-crystal data). The code must match a
    /// _diffrn_refln_scale_group_code in the reflection list.
    Code(Vec<CharString>),
}

impl CifTerm for DiffrnScaleGroupLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnScaleGroupLoopItem::I_net(_) => "i_net",
            DiffrnScaleGroupLoopItem::Code(_) => "code",
        };
        Tag::new(format!("diffrn_scale_group_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnScaleGroupLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnScaleGroupLoopItem::I_net(v) => v.iter().cloned().map(Value::from).collect(),
            DiffrnScaleGroupLoopItem::Code(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
