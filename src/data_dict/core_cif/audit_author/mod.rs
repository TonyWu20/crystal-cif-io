use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditAuthorTerms {
    Email(CharString),
    Fax(CharString),
    Id(CharString),
    Id_orcid(CharString),
    Phone(CharString),
}

impl CifTerm for AuditAuthorTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditAuthorTerms::Email(_) => "email",
            AuditAuthorTerms::Fax(_) => "fax",
            AuditAuthorTerms::Id(_) => "id",
            AuditAuthorTerms::Id_orcid(_) => "id_orcid",
            AuditAuthorTerms::Phone(_) => "phone",
        };
        Tag::new(format!("audit_author_{suffix}"))
    }
}

impl SingleValueTerm for AuditAuthorTerms {
    fn value(&self) -> Value {
        match self {
            AuditAuthorTerms::Email(s)
            | AuditAuthorTerms::Fax(s)
            | AuditAuthorTerms::Id(s)
            | AuditAuthorTerms::Id_orcid(s)
            | AuditAuthorTerms::Phone(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for AuditAuthorTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

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
