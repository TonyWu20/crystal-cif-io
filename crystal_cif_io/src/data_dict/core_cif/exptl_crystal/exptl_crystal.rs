//! The `exptl_crystal` category of the typed document layer (roadmap D12).
//!
//! The `exptl_crystal` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<ExptlCrystal>` group:
//! it emits one `MultiValues` loop and fills the rows from the columns.
//! The colour closed sets keep the typed enums.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::exptl_crystal::{ColourModifier, ColourPrimary, Lustre};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};

#[derive(Debug, Clone, Default)]
/// The `exptl_crystal` category struct. One field per registry item.
pub struct ExptlCrystal {
    /// The `_exptl_crystal_colour` item. The field holds the column values, one per loop row.
    ///
    /// The colour of the crystal.
    pub colour: Option<CharString>,
    /// The `_exptl_crystal_colour_lustre` item. The field holds the column values, one per loop row.
    ///
    /// The enumeration list of standardized names developed for the
    /// International Centre for Diffraction Data. The colour of a crystal
    /// is given by the combination of _exptl_crystal_colour_modifier with
    /// _exptl_crystal_colour_primary, as in 'dark-green' or
    /// 'bluish-violet', if necessary combined with
    /// _exptl_crystal_colour_lustre, as in 'metallic-green'. Note that
    /// 'dull' and 'clear' should no longer be used.
    pub colour_lustre: Option<Lustre>,
    /// The `_exptl_crystal_colour_modifier` item. The field holds the column values, one per loop row.
    ///
    /// The enumeration list of standardized names developed for the
    /// International Centre for Diffraction Data. The colour of a crystal
    /// is given by the combination of _exptl_crystal_colour_modifier with
    /// _exptl_crystal_colour_primary, as in 'dark-green' or
    /// 'bluish-violet', if necessary combined with
    /// _exptl_crystal_colour_lustre, as in 'metallic-green'.
    pub colour_modifier: Option<ColourModifier>,
    /// The `_exptl_crystal_colour_primary` item. The field holds the column values, one per loop row.
    ///
    /// The enumeration list of standardized names developed for the
    /// International Centre for Diffraction Data. The colour of a crystal
    /// is given by the combination of _exptl_crystal_colour_modifier with
    /// _exptl_crystal_colour_primary, as in 'dark-green' or
    /// 'bluish-violet', if necessary combined with
    /// _exptl_crystal_colour_lustre, as in 'metallic-green'.
    pub colour_primary: Option<ColourPrimary>,
    /// The `_exptl_crystal_density_diffrn` item. The field holds the column values, one per loop row.
    ///
    /// Density values calculated from the crystal cell and contents. The
    /// units are megagrams per cubic metre (grams per cubic centimetre).
    /// Units: Mg/m^3^ (megagram per metre cubed).
    pub density_diffrn: Option<Numeric>,
    /// The `_exptl_crystal_density_meas` item. The field holds the column values, one per loop row.
    ///
    /// Density values measured using standard chemical and physical
    /// methods. The units are megagrams per cubic metre (grams per cubic
    /// centimetre).
    /// Units: Mg/m^3^ (megagram per metre cubed).
    pub density_meas: Option<Numeric>,
    /// The `_exptl_crystal_density_meas_gt` item. The field holds the column values, one per loop row.
    ///
    /// The value above which the density measured using standard chemical
    /// and physical methods lies. The units are megagrams per cubic metre
    /// (grams per cubic centimetre). _exptl_crystal_density_meas_gt and
    /// _exptl_crystal_density_meas_lt should not be used to report new
    /// experimental work, for which _exptl_crystal_density_meas should be
    /// used. These items are intended for use in reporting information in
    /// existing databases and archives which would be misleading if
    /// reported under _exptl_crystal_density_meas.
    /// Units: Mg/m^3^ (megagram per metre cubed).
    pub density_meas_gt: Option<Numeric>,
    /// The `_exptl_crystal_density_meas_lt` item. The field holds the column values, one per loop row.
    ///
    /// The value below which the density measured using standard chemical
    /// and physical methods lies. The units are megagrams per cubic metre
    /// (grams per cubic centimetre). _exptl_crystal_density_meas_gt and
    /// _exptl_crystal_density_meas_lt should not be used to report new
    /// experimental work, for which _exptl_crystal_density_meas should be
    /// used. These items are intended for use in reporting information in
    /// existing databases and archives which would be misleading if
    /// reported under _exptl_crystal_density_meas.
    /// Units: Mg/m^3^ (megagram per metre cubed).
    pub density_meas_lt: Option<Numeric>,
    /// The `_exptl_crystal_density_meas_temp` item. The field holds the column values, one per loop row.
    ///
    /// Temperature in kelvins at which _exptl_crystal_density_meas was
    /// determined.
    /// Units: K (kelvin).
    pub density_meas_temp: Option<Numeric>,
    /// The `_exptl_crystal_density_meas_temp_gt` item. The field holds the column values, one per loop row.
    ///
    /// Temperature in kelvins above which _exptl_crystal_density_meas was
    /// determined. _exptl_crystal_density_meas_temp_gt and
    /// _exptl_crystal_density_meas_temp_lt should not be used for reporting
    /// new work, for which the correct temperature of measurement should be
    /// given. These items are intended for use in reporting information
    /// stored in databases or archives which would be misleading if
    /// reported under _exptl_crystal_density_meas_temp.
    /// Units: K (kelvin).
    pub density_meas_temp_gt: Option<Numeric>,
    /// The `_exptl_crystal_density_meas_temp_lt` item. The field holds the column values, one per loop row.
    ///
    /// Temperature in kelvins below which _exptl_crystal_density_meas was
    /// determined. _exptl_crystal_density_meas_temp_gt and
    /// _exptl_crystal_density_meas_temp_lt should not be used for reporting
    /// new work, for which the correct temperature of measurement should be
    /// given. These items are intended for use in reporting information
    /// stored in databases or archives which would be misleading if
    /// reported under _exptl_crystal_density_meas_temp.
    /// Units: K (kelvin).
    pub density_meas_temp_lt: Option<Numeric>,
    /// The `_exptl_crystal_density_method` item. The field holds the column values, one per loop row.
    ///
    /// The method used to measure _exptl_crystal_density_meas.
    pub density_method: Option<CharString>,
    /// The `_exptl_crystal_description` item. The field holds the column values, one per loop row.
    ///
    /// A description of the quality and habit of the crystal. The crystal
    /// dimensions should not normally be reported here; use instead
    /// _exptl_crystal_size_ for the gross dimensions of the crystal and
    /// _exptl_crystal_face_ to describe the relationship between individual
    /// faces.
    pub description: Option<CharString>,
    /// The `_exptl_crystal_F_000` item. The field holds the column values, one per loop row.
    ///
    /// The expression for a structure factor evaluated in the zeroth-order
    /// case h = k = l = 0, F(000). This may contain dispersion
    /// contributions and is calculated as F(000) = [ (sum f~r~)^2^ + (sum
    /// f~i~)^2^ ]^1/2^ f~r~ = real part of the scattering factors at theta
    /// = 0 f~i~ = imaginary part of the scattering factors at theta = 0 the
    /// sum is taken over each atom in the unit cell For X-rays,
    /// non-dispersive F(000) is a positive number and counts the effective
    /// number of electrons in the unit cell; for neutrons, non-dispersive
    /// F(000) (which may be negative) counts the total nuclear scattering
    /// power in the unit cell. See
    /// `http://reference.iucr.org/dictionary/F(000`)
    pub f_000: Option<Numeric>,
    /// The `_exptl_crystal_id` item. The field holds the column values, one per loop row.
    ///
    /// Code identifying each crystal if multiple crystals are used. It is
    /// used to link with _diffrn_refln_crystal_id in the intensity
    /// measurements and with _refln_crystal_id in the _refln_ list.
    pub id: Option<CharString>,
    /// The `_exptl_crystal_preparation` item. The field holds the column values, one per loop row.
    ///
    /// Details of crystal growth and preparation of the crystal (e.g.
    /// mounting) prior to the intensity measurements.
    pub preparation: Option<CharString>,
    /// The `_exptl_crystal_pressure_history` item. The field holds the column values, one per loop row.
    ///
    /// Relevant details concerning the pressure history of the sample.
    pub pressure_history: Option<CharString>,
    /// The `_exptl_crystal_recrystallization_method` item. The field holds the column values, one per loop row.
    ///
    /// Describes the method used to recrystallize the sample. Sufficient
    /// details should be given for the procedure to be repeated. The
    /// temperature or temperatures should be given as well as details of
    /// the solvent, flux or carrier gas with concentrations or pressures
    /// and ambient atmosphere.
    pub recrystallization_method: Option<CharString>,
    /// The `_exptl_crystal_size_length` item. The field holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    pub size_length: Option<Numeric>,
    /// The `_exptl_crystal_size_max` item. The field holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    pub size_max: Option<Numeric>,
    /// The `_exptl_crystal_size_mid` item. The field holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    pub size_mid: Option<Numeric>,
    /// The `_exptl_crystal_size_min` item. The field holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    pub size_min: Option<Numeric>,
    /// The `_exptl_crystal_size_rad` item. The field holds the column values, one per loop row.
    ///
    /// The maximum, medial and minimum dimensions in millimetres of the
    /// crystal. If the crystal is a sphere, then the *_rad item is its
    /// radius. If the crystal is a cylinder, then the *_rad item is its
    /// radius and the *_length item is its length. These may appear in a
    /// list with _exptl_crystal_id if multiple crystals are used in the
    /// experiment.
    /// Units: mm (millimetre).
    pub size_rad: Option<Numeric>,
    /// The `_exptl_crystal_thermal_history` item. The field holds the column values, one per loop row.
    ///
    /// Relevant details concerning the thermal history of the sample.
    pub thermal_history: Option<CharString>,
}

