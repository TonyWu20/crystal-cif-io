use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, UnquotedString, UnsignedInteger, Value};

mod bravais_type;
mod centring_type;
mod crystal_system;
mod it_coordinate_system_code;
mod it_number;
mod space_group;

pub use bravais_type::BravaisType;
pub use centring_type::CentringType;
pub use crystal_system::CrystalSystemCif;
pub use it_coordinate_system_code::ItCoordinateSystemCode;
pub use it_number::ITNumber;
pub use space_group::SpaceGroup;

/// The single-value terms of the `space_group` category: one
/// variant per registry single-value item, in item order.
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SpaceGroupItem {
    /// The `space_group_crystal_system` item. The variant holds the single value.
    Crystal_system(CrystalSystemCif),
    /// The `space_group_id` item. The variant holds the single value.
    Id(usize),
    /// The `space_group_IT_number` item. The variant holds the single value.
    IT_number(ITNumber),
    /// The `space_group_name_H-M_alt` item. The variant holds the single value.
    Name_H_M_alt(String),
    /// The `space_group_name_Hall` item. The variant holds the single value.
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
    /// The `space_group_name_h_m_alt_description` item. The variant holds the single value.
    Name_h_m_alt_description(CharString),
    /// The `space_group_name_h_m_ref` item. The variant holds the single value.
    Name_h_m_ref(CharString),
    /// The `space_group_name_schoenflies` item. The variant holds the single value.
    Name_schoenflies(CharString),
    /// The `space_group_patterson_name_h_m` item. The variant holds the single value.
    Patterson_name_h_m(CharString),
    /// The `space_group_point_group_h_m` item. The variant holds the single value.
    Point_group_h_m(CharString),
    /// The `space_group_reference_setting` item. The variant holds the single value.
    Reference_setting(CharString),
    /// The dotted item name is recorded as written in the shell
    /// dictionary; the tag keeps the dot.
    Symmetry_name_h_m(CharString),
    /// The `space_group_transform_pp_abc` item. The variant holds the single value.
    Transform_pp_abc(CharString),
    /// The `space_group_transform_qq_xyz` item. The variant holds the single value.
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
/// The loop-value terms of the `space_group` category: one variant per

/// registry loop-value item, in item order.
pub enum SpaceGroupLoopItem {
    /// The `_space_group_symop_id` item. The variant holds the column values, one per loop row.
    ///
    /// An arbitrary identifier that uniquely labels each symmetry operation
    /// in the list. In order for the defaults to work correctly, the
    /// identity operation should have _space_group_symop_id or
    /// _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    Symop_id(Vec<UnsignedInteger>),
    /// The `_space_group_symop_operation_xyz` item. The variant holds the column values, one per loop row.
    ///
    /// A parsable string giving one of the symmetry operations of the space
    /// group in algebraic form. If W is a matrix representation of the
    /// rotational part of the symmetry operation defined by the positions
    /// and signs of x, y and z, and w is a column of translations defined
    /// by fractions, an equivalent position X' is generated from a given
    /// position X by the equation X' = WX + w (Note: X is used to represent
    /// bold_italics_x in International Tables for Crystallography Vol. A,
    /// Part 5) When a list of symmetry operations is given, it must contain
    /// a complete set of coordinate representatives which generates all the
    /// operations of the space group by the addition of all primitive
    /// translations of the space group. Such representatives are to be
    /// found as the coordinates of the general-equivalent position in
    /// International Tables for Crystallography Vol. A (2002), to which it
    /// is necessary to add any centring translations shown above the
    /// general-equivalent position. That is to say, it is necessary to list
    /// explicitly all the symmetry operations required to generate all the
    /// atoms in the unit cell defined by the setting used. In order for the
    /// defaults to work correctly, the identity operation should have
    /// _space_group_symop_id or _symmetry_equiv_pos_site_id set to 1, and
    /// _space_group_symop_operation_xyz or _symmetry_equiv_pos_as_xyz set
    /// to x,y,z; i.e. the operation labelled 1 should be the identity
    /// operation.
    Symop_operation_xyz(Vec<CharString>),
    /// The `symop_sg_id` item. The variant holds the column values, one per loop row.
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
