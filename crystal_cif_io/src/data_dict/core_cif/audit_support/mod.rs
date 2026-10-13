mod award_type;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::award_type::AwardType;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditSupportTerms {
    Award_number(CharString),
    Award_recipient(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Award_type(AwardType),
    Funding_organization(CharString),
    Funding_organization_doi(CharString),
    Funding_organization_url(CharString),
    Id(CharString),
    Special_details(CharString),
}

impl CifTerm for AuditSupportTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditSupportTerms::Award_number(_) => "award_number",
            AuditSupportTerms::Award_recipient(_) => "award_recipient",
            AuditSupportTerms::Award_type(_) => "award_type",
            AuditSupportTerms::Funding_organization(_) => "funding_organization",
            AuditSupportTerms::Funding_organization_doi(_) => "funding_organization_doi",
            AuditSupportTerms::Funding_organization_url(_) => "funding_organization_url",
            AuditSupportTerms::Id(_) => "id",
            AuditSupportTerms::Special_details(_) => "special_details",
        };
        Tag::new(format!("audit_support_{suffix}"))
    }
}

impl SingleValueTerm for AuditSupportTerms {
    fn value(&self) -> Value {
        match self {
            AuditSupportTerms::Award_number(s)
            | AuditSupportTerms::Award_recipient(s)
            | AuditSupportTerms::Funding_organization(s)
            | AuditSupportTerms::Funding_organization_doi(s)
            | AuditSupportTerms::Funding_organization_url(s)
            | AuditSupportTerms::Id(s)
            | AuditSupportTerms::Special_details(s) => Value::CharString(s.clone()),
            AuditSupportTerms::Award_type(v) => Value::from(*v),
        }
    }
}

impl Display for AuditSupportTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
