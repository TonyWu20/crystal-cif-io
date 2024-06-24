use crate::data_dict::{DataLabel, SingleValueSection};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditItem {
    Block_code,
    Block_doi,
    Creation_date(String),
    Creation_method(String),
    Update_record,
}

pub type AuditSection = SingleValueSection<AuditItem>;

impl DataLabel for AuditItem {
    fn category_prefix(&self) -> String {
        "audit".to_string()
    }

    fn tag(&self) -> String {
        match self {
            AuditItem::Block_code => todo!(),
            AuditItem::Block_doi => todo!(),
            AuditItem::Creation_date(_) => "creation_date".to_string(),
            AuditItem::Creation_method(_) => "creation_method".to_string(),
            AuditItem::Update_record => todo!(),
        }
    }

    fn value_string(&self) -> String {
        match self {
            AuditItem::Block_code => todo!(),
            AuditItem::Block_doi => todo!(),
            AuditItem::Creation_date(d) => d.to_string(),
            AuditItem::Creation_method(s) => s.to_string(),
            AuditItem::Update_record => todo!(),
        }
    }
}
