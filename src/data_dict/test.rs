use std::{fs::read_to_string, path::Path};

use castep_cell_io::CellParser;

use super::DataBlock;

#[test]
fn test_data_block() {
    let cell_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("SAC_GDY_V_Co.cell");
    let cell_content = read_to_string(cell_path).expect("Read error");
    let cell_doc = CellParser::from(&cell_content)
        .parse()
        .expect("Parse error");
    let data_block = DataBlock::from(&cell_doc);
    println!("{data_block}");
}
