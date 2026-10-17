//! The `space_group` category of the typed document layer (roadmap D12).
//!
//! The `space_group` category struct. One `Option` field per registry
//! item. `it_number` keeps the clamped `ITNumber` newtype. The
//! closed sets are `CrystalSystemCif`, `BravaisType`,
//! `CentringType`, and `ItCoordinateSystemCode`.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::space_group::{
    BravaisType, CentringType, CrystalSystemCif, ITNumber, ItCoordinateSystemCode,
};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Number, Numeric, Tag, UnquotedString, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `space_group` category struct. One field per registry item.
pub struct SpaceGroup {
    /// The `_space_group_Bravais_type` item. The field holds the single value.
    pub bravais_type: Option<BravaisType>,
    /// The `_space_group_centring_type` item. The field holds the single value.
    pub centring_type: Option<CentringType>,
    /// The `_space_group_crystal_system` item. The field holds the single value.
    ///
    /// The name of the system of geometric crystal classes of space groups
    /// (crystal system) to which the space group belongs. Note that
    /// rhombohedral space groups belong to the trigonal system.
    pub crystal_system: Option<CrystalSystemCif>,
    /// The `_space_group_id` item. An identifier of the space group,
    /// when the document carries no IT number.
    pub id: Option<Numeric>,
    /// The `_space_group_IT_number` item. The field holds the single value.
    ///
    /// The number as assigned in International Tables for Crystallography
    /// Vol. A, specifying the proper affine class (i.e. the
    /// orientation-preserving affine class) of space groups
    /// (crystallographic space-group type) to which the space group
    /// belongs. This number defines the space-group type but not the
    /// coordinate system in which it is expressed.
    pub it_number: Option<ITNumber>,
    /// The `_space_group_IT_coordinate_system_code` item. The field holds the single value.
    pub it_coordinate_system_code: Option<ItCoordinateSystemCode>,
    /// The `_space_group_Laue_class` item. The field holds the single value.
    pub laue_class: Option<CharString>,
    /// The `_space_group_multiplicity` item. The field holds the single value.
    pub multiplicity: Option<Numeric>,
    /// The `_space_group_name_H-M_alt` item. The field holds the single value.
    ///
    /// _space_group_name_H-M_alt allows any Hermann-Mauguin symbol to be
    /// given. The way in which this item is used is determined by the user
    /// and in general is not intended to be interpreted by computer. It
    /// may, for example, be used to give one of the extended
    /// Hermann-Mauguin symbols given in Table 4.3.2.1 of International
    /// Tables for Crystallography Vol. A (2002) or a Hermann-Mauguin symbol
    /// for a conventional or unconventional setting. Each component of the
    /// space-group name is separated by a space or an underscore. The use
    /// of a space is strongly recommended. The underscore is only retained
    /// because it was used in older files. It should not be used in new
    /// CIFs. Subscripts should appear without special symbols. Bars should
    /// be given as negative signs before the numbers to which they apply.
    /// The commonly used Hermann-Mauguin symbol determines the space- group
    /// type uniquely but a given space-group type may be described by more
    /// than one Hermann-Mauguin symbol. The space- group type is best
    /// described using _space_group_IT_number. The Hermann-Mauguin symbol
    /// may contain information on the choice of basis, but not on the
    /// choice of origin. To define the setting uniquely, use
    /// _space_group_name_Hall or list the symmetry operations.
    pub name_h_m_alt: Option<CharString>,
    /// The `_space_group_name_Hall` item. The field holds the single value.
    ///
    /// Space-group symbol defined by Hall. Each component of the
    /// space-group name is separated by a space or an underscore. The use
    /// of a space is strongly recommended. The underscore is only retained
    /// because it was used in older files. It should not be used in new
    /// CIFs. _space_group_name_Hall uniquely defines the space group and
    /// its reference to a particular coordinate system. Ref: Hall, S. R.
    /// (1981). Acta Cryst. A37, 517-525; erratum (1981), A37, 921. [See
    /// also International Tables for Crystallography, Vol. B (2001),
    /// Chapter 1.4, Appendix 1.4.2]
    pub name_hall: Option<CharString>,
    /// The `_space_group_name_H_M_alt_description` item. The field holds the single value.
    pub name_h_m_alt_description: Option<CharString>,
    /// The `_space_group_name_H_M_ref` item. The field holds the single value.
    pub name_h_m_ref: Option<CharString>,
    /// The `_space_group_name_Schoenflies` item. The field holds the single value.
    pub name_schoenflies: Option<CharString>,
    /// The `_space_group_Patterson_name_H_M` item. The field holds the single value.
    pub patterson_name_h_m: Option<CharString>,
    /// The `_space_group_point_group_H_M` item. The field holds the single value.
    pub point_group_h_m: Option<CharString>,
    /// The `_space_group_reference_setting` item. The field holds the single value.
    pub reference_setting: Option<CharString>,
    /// The `_space_group_transform_Pp_abc` item. The field holds the single value.
    pub transform_pp_abc: Option<CharString>,
    /// The `_space_group_transform_Qq_xyz` item. The field holds the single value.
    pub transform_qq_xyz: Option<CharString>,
    /// The `symmetry.space_group_name_H-M` item (the dotted shell
    /// spelling). The Hermann-Mauguin symbol of the space group.
    pub symmetry_name_h_m: Option<CharString>,
}

