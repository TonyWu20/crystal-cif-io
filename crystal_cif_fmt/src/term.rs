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

pub trait CifTerm {
    fn tag(&self) -> Tag;
}

pub trait SingleValueTerm: CifTerm {
    fn value(&self) -> Value;
    fn to_single_value_data(&self) -> DataItems {
        DataItems::SingleValue(SingleLineData::from_tag_value((self.tag(), self.value())))
    }
}

pub trait LoopValueTerm: CifTerm {
    fn values(&self) -> Vec<Value>;
    fn to_loop_column(&self) -> LoopColumn {
        LoopColumn::new(self.tag(), self.values())
    }
}
