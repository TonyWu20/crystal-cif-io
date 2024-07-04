use std::fmt::Display;

use data_dict::DataBlock;
pub mod data_dict;

mod grammar;
mod parsing;

pub struct CifFile {
    data_blocks: Vec<DataBlock>,
}

impl CifFile {
    pub fn new(data_blocks: Vec<DataBlock>) -> Self {
        Self { data_blocks }
    }
}

impl Display for CifFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = self
            .data_blocks
            .iter()
            .map(|block| format!("{block}"))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{content}")
    }
}