impl CifCategory for Vec<ExptlCrystal> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.colour.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.colour {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_colour".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.colour_lustre.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.colour_lustre {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_colour_lustre".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.colour_modifier.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.colour_modifier {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_colour_modifier".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.colour_primary.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.colour_primary {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_colour_primary".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_diffrn.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_diffrn {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_diffrn".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_meas".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_meas_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_meas_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_meas_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_lt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_meas_lt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_meas_temp.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_temp {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_meas_temp".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_meas_temp_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_temp_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_meas_temp_gt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_meas_temp_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_temp_lt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_meas_temp_lt".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.density_method.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_method {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_density_method".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.description.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.description {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_description".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.f_000.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_000 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_F_000".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.preparation.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.preparation {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_preparation".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.pressure_history.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.pressure_history {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_pressure_history".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.recrystallization_method.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.recrystallization_method {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_recrystallization_method".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.size_length.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_length {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_size_length".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.size_max.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_max {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_size_max".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.size_mid.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_mid {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_size_mid".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.size_min.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_min {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_size_min".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.size_rad.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_rad {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_size_rad".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.thermal_history.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.thermal_history {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("exptl_crystal_thermal_history".to_string()),
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
            match m {
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    // A loop is this category only when it carries one of
                    // the category column tags. A sibling loop of another
                    // category stays in the residue.
                    let ours = [
                        "exptl_crystal_colour",
                        "exptl_crystal_colour_lustre",
                        "exptl_crystal_colour_modifier",
                        "exptl_crystal_colour_primary",
                        "exptl_crystal_density_diffrn",
                        "exptl_crystal_density_meas",
                        "exptl_crystal_density_meas_gt",
                        "exptl_crystal_density_meas_lt",
                        "exptl_crystal_density_meas_temp",
                        "exptl_crystal_density_meas_temp_gt",
                        "exptl_crystal_density_meas_temp_lt",
                        "exptl_crystal_density_method",
                        "exptl_crystal_description",
                        "exptl_crystal_f_000",
                        "exptl_crystal_id",
                        "exptl_crystal_preparation",
                        "exptl_crystal_pressure_history",
                        "exptl_crystal_recrystallization_method",
                        "exptl_crystal_size_length",
                        "exptl_crystal_size_max",
                        "exptl_crystal_size_mid",
                        "exptl_crystal_size_min",
                        "exptl_crystal_size_rad",
                        "exptl_crystal_thermal_history",
                    ];
                    let is_ours = cols
                        .iter()
                        .any(|c| ours.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = exptlcrystal_rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn exptlcrystal_rows_from_columns(
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) -> Vec<ExptlCrystal> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ExptlCrystal::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_colour", v) {
                Ok(val) => rows[i].colour = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour_lustre")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "exptl_crystal_colour_lustre",
                v,
                |c| Lustre::try_from(c).ok(),
                "Lustre",
            ) {
                Ok(val) => rows[i].colour_lustre = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour_modifier")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "exptl_crystal_colour_modifier",
                v,
                |c| ColourModifier::try_from(c).ok(),
                "ColourModifier",
            ) {
                Ok(val) => rows[i].colour_modifier = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour_primary")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "exptl_crystal_colour_primary",
                v,
                |c| ColourPrimary::try_from(c).ok(),
                "ColourPrimary",
            ) {
                Ok(val) => rows[i].colour_primary = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_diffrn")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_diffrn", v) {
                Ok(val) => rows[i].density_diffrn = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas", v) {
                Ok(val) => rows[i].density_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_gt", v) {
                Ok(val) => rows[i].density_meas_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_lt", v) {
                Ok(val) => rows[i].density_meas_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_temp")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_temp", v) {
                Ok(val) => rows[i].density_meas_temp = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_temp_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_temp_gt", v) {
                Ok(val) => rows[i].density_meas_temp_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_temp_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_temp_lt", v) {
                Ok(val) => rows[i].density_meas_temp_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_method")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_density_method", v) {
                Ok(val) => rows[i].density_method = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_description")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_description", v) {
                Ok(val) => rows[i].description = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_f_000")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_f_000", v) {
                Ok(val) => rows[i].f_000 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_id", v) {
                Ok(val) => rows[i].id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_preparation")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_preparation", v) {
                Ok(val) => rows[i].preparation = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_pressure_history")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_pressure_history", v) {
                Ok(val) => rows[i].pressure_history = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_recrystallization_method")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_recrystallization_method", v) {
                Ok(val) => rows[i].recrystallization_method = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_length")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_length", v) {
                Ok(val) => rows[i].size_length = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_max")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_max", v) {
                Ok(val) => rows[i].size_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_mid")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_mid", v) {
                Ok(val) => rows[i].size_mid = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_min")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_min", v) {
                Ok(val) => rows[i].size_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_rad")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_rad", v) {
                Ok(val) => rows[i].size_rad = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_thermal_history")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_thermal_history", v) {
                Ok(val) => rows[i].thermal_history = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
