//! The `audit_contact_author` category of the typed document layer (roadmap D12).
//!
//! The `audit_contact_author` single-value struct. One `Option` field per registry item.
//!
//! Char items read leniently: the tier corpus records them as text
//! fields. The closed sets keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct AuditContactAuthor {
    pub address: Option<CharString>,
    pub email: Option<CharString>,
    pub fax: Option<CharString>,
    pub id: Option<CharString>,
    pub id_iucr: Option<CharString>,
    pub id_orcid: Option<CharString>,
    pub name: Option<CharString>,
    pub phone: Option<CharString>,
}

impl CifCategory for AuditContactAuthor {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.address {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_address".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.email {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_email".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.fax {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_fax".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_id".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.id_iucr {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_id_iucr".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.id_orcid {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_id_orcid".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_name".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.phone {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_contact_author_phone".to_string()),
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
                    "audit_contact_author_address" => {
                        match convert::read_char_text("audit_contact_author_address", value) {
                            Ok(v) => self.address = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_contact_author_email" => {
                        match convert::read_char_text("audit_contact_author_email", value) {
                            Ok(v) => self.email = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_contact_author_fax" => {
                        match convert::read_char_text("audit_contact_author_fax", value) {
                            Ok(v) => self.fax = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_contact_author_id" => {
                        match convert::read_char_text("audit_contact_author_id", value) {
                            Ok(v) => self.id = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_contact_author_id_iucr" => {
                        match convert::read_char_text("audit_contact_author_id_iucr", value) {
                            Ok(v) => self.id_iucr = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_contact_author_id_orcid" => {
                        match convert::read_char_text("audit_contact_author_id_orcid", value) {
                            Ok(v) => self.id_orcid = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_contact_author_name" => {
                        match convert::read_char_text("audit_contact_author_name", value) {
                            Ok(v) => self.name = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_contact_author_phone" => {
                        match convert::read_char_text("audit_contact_author_phone", value) {
                            Ok(v) => self.phone = v,
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
