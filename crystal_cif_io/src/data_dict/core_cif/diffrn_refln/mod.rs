mod detect_slit_horiz;
mod detect_slit_vert;
mod elapsed_time;
mod intensity_net;
mod intensity_sigma;
mod intensity_u;
mod scan_mode;
mod scan_mode_backgd;
mod scan_rate;
mod scan_time_backgd;
mod scan_width;
mod sint_lambda;
mod wavelength;

use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};

pub use self::detect_slit_horiz::DetectSlitHoriz;
pub use self::detect_slit_vert::DetectSlitVert;
pub use self::elapsed_time::ElapsedTime;
pub use self::intensity_net::IntensityNet;
pub use self::intensity_sigma::IntensitySigma;
pub use self::intensity_u::IntensityU;
pub use self::scan_mode::ScanMode;
pub use self::scan_mode_backgd::ScanModeBackgd;
pub use self::scan_rate::ScanRate;
pub use self::scan_time_backgd::ScanTimeBackgd;
pub use self::scan_width::ScanWidth;
pub use self::sint_lambda::SintLambda;
pub use self::wavelength::Wavelength;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnTerms {
    Hkl(Matrix),
    Id(CharString),
    Intensity_bg_1(Numeric),
    Intensity_bg_2(Numeric),
    Intensity_peak(Numeric),
    Intensity_total(Numeric),
}

impl CifTerm for DiffrnReflnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnReflnTerms::Hkl(_) => "hkl",
            DiffrnReflnTerms::Id(_) => "id",
            DiffrnReflnTerms::Intensity_bg_1(_) => "intensity_bg_1",
            DiffrnReflnTerms::Intensity_bg_2(_) => "intensity_bg_2",
            DiffrnReflnTerms::Intensity_peak(_) => "intensity_peak",
            DiffrnReflnTerms::Intensity_total(_) => "intensity_total",
        };
        Tag::new(format!("diffrn_refln_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnReflnTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnReflnTerms::Hkl(m) => matrix::to_value(m),
            DiffrnReflnTerms::Id(s) => Value::CharString(s.clone()),
            DiffrnReflnTerms::Intensity_bg_1(n)
            | DiffrnReflnTerms::Intensity_bg_2(n)
            | DiffrnReflnTerms::Intensity_peak(n)
            | DiffrnReflnTerms::Intensity_total(n) => Value::Numeric(*n),
        }
    }
}

impl Display for DiffrnReflnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnLoopItem {
    Angle_chi(Vec<Numeric>),
    Angle_kappa(Vec<Numeric>),
    Angle_omega(Vec<Numeric>),
    Angle_phi(Vec<Numeric>),
    Angle_psi(Vec<Numeric>),
    Angle_theta(Vec<Numeric>),
    Attenuator_code(Vec<CharString>),
    Class_code(Vec<CharString>),
    Counts_bg_1(Vec<Numeric>),
    Counts_bg_2(Vec<Numeric>),
    Counts_net(Vec<Numeric>),
    Counts_peak(Vec<Numeric>),
    Counts_total(Vec<Numeric>),
    Crystal_id(Vec<CharString>),
    /// range constraint recorded in the dictionary _enumeration
    Detect_slit_horiz(Vec<DetectSlitHoriz>),
    /// range constraint recorded in the dictionary _enumeration
    Detect_slit_vert(Vec<DetectSlitVert>),
    /// range constraint recorded in the dictionary _enumeration
    Elapsed_time(Vec<ElapsedTime>),
    Index_h(Vec<Numeric>),
    Index_k(Vec<Numeric>),
    Index_l(Vec<Numeric>),
    /// range constraint recorded in the dictionary _enumeration
    Intensity_net(Vec<IntensityNet>),
    /// range constraint recorded in the dictionary _enumeration
    Intensity_sigma(Vec<IntensitySigma>),
    /// range constraint recorded in the dictionary _enumeration
    Intensity_u(Vec<IntensityU>),
    Scale_group_code(Vec<CharString>),
    /// closed code set recorded in the dictionary _enumeration
    Scan_mode(Vec<ScanMode>),
    /// closed code set recorded in the dictionary _enumeration
    Scan_mode_backgd(Vec<ScanModeBackgd>),
    /// range constraint recorded in the dictionary _enumeration
    Scan_rate(Vec<ScanRate>),
    /// range constraint recorded in the dictionary _enumeration
    Scan_time_backgd(Vec<ScanTimeBackgd>),
    /// range constraint recorded in the dictionary _enumeration
    Scan_width(Vec<ScanWidth>),
    /// range constraint recorded in the dictionary _enumeration
    SintLambda(Vec<SintLambda>),
    Standard_code(Vec<CharString>),
    /// range constraint recorded in the dictionary _enumeration
    Wavelength(Vec<Wavelength>),
    Wavelength_id(Vec<CharString>),
}

