use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crate::grammar::{CharString, Tag, UnquotedString, UnsignedInteger, Value};

mod crystal_system;
mod it_number;

pub use crystal_system::CrystalSystemCif;
pub use it_number::ITNumber;

#[cfg(feature = "chemrust-core")]
pub mod chemrust_impl;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SpaceGroupItem {
    Crystal_system(CrystalSystemCif),
    Id(usize),
    IT_number(ITNumber),
    Name_H_M_alt(String),
    Name_Hall(String),
}

impl CifTerm for SpaceGroupItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupItem::Crystal_system(_) => "crystal_system",
            SpaceGroupItem::Id(_) => "id",
            SpaceGroupItem::IT_number(_) => "IT_number",
            SpaceGroupItem::Name_H_M_alt(_) => "name_H-M_alt",
            SpaceGroupItem::Name_Hall(_) => "name_Hall",
        };
        Tag::new(format!("space_group_{}", suffix))
    }
}

impl SingleValueTerm for SpaceGroupItem {
    fn value(&self) -> Value {
        let value_string = match self {
            SpaceGroupItem::Crystal_system(item) => format!("{item}"),
            SpaceGroupItem::Id(item) => format!("{item}"),
            SpaceGroupItem::IT_number(item) => format!("{item}"),
            SpaceGroupItem::Name_H_M_alt(item) | SpaceGroupItem::Name_Hall(item) => {
                item.to_string()
            }
        };
        Value::CharString(CharString::Unquoted(UnquotedString::new(value_string)))
    }
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SpaceGroupLoopItem {
    Symop_id(Vec<UnsignedInteger>),
    Symop_operation_xyz(Vec<CharString>),
    Symop_sg_id(Vec<UnsignedInteger>),
}

impl CifTerm for SpaceGroupLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            SpaceGroupLoopItem::Symop_id(_) => "symop_id",
            SpaceGroupLoopItem::Symop_operation_xyz(_) => "symop_operation_xyz",
            SpaceGroupLoopItem::Symop_sg_id(_) => "symop_sg_id",
        };
        Tag::new(format!("space_group_{}", suffix))
    }
}

impl LoopValueTerm for SpaceGroupLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            SpaceGroupLoopItem::Symop_sg_id(uis) | SpaceGroupLoopItem::Symop_id(uis) => {
                uis.iter().map(|&u| u.into()).collect()
            }
            SpaceGroupLoopItem::Symop_operation_xyz(ops) => {
                ops.iter().map(|op| op.clone().into()).collect()
            }
        }
    }
}
