mod element;
mod publ_body;

mod format;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::element::Element;
pub use self::format::Format;
pub use self::publ_body::PublBody;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublBodyLoopItem {
    Contents(Vec<CharString>),
    /// closed code set recorded in the dictionary _enumeration
    Element(Vec<Element>),
    /// closed code set recorded in the dictionary _enumeration
    Format(Vec<Format>),
    Label(Vec<CharString>),
    Title(Vec<CharString>),
}

impl CifTerm for PublBodyLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            PublBodyLoopItem::Contents(_) => "contents",
            PublBodyLoopItem::Element(_) => "element",
            PublBodyLoopItem::Format(_) => "format",
            PublBodyLoopItem::Label(_) => "label",
            PublBodyLoopItem::Title(_) => "title",
        };
        Tag::new(format!("publ_body_{suffix}"))
    }
}

impl LoopValueTerm for PublBodyLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            PublBodyLoopItem::Contents(s)
            | PublBodyLoopItem::Label(s)
            | PublBodyLoopItem::Title(s) => s.iter().cloned().map(Value::from).collect(),
            PublBodyLoopItem::Element(v) => v.iter().cloned().map(Value::from).collect(),
            PublBodyLoopItem::Format(m) => m.iter().cloned().map(Value::from).collect(),
        }
    }
}
