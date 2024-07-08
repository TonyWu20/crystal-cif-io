pub mod core_cif;

use crate::grammar::{CIFDataType, CIFValue, DataItems, Tag};

pub trait CifTerm {
    type Value: CIFValue;
    type DataForm: CIFDataType;
    fn category_prefix(&self) -> String;
    fn tag(&self) -> Tag;
    fn value(&self) -> &Self::Value;
    fn to_data_item(&self) -> DataItems;
}

// #[derive(Debug, Clone)]
// pub enum CifData {
//     Audit(AuditSection),
//     SpaceGroup(SpaceGroupSection),
//     SpaceGroupLoop(SpaceGroupLoopData),
//     CellData(CellDataSection),
//     AtomSiteLoop(LoopAtomSiteData),
//     Else,
// }

// impl Display for CifData {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CifData::SpaceGroup(v) => write!(f, "{v}"),
//             CifData::SpaceGroupLoop(v) => write!(f, "{v}"),
//             CifData::CellData(v) => write!(f, "{v}"),
//             CifData::AtomSiteLoop(v) => write!(f, "{v}"),
//             CifData::Audit(v) => write!(f, "{v}"),
//             CifData::Else => Ok(()),
//         }
//     }
// }
