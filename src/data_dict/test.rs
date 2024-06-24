use std::{fs::read_to_string, path::Path};

use castep_cell_io::CellParser;

use super::{
    core_cif::{
        atom_site::LoopAtomSiteData,
        space_group::{CrystalSystem, ITNumber, SpaceGroupItem, SpaceGroupSection},
    },
    DataBlock,
};
use crate::data_dict::{
    core_cif::{
        cell::CellDataSection,
        space_group::{SpaceGroupLoopData, SpaceGroupLoopItem},
    },
    CifData, LoopDataEntry,
};

#[test]
fn test_data_block() {
    let space_group_section = SpaceGroupSection::init_builder()
        .add_entry(SpaceGroupItem::Crystal_system(CrystalSystem::Triclinic))
        .add_entry(SpaceGroupItem::IT_number(ITNumber::new(1)))
        .finish();
    let mut data_block = DataBlock::init("test".to_string());
    data_block.add_section(CifData::SpaceGroup(space_group_section));
    data_block.add_section(CifData::SpaceGroupLoop(
        SpaceGroupLoopData::init_builder()
            .add_entry(
                LoopDataEntry::init_builder()
                    .add_entry(SpaceGroupLoopItem::Symop_operation_xyz("x,y,z".to_string()))
                    .finish(),
            )
            .finish(),
    ));
    let cell_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("SAC_GDY_V_Co.cell");
    let cell_content = read_to_string(cell_path).expect("Read error");
    let cell_doc = CellParser::from(&cell_content)
        .parse()
        .expect("Parse error");
    data_block.add_section(CifData::CellData(CellDataSection::from(
        cell_doc.model_description().lattice_block(),
    )));
    let loop_atoms = LoopAtomSiteData::from(cell_doc.model_description().ionic_pos_block());
    data_block.add_section(CifData::AtomSiteLoop(loop_atoms));
    println!("{data_block}");
}
