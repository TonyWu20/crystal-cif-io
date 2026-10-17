mod detect_slit_horiz;
mod detect_slit_vert;
mod diffrn_refln;
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
pub use self::diffrn_refln::DiffrnRefln;
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

/// The single-value terms of the `diffrn_refln` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnTerms {
    /// The `_diffrn_refln_hkl` item. The variant holds the single value.
    Hkl(Matrix),
    /// The `_diffrn_refln_id` item. The variant holds the single value.
    Id(CharString),
    /// The `_diffrn_refln_intensity_bg_1` item. The variant holds the single value.
    Intensity_bg_1(Numeric),
    /// The `_diffrn_refln_intensity_bg_2` item. The variant holds the single value.
    Intensity_bg_2(Numeric),
    /// The `_diffrn_refln_intensity_peak` item. The variant holds the single value.
    Intensity_peak(Numeric),
    /// The `_diffrn_refln_intensity_total` item. The variant holds the single value.
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

/// The loop-value terms of the `diffrn_refln` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnLoopItem {
    /// The `_diffrn_refln_angle_chi` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    Angle_chi(Vec<Numeric>),
    /// The `_diffrn_refln_angle_kappa` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    Angle_kappa(Vec<Numeric>),
    /// The `_diffrn_refln_angle_omega` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    Angle_omega(Vec<Numeric>),
    /// The `_diffrn_refln_angle_phi` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    Angle_phi(Vec<Numeric>),
    /// The `_diffrn_refln_angle_psi` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    Angle_psi(Vec<Numeric>),
    /// The `_diffrn_refln_angle_theta` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    Angle_theta(Vec<Numeric>),
    /// The `_diffrn_refln_attenuator_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying the attenuator setting for this reflection.
    /// This code must match one of the _diffrn_attenuator_code values.
    Attenuator_code(Vec<CharString>),
    /// The `_diffrn_refln_class_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying the class to which this reflection has been
    /// assigned. This code must match a value of _diffrn_reflns_class_code.
    /// Reflections may be grouped into classes for a variety of purposes.
    /// For example, for modulated structures each reflection class may be
    /// defined by the number m=sum|m~i~|, where the m~i~ are the integer
    /// coefficients that, in addition to h,k,l, index the corresponding
    /// diffraction vector in the basis defined for the reciprocal lattice.
    Class_code(Vec<CharString>),
    /// The `_diffrn_refln_counts_bg_1` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    Counts_bg_1(Vec<Numeric>),
    /// The `_diffrn_refln_counts_bg_2` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    Counts_bg_2(Vec<Numeric>),
    /// The `_diffrn_refln_counts_net` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    Counts_net(Vec<Numeric>),
    /// The `_diffrn_refln_counts_peak` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    Counts_peak(Vec<Numeric>),
    /// The `_diffrn_refln_counts_total` item. The variant holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    Counts_total(Vec<Numeric>),
    /// The `_diffrn_refln_crystal_id` item. The variant holds the column values, one per loop row.
    ///
    /// Code identifying each crystal if multiple crystals are used. Is used
    /// to link with _exptl_crystal_id in the _exptl_crystal_ list.
    Crystal_id(Vec<CharString>),
    /// The `_diffrn_refln_detect_slit_horiz` item. The variant holds the column values, one per loop row.
    ///
    /// Total slit apertures in degrees in the diffraction plane (*_horiz)
    /// and perpendicular to the diffraction plane (*_vert).
    /// Units: deg (degree of arc).
    /// range constraint recorded in the dictionary _enumeration
    Detect_slit_horiz(Vec<DetectSlitHoriz>),
    /// The `_diffrn_refln_detect_slit_vert` item. The variant holds the column values, one per loop row.
    ///
    /// Total slit apertures in degrees in the diffraction plane (*_horiz)
    /// and perpendicular to the diffraction plane (*_vert).
    /// Units: deg (degree of arc).
    /// range constraint recorded in the dictionary _enumeration
    Detect_slit_vert(Vec<DetectSlitVert>),
    /// The `_diffrn_refln_elapsed_time` item. The variant holds the column values, one per loop row.
    ///
    /// Elapsed time in minutes from the start of the diffraction experiment
    /// to the measurement of this intensity.
    /// Units: min (minute).
    /// range constraint recorded in the dictionary _enumeration
    Elapsed_time(Vec<ElapsedTime>),
    /// The `_diffrn_refln_index_h` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of a measured reflection. These need not match the
    /// _refln_index_h, *_k, *_l values if a transformation of the original
    /// measured cell has taken place. Details of the cell transformation
    /// are given in _diffrn_reflns_reduction_process. See also
    /// _diffrn_reflns_transf_matrix_.
    Index_h(Vec<Numeric>),
    /// The `_diffrn_refln_index_k` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of a measured reflection. These need not match the
    /// _refln_index_h, *_k, *_l values if a transformation of the original
    /// measured cell has taken place. Details of the cell transformation
    /// are given in _diffrn_reflns_reduction_process. See also
    /// _diffrn_reflns_transf_matrix_.
    Index_k(Vec<Numeric>),
    /// The `_diffrn_refln_index_l` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of a measured reflection. These need not match the
    /// _refln_index_h, *_k, *_l values if a transformation of the original
    /// measured cell has taken place. Details of the cell transformation
    /// are given in _diffrn_reflns_reduction_process. See also
    /// _diffrn_reflns_transf_matrix_.
    Index_l(Vec<Numeric>),
    /// The `_diffrn_refln_intensity_net` item. The variant holds the column values, one per loop row.
    ///
    /// Net intensity calculated from the diffraction counts after the
    /// attenuator and standard scales have been applied.
    /// range constraint recorded in the dictionary _enumeration
    Intensity_net(Vec<IntensityNet>),
    /// The `_diffrn_refln_intensity_sigma` item. The variant holds the column values, one per loop row.
    ///
    /// Standard uncertainty (e.s.d.) of the net intensity calculated from
    /// the diffraction counts after the attenuator and standard scales have
    /// been applied.
    /// range constraint recorded in the dictionary _enumeration
    Intensity_sigma(Vec<IntensitySigma>),
    /// The `_diffrn_refln_intensity_u` item. The variant holds the column values, one per loop row.
    ///
    /// Standard uncertainty of the net intensity calculated from the
    /// diffraction counts after the attenuator and standard scales have
    /// been applied.
    /// range constraint recorded in the dictionary _enumeration
    Intensity_u(Vec<IntensityU>),
    /// The `_diffrn_refln_scale_group_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying the scale applicable to this reflection. This
    /// code must match with a specified _diffrn_scale_group_code value.
    Scale_group_code(Vec<CharString>),
    /// The `_diffrn_refln_scan_mode` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying the mode of scanning for measurements using a
    /// diffractometer. See _diffrn_refln_scan_width and
    /// _diffrn_refln_scan_mode_backgd.
    /// closed code set recorded in the dictionary _enumeration
    Scan_mode(Vec<ScanMode>),
    /// The `_diffrn_refln_scan_mode_backgd` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying the mode of scanning a reflection to measure
    /// the background intensity.
    /// closed code set recorded in the dictionary _enumeration
    Scan_mode_backgd(Vec<ScanModeBackgd>),
    /// The `_diffrn_refln_scan_rate` item. The variant holds the column values, one per loop row.
    ///
    /// The rate of scanning a reflection in degrees per minute to measure
    /// the intensity.
    /// Units: deg/min (degree of arc per minute).
    /// range constraint recorded in the dictionary _enumeration
    Scan_rate(Vec<ScanRate>),
    /// The `_diffrn_refln_scan_time_backgd` item. The variant holds the column values, one per loop row.
    ///
    /// The time spent measuring each background in seconds.
    /// Units: s (second).
    /// range constraint recorded in the dictionary _enumeration
    Scan_time_backgd(Vec<ScanTimeBackgd>),
    /// The `_diffrn_refln_scan_width` item. The variant holds the column values, one per loop row.
    ///
    /// The scan width in degrees of the scan mode defined by the code
    /// _diffrn_refln_scan_mode.
    /// Units: deg (degree of arc).
    /// range constraint recorded in the dictionary _enumeration
    Scan_width(Vec<ScanWidth>),
    /// The `_diffrn_refln_sint/lambda` item. The variant holds the column values, one per loop row.
    ///
    /// The (sin theta)/lambda value in reciprocal angstroms for this
    /// reflection.
    /// Units: A^-1^ (reciprocal angstrom).
    /// range constraint recorded in the dictionary _enumeration
    SintLambda(Vec<SintLambda>),
    /// The `_diffrn_refln_standard_code` item. The variant holds the column values, one per loop row.
    ///
    /// A code indicating that this reflection was measured as a standard
    /// reflection. The value must be '.' or match one of the
    /// _diffrn_standard_refln_code values.
    Standard_code(Vec<CharString>),
    /// The `_diffrn_refln_wavelength` item. The variant holds the column values, one per loop row.
    ///
    /// The mean wavelength in angstroms of the radiation used to measure
    /// the intensity of this reflection. This is an important parameter for
    /// reflections measured using energy-dispersive detectors or the Laue
    /// method.
    /// Units: A (angstrom).
    /// range constraint recorded in the dictionary _enumeration
    Wavelength(Vec<Wavelength>),
    /// The `_diffrn_refln_wavelength_id` item. The variant holds the column values, one per loop row.
    ///
    /// Code identifying the wavelength in the _diffrn_radiation_ list.
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
