mod extra_defn;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::extra_defn::ExtraDefn;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublManuscriptInclLoopItem {
    /// closed code set recorded in the dictionary _enumeration
    Extra_defn(Vec<ExtraDefn>),
    Extra_info(Vec<CharString>),
    Extra_item(Vec<CharString>),
}

impl CifTerm for PublManuscriptInclLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            PublManuscriptInclLoopItem::Extra_defn(_) => "extra_defn",
            PublManuscriptInclLoopItem::Extra_info(_) => "extra_info",
            PublManuscriptInclLoopItem::Extra_item(_) => "extra_item",
        };
        Tag::new(format!("publ_manuscript_incl_{suffix}"))
    }
}

impl LoopValueTerm for PublManuscriptInclLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            PublManuscriptInclLoopItem::Extra_defn(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            PublManuscriptInclLoopItem::Extra_info(s)
            | PublManuscriptInclLoopItem::Extra_item(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
