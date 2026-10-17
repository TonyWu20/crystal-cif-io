//! The `diffrn_refln` category of the typed document layer (roadmap D12, tier T5).
//!
//! The loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<DiffrnRefln>` group: it emits
//! one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use {
    crate::data_dict::core_cif::diffrn_refln::DetectSlitHoriz,
    crate::data_dict::core_cif::diffrn_refln::DetectSlitVert,
    crate::data_dict::core_cif::diffrn_refln::ElapsedTime,
    crate::data_dict::core_cif::diffrn_refln::IntensityNet,
    crate::data_dict::core_cif::diffrn_refln::IntensitySigma,
    crate::data_dict::core_cif::diffrn_refln::IntensityU,
    crate::data_dict::core_cif::diffrn_refln::ScanMode,
    crate::data_dict::core_cif::diffrn_refln::ScanModeBackgd,
    crate::data_dict::core_cif::diffrn_refln::ScanRate,
    crate::data_dict::core_cif::diffrn_refln::ScanTimeBackgd,
    crate::data_dict::core_cif::diffrn_refln::ScanWidth,
    crate::data_dict::core_cif::diffrn_refln::SintLambda,
    crate::data_dict::core_cif::diffrn_refln::Wavelength,
};

#[derive(Debug, Clone, Default)]
/// The `diffrn_refln` category struct. One field per registry item.
pub struct DiffrnRefln {
    /// The `_diffrn_refln_angle_chi` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    pub angle_chi: Option<Numeric>,
    /// The `_diffrn_refln_angle_kappa` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    pub angle_kappa: Option<Numeric>,
    /// The `_diffrn_refln_angle_omega` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    pub angle_omega: Option<Numeric>,
    /// The `_diffrn_refln_angle_phi` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    pub angle_phi: Option<Numeric>,
    /// The `_diffrn_refln_angle_psi` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    pub angle_psi: Option<Numeric>,
    /// The `_diffrn_refln_angle_theta` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer angles of a reflection in degrees. These
    /// correspond to the specified orientation matrix and the original
    /// measured cell before any subsequent cell transformations.
    /// Units: deg (degree of arc).
    pub angle_theta: Option<Numeric>,
    /// The `_diffrn_refln_attenuator_code` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying the attenuator setting for this reflection.
    /// This code must match one of the _diffrn_attenuator_code values.
    pub attenuator_code: Option<CharString>,
    /// The `_diffrn_refln_class_code` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying the class to which this reflection has been
    /// assigned. This code must match a value of _diffrn_reflns_class_code.
    /// Reflections may be grouped into classes for a variety of purposes.
    /// For example, for modulated structures each reflection class may be
    /// defined by the number m=sum|m~i~|, where the m~i~ are the integer
    /// coefficients that, in addition to h,k,l, index the corresponding
    /// diffraction vector in the basis defined for the reciprocal lattice.
    pub class_code: Option<CharString>,
    /// The `_diffrn_refln_counts_bg_1` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    pub counts_bg_1: Option<Numeric>,
    /// The `_diffrn_refln_counts_bg_2` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    pub counts_bg_2: Option<Numeric>,
    /// The `_diffrn_refln_counts_net` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    pub counts_net: Option<Numeric>,
    /// The `_diffrn_refln_counts_peak` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    pub counts_peak: Option<Numeric>,
    /// The `_diffrn_refln_counts_total` item. The field holds the column values, one per loop row.
    ///
    /// The diffractometer counts for the measurements: background before
    /// the peak, background after the peak, net counts after background
    /// removed, counts for peak scan or position, and the total counts
    /// (background plus peak).
    pub counts_total: Option<Numeric>,
    /// The `_diffrn_refln_crystal_id` item. The field holds the column values, one per loop row.
    ///
    /// Code identifying each crystal if multiple crystals are used. Is used
    /// to link with _exptl_crystal_id in the _exptl_crystal_ list.
    pub crystal_id: Option<CharString>,
    /// The `_diffrn_refln_detect_slit_horiz` item. The field holds the column values, one per loop row.
    ///
    /// Total slit apertures in degrees in the diffraction plane (*_horiz)
    /// and perpendicular to the diffraction plane (*_vert).
    /// Units: deg (degree of arc).
    pub detect_slit_horiz: Option<DetectSlitHoriz>,
    /// The `_diffrn_refln_detect_slit_vert` item. The field holds the column values, one per loop row.
    ///
    /// Total slit apertures in degrees in the diffraction plane (*_horiz)
    /// and perpendicular to the diffraction plane (*_vert).
    /// Units: deg (degree of arc).
    pub detect_slit_vert: Option<DetectSlitVert>,
    /// The `_diffrn_refln_elapsed_time` item. The field holds the column values, one per loop row.
    ///
    /// Elapsed time in minutes from the start of the diffraction experiment
    /// to the measurement of this intensity.
    /// Units: min (minute).
    pub elapsed_time: Option<ElapsedTime>,
    /// The `_diffrn_refln_index_h` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of a measured reflection. These need not match the
    /// _refln_index_h, *_k, *_l values if a transformation of the original
    /// measured cell has taken place. Details of the cell transformation
    /// are given in _diffrn_reflns_reduction_process. See also
    /// _diffrn_reflns_transf_matrix_.
    pub index_h: Option<Numeric>,
    /// The `_diffrn_refln_index_k` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of a measured reflection. These need not match the
    /// _refln_index_h, *_k, *_l values if a transformation of the original
    /// measured cell has taken place. Details of the cell transformation
    /// are given in _diffrn_reflns_reduction_process. See also
    /// _diffrn_reflns_transf_matrix_.
    pub index_k: Option<Numeric>,
    /// The `_diffrn_refln_index_l` item. The field holds the column values, one per loop row.
    ///
    /// Miller indices of a measured reflection. These need not match the
    /// _refln_index_h, *_k, *_l values if a transformation of the original
    /// measured cell has taken place. Details of the cell transformation
    /// are given in _diffrn_reflns_reduction_process. See also
    /// _diffrn_reflns_transf_matrix_.
    pub index_l: Option<Numeric>,
    /// The `_diffrn_refln_intensity_net` item. The field holds the column values, one per loop row.
    ///
    /// Net intensity calculated from the diffraction counts after the
    /// attenuator and standard scales have been applied.
    pub intensity_net: Option<IntensityNet>,
    /// The `_diffrn_refln_intensity_sigma` item. The field holds the column values, one per loop row.
    ///
    /// Standard uncertainty (e.s.d.) of the net intensity calculated from
    /// the diffraction counts after the attenuator and standard scales have
    /// been applied.
    pub intensity_sigma: Option<IntensitySigma>,
    /// The `_diffrn_refln_intensity_u` item. The field holds the column values, one per loop row.
    ///
    /// Standard uncertainty of the net intensity calculated from the
    /// diffraction counts after the attenuator and standard scales have
    /// been applied.
    pub intensity_u: Option<IntensityU>,
    /// The `_diffrn_refln_scale_group_code` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying the scale applicable to this reflection. This
    /// code must match with a specified _diffrn_scale_group_code value.
    pub scale_group_code: Option<CharString>,
    /// The `_diffrn_refln_scan_mode` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying the mode of scanning for measurements using a
    /// diffractometer. See _diffrn_refln_scan_width and
    /// _diffrn_refln_scan_mode_backgd.
    pub scan_mode: Option<ScanMode>,
    /// The `_diffrn_refln_scan_mode_backgd` item. The field holds the column values, one per loop row.
    ///
    /// The code identifying the mode of scanning a reflection to measure
    /// the background intensity.
    pub scan_mode_backgd: Option<ScanModeBackgd>,
    /// The `_diffrn_refln_scan_rate` item. The field holds the column values, one per loop row.
    ///
    /// The rate of scanning a reflection in degrees per minute to measure
    /// the intensity.
    /// Units: deg/min (degree of arc per minute).
    pub scan_rate: Option<ScanRate>,
    /// The `_diffrn_refln_scan_time_backgd` item. The field holds the column values, one per loop row.
    ///
    /// The time spent measuring each background in seconds.
    /// Units: s (second).
    pub scan_time_backgd: Option<ScanTimeBackgd>,
    /// The `_diffrn_refln_scan_width` item. The field holds the column values, one per loop row.
    ///
    /// The scan width in degrees of the scan mode defined by the code
    /// _diffrn_refln_scan_mode.
    /// Units: deg (degree of arc).
    pub scan_width: Option<ScanWidth>,
    /// The `_diffrn_refln_sint/lambda` item. The field holds the column values, one per loop row.
    ///
    /// The (sin theta)/lambda value in reciprocal angstroms for this
    /// reflection.
    /// Units: A^-1^ (reciprocal angstrom).
    pub sint_lambda: Option<SintLambda>,
    /// The `_diffrn_refln_standard_code` item. The field holds the column values, one per loop row.
    ///
    /// A code indicating that this reflection was measured as a standard
    /// reflection. The value must be '.' or match one of the
    /// _diffrn_standard_refln_code values.
    pub standard_code: Option<CharString>,
    /// The `_diffrn_refln_wavelength` item. The field holds the column values, one per loop row.
    ///
    /// The mean wavelength in angstroms of the radiation used to measure
    /// the intensity of this reflection. This is an important parameter for
    /// reflections measured using energy-dispersive detectors or the Laue
    /// method.
    /// Units: A (angstrom).
    pub wavelength: Option<Wavelength>,
    /// The `_diffrn_refln_wavelength_id` item. The field holds the column values, one per loop row.
    ///
    /// Code identifying the wavelength in the _diffrn_radiation_ list.
    pub wavelength_id: Option<CharString>,
}
impl CifCategory for Vec<DiffrnRefln> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.angle_chi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_chi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_angle_chi".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.angle_kappa.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_kappa {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_angle_kappa".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.angle_omega.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_omega {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_angle_omega".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.angle_phi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_phi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_angle_phi".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.angle_psi.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_psi {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_angle_psi".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.angle_theta.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.angle_theta {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_angle_theta".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.attenuator_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.attenuator_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_attenuator_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.class_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.class_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_class_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.counts_bg_1.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.counts_bg_1 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_counts_bg_1".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.counts_bg_2.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.counts_bg_2 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_counts_bg_2".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.counts_net.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.counts_net {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_counts_net".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.counts_peak.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.counts_peak {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_counts_peak".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.counts_total.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.counts_total {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_counts_total".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.crystal_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.crystal_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_crystal_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.detect_slit_horiz.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.detect_slit_horiz {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_detect_slit_horiz".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.detect_slit_vert.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.detect_slit_vert {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_detect_slit_vert".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.elapsed_time.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.elapsed_time {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_elapsed_time".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_h.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_h {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_index_h".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_k.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_k {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_index_k".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.index_l.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.index_l {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_index_l".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.intensity_net.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_net {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_intensity_net".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.intensity_sigma.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_sigma {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_intensity_sigma".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.intensity_u.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.intensity_u {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_intensity_u".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scale_group_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scale_group_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_scale_group_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scan_mode.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scan_mode {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_scan_mode".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scan_mode_backgd.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scan_mode_backgd {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_scan_mode_backgd".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scan_rate.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scan_rate {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_scan_rate".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scan_time_backgd.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scan_time_backgd {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_scan_time_backgd".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.scan_width.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scan_width {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_scan_width".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.sint_lambda.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.sint_lambda {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_sint/lambda".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.standard_code.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.standard_code {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_standard_code".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.wavelength.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wavelength {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_wavelength".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.wavelength_id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wavelength_id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_refln_wavelength_id".to_string()),
                values,
            ));
        }
        if columns.is_empty() {
            return Vec::new();
        }
        vec![DataBlockMember::DataItems(DataItems::MultiValues(
            LoopColumns::new(columns),
        ))]
    }
    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::MultiValues(cols)) = m {
                let cols = cols.columns();
                // A loop is this category only when it carries one of its
                // own loop column tags. A loop without our tags stays in
                // the residue instead of overwriting the rows.
                let own_tags = [
                    "diffrn_refln_angle_chi",
                    "diffrn_refln_angle_kappa",
                    "diffrn_refln_angle_omega",
                    "diffrn_refln_angle_phi",
                    "diffrn_refln_angle_psi",
                    "diffrn_refln_angle_theta",
                    "diffrn_refln_attenuator_code",
                    "diffrn_refln_class_code",
                    "diffrn_refln_counts_bg_1",
                    "diffrn_refln_counts_bg_2",
                    "diffrn_refln_counts_net",
                    "diffrn_refln_counts_peak",
                    "diffrn_refln_counts_total",
                    "diffrn_refln_crystal_id",
                    "diffrn_refln_detect_slit_horiz",
                    "diffrn_refln_detect_slit_vert",
                    "diffrn_refln_elapsed_time",
                    "diffrn_refln_index_h",
                    "diffrn_refln_index_k",
                    "diffrn_refln_index_l",
                    "diffrn_refln_intensity_net",
                    "diffrn_refln_intensity_sigma",
                    "diffrn_refln_intensity_u",
                    "diffrn_refln_scale_group_code",
                    "diffrn_refln_scan_mode",
                    "diffrn_refln_scan_mode_backgd",
                    "diffrn_refln_scan_rate",
                    "diffrn_refln_scan_time_backgd",
                    "diffrn_refln_scan_width",
                    "diffrn_refln_sint/lambda",
                    "diffrn_refln_standard_code",
                    "diffrn_refln_wavelength",
                    "diffrn_refln_wavelength_id",
                ];
                let is_ours = cols
                    .iter()
                    .any(|c| own_tags.contains(&c.tag().as_ref().to_lowercase().as_str()));
                if is_ours {
                    merge_rows(self, cols, &mut errors);
                } else {
                    residue.push(m.clone());
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}

fn merge_rows(rows: &mut Vec<DiffrnRefln>, cols: &[LoopColumn], errors: &mut Vec<CifError>) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnRefln::default());
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_angle_chi")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_angle_chi", v) {
                Ok(val) => rows[i].angle_chi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_angle_kappa")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_angle_kappa", v) {
                Ok(val) => rows[i].angle_kappa = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_angle_omega")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_angle_omega", v) {
                Ok(val) => rows[i].angle_omega = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_angle_phi")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_angle_phi", v) {
                Ok(val) => rows[i].angle_phi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_angle_psi")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_angle_psi", v) {
                Ok(val) => rows[i].angle_psi = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_angle_theta")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_angle_theta", v) {
                Ok(val) => rows[i].angle_theta = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_attenuator_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_refln_attenuator_code", v) {
                Ok(val) => rows[i].attenuator_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_class_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_refln_class_code", v) {
                Ok(val) => rows[i].class_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_counts_bg_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_counts_bg_1", v) {
                Ok(val) => rows[i].counts_bg_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_counts_bg_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_counts_bg_2", v) {
                Ok(val) => rows[i].counts_bg_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_counts_net")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_counts_net", v) {
                Ok(val) => rows[i].counts_net = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_counts_peak")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_counts_peak", v) {
                Ok(val) => rows[i].counts_peak = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_counts_total")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_counts_total", v) {
                Ok(val) => rows[i].counts_total = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_crystal_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_refln_crystal_id", v) {
                Ok(val) => rows[i].crystal_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_detect_slit_horiz")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype(
                "diffrn_refln_detect_slit_horiz",
                v,
                DetectSlitHoriz::new,
            ) {
                Ok(val) => rows[i].detect_slit_horiz = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_detect_slit_vert")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype(
                "diffrn_refln_detect_slit_vert",
                v,
                DetectSlitVert::new,
            ) {
                Ok(val) => rows[i].detect_slit_vert = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_elapsed_time")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_refln_elapsed_time", v, ElapsedTime::new) {
                Ok(val) => rows[i].elapsed_time = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_index_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_index_h", v) {
                Ok(val) => rows[i].index_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_index_k")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_index_k", v) {
                Ok(val) => rows[i].index_k = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_index_l")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("diffrn_refln_index_l", v) {
                Ok(val) => rows[i].index_l = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_intensity_net")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_refln_intensity_net", v, IntensityNet::new) {
                Ok(val) => rows[i].intensity_net = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_intensity_sigma")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype(
                "diffrn_refln_intensity_sigma",
                v,
                IntensitySigma::new,
            ) {
                Ok(val) => rows[i].intensity_sigma = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_intensity_u")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_refln_intensity_u", v, IntensityU::new) {
                Ok(val) => rows[i].intensity_u = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_scale_group_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_refln_scale_group_code", v) {
                Ok(val) => rows[i].scale_group_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_scan_mode")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("diffrn_refln_scan_mode", v, ScanMode::parse, "ScanMode")
            {
                Ok(val) => rows[i].scan_mode = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_scan_mode_backgd")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "diffrn_refln_scan_mode_backgd",
                v,
                ScanModeBackgd::parse,
                "ScanModeBackgd",
            ) {
                Ok(val) => rows[i].scan_mode_backgd = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_scan_rate")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_refln_scan_rate", v, ScanRate::new) {
                Ok(val) => rows[i].scan_rate = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_scan_time_backgd")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype(
                "diffrn_refln_scan_time_backgd",
                v,
                ScanTimeBackgd::new,
            ) {
                Ok(val) => rows[i].scan_time_backgd = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_scan_width")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_refln_scan_width", v, ScanWidth::new) {
                Ok(val) => rows[i].scan_width = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_sint/lambda")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_refln_sint/lambda", v, SintLambda::new) {
                Ok(val) => rows[i].sint_lambda = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_standard_code")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_refln_standard_code", v) {
                Ok(val) => rows[i].standard_code = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_wavelength")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_refln_wavelength", v, Wavelength::new) {
                Ok(val) => rows[i].wavelength = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_refln_wavelength_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_refln_wavelength_id", v) {
                Ok(val) => rows[i].wavelength_id = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
