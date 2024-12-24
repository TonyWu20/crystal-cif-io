use chemrust_core::data::symmetry::SymmetryInfo;
use crystallographic_group::database::{LookUpSpaceGroup, DEFAULT_SPACE_GROUP_SYMBOLS};
use crystallographic_group::hall_symbols::HallSymbolNotation;

use crate::data_dict::{LoopValueTerm, SingleValueTerm};
use crate::grammar::{CharString, DataItems, LoopUnit, UnquotedString};

use super::{CrystalSystemCif, ITNumber, SpaceGroupItem, SpaceGroupLoopItem};
pub(crate) fn basic_space_group_data<T: SymmetryInfo>(model: &T) -> Vec<DataItems> {
    let space_group = DEFAULT_SPACE_GROUP_SYMBOLS
        .get_hm_full_notation((model.get_space_group_it_num() - 1) as usize)
        .expect("Invalid space group number");
    let symmetry_ops = SpaceGroupLoopItem::Symop_operation_xyz(
        HallSymbolNotation::try_from_str(space_group)
            .expect("Incorrect space group hall notation")
            .general_positions()
            .derive_full_sets()
            .iter()
            .flat_map(|v| {
                v.iter()
                    .map(|mat| mat.jones_faithful_repr())
                    .map(|repr| CharString::Unquoted(UnquotedString::new(repr)))
                    .collect::<Vec<CharString>>()
            })
            .collect::<Vec<CharString>>(),
    );
    let symmetry_ops_column = symmetry_ops.to_loop_column();
    let symmetry_ops_loop = LoopUnit::builder()
        .with_value_columns(vec![symmetry_ops_column])
        .build();
    let items = if model.make_symmetry() {
        let crystal_system = model.get_crystal_system().into();
        let it_number = model.get_space_group_it_num();
        [
            SpaceGroupItem::Crystal_system(crystal_system).to_single_value_data(),
            SpaceGroupItem::IT_number(ITNumber::new(it_number)).to_single_value_data(),
            DataItems::MultiValues(symmetry_ops_loop.into()),
        ]
    } else {
        [
            SpaceGroupItem::Crystal_system(CrystalSystemCif::Triclinic).to_single_value_data(),
            SpaceGroupItem::IT_number(ITNumber::new(1)).to_single_value_data(),
            DataItems::MultiValues(symmetry_ops_loop.into()),
        ]
    };
    items.to_vec()
}
