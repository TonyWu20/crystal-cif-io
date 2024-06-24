#![allow(dead_code)]

use data_dict::DataBlock;
mod data_dict;

pub struct CifFile {
    data_blocks: Vec<DataBlock>,
}
