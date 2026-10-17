//! The `geom` category of the typed document layer (roadmap D12).
//!
//! The `Geom` single-value struct: the special-list distance
//! increments, the distance cutoffs, and the special details.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `geom` category struct. One field per registry item.
pub struct Geom {
    /// The `_geom_bond_distance_incr` item. The field holds the single value.
    pub bond_distance_incr: Option<Numeric>,
    /// The `_geom_contact_distance_incr` item. The field holds the single value.
    pub contact_distance_incr: Option<Numeric>,
    /// The `_geom_min_bond_distance_cutoff` item. The field holds the single value.
    pub min_bond_distance_cutoff: Option<Numeric>,
    /// The `_geom_min_contact_distance_cutoff` item. The field holds the single value.
    pub min_contact_distance_cutoff: Option<Numeric>,
    /// The `_geom_special_details` item. The field holds the single value.
    ///
    /// The description of geometrical information not covered by the
    /// existing data names in the geometry categories, such as
    /// least-squares planes.
    pub special_details: Option<CharString>,
}

impl CifCategory for Geom {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.bond_distance_incr {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_bond_distance_incr".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.contact_distance_incr {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_contact_distance_incr".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.min_bond_distance_cutoff {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_min_bond_distance_cutoff".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.min_contact_distance_cutoff {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_min_contact_distance_cutoff".to_string()),
                    Value::Numeric(*v),
                )),
            )));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_special_details".to_string()),
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
                    "geom_bond_distance_incr" => {
                        match convert::read_numeric("geom_bond_distance_incr", value) {
                            Ok(v) => self.bond_distance_incr = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "geom_contact_distance_incr" => {
                        match convert::read_numeric("geom_contact_distance_incr", value) {
                            Ok(v) => self.contact_distance_incr = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "geom_min_bond_distance_cutoff" => {
                        match convert::read_numeric("geom_min_bond_distance_cutoff", value) {
                            Ok(v) => self.min_bond_distance_cutoff = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "geom_min_contact_distance_cutoff" => {
                        match convert::read_numeric("geom_min_contact_distance_cutoff", value) {
                            Ok(v) => self.min_contact_distance_cutoff = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "geom_special_details" => {
                        match convert::read_char_text("geom_special_details", value) {
                            Ok(v) => self.special_details = v,
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
