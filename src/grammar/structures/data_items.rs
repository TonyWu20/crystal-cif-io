use crate::{grammar::tags_values::Value, LoopColumn};
use std::fmt::Display;

use winnow::{combinator::alt, error::StrContext, Parser};

use crate::grammar::SyntacticUnit;

use super::{
    loop_struct::{LoopColumns, LoopUnit},
    tag_value_line::SingleLineData,
};

#[derive(Debug, Clone)]
pub enum DataItems {
    SingleValue(SingleLineData),
    MultiValues(LoopColumns),
}

pub trait CIFDataType {}

impl CIFDataType for SingleLineData {}
impl CIFDataType for LoopUnit {}

impl DataItems {
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

    pub fn get_loop_column_values_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<LoopColumn> {
        if let DataItems::MultiValues(loop_unit) = self {
            loop_unit.find_loop_column_by_tag(tag).cloned()
        } else {
            None
        }
    }

    pub fn as_single_value(&self) -> Option<&SingleLineData> {
        if let Self::SingleValue(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_multi_values(&self) -> Option<&LoopColumns> {
        if let Self::MultiValues(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl SyntacticUnit for DataItems {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            SingleLineData::parser
                .map(DataItems::SingleValue)
                .context(StrContext::Label("Single line data")),
            LoopUnit::parser
                .map(LoopColumns::from)
                .map(DataItems::MultiValues)
                .context(StrContext::Label("Loop")),
        ))
        .parse_next(input)
    }

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
