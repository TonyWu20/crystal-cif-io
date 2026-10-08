use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditConformLoopItem {
    Dict_location(Vec<CharString>),
    Dict_name(Vec<CharString>),
    Dict_version(Vec<CharString>),
}

impl CifTerm for AuditConformLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditConformLoopItem::Dict_location(_) => "dict_location",
            AuditConformLoopItem::Dict_name(_) => "dict_name",
            AuditConformLoopItem::Dict_version(_) => "dict_version",
        };
        Tag::new(format!("audit_conform_{suffix}"))
    }
}

impl LoopValueTerm for AuditConformLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AuditConformLoopItem::Dict_location(s)
            | AuditConformLoopItem::Dict_name(s)
            | AuditConformLoopItem::Dict_version(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
