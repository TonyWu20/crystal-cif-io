mod convergence_angle;
mod current;
mod device;
mod diffrn_source;
mod ed_diffracting_area_selection;
mod power;
mod take_off_angle;
mod target;
mod voltage;

use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::convergence_angle::ConvergenceAngle;
pub use self::current::Current;
pub use self::device::Device;
pub use self::diffrn_source::DiffrnSource;
pub use self::ed_diffracting_area_selection::EdDiffractingAreaSelection;
pub use self::power::Power;
pub use self::take_off_angle::TakeOffAngle;
pub use self::target::Target;
pub use self::voltage::Voltage;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnSourceTerms {
    Beamline(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Convergence_angle(ConvergenceAngle),
    /// range constraint recorded in the dictionary _enumeration
    Current(Current),
    Details(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Device(Device),
    Diffrn_radiation_source(CharString),
    Diffrn_source(CharString),
    /// closed code set recorded in the dictionary _enumeration_set
    Ed_diffracting_area_selection(EdDiffractingAreaSelection),
    Facility(CharString),
    Make(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Power(Power),
    Size(CharString),
    /// range constraint recorded in the dictionary _enumeration
    TakeOffAngle(TakeOffAngle),
    /// closed code set recorded in the dictionary _enumeration
    Target(Target),
    Type(CharString),
    /// range constraint recorded in the dictionary _enumeration
    Voltage(Voltage),
}

impl CifTerm for DiffrnSourceTerms {
    fn tag(&self) -> Tag {
        let tag = match self {
            DiffrnSourceTerms::Beamline(_) => "diffrn_source_beamline",
            DiffrnSourceTerms::Convergence_angle(_) => "diffrn_source_convergence_angle",
            DiffrnSourceTerms::Current(_) => "diffrn_source_current",
            DiffrnSourceTerms::Details(_) => "diffrn_source_details",
            DiffrnSourceTerms::Device(_) => "diffrn_source_device",
            DiffrnSourceTerms::Diffrn_radiation_source(_) => "diffrn_radiation_source",
            DiffrnSourceTerms::Diffrn_source(_) => "diffrn_source",
            DiffrnSourceTerms::Ed_diffracting_area_selection(_) => {
                "diffrn_source_ed_diffracting_area_selection"
            }
            DiffrnSourceTerms::Facility(_) => "diffrn_source_facility",
            DiffrnSourceTerms::Make(_) => "diffrn_source_make",
            DiffrnSourceTerms::Power(_) => "diffrn_source_power",
            DiffrnSourceTerms::Size(_) => "diffrn_source_size",
            DiffrnSourceTerms::TakeOffAngle(_) => "diffrn_source_take-off_angle",
            DiffrnSourceTerms::Target(_) => "diffrn_source_target",
            DiffrnSourceTerms::Type(_) => "diffrn_source_type",
            DiffrnSourceTerms::Voltage(_) => "diffrn_source_voltage",
        };
        Tag::new(tag.to_string())
    }
}

impl SingleValueTerm for DiffrnSourceTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnSourceTerms::Beamline(s)
            | DiffrnSourceTerms::Details(s)
            | DiffrnSourceTerms::Diffrn_radiation_source(s)
            | DiffrnSourceTerms::Diffrn_source(s)
            | DiffrnSourceTerms::Facility(s)
            | DiffrnSourceTerms::Make(s)
            | DiffrnSourceTerms::Size(s)
            | DiffrnSourceTerms::Type(s) => Value::CharString(s.clone()),
            DiffrnSourceTerms::Convergence_angle(v) => Value::from(*v),
            DiffrnSourceTerms::Current(m) => Value::from(*m),
            DiffrnSourceTerms::Device(c) => Value::from(*c),
            DiffrnSourceTerms::Ed_diffracting_area_selection(b) => Value::from(*b),
            DiffrnSourceTerms::Power(t) => Value::from(*t),
            DiffrnSourceTerms::TakeOffAngle(u) => Value::from(*u),
            DiffrnSourceTerms::Target(w) => Value::from(*w),
            DiffrnSourceTerms::Voltage(x) => Value::from(*x),
        }
    }
}

impl Display for DiffrnSourceTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
