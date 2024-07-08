pub mod core_cif;

mod data_types;

use std::fmt::Display;

use chemrust_core::data::lattice::CrystalModel;
use chemrust_core::data::symmetry::SymmetryInfo;
use chrono::Utc;
pub use data_types::num_value::NumValue;
pub use data_types::{DataLabel, LoopData, LoopDataEntry, LoopDataLabel, SingleValueSection};

use self::core_cif::atom_site::LoopAtomSiteData;
use self::core_cif::audit::{AuditItem, AuditSection};
use self::core_cif::cell::CellDataSection;
use self::core_cif::space_group::{SpaceGroupLoopData, SpaceGroupSection};

#[derive(Debug, Clone)]
pub enum CifData {
    Audit(AuditSection),
    SpaceGroup(SpaceGroupSection),
    SpaceGroupLoop(SpaceGroupLoopData),
    CellData(CellDataSection),
    AtomSiteLoop(LoopAtomSiteData),
    Else,
}

impl Display for CifData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CifData::SpaceGroup(v) => write!(f, "{v}"),
            CifData::SpaceGroupLoop(v) => write!(f, "{v}"),
            CifData::CellData(v) => write!(f, "{v}"),
            CifData::AtomSiteLoop(v) => write!(f, "{v}"),
            CifData::Audit(v) => write!(f, "{v}"),
            CifData::Else => Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataBlock {
    name: String,
    sections: Vec<CifData>,
}

impl DataBlock {
    pub fn new(name: String, sections: Vec<CifData>) -> Self {
        Self { name, sections }
    }

    pub fn init_with_builder() -> DataBlockBuilder {
        DataBlockBuilder::default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sections(&self) -> &[CifData] {
        &self.sections
    }

    pub fn add_section(&mut self, section: CifData) -> &mut Self {
        self.sections.push(section);
        self
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone)]
pub struct DataBlockBuilder {
    name: Option<String>,
    sections: Vec<CifData>,
}

impl DataBlockBuilder {
    pub fn with_name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.name = Some(name.as_ref().into());
        self
    }

    pub fn add_section(mut self, section: CifData) -> Self {
        self.sections.push(section);
        self
    }

    pub fn finish(self) -> DataBlock {
        DataBlock::new(self.name.unwrap_or("sample".to_string()), self.sections)
    }
}

impl Default for DataBlockBuilder {
    fn default() -> Self {
        let audit = AuditSection::init_builder()
            .add_entry(AuditItem::Creation_date(format!(
                "{}",
                Utc::now().format("%Y-%m-%d")
            )))
            .add_entry(AuditItem::Creation_method("crystal-cif-io".to_string()))
            .finish();
        Self {
            name: None,
            sections: vec![CifData::Audit(audit)],
        }
    }
}

impl Display for DataBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_name = format!("data_{}", self.name());
        let content = self
            .sections
            .iter()
            .map(|data| format!("{data}"))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", [data_name, content].join("\n"))
    }
}

impl<T: CrystalModel + SymmetryInfo> From<&T> for DataBlock {
    fn from(value: &T) -> Self {
        DataBlock::init_with_builder()
            .add_section(CifData::SpaceGroup(SpaceGroupSection::from(value)))
            .add_section(CifData::SpaceGroupLoop(SpaceGroupLoopData::from(value)))
            .add_section(CifData::CellData(CellDataSection::from(
                value.get_cell_parameters(),
            )))
            .add_section(CifData::AtomSiteLoop(LoopAtomSiteData::from(
                value.get_atom_data(),
            )))
            .finish()
    }
}

#[cfg(test)]
mod test;
