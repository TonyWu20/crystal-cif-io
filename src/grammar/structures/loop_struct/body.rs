use winnow::{
    combinator::{preceded, repeat},
    Parser,
};

use crate::grammar::{tags_values::Value, whitespace_comments::WhiteSpace, SyntacticUnit, Tag};

#[derive(Debug, Clone)]
pub struct LoopBody {
    values: Vec<Value>,
}

impl LoopBody {
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// Get nth column value, suitable for viewing data of the same tag in the loop.
    pub fn nth_column_values(&self, nth: usize, column_width: usize) -> Vec<Value> {
        self.values
            .chunks(column_width)
            .map(|chunk| chunk[nth].clone())
            .collect()
    }

    /// Create `LoopBody` from a uniform array of `Vec<Value>` columns.
    pub fn from_columns(columns: &[LoopColumn], column_length: usize) -> Self {
        Self::new(
            (0..column_length)
                .flat_map(|i| {
                    columns
                        .iter()
                        .map(|c| c.as_ref()[i].clone())
                        .collect::<Vec<Value>>()
                })
                .collect::<Vec<Value>>(),
        )
    }
}

impl SyntacticUnit for LoopBody {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (
            Value::parser,
            repeat(0.., preceded(WhiteSpace::parser, Value::parser)),
        )
            .map(|(first, following): (Value, Vec<Value>)| {
                let mut values = vec![first];
                values.extend(following);
                LoopBody::new(values)
            })
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.values()
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Debug, Clone)]
pub struct LoopColumn {
    tag: Tag,
    values: Vec<Value>,
}

impl AsRef<[Value]> for LoopColumn {
    fn as_ref(&self) -> &[Value] {
        <Vec<Value> as AsRef<[Value]>>::as_ref(&self.values)
    }
}

impl LoopColumn {
    pub fn new(tag: Tag, values: Vec<Value>) -> Self {
        Self { tag, values }
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }

    pub fn tag(&self) -> &str {
        self.tag.as_ref()
    }
}
