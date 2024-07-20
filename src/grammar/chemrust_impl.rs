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

#[cfg(feature = "serde")]
mod serde {
    use std::fmt::Display;

    use castep_periodic_table::element::ElementSymbol;
    use chemrust_core::data::geom::coordinates::CoordData;
    use serde::{
        de::{self, Visitor},
        Deserialize,
    };
    pub struct CifModel {
        cell_constant: LatticeConstant,
        atom_site_type_symbols: Vec<ElementSymbol>,
        atom_site_xyz: Vec<CoordData>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct LatticeConstant {
        a: f32,
        b: f32,
        c: f32,
        alpha: f32,
        beta: f32,
        gamma: f32,
    }

    #[derive(Debug)]
    pub enum DataError {
        Msg(String),
        DataNotFound,
    }

    impl Display for DataError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DataError::Msg(m) => write!(f, "{m}"),
                DataError::DataNotFound => f.write_str("Requested data not found"),
            }
        }
    }

    impl std::error::Error for DataError {}

    impl<'de> Deserialize<'de> for LatticeConstant {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
            #[derive(Deserialize)]
            #[serde(field_identifier, rename_all = "lowercase")]
            enum Field {
                Cell_length_a,
                Cell_length_b,
                Cell_length_c,
                Cell_angle_alpha,
                Cell_angle_beta,
                Cell_angle_gamma,
            }
            struct ConstantVisitor;
            impl<'de> Visitor<'de> for ConstantVisitor {
                type Value = LatticeConstant;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("Data of cell constants")
                }
                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
                {
                    let a = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let b = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let c = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let alpha = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let beta = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let gamma = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    Ok(LatticeConstant {
                        a,
                        b,
                        c,
                        alpha,
                        beta,
                        gamma,
                    })
                }
                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>,
                {
                    let mut a = None;
                    let mut b = None;
                    let mut c = None;
                    let mut alpha = None;
                    let mut beta = None;
                    let mut gamma = None;
                    while let Some(key) = map.next_key()? {
                        let value: f32 = map.next_value()?;
                        match key {
                            Field::Cell_length_a => {
                                if a.is_some() {
                                    return Err(de::Error::duplicate_field("cell_length_a"));
                                }
                                a = Some(value);
                            }
                            Field::Cell_length_b => {
                                if b.is_some() {
                                    return Err(de::Error::duplicate_field("cell_length_b"));
                                }
                                b = Some(value);
                            }
                            Field::Cell_length_c => {
                                if c.is_some() {
                                    return Err(de::Error::duplicate_field("cell_length_c"));
                                }
                                c = Some(value);
                            }
                            Field::Cell_angle_alpha => {
                                if alpha.is_some() {
                                    return Err(de::Error::duplicate_field("cell_angle_alpha"));
                                }
                                alpha = Some(value);
                            }
                            Field::Cell_angle_beta => {
                                if beta.is_some() {
                                    return Err(de::Error::duplicate_field("cell_angle_beta"));
                                }
                                beta = Some(value);
                            }
                            Field::Cell_angle_gamma => {
                                if gamma.is_some() {
                                    return Err(de::Error::duplicate_field("cell_angle_gamma"));
                                }
                                gamma = Some(value);
                            }
                        }
                    }
                    let a = a.ok_or_else(|| de::Error::missing_field("cell_length_a"))?;
                    let b = b.ok_or_else(|| de::Error::missing_field("cell_length_a"))?;
                    let c = c.ok_or_else(|| de::Error::missing_field("cell_length_a"))?;
                    let alpha =
                        alpha.ok_or_else(|| de::Error::missing_field("cell_angle_alpha"))?;
                    let beta = beta.ok_or_else(|| de::Error::missing_field("cell_angle_beta"))?;
                    let gamma =
                        gamma.ok_or_else(|| de::Error::missing_field("cell_angle_gamma"))?;
                    Ok(LatticeConstant {
                        a,
                        b,
                        c,
                        alpha,
                        beta,
                        gamma,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "cell_length_a",
                "cell_length_b",
                "cell_length_c",
                "cell_angle_alpha",
                "cell_angle_beta",
                "cell_angle_gamma",
            ];
            deserializer.deserialize_struct("LatticeConstant", FIELDS, ConstantVisitor)
        }
    }
}
