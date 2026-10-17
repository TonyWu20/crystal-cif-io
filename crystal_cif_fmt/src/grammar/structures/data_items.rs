use std::fmt::Display;

use crate::grammar::SyntacticUnit;

use crate::LoopColumn;

use super::{
    loop_struct::{LoopColumns, LoopUnit},
    tag_value_line::SingleLineData,
};

/// The data items of one data block: a single-value tag line
/// or a loop unit, in the order the file holds them.
#[derive(Debug, Clone)]
pub enum DataItems {
    /// One `tag value` line, single value form.
    SingleValue(SingleLineData),
    /// A `loop_` unit: the loop header columns and rows.
    MultiValues(LoopColumns),
}

/// Marker trait for the types that carry CIF data items.
pub trait CIFDataType {}

impl CIFDataType for SingleLineData {}
impl CIFDataType for LoopUnit {}

impl DataItems {
    /// The single-value line that carries the tag, for the
    /// single-value form. `None` for a loop unit or a tag miss.
    pub fn get_single_value_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<&SingleLineData> {
        match self {
            DataItems::SingleValue(tv) => {
                if tag.as_ref() == tv.tag().as_ref() {
                    Some(tv)
                } else {
                    None
                }
            }
            DataItems::MultiValues(_) => None,
        }
    }

    /// The loop column that carries the tag, for the loop form.
    /// The values are a copy. `None` for the single-value form
    /// or a tag miss.
    pub fn get_loop_column_values_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<LoopColumn> {
        if let DataItems::MultiValues(loop_unit) = self {
            loop_unit.find_loop_column_by_tag(tag).cloned()
        } else {
            None
        }
    }

    /// The single-value payload, when the item is single-value.
    pub fn as_single_value(&self) -> Option<&SingleLineData> {
        if let Self::SingleValue(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// The loop columns, when the item is a loop unit.
    pub fn as_multi_values(&self) -> Option<&LoopColumns> {
        if let Self::MultiValues(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl crate::grammar::SyntacticUnit for DataItems {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            DataItems::SingleValue(v) => format!("{v}"),
            DataItems::MultiValues(v) => format!("\n{}\n", LoopUnit::from(v)),
        }
    }
}

impl Display for DataItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
