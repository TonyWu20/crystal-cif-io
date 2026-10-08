use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditContactAuthorTerms {
    Address(CharString),
    Email(CharString),
    Fax(CharString),
    Name(CharString),
    Phone(CharString),
}

impl CifTerm for AuditContactAuthorTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditContactAuthorTerms::Address(_) => "address",
            AuditContactAuthorTerms::Email(_) => "email",
            AuditContactAuthorTerms::Fax(_) => "fax",
            AuditContactAuthorTerms::Name(_) => "name",
            AuditContactAuthorTerms::Phone(_) => "phone",
        };
        Tag::new(format!("audit_contact_author_{suffix}"))
    }
}

impl SingleValueTerm for AuditContactAuthorTerms {
    fn value(&self) -> Value {
        match self {
            AuditContactAuthorTerms::Address(s)
            | AuditContactAuthorTerms::Email(s)
            | AuditContactAuthorTerms::Fax(s)
            | AuditContactAuthorTerms::Name(s)
            | AuditContactAuthorTerms::Phone(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for AuditContactAuthorTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
