pub mod core_cif;

mod data_types;

use std::fmt::Display;

use chrono::{Utc};
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
}

impl Display for CifData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CifData::SpaceGroup(v) => write!(f, "{v}"),
            CifData::SpaceGroupLoop(v) => write!(f, "{v}"),
            CifData::CellData(v) => write!(f, "{v}"),
            CifData::AtomSiteLoop(v) => write!(f, "{v}"),
            CifData::Audit(v) => write!(f, "{v}"),
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

    pub fn init(name: String) -> Self {
        let audit = AuditSection::init_builder()
            .add_entry(AuditItem::Creation_date(format!(
                "{}",
                Utc::now().format("%Y-%m-%d")
            )))
            .add_entry(AuditItem::Creation_method("crystal-cif-io".to_string()))
            .finish();
        Self::new(name, vec![CifData::Audit(audit)])
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

#[cfg(test)]
mod test;
