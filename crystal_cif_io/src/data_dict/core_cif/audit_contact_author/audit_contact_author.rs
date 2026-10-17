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
/// The `audit_contact_author` category struct. One field per registry item.
pub struct AuditContactAuthor {
    /// The `_audit_contact_author_address` item. The field holds the single value.
    ///
    /// The mailing address of the author of the data block to whom
    /// correspondence should be addressed.
    pub address: Option<CharString>,
    /// The `_audit_contact_author_email` item. The field holds the single value.
    ///
    /// The electronic mail address of the author of the data block to whom
    /// correspondence should be addressed, in a form recognizable to
    /// international networks. The format of e-mail addresses is given in
    /// Section 3.4, Address Specification, of Internet Message Format, RFC
    /// 2822, P. Resnick (Editor), Network Standards Group, April 2001.
    pub email: Option<CharString>,
    /// The `_audit_contact_author_fax` item. The field holds the single value.
    ///
    /// The facsimile telephone number of the author of the data block to
    /// whom correspondence should be addressed. The recommended style
    /// starts with the international dialing prefix, followed by the area
    /// code in parentheses, followed by the local number with no spaces.
    pub fax: Option<CharString>,
    /// The `_audit_contact_author_id` item. The field holds the single value.
    pub id: Option<CharString>,
    /// The `_audit_contact_author_id_IUCr` item. The field holds the single value.
    pub id_iucr: Option<CharString>,
    /// The `_audit_contact_author_id_ORCID` item. The field holds the single value.
    pub id_orcid: Option<CharString>,
    /// The `_audit_contact_author_name` item. The field holds the single value.
    ///
    /// The name of the author of the data block to whom correspondence
    /// should be addressed. The family name(s), followed by a comma and
    /// including any dynastic components, precedes the first name(s) or
    /// initial(s).
    pub name: Option<CharString>,
    /// The `_audit_contact_author_phone` item. The field holds the single value.
    ///
    /// The telephone number of the author of the data block to whom
    /// correspondence should be addressed. The recommended style starts
    /// with the international dialing prefix, followed by the area code in
    /// parentheses, followed by the local number and any extension number
    /// prefixed by 'x', with no spaces.
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
