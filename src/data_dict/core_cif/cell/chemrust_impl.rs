use chemrust_core::data::lattice::UnitCellParameters;

use crate::{data_dict::SingleValueTerm, grammar::DataItems, DataBlockMember};

use super::CellTerms;
/// For simple creation
pub(crate) fn basic_cell_data<T: UnitCellParameters>(value: &T) -> Vec<DataItems> {
    let cell_terms = [
        CellTerms::Length_a(value.length_a().into()),
        CellTerms::Length_b(value.length_b().into()),
        CellTerms::Length_c(value.length_c().into()),
        CellTerms::Angle_alpha(value.angle_alpha().into()),
        CellTerms::Angle_beta(value.angle_beta().into()),
        CellTerms::Angle_gamma(value.angle_gamma().into()),
    ];

    cell_terms
        .iter()
        .map(|t| t.to_single_value_data())
        .collect()
}

pub fn from_unit_cell_parameters(value: &impl UnitCellParameters) -> Vec<DataBlockMember> {
    [
        CellTerms::Length_a(value.length_a().into()),
        CellTerms::Length_b(value.length_b().into()),
        CellTerms::Length_c(value.length_c().into()),
        CellTerms::Angle_alpha(value.angle_alpha().into()),
        CellTerms::Angle_beta(value.angle_beta().into()),
        CellTerms::Angle_gamma(value.angle_gamma().into()),
    ]
    .iter()
    .map(|t| t.to_single_value_data())
    .map(DataBlockMember::DataItems)
    .collect::<Vec<DataBlockMember>>()
}
