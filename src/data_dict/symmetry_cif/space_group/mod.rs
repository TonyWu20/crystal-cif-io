use crate::data_dict::DataCategory;

mod crystal_system;
mod it_number;

pub struct SpaceGroup;

impl DataCategory for SpaceGroup {
    fn category_prefix() -> String {
        "space_group".to_string()
    }
}
