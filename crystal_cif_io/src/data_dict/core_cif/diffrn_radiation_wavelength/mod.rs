mod determination;
mod diffrn_radiation_wavelength;
mod diffrn_radiation_wavelength_row;
mod wt;
mod xray_symbol;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::determination::Determination;
pub use self::diffrn_radiation_wavelength::DiffrnRadiationWavelength;
pub use self::diffrn_radiation_wavelength_row::DiffrnRadiationWavelengthRow;
pub use self::wt::Wt;
pub use self::xray_symbol::XraySymbol;

/// The single-value terms of the `diffrn_radiation_wavelength` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnRadiationWavelengthTerms {
    /// The `_diffrn_radiation_wavelength_type` item. The variant holds the single value.
    Type(CharString),
    /// The `_diffrn_radiation_wavelength_xray_symbol` item. The variant holds the single value.
    /// closed code set recorded in the dictionary _enumeration_set
    Xray_symbol(XraySymbol),
}

impl CifTerm for DiffrnRadiationWavelengthTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnRadiationWavelengthTerms::Type(_) => "type",
            DiffrnRadiationWavelengthTerms::Xray_symbol(_) => "xray_symbol",
        };
        Tag::new(format!("diffrn_radiation_wavelength_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnRadiationWavelengthTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnRadiationWavelengthTerms::Type(s) => Value::CharString(s.clone()),
            DiffrnRadiationWavelengthTerms::Xray_symbol(v) => Value::from(*v),
        }
    }
}

impl Display for DiffrnRadiationWavelengthTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `diffrn_radiation_wavelength` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnRadiationWavelengthLoopItem {
    /// The `_diffrn_radiation_wavelength_details` item. The variant holds the column values, one per loop row.
    ///
    /// Information about the determination of the radiation wavelength that
    /// is not conveyed completely by an enumerated value of
    /// _diffrn_radiation_wavelength_determination.
    Details(Vec<CharString>),
    /// The `_diffrn_radiation_wavelength_determination` item. The variant holds the column values, one per loop row.
    ///
    /// The method of determination of incident wavelength.
    /// closed code set recorded in the dictionary _enumeration
    Determination(Vec<Determination>),
    /// The `_diffrn_radiation_wavelength` item. The variant holds the column values, one per loop row.
    ///
    /// The radiation wavelength in angstroms.
    /// Units: A (angstrom).
    /// range constraint recorded in the dictionary _enumeration
    Diffrn_radiation_wavelength(Vec<DiffrnRadiationWavelength>),
    /// The `_diffrn_radiation_wavelength_id` item. The variant holds the column values, one per loop row.
    ///
    /// An arbitrary code identifying each value of
    /// _diffrn_radiation_wavelength. Items in the DIFFRN_RADIATION category
    /// are looped when multiple wavelengths are used. This code is used to
    /// link with the _diffrn_refln_ list. It must match with one of the
    /// _diffrn_refln_wavelength_id codes.
    Id(Vec<CharString>),
    /// The `_diffrn_radiation_wavelength_wt` item. The variant holds the column values, one per loop row.
    ///
    /// The relative weight of a wavelength identified by the code
    /// _diffrn_radiation_wavelength_id in the list of wavelengths.
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
