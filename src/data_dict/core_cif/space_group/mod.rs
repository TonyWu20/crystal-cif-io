use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crate::grammar::{CharString, Numeric, Tag, UnquotedString, UnsignedInteger, Value};

mod crystal_system;
mod it_number;

pub use crystal_system::CrystalSystemCif;
pub use it_number::ITNumber;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SpaceGroupItem {
    Crystal_system(CrystalSystemCif),
    Id(usize),
    IT_number(ITNumber),
    Name_H_M_alt(String),
    Name_Hall(String),
    /// The closed code set is recorded in the shell dictionary;
    /// the plain text form is kept.
    Bravais_type(CharString),
    /// The closed code set is recorded in the shell dictionary;
    /// the plain text form is kept.
    Centring_type(CharString),
    /// The closed code set is recorded in the shell dictionary;
    /// the plain text form is kept.
    It_coordinate_system_code(CharString),
    /// The closed code set is recorded in the shell dictionary;
    /// the plain text form is kept.
    Laue_class(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `1:192`); the plain numeric form is kept.
    Multiplicity(Numeric),
    Name_h_m_alt_description(CharString),
    Name_h_m_ref(CharString),
    Name_schoenflies(CharString),
    Patterson_name_h_m(CharString),
    Point_group_h_m(CharString),
    Reference_setting(CharString),
    /// The dotted item name is recorded as written in the shell
    /// dictionary; the tag keeps the dot.
    Symmetry_name_h_m(CharString),
    Transform_pp_abc(CharString),
    Transform_qq_xyz(CharString),
}

impl CifTerm for SpaceGroupItem {
    fn tag(&self) -> Tag {
        let name = match self {
            SpaceGroupItem::Bravais_type(_) => "space_group_bravais_type",
            SpaceGroupItem::Centring_type(_) => "space_group_centring_type",
            SpaceGroupItem::Crystal_system(_) => "space_group_crystal_system",
            SpaceGroupItem::Id(_) => "space_group_id",
            SpaceGroupItem::IT_number(_) => "space_group_IT_number",
            SpaceGroupItem::It_coordinate_system_code(_) => "space_group_it_coordinate_system_code",
            SpaceGroupItem::Laue_class(_) => "space_group_laue_class",
            SpaceGroupItem::Multiplicity(_) => "space_group_multiplicity",
            SpaceGroupItem::Name_H_M_alt(_) => "space_group_name_H-M_alt",
            SpaceGroupItem::Name_Hall(_) => "space_group_name_Hall",
            SpaceGroupItem::Name_h_m_alt_description(_) => "space_group_name_h_m_alt_description",
            SpaceGroupItem::Name_h_m_ref(_) => "space_group_name_h_m_ref",
            SpaceGroupItem::Name_schoenflies(_) => "space_group_name_schoenflies",
            SpaceGroupItem::Patterson_name_h_m(_) => "space_group_patterson_name_h_m",
            SpaceGroupItem::Point_group_h_m(_) => "space_group_point_group_h_m",
            SpaceGroupItem::Reference_setting(_) => "space_group_reference_setting",
            SpaceGroupItem::Symmetry_name_h_m(_) => "symmetry.space_group_name_h-m",
            SpaceGroupItem::Transform_pp_abc(_) => "space_group_transform_pp_abc",
            SpaceGroupItem::Transform_qq_xyz(_) => "space_group_transform_qq_xyz",
        };
        Tag::new(name.to_string())
    }
}

impl SingleValueTerm for SpaceGroupItem {
    fn value(&self) -> Value {
        match self {
            SpaceGroupItem::Multiplicity(n) => Value::Numeric(*n),
            _ => {
                let value_string = match self {
                    SpaceGroupItem::Crystal_system(item) => format!("{item}"),
                    SpaceGroupItem::Multiplicity(_) => {
                        unreachable!("the multiplicity arm returns above")
                    }
                    SpaceGroupItem::Id(item) => format!("{item}"),
                    SpaceGroupItem::IT_number(item) => format!("{item}"),
                    SpaceGroupItem::Name_H_M_alt(item) | SpaceGroupItem::Name_Hall(item) => {
                        item.to_string()
                    }
                    SpaceGroupItem::Bravais_type(item)
                    | SpaceGroupItem::Centring_type(item)
                    | SpaceGroupItem::It_coordinate_system_code(item)
                    | SpaceGroupItem::Laue_class(item)
                    | SpaceGroupItem::Name_h_m_alt_description(item)
                    | SpaceGroupItem::Name_h_m_ref(item)
                    | SpaceGroupItem::Name_schoenflies(item)
                    | SpaceGroupItem::Patterson_name_h_m(item)
                    | SpaceGroupItem::Point_group_h_m(item)
                    | SpaceGroupItem::Reference_setting(item)
                    | SpaceGroupItem::Symmetry_name_h_m(item)
                    | SpaceGroupItem::Transform_pp_abc(item)
                    | SpaceGroupItem::Transform_qq_xyz(item) => item.content().to_string(),
                };
                Value::CharString(CharString::Unquoted(UnquotedString::new(value_string)))
            }
        }
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
