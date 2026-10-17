mod audit_conform;

pub use self::audit_conform::{AuditConform, AuditConformRow};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

/// The single-value terms of the `audit_conform` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditConformTerms {
    /// The `_audit_conform_dict_doi` item. The variant holds the single value.
    Dict_doi(CharString),
}

impl CifTerm for AuditConformTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditConformTerms::Dict_doi(_) => "dict_doi",
        };
        Tag::new(format!("audit_conform_{suffix}"))
    }
}

impl SingleValueTerm for AuditConformTerms {
    fn value(&self) -> Value {
        match self {
            AuditConformTerms::Dict_doi(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for AuditConformTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `audit_conform` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditConformLoopItem {
    /// The `_audit_conform_dict_location` item. The variant holds the column values, one per loop row.
    ///
    /// A file name or uniform resource locator (URL) for the dictionary to
    /// which the current data block conforms.
    Dict_location(Vec<CharString>),
    /// The `_audit_conform_dict_name` item. The variant holds the column values, one per loop row.
    ///
    /// The string identifying the highest-level dictionary defining data
    /// names used in this file.
    Dict_name(Vec<CharString>),
    /// The `_audit_conform_dict_version` item. The variant holds the column values, one per loop row.
    ///
    /// The version number of the dictionary to which the current data block
    /// conforms.
    Dict_version(Vec<CharString>),
}

impl CifTerm for AuditConformLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditConformLoopItem::Dict_location(_) => "dict_location",
            AuditConformLoopItem::Dict_name(_) => "dict_name",
            AuditConformLoopItem::Dict_version(_) => "dict_version",
        };
        Tag::new(format!("audit_conform_{suffix}"))
    }
}

impl LoopValueTerm for AuditConformLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AuditConformLoopItem::Dict_location(s)
            | AuditConformLoopItem::Dict_name(s)
            | AuditConformLoopItem::Dict_version(s) => s.iter().cloned().map(Value::from).collect(),
        }
    }
}
