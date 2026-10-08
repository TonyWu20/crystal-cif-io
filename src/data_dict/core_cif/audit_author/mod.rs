use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditAuthorLoopItem {
    Address(Vec<CharString>),
    Name(Vec<CharString>),
}

impl CifTerm for AuditAuthorLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditAuthorLoopItem::Address(_) => "address",
            AuditAuthorLoopItem::Name(_) => "name",
        };
        Tag::new(format!("audit_author_{suffix}"))
    }
}

impl LoopValueTerm for AuditAuthorLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AuditAuthorLoopItem::Address(s) | AuditAuthorLoopItem::Name(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
