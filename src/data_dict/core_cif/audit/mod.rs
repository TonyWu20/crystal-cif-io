use chrono::Utc;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, DataItems, SingleQuotedString, Tag, Value},
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
    fn category_prefix(&self) -> String {
        "audit".to_string()
    }

    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditItem::Block_code => "block_code",
            AuditItem::Block_doi => "block_doi",
            AuditItem::Creation_date(_) => "creation_date",
            AuditItem::Creation_method(_) => "creation_method",
            AuditItem::Update_record => "Update_record",
        };
        Tag::new(format!("{}_{}", self.category_prefix(), suffix))
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

pub(crate) fn default_audit_data() -> Vec<DataItems> {
    [
        AuditItem::Creation_date(
            SingleQuotedString::new(format!("{}", Utc::now().format("%Y-%m-%d"))).into(),
        ),
        AuditItem::Creation_method(
            SingleQuotedString::new("crystal-cif-io by TonyWu20".to_string()).into(),
        ),
    ]
    .map(|item| item.to_single_value_data())
    .to_vec()
}
