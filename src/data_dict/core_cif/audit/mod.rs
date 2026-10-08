use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditItem {
    Block_code(CharString),
    Block_doi(CharString),
    Creation_date(CharString),
    Creation_method(CharString),
    Update_record(CharString),
}

impl CifTerm for AuditItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditItem::Block_code(_) => "block_code",
            AuditItem::Block_doi(_) => "block_doi",
            AuditItem::Creation_date(_) => "creation_date",
            AuditItem::Creation_method(_) => "creation_method",
            AuditItem::Update_record(_) => "Update_record",
        };
        Tag::new(format!("audit_{}", suffix))
    }
}

impl SingleValueTerm for AuditItem {
    fn value(&self) -> Value {
        match self {
            AuditItem::Block_code(v)
            | AuditItem::Block_doi(v)
            | AuditItem::Creation_date(v)
            | AuditItem::Creation_method(v)
            | AuditItem::Update_record(v) => Value::from(v.clone()),
        }
    }
}
