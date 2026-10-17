use crate::{
    grammar::{tags_values::Value, SyntacticUnit, Tag},
    term::{CifTerm, LoopValueTerm},
};

/// The value rows of a loop, flattened in row-major order:
/// row by row, one value per loop tag per row.
#[derive(Debug, Clone)]
pub struct LoopBody {
    values: Vec<Value>,
}

impl LoopBody {
    /// Build the body from the flattened value rows.
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }

    /// The values, in row-major order.
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
    pub fn from_columns<T: LoopValueTerm>(columns: &[T], column_length: usize) -> Self {
        Self::new(
            (0..column_length)
                .flat_map(|i| {
                    columns
                        .iter()
                        .map(|c| c.values()[i].clone())
                        .collect::<Vec<Value>>()
                })
                .collect::<Vec<Value>>(),
        )
    }
}

impl SyntacticUnit for LoopBody {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        self.values()
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// One loop column: the loop tag and the column values, one
/// value per loop row.
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
    /// Build the column from its tag and values.
    pub fn new(tag: Tag, values: Vec<Value>) -> Self {
        Self { tag, values }
    }

    /// The column values, one per loop row.
    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// The loop tag.
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// The loop tag, mutable.
    pub fn tag_mut(&mut self) -> &mut Tag {
        &mut self.tag
    }

    /// The column values, mutable.
    pub fn values_mut(&mut self) -> &mut Vec<Value> {
        &mut self.values
    }
}

impl CifTerm for LoopColumn {
    fn tag(&self) -> Tag {
        self.tag.clone()
    }
}

impl LoopValueTerm for LoopColumn {
    fn values(&self) -> Vec<Value> {
        self.values().to_vec()
    }
}
