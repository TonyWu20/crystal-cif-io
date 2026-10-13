use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm, SingleValueTerm},
    grammar::{CharString, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublAuthorTerms {
    Id(CharString),
    Id_audit(CharString),
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

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublAuthorLoopItem {
    Address(Vec<CharString>),
    Email(Vec<CharString>),
    Footnote(Vec<CharString>),
    Id_iucr(Vec<CharString>),
    Id_orcid(Vec<CharString>),
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
