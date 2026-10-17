//! The term materialization traits (roadmap D13).
//!
//! A term names a dictionary tag and materializes into the IR
//! value types. The data-dictionary layer of the
//! `crystal-cif-io` crate implements the traits on its per-
//! category term enums. The parser types (`SingleLineData`,
//! `LoopColumn`) implement them inside this crate. The traits
//! sit in the parser crate so the IR can carry them without a
//! dependency on the data layer.

use crate::grammar::{DataItems, LoopColumn, SingleLineData, Tag, Value};

/// A CIF term: one dictionary tag and its materialized value
/// form.
pub trait CifTerm {
    /// The tag of the term.
    fn tag(&self) -> Tag;
}

/// A single-value term: the tag and one value.
pub trait SingleValueTerm: CifTerm {
    /// The single value of the term.
    fn value(&self) -> Value;
    /// The term as a single-value data item.
    fn to_single_value_data(&self) -> DataItems {
        DataItems::SingleValue(SingleLineData::from_tag_value((self.tag(), self.value())))
    }
}

/// A loop-value term: the loop tag and one column of values,
/// one value per loop row.
pub trait LoopValueTerm: CifTerm {
    /// The column values, one per loop row.
    fn values(&self) -> Vec<Value>;
    /// The term as a loop column.
    fn to_loop_column(&self) -> LoopColumn {
        LoopColumn::new(self.tag(), self.values())
    }
}
