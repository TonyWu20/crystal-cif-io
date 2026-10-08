use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

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
