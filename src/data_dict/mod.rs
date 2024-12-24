pub mod core_cif;

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
