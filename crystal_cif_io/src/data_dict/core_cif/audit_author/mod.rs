mod audit_author;

pub use self::audit_author::{AuditAuthor, AuditAuthorRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `audit_author` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditAuthorTerms {
    /// The `_audit_author_email` item. The variant holds the single value.
    Email(CharString),
    /// The `_audit_author_fax` item. The variant holds the single value.
    Fax(CharString),
    /// The `_audit_author_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_audit_author_id_orcid` item. The variant holds the single value.
    Id_orcid(CharString),
    /// The `_audit_author_phone` item. The variant holds the single value.
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

/// The loop-value terms of the `audit_author` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditAuthorLoopItem {
    /// The `_audit_author_address` item. The variant holds the column values, one per loop row.
    ///
    /// The address of an author of this data block. If there are multiple
    /// authors, _audit_author_address is looped with _audit_author_name.
    Address(Vec<CharString>),
    /// The `_audit_author_name` item. The variant holds the column values, one per loop row.
    ///
    /// The name of an author of this data block. If there are multiple
    /// authors, _audit_author_name is looped with _audit_author_address.
    /// The family name(s), followed by a comma and including any dynastic
    /// components, precedes the first name(s) or initial(s).
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
