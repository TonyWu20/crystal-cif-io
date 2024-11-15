use std::str::FromStr;

use castep_periodic_table::element::ElementSymbol;
use chemrust_core::data::{
    atom::CoreAtomData, geom::coordinates::CoordData, lattice::{CellConstants, CrystalModel, UnitCellParameters}, symmetry::SymmetryInfo,
};
use nalgebra::Point3;

use crate::{
    data_dict::
        core_cif::{
            atom_site::chemrust_impl::basic_atom_site_data, audit::default_audit_data,
            cell::chemrust_impl::basic_cell_data,
            space_group::chemrust_impl::basic_space_group_data,
        },
    LoopColumn,
};

use super::{
    structures::{DataBlock, DataBlockHeading, DataBlockMember},
    CifDocument, SyntacticUnit,
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

impl UnitCellParameters for DataBlock {
    fn lattice_bases(&self) -> nalgebra::Matrix3<f64> {
        let length_a = self["cell_length_a"].as_single_value().and_then(|v| v.value().as_numeric())
            .and_then(|n| n.number().as_float().map(|&float| f64::from(float)))
            .expect("float to f64");
        let length_b = self["cell_length_b"].as_single_value().and_then(|v| v.value().as_numeric())
            .and_then(|n| n.number().as_float().map(|&float| f64::from(float)))
            .expect("float to f64");
        let length_c = self["cell_length_c"].as_single_value().and_then(|v| v.value().as_numeric())
            .and_then(|n| n.number().as_float().map(|&float| f64::from(float)))
            .expect("float to f64");
        let alpha = self["cell_angle_alpha"].as_single_value().and_then(|v| v.value().as_numeric())
            .and_then(|n| n.number().as_float().map(|&float| f64::from(float)))
            .expect("float to f64");
        let beta = self["cell_angle_beta"].as_single_value().and_then(|v| v.value().as_numeric())
            .and_then(|n| n.number().as_float().map(|&float| f64::from(float)))
            .expect("float to f64");
        let gamma = self["cell_angle_gamma"].as_single_value().and_then(|v| v.value().as_numeric())
            .and_then(|n| n.number().as_float().map(|&float| f64::from(float)))
            .expect("float to f64");
        let cell_constants = CellConstants::new(length_a, length_b, length_c, alpha, beta, gamma);
        cell_constants.lattice_bases()
    }
}

impl CoreAtomData for DataBlock {
    fn indices_repr(&self) -> Vec<usize> {
        let atom_sites = &self["atom_site_label"].as_multi_values().expect("this data block does not have atom sites data");
        let labels: &LoopColumn = &atom_sites["atom_site_label"];
        let len = labels.values().len();
        (0..len).collect()
    }

    fn symbols_repr(&self) -> Vec<ElementSymbol> {
        let atom_sites = &self["atom_site_label"].as_multi_values().expect("this data block does not have atom sites data");
            let symbols = &atom_sites["atom_site_type_symbol"];
            symbols
                .values()
                .iter()
                .map(|value| {
                    value
                        .as_char_string()
                        .map(|char_string| {
                            
                            ElementSymbol::from_str(char_string.as_ref()).expect("Single element symbol from periodic table. Multi-element symbol representation is not supported.")
                        })
                        .expect("the element symbol value should be <CharString>")
                })
                .collect()
    }

    fn coords_repr(&self) -> Vec<CoordData> {
        let atom_sites = &self["atom_site_label"].as_multi_values().expect("this data block does not have atom sites data");
        if atom_sites.find_loop_column_by_tag("atom_site_fract_x").is_some() &&
        atom_sites.find_loop_column_by_tag("atom_site_fract_y").is_some() &&
        atom_sites.find_loop_column_by_tag("atom_site_fract_z").is_some() {
            let fract_x = &atom_sites["atom_site_fract_x"];
            let fract_y =  &atom_sites["atom_site_fract_y"];
            let fract_z =  &atom_sites["atom_site_fract_z"];
            fract_x.values().iter().zip(fract_y.values().iter()).zip(fract_z.values().iter())
            .map(|((x,y), z)| {
                    let (x, y, z) = (*x.as_numeric().expect("Numeric").number().as_float().expect("fractional coord should be float"),
                        *y.as_numeric().expect("Numeric").number().as_float().expect("fractional coord should be float"),
                        *z.as_numeric().expect("Numeric").number().as_float().expect("fractional coord should be float"));
                    let (x,y,z) = ((*x).into(), ( *y ).into(), ( *z ).into());
                    CoordData::Fractional(Point3::new(x, y, z))

                    
                }).collect()
        } else if  

        atom_sites.find_loop_column_by_tag("atom_site_cartn_x").is_some() &&
        atom_sites.find_loop_column_by_tag("atom_site_cartn_y").is_some() &&
        atom_sites.find_loop_column_by_tag("atom_site_cartn_z").is_some() {
            let cartn_x = &atom_sites["atom_site_cartn_x"];
            let cartn_y =  &atom_sites["atom_site_cartn_y"];
            let cartn_z =  &atom_sites["atom_site_cartn_z"];
            cartn_x.values().iter().zip(cartn_y.values().iter()).zip(cartn_z.values().iter())
            .map(|((x,y), z)| {
                    let (x, y, z) = (*x.as_numeric().expect("Numeric").number().as_float().expect("coord should be float"),
                        *y.as_numeric().expect("Numeric").number().as_float().expect("coord should be float"),
                        *z.as_numeric().expect("Numeric").number().as_float().expect("coord should be float"));
                    let (x,y,z) = ((*x).into(), ( *y ).into(), ( *z ).into());
                    CoordData::Cartesian(Point3::new(x, y, z))
                }).collect()
        } else {
            panic!("the data block does not have sufficient coordinate data (x, y, and z)")
        }

    }

    fn labels_repr(&self) -> Vec<Option<String>> {
        let atom_sites = &self["atom_site_label"].as_multi_values().expect("this data block does not have atom sites data");
        atom_sites["atom_site_label"].values().iter()
        .map(|val| val.as_char_string().map(|v| v.formatted_output()))
        .collect()
    }

}

impl CrystalModel for DataBlock {
    type LatticeData=DataBlock;

    type AtomData=DataBlock;

    fn get_cell_parameters(&self) -> &Self::LatticeData {
        self
    }

    fn get_atom_data(&self) -> &Self::AtomData {
        self
    }

    fn get_cell_parameters_mut(&mut self) -> &mut Self::LatticeData {
        self
    }

    fn get_atom_data_mut(&mut self) -> &mut Self::AtomData {
        self
    }
}
