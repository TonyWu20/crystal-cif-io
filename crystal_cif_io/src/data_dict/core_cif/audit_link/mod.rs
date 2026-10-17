mod audit_link;

pub use self::audit_link::AuditLink;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The loop-value terms of the `audit_link` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditLinkLoopItem {
    /// The `_audit_link_block_code` item. The variant holds the column values, one per loop row.
    ///
    /// The value of _audit_block_code associated with a data block in the
    /// current file related to the current data block. The special value
    /// '.' may be used to refer to the current data block for completeness.
    Block_code(Vec<CharString>),
    /// The `_audit_link_block_description` item. The variant holds the column values, one per loop row.
    ///
    /// A textual description of the relationship of the referenced data
    /// block to the current one.
    Block_description(Vec<CharString>),
}

impl CifTerm for AuditLinkLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditLinkLoopItem::Block_code(_) => "block_code",
            AuditLinkLoopItem::Block_description(_) => "block_description",
        };
        Tag::new(format!("audit_link_{suffix}"))
    }
}

impl LoopValueTerm for AuditLinkLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AuditLinkLoopItem::Block_code(s) | AuditLinkLoopItem::Block_description(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
