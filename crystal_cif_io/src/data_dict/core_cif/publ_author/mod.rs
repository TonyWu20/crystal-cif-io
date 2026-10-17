mod publ_author;

pub use self::publ_author::{PublAuthor, PublAuthorRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `publ_author` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublAuthorTerms {
    /// The `_publ_author_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_publ_author_id_audit` item. The variant holds the single value.
    Id_audit(CharString),
    /// The `_publ_author_phone` item. The variant holds the single value.
    Phone(CharString),
}

impl CifTerm for PublAuthorTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            PublAuthorTerms::Id(_) => "id",
            PublAuthorTerms::Id_audit(_) => "id_audit",
            PublAuthorTerms::Phone(_) => "phone",
        };
        Tag::new(format!("publ_author_{suffix}"))
    }
}

impl SingleValueTerm for PublAuthorTerms {
    fn value(&self) -> Value {
        match self {
            PublAuthorTerms::Id(s) | PublAuthorTerms::Id_audit(s) | PublAuthorTerms::Phone(s) => {
                Value::CharString(s.clone())
            }
        }
    }
}

impl Display for PublAuthorTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `publ_author` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublAuthorLoopItem {
    /// The `_publ_author_address` item. The variant holds the column values, one per loop row.
    ///
    /// The address of a publication author. If there is more than one
    /// author, this will be looped with _publ_author_name.
    Address(Vec<CharString>),
    /// The `_publ_author_email` item. The variant holds the column values, one per loop row.
    ///
    /// The e-mail address of a publication author. If there is more than
    /// one author, this will be looped with _publ_author_name. The format
    /// of e-mail addresses is given in Section 3.4, Address Specification,
    /// of Internet Message Format, RFC 2822, P. Resnick (Editor), Network
    /// Standards Group, April 2001.
    Email(Vec<CharString>),
    /// The `_publ_author_footnote` item. The variant holds the column values, one per loop row.
    ///
    /// A footnote accompanying an author's name in the list of authors of a
    /// paper. Typically indicates sabbatical address, additional
    /// affiliations or date of decease.
    Footnote(Vec<CharString>),
    /// The `_publ_author_id_iucr` item. The variant holds the column values, one per loop row.
    ///
    /// Identifier in the IUCr contact database of a publication author.
    /// This identifier may be available from the World Directory of
    /// Crystallographers (`http://wdc.iucr.org`).
    Id_iucr(Vec<CharString>),
    /// The `_publ_author_id_orcid` item. The variant holds the column values, one per loop row.
    ///
    /// Identifier in the ORCID Registry of a publication author. ORCID is
    /// an open, non-profit, community-driven service to provide a registry
    /// of unique researcher identifiers (`http://orcid.org`).
    Id_orcid(Vec<CharString>),
    /// The `_publ_author_name` item. The variant holds the column values, one per loop row.
    ///
    /// The name of a publication author. If there are multiple authors,
    /// this will be looped with _publ_author_address. The family name(s),
    /// followed by a comma and including any dynastic components, precedes
    /// the first names or initials.
    Name(Vec<CharString>),
}

impl CifTerm for PublAuthorLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            PublAuthorLoopItem::Address(_) => "address",
            PublAuthorLoopItem::Email(_) => "email",
            PublAuthorLoopItem::Footnote(_) => "footnote",
            PublAuthorLoopItem::Id_iucr(_) => "id_iucr",
            PublAuthorLoopItem::Id_orcid(_) => "id_orcid",
            PublAuthorLoopItem::Name(_) => "name",
        };
        Tag::new(format!("publ_author_{suffix}"))
    }
}

impl LoopValueTerm for PublAuthorLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            PublAuthorLoopItem::Address(s)
            | PublAuthorLoopItem::Email(s)
            | PublAuthorLoopItem::Footnote(s)
            | PublAuthorLoopItem::Id_iucr(s)
            | PublAuthorLoopItem::Id_orcid(s)
            | PublAuthorLoopItem::Name(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
