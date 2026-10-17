//! The `diffrn_radiation_wavelength` category of the typed document layer (roadmap D12, tier T5).
//!
//! The loop-row struct. One `Option` field per registry column.
//! `CifCategory` is on the `Vec<DiffrnRadiationWavelengthRow>` group: it emits
//! one `MultiValues` loop and fills the rows from the columns.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use {
    crate::data_dict::core_cif::diffrn_radiation_wavelength::Determination,
    crate::data_dict::core_cif::diffrn_radiation_wavelength::DiffrnRadiationWavelength,
    crate::data_dict::core_cif::diffrn_radiation_wavelength::Wt,
};

#[derive(Debug, Clone, Default)]
/// One loop row of the `diffrn_radiation_wavelength` category: the per-observation values of the loop items.
pub struct DiffrnRadiationWavelengthRow {
    /// The `_diffrn_radiation_wavelength` item. The field holds the column values, one per loop row.
    ///
    /// The radiation wavelength in angstroms.
    /// Units: A (angstrom).
    pub diffrn_radiation_wavelength: Option<DiffrnRadiationWavelength>,
    /// The `_diffrn_radiation_wavelength_details` item. The field holds the column values, one per loop row.
    ///
    /// Information about the determination of the radiation wavelength that
    /// is not conveyed completely by an enumerated value of
    /// _diffrn_radiation_wavelength_determination.
    pub details: Option<CharString>,
    /// The `_diffrn_radiation_wavelength_determination` item. The field holds the column values, one per loop row.
    ///
    /// The method of determination of incident wavelength.
    pub determination: Option<Determination>,
    /// The `_diffrn_radiation_wavelength_id` item. The field holds the column values, one per loop row.
    ///
    /// An arbitrary code identifying each value of
    /// _diffrn_radiation_wavelength. Items in the DIFFRN_RADIATION category
    /// are looped when multiple wavelengths are used. This code is used to
    /// link with the _diffrn_refln_ list. It must match with one of the
    /// _diffrn_refln_wavelength_id codes.
    pub id: Option<CharString>,
    /// The `_diffrn_radiation_wavelength_wt` item. The field holds the column values, one per loop row.
    ///
    /// The relative weight of a wavelength identified by the code
    /// _diffrn_radiation_wavelength_id in the list of wavelengths.
    pub wt: Option<Wt>,
}
impl CifCategory for Vec<DiffrnRadiationWavelengthRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.diffrn_radiation_wavelength.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.diffrn_radiation_wavelength {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_radiation_wavelength".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.details.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.details {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_radiation_wavelength_details".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.determination.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.determination {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_radiation_wavelength_determination".to_string()),
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
                Tag::new("diffrn_radiation_wavelength_id".to_string()),
                values,
            ));
        }
        if self.iter().any(|r| r.wt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.wt {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(
                Tag::new("diffrn_radiation_wavelength_wt".to_string()),
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
                    "diffrn_radiation_wavelength",
                    "diffrn_radiation_wavelength_details",
                    "diffrn_radiation_wavelength_determination",
                    "diffrn_radiation_wavelength_id",
                    "diffrn_radiation_wavelength_wt",
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

fn merge_rows(
    rows: &mut Vec<DiffrnRadiationWavelengthRow>,
    cols: &[LoopColumn],
    errors: &mut Vec<CifError>,
) {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    rows.resize(n, DiffrnRadiationWavelengthRow::default());
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_radiation_wavelength")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype(
                "diffrn_radiation_wavelength",
                v,
                DiffrnRadiationWavelength::new,
            ) {
                Ok(val) => rows[i].diffrn_radiation_wavelength = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_radiation_wavelength_details")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_radiation_wavelength_details", v) {
                Ok(val) => rows[i].details = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_radiation_wavelength_determination")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set(
                "diffrn_radiation_wavelength_determination",
                v,
                Determination::parse,
                "Determination",
            ) {
                Ok(val) => rows[i].determination = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_radiation_wavelength_id")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("diffrn_radiation_wavelength_id", v) {
                Ok(val) => rows[i].id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "diffrn_radiation_wavelength_wt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_ranged_newtype("diffrn_radiation_wavelength_wt", v, Wt::new) {
                Ok(val) => rows[i].wt = val,
                Err(e) => errors.push(e),
            }
        }
    }
}