impl CifTerm for DiffrnReflnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnReflnLoopItem::Angle_chi(_) => "angle_chi",
            DiffrnReflnLoopItem::Angle_kappa(_) => "angle_kappa",
            DiffrnReflnLoopItem::Angle_omega(_) => "angle_omega",
            DiffrnReflnLoopItem::Angle_phi(_) => "angle_phi",
            DiffrnReflnLoopItem::Angle_psi(_) => "angle_psi",
            DiffrnReflnLoopItem::Angle_theta(_) => "angle_theta",
            DiffrnReflnLoopItem::Attenuator_code(_) => "attenuator_code",
            DiffrnReflnLoopItem::Class_code(_) => "class_code",
            DiffrnReflnLoopItem::Counts_bg_1(_) => "counts_bg_1",
            DiffrnReflnLoopItem::Counts_bg_2(_) => "counts_bg_2",
            DiffrnReflnLoopItem::Counts_net(_) => "counts_net",
            DiffrnReflnLoopItem::Counts_peak(_) => "counts_peak",
            DiffrnReflnLoopItem::Counts_total(_) => "counts_total",
            DiffrnReflnLoopItem::Crystal_id(_) => "crystal_id",
            DiffrnReflnLoopItem::Detect_slit_horiz(_) => "detect_slit_horiz",
            DiffrnReflnLoopItem::Detect_slit_vert(_) => "detect_slit_vert",
            DiffrnReflnLoopItem::Elapsed_time(_) => "elapsed_time",
            DiffrnReflnLoopItem::Index_h(_) => "index_h",
            DiffrnReflnLoopItem::Index_k(_) => "index_k",
            DiffrnReflnLoopItem::Index_l(_) => "index_l",
            DiffrnReflnLoopItem::Intensity_net(_) => "intensity_net",
            DiffrnReflnLoopItem::Intensity_sigma(_) => "intensity_sigma",
            DiffrnReflnLoopItem::Intensity_u(_) => "intensity_u",
            DiffrnReflnLoopItem::Scale_group_code(_) => "scale_group_code",
            DiffrnReflnLoopItem::Scan_mode(_) => "scan_mode",
            DiffrnReflnLoopItem::Scan_mode_backgd(_) => "scan_mode_backgd",
            DiffrnReflnLoopItem::Scan_rate(_) => "scan_rate",
            DiffrnReflnLoopItem::Scan_time_backgd(_) => "scan_time_backgd",
            DiffrnReflnLoopItem::Scan_width(_) => "scan_width",
            DiffrnReflnLoopItem::SintLambda(_) => "sint/lambda",
            DiffrnReflnLoopItem::Standard_code(_) => "standard_code",
            DiffrnReflnLoopItem::Wavelength(_) => "wavelength",
            DiffrnReflnLoopItem::Wavelength_id(_) => "wavelength_id",
        };
        Tag::new(format!("diffrn_refln_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnReflnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnReflnLoopItem::Angle_chi(n)
            | DiffrnReflnLoopItem::Angle_kappa(n)
            | DiffrnReflnLoopItem::Angle_omega(n)
            | DiffrnReflnLoopItem::Angle_phi(n)
            | DiffrnReflnLoopItem::Angle_psi(n)
            | DiffrnReflnLoopItem::Angle_theta(n)
            | DiffrnReflnLoopItem::Counts_bg_1(n)
            | DiffrnReflnLoopItem::Counts_bg_2(n)
            | DiffrnReflnLoopItem::Counts_net(n)
            | DiffrnReflnLoopItem::Counts_peak(n)
            | DiffrnReflnLoopItem::Counts_total(n)
            | DiffrnReflnLoopItem::Index_h(n)
            | DiffrnReflnLoopItem::Index_k(n)
            | DiffrnReflnLoopItem::Index_l(n) => n.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Attenuator_code(s)
            | DiffrnReflnLoopItem::Class_code(s)
            | DiffrnReflnLoopItem::Crystal_id(s)
            | DiffrnReflnLoopItem::Scale_group_code(s)
            | DiffrnReflnLoopItem::Standard_code(s)
            | DiffrnReflnLoopItem::Wavelength_id(s) => s.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Detect_slit_horiz(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            DiffrnReflnLoopItem::Detect_slit_vert(m) => {
                m.iter().cloned().map(Value::from).collect()
            }
            DiffrnReflnLoopItem::Elapsed_time(c) => c.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Intensity_net(b) => b.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Intensity_sigma(t) => t.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Intensity_u(u) => u.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Scan_mode(w) => w.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Scan_mode_backgd(x) => {
                x.iter().cloned().map(Value::from).collect()
            }
            DiffrnReflnLoopItem::Scan_rate(y) => y.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Scan_time_backgd(z) => {
                z.iter().cloned().map(Value::from).collect()
            }
            DiffrnReflnLoopItem::Scan_width(q) => q.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::SintLambda(r) => r.iter().cloned().map(Value::from).collect(),
            DiffrnReflnLoopItem::Wavelength(f) => f.iter().cloned().map(Value::from).collect(),
        }
    }
}
