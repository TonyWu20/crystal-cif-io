mod i_net;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::i_net::INet;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnScaleGroupLoopItem {
    /// range constraint recorded in the dictionary _enumeration
    I_net(Vec<INet>),
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
