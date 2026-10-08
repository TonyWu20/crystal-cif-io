use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditItem {
    Block_code,
    Block_doi,
    Creation_date(CharString),
    Creation_method(CharString),
    Update_record,
}

impl CifTerm for AuditItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditItem::Block_code => "block_code",
            AuditItem::Block_doi => "block_doi",
            AuditItem::Creation_date(_) => "creation_date",
            AuditItem::Creation_method(_) => "creation_method",
            AuditItem::Update_record => "Update_record",
        };
        Tag::new(format!("audit_{}", suffix))
    }
}

impl SingleValueTerm for AuditItem {
    fn value(&self) -> Value {
        match self {
            AuditItem::Block_code => todo!(),
            AuditItem::Block_doi => todo!(),
            AuditItem::Creation_date(d) => Value::from(d.clone()),
            AuditItem::Creation_method(method) => Value::from(method.clone()),
            AuditItem::Update_record => todo!(),
        }
    }
}
