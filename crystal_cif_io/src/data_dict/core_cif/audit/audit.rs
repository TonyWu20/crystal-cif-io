//! The `audit` category of the typed document layer (roadmap D12).
//!
//! The `audit` single-value struct. One `Option` field per registry item.
//!
//! Char items read leniently: the tier corpus records them as text
//! fields. The closed sets keep the typed enum.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::audit::Schema;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
pub struct Audit {
    pub block_code: Option<CharString>,
    pub block_doi: Option<CharString>,
    pub creation_date: Option<CharString>,
    pub creation_method: Option<CharString>,
    pub schema: Option<Schema>,
    pub update_record: Option<CharString>,
}

impl CifCategory for Audit {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.block_code {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_block_code".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.block_doi {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_block_doi".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.creation_date {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_creation_date".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.creation_method {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_creation_method".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.schema {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_schema".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.update_record {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("audit_update_record".to_string()),
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
                    "audit_block_code" => {
                        match convert::read_char_text("audit_block_code", value) {
                            Ok(v) => self.block_code = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_block_doi" => match convert::read_char_text("audit_block_doi", value) {
                        Ok(v) => self.block_doi = v,
                        Err(e) => errors.push(e),
                    },
                    "audit_creation_date" => {
                        match convert::read_char_text("audit_creation_date", value) {
                            Ok(v) => self.creation_date = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_creation_method" => {
                        match convert::read_char_text("audit_creation_method", value) {
                            Ok(v) => self.creation_method = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_schema" => {
                        match convert::read_closed_set(
                            "audit_schema",
                            value,
                            Schema::parse,
                            "Schema",
                        ) {
                            Ok(v) => self.schema = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "audit_update_record" => {
                        match convert::read_char_text("audit_update_record", value) {
                            Ok(v) => self.update_record = v,
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
