mod element;
mod publ_body;

mod format;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::element::Element;
pub use self::format::Format;
pub use self::publ_body::PublBody;

/// The loop-value terms of the `publ_body` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublBodyLoopItem {
    /// The `_publ_body_contents` item. The variant holds the column values, one per loop row.
    ///
    /// A text section of a paper.
    Contents(Vec<CharString>),
    /// The `_publ_body_element` item. The variant holds the column values, one per loop row.
    ///
    /// The functional role of the associated text section.
    /// closed code set recorded in the dictionary _enumeration
    Element(Vec<Element>),
    /// The `_publ_body_format` item. The variant holds the column values, one per loop row.
    ///
    /// Code indicating the appropriate typesetting conventions for accented
    /// characters and special symbols in the text section.
    /// closed code set recorded in the dictionary _enumeration
    Format(Vec<Format>),
    /// The `_publ_body_label` item. The variant holds the column values, one per loop row.
    ///
    /// Code identifying the section of text. The combination of this with
    /// _publ_body_element must be unique.
    Label(Vec<CharString>),
    /// The `_publ_body_title` item. The variant holds the column values, one per loop row.
    ///
    /// Title of the associated section of text.
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
