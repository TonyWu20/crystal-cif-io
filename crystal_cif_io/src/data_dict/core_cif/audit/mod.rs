mod audit;
mod schema;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::schema::Schema;
pub use audit::Audit;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AuditItem {
    Block_code(CharString),
    Block_doi(CharString),
    Creation_date(CharString),
    Creation_method(CharString),
    /// closed code set recorded in the dictionary _enumeration
    Schema(Schema),
    Update_record(CharString),
}

impl CifTerm for AuditItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AuditItem::Block_code(_) => "block_code",
            AuditItem::Block_doi(_) => "block_doi",
            AuditItem::Creation_date(_) => "creation_date",
            AuditItem::Creation_method(_) => "creation_method",
            AuditItem::Schema(_) => "schema",
            AuditItem::Update_record(_) => "update_record",
        };
        Tag::new(format!("audit_{}", suffix))
    }
}

impl SingleValueTerm for AuditItem {
    fn value(&self) -> Value {
        match self {
            AuditItem::Block_code(v)
            | AuditItem::Block_doi(v)
            | AuditItem::Creation_date(v)
            | AuditItem::Creation_method(v)
            | AuditItem::Update_record(v) => Value::from(v.clone()),
            AuditItem::Schema(v) => Value::from(*v),
        }
    }
}
