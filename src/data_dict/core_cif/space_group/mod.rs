use crate::data_dict::{DataLabel, LoopData, LoopDataLabel, SingleValueSection};

mod crystal_system;
mod it_number;

#[derive(Debug, Default, Clone, Copy)]
pub struct SpaceGroup;

pub type SpaceGroupLoopData = LoopData<SpaceGroupLoopItem>;
pub type SpaceGroupSection = SingleValueSection<SpaceGroupItem>;

pub use crystal_system::CrystalSystem;
pub use it_number::ITNumber;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SpaceGroupItem {
    Crystal_system(CrystalSystem),
    Id(usize),
    IT_number(ITNumber),
    Name_H_M_alt(String),
    Name_Hall(String),
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SpaceGroupLoopItem {
    Symop_id(usize),
    Symop_operation_xyz(String),
    Symop_sg_id(usize),
}

impl DataLabel for SpaceGroupItem {
    fn category_prefix(&self) -> String {
        "space_group".to_string()
    }

    fn tag(&self) -> String {
        match self {
            SpaceGroupItem::Crystal_system(_) => "crystal_system".to_string(),
            SpaceGroupItem::Id(_) => "id".to_string(),
            SpaceGroupItem::IT_number(_) => "IT_number".to_string(),
            SpaceGroupItem::Name_H_M_alt(_) => "name_H-M_alt".to_string(),
            SpaceGroupItem::Name_Hall(_) => "name_Hall".to_string(),
        }
    }

    fn value_string(&self) -> String {
        match self {
            SpaceGroupItem::Crystal_system(item) => format!("{item}"),
            SpaceGroupItem::Id(item) => format!("{item}"),
            SpaceGroupItem::IT_number(item) => format!("{item}"),
            SpaceGroupItem::Name_H_M_alt(item) | SpaceGroupItem::Name_Hall(item) => {
                item.to_string()
            }
        }
    }
}

impl DataLabel for SpaceGroupLoopItem {
    fn category_prefix(&self) -> String {
        "space_group".to_string()
    }

    fn tag(&self) -> String {
        match self {
            SpaceGroupLoopItem::Symop_id(_) => "symop_id".to_string(),
            SpaceGroupLoopItem::Symop_operation_xyz(_) => "symop_operation_xyz".to_string(),
            SpaceGroupLoopItem::Symop_sg_id(_) => "symop_sg_id".to_string(),
        }
    }

    fn value_string(&self) -> String {
        match self {
            SpaceGroupLoopItem::Symop_id(u) | SpaceGroupLoopItem::Symop_sg_id(u) => format!("{u}"),
            SpaceGroupLoopItem::Symop_operation_xyz(s) => s.to_string(),
        }
    }
}

impl LoopDataLabel for SpaceGroupLoopItem {}

#[cfg(test)]
mod test {
    use crate::data_dict::{
        core_cif::space_group::{SpaceGroupLoopData, SpaceGroupLoopItem},
        LoopDataEntry,
    };

    use super::{
        crystal_system::CrystalSystem, it_number::ITNumber, SpaceGroupItem, SpaceGroupSection,
    };

    #[test]
    fn space_group_data() {
        let space_group_section = SpaceGroupSection::init_builder()
            .add_entry(SpaceGroupItem::Id(1))
            .add_entry(SpaceGroupItem::Crystal_system(CrystalSystem::Triclinic))
            .add_entry(SpaceGroupItem::IT_number(ITNumber::new(1)))
            .finish();
        println!("{space_group_section}");
        let space_group_loop_data = SpaceGroupLoopData::init_builder()
            .add_entry(
                LoopDataEntry::init_builder()
                    .add_entry(SpaceGroupLoopItem::Symop_operation_xyz("x,y,z".to_string()))
                    .finish(),
            )
            .finish();
        println!("{space_group_loop_data}");
    }
}