impl CifCategory for SpaceGroup {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.bravais_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_bravais_type".to_string()),
                    Value::CharString(CharString::Unquoted(UnquotedString::new(v.to_string()))),
                )),
            )));
        }
        if let Some(v) = &self.centring_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_centring_type".to_string()),
                    Value::CharString(CharString::Unquoted(UnquotedString::new(v.to_string()))),
                )),
            )));
        }
        if let Some(v) = &self.crystal_system {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_crystal_system".to_string()),
                    Value::CharString(CharString::Unquoted(UnquotedString::new(v.to_string()))),
                )),
            )));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_id".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.it_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_IT_number".to_string()),
                    Value::Numeric(Numeric::new(Number::from(u64::from(*v) as u32), None)),
                )),
            )));
        }
        if let Some(v) = &self.it_coordinate_system_code {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_it_coordinate_system_code".to_string()),
                    Value::CharString(CharString::Unquoted(UnquotedString::new(v.to_string()))),
                )),
            )));
        }
        if let Some(v) = &self.laue_class {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_laue_class".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.multiplicity {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_multiplicity".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.name_h_m_alt {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_name_H-M_alt".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_hall {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_name_Hall".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_h_m_alt_description {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_name_h_m_alt_description".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_h_m_ref {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_name_h_m_ref".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_schoenflies {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_name_schoenflies".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.patterson_name_h_m {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_patterson_name_h_m".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.point_group_h_m {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_point_group_h_m".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.reference_setting {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_reference_setting".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.transform_pp_abc {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_transform_pp_abc".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.transform_qq_xyz {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("space_group_transform_qq_xyz".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.symmetry_name_h_m {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("symmetry.space_group_name_h-m".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        out
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "space_group_bravais_type" => {
                        match convert::read_closed_set(
                            "space_group_bravais_type",
                            value,
                            BravaisType::parse,
                            "BravaisType",
                        ) {
                            Ok(v) => self.bravais_type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_centring_type" => {
                        match convert::read_closed_set(
                            "space_group_centring_type",
                            value,
                            CentringType::parse,
                            "CentringType",
                        ) {
                            Ok(v) => self.centring_type = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_crystal_system" => {
                        match convert::read_closed_set(
                            "space_group_crystal_system",
                            value,
                            |c| CrystalSystemCif::try_from(c).ok(),
                            "CrystalSystemCif",
                        ) {
                            Ok(v) => self.crystal_system = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_id" => match convert::read_numeric("space_group_id", value) {
                        Ok(v) => self.id = v,
                        Err(e) => errors.push(e),
                    },
                    "space_group_it_number" | "symmetry_int_tables_number" => {
                        match convert::read_it_number("space_group_it_number", value) {
                            Ok(v) => self.it_number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_it_coordinate_system_code" => {
                        match convert::read_closed_set(
                            "space_group_it_coordinate_system_code",
                            value,
                            ItCoordinateSystemCode::parse,
                            "ItCoordinateSystemCode",
                        ) {
                            Ok(v) => self.it_coordinate_system_code = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_laue_class" => {
                        match convert::read_char("space_group_laue_class", value) {
                            Ok(v) => self.laue_class = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_multiplicity" => {
                        match convert::read_numeric("space_group_multiplicity", value) {
                            Ok(v) => self.multiplicity = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_h-m_alt" => {
                        match convert::read_char("space_group_name_h-m_alt", value) {
                            Ok(v) => self.name_h_m_alt = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_hall" | "symmetry_space_group_name_hall" => {
                        match convert::read_char("space_group_name_hall", value) {
                            Ok(v) => self.name_hall = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_h_m_alt_description" => {
                        match convert::read_char("space_group_name_h_m_alt_description", value) {
                            Ok(v) => self.name_h_m_alt_description = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_h_m_ref" => {
                        match convert::read_char("space_group_name_h_m_ref", value) {
                            Ok(v) => self.name_h_m_ref = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_name_schoenflies" => {
                        match convert::read_char("space_group_name_schoenflies", value) {
                            Ok(v) => self.name_schoenflies = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_patterson_name_h_m" => {
                        match convert::read_char("space_group_patterson_name_h_m", value) {
                            Ok(v) => self.patterson_name_h_m = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_point_group_h_m" => {
                        match convert::read_char("space_group_point_group_h_m", value) {
                            Ok(v) => self.point_group_h_m = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_reference_setting" => {
                        match convert::read_char("space_group_reference_setting", value) {
                            Ok(v) => self.reference_setting = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_transform_pp_abc" => {
                        match convert::read_char("space_group_transform_pp_abc", value) {
                            Ok(v) => self.transform_pp_abc = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "space_group_transform_qq_xyz" => {
                        match convert::read_char("space_group_transform_qq_xyz", value) {
                            Ok(v) => self.transform_qq_xyz = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "symmetry.space_group_name_h-m" | "symmetry_space_group_name_h-m" => {
                        match convert::read_char("symmetry_space_group_name_h-m", value) {
                            Ok(v) => self.symmetry_name_h_m = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    _ => residue.push(m.clone()),
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}
