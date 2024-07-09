use chemrust_core::data::{lattice::CrystalModel, symmetry::SymmetryInfo};

use crate::data_dict::core_cif::{
    atom_site::chemrust_impl::basic_atom_site_data, audit::default_audit_data,
    cell::chemrust_impl::basic_cell_data, space_group::chemrust_impl::basic_space_group_data,
};

use super::{
    structures::{DataBlock, DataBlockHeading, DataBlockMember},
    CifDocument,
};

pub fn to_cif_document<T: CrystalModel + SymmetryInfo>(model: &T, data_name: &str) -> CifDocument {
    let datablock_members = [
        default_audit_data(),
        basic_space_group_data(model),
        basic_cell_data(model.get_cell_parameters()),
        [basic_atom_site_data(model.get_atom_data())].to_vec(),
    ]
    .into_iter()
    .flat_map(|items| {
        items
            .into_iter()
            .map(DataBlockMember::DataItems)
            .collect::<Vec<DataBlockMember>>()
    })
    .collect();
    let heading = DataBlockHeading::new(data_name.to_string());
    let data_block = DataBlock::from_heading_members((heading, datablock_members));
    CifDocument::new(None, Some(vec![data_block]))
}
