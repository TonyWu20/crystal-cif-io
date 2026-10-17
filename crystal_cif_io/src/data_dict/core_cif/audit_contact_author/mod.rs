mod audit_contact_author;

pub use self::audit_contact_author::AuditContactAuthor;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `audit_contact_author` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditContactAuthorTerms {
    /// The `_audit_contact_author_address` item. The variant holds the single value.
    ///
    /// The mailing address of the author of the data block to whom
    /// correspondence should be addressed.
    Address(CharString),
    /// The `_audit_contact_author_email` item. The variant holds the single value.
    ///
    /// The electronic mail address of the author of the data block to whom
    /// correspondence should be addressed, in a form recognizable to
    /// international networks. The format of e-mail addresses is given in
    /// Section 3.4, Address Specification, of Internet Message Format, RFC
    /// 2822, P. Resnick (Editor), Network Standards Group, April 2001.
    Email(CharString),
    /// The `_audit_contact_author_fax` item. The variant holds the single value.
    ///
    /// The facsimile telephone number of the author of the data block to
    /// whom correspondence should be addressed. The recommended style
    /// starts with the international dialing prefix, followed by the area
    /// code in parentheses, followed by the local number with no spaces.
    Fax(CharString),
    /// The `_audit_contact_author_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_audit_contact_author_id_iucr` item. The variant holds the single value.
    Id_iucr(CharString),
    /// The `_audit_contact_author_id_orcid` item. The variant holds the single value.
    Id_orcid(CharString),
    /// The `_audit_contact_author_name` item. The variant holds the single value.
    ///
    /// The name of the author of the data block to whom correspondence
    /// should be addressed. The family name(s), followed by a comma and
    /// including any dynastic components, precedes the first name(s) or
    /// initial(s).
    Name(CharString),
    /// The `_audit_contact_author_phone` item. The variant holds the single value.
    ///
    /// The telephone number of the author of the data block to whom
    /// correspondence should be addressed. The recommended style starts
    /// with the international dialing prefix, followed by the area code in
    /// parentheses, followed by the local number and any extension number
    /// prefixed by 'x', with no spaces.
    Phone(CharString),
}

impl CifTerm for AuditContactAuthorTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditContactAuthorTerms::Address(_) => "address",
            AuditContactAuthorTerms::Email(_) => "email",
            AuditContactAuthorTerms::Fax(_) => "fax",
            AuditContactAuthorTerms::Id(_) => "id",
            AuditContactAuthorTerms::Id_iucr(_) => "id_iucr",
            AuditContactAuthorTerms::Id_orcid(_) => "id_orcid",
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
            | AuditContactAuthorTerms::Id(s)
            | AuditContactAuthorTerms::Id_iucr(s)
            | AuditContactAuthorTerms::Id_orcid(s)
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
