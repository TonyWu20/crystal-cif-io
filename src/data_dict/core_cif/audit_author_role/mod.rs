mod role;

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::role::Role;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditAuthorRoleTerms {
    Id(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Role(Role),
    Special_details(CharString),
}

impl CifTerm for AuditAuthorRoleTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditAuthorRoleTerms::Id(_) => "id",
            AuditAuthorRoleTerms::Role(_) => "role",
            AuditAuthorRoleTerms::Special_details(_) => "special_details",
        };
        Tag::new(format!("audit_author_role_{suffix}"))
    }
}

impl SingleValueTerm for AuditAuthorRoleTerms {
    fn value(&self) -> Value {
        match self {
            AuditAuthorRoleTerms::Id(s) | AuditAuthorRoleTerms::Special_details(s) => {
                Value::CharString(s.clone())
            }
            AuditAuthorRoleTerms::Role(v) => Value::from(*v),
        }
    }
}

impl Display for AuditAuthorRoleTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
