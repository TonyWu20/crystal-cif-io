use winnow::{
    combinator::{preceded, repeat},
    Parser,
};

use crate::grammar::{tags_values::Value, whitespace_comments::WhiteSpace, SyntacticUnit};

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

    pub fn nth_column_values(&self, nth: usize, column_size: usize) -> Vec<Value> {
        self.values
            .chunks(column_size)
            .map(|chunk| chunk[nth].clone())
            .collect()
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
