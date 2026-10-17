mod extra_defn;
mod publ_manuscript_incl;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::extra_defn::ExtraDefn;
pub use self::publ_manuscript_incl::PublManuscriptIncl;

/// The loop-value terms of the `publ_manuscript_incl` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PublManuscriptInclLoopItem {
    /// The `_publ_manuscript_incl_extra_defn` item. The variant holds the column values, one per loop row.
    ///
    /// Flags whether the corresponding data item marked for inclusion in a
    /// journal request list is a standard CIF definition or not.
    /// closed code set recorded in the dictionary _enumeration
    Extra_defn(Vec<ExtraDefn>),
    /// The `_publ_manuscript_incl_extra_info` item. The variant holds the column values, one per loop row.
    ///
    /// A short note indicating the reason why the author wishes the
    /// corresponding data item marked for inclusion in the journal request
    /// list to be published.
    Extra_info(Vec<CharString>),
    /// The `_publ_manuscript_incl_extra_item` item. The variant holds the column values, one per loop row.
    ///
    /// Specifies the inclusion of specific data into a manuscript which are
    /// not normally requested by the journal. The values of this item are
    /// the extra data names (which MUST be enclosed in single quotes) that
    /// will be added to the journal request list.
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
