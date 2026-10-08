mod determination;
mod diffrn_radiation_wavelength;
mod wt;

use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::determination::Determination;
pub use self::diffrn_radiation_wavelength::DiffrnRadiationWavelength;
pub use self::wt::Wt;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnRadiationWavelengthLoopItem {
    Details(Vec<CharString>),
    /// closed code set recorded in the dictionary _enumeration
    Determination(Vec<Determination>),
    /// range constraint recorded in the dictionary _enumeration
    Diffrn_radiation_wavelength(Vec<DiffrnRadiationWavelength>),
    Id(Vec<CharString>),
    /// range constraint recorded in the dictionary _enumeration
    Wt(Vec<Wt>),
}

impl CifTerm for DiffrnRadiationWavelengthLoopItem {
    fn tag(&self) -> Tag {
        let tag = match self {
            DiffrnRadiationWavelengthLoopItem::Details(_) => "diffrn_radiation_wavelength_details",
            DiffrnRadiationWavelengthLoopItem::Determination(_) => {
                "diffrn_radiation_wavelength_determination"
            }
            DiffrnRadiationWavelengthLoopItem::Diffrn_radiation_wavelength(_) => {
                "diffrn_radiation_wavelength"
            }
            DiffrnRadiationWavelengthLoopItem::Id(_) => "diffrn_radiation_wavelength_id",
            DiffrnRadiationWavelengthLoopItem::Wt(_) => "diffrn_radiation_wavelength_wt",
        };
        Tag::new(tag.to_string())
    }
}

impl LoopValueTerm for DiffrnRadiationWavelengthLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnRadiationWavelengthLoopItem::Details(s)
            | DiffrnRadiationWavelengthLoopItem::Id(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
            DiffrnRadiationWavelengthLoopItem::Determination(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            DiffrnRadiationWavelengthLoopItem::Diffrn_radiation_wavelength(m) => {
                m.iter().cloned().map(Value::from).collect()
            }
            DiffrnRadiationWavelengthLoopItem::Wt(c) => {
                c.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
