use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditLinkLoopItem {
    Block_code(Vec<CharString>),
    Block_description(Vec<CharString>),
}

impl CifTerm for AuditLinkLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditLinkLoopItem::Block_code(_) => "block_code",
            AuditLinkLoopItem::Block_description(_) => "block_description",
        };
        Tag::new(format!("audit_link_{suffix}"))
    }
}

impl LoopValueTerm for AuditLinkLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AuditLinkLoopItem::Block_code(s) | AuditLinkLoopItem::Block_description(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
