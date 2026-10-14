//! The `exptl_crystal` category of the typed document layer (roadmap D12).
//!
//! The `exptl_crystal` loop-row struct. One `Option` field per
//! registry column. `CifCategory` is on the `Vec<ExptlCrystal>` group:
//! it emits one `MultiValues` loop and fills the rows from the columns.
//! The colour closed sets keep the typed enums.

use crystal_cif_fmt::grammar::{Tag, Value, Numeric, CharString};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns};
use crate::data_dict::core_cif::exptl_crystal::{ColourModifier, ColourPrimary, Lustre};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct ExptlCrystal {
    pub colour: Option<CharString>,
    pub colour_lustre: Option<Lustre>,
    pub colour_modifier: Option<ColourModifier>,
    pub colour_primary: Option<ColourPrimary>,
    pub density_diffrn: Option<Numeric>,
    pub density_meas: Option<Numeric>,
    pub density_meas_gt: Option<Numeric>,
    pub density_meas_lt: Option<Numeric>,
    pub density_meas_temp: Option<Numeric>,
    pub density_meas_temp_gt: Option<Numeric>,
    pub density_meas_temp_lt: Option<Numeric>,
    pub density_method: Option<CharString>,
    pub description: Option<CharString>,
    pub f_000: Option<Numeric>,
    pub id: Option<CharString>,
    pub preparation: Option<CharString>,
    pub pressure_history: Option<CharString>,
    pub recrystallization_method: Option<CharString>,
    pub size_length: Option<Numeric>,
    pub size_max: Option<Numeric>,
    pub size_mid: Option<Numeric>,
    pub size_min: Option<Numeric>,
    pub size_rad: Option<Numeric>,
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
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_colour".to_string()), values));
        }
        if self.iter().any(|r| r.colour_lustre.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.colour_lustre {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_colour_lustre".to_string()), values));
        }
        if self.iter().any(|r| r.colour_modifier.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.colour_modifier {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_colour_modifier".to_string()), values));
        }
        if self.iter().any(|r| r.colour_primary.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.colour_primary {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_colour_primary".to_string()), values));
        }
        if self.iter().any(|r| r.density_diffrn.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_diffrn {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_diffrn".to_string()), values));
        }
        if self.iter().any(|r| r.density_meas.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_meas".to_string()), values));
        }
        if self.iter().any(|r| r.density_meas_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_meas_gt".to_string()), values));
        }
        if self.iter().any(|r| r.density_meas_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_lt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_meas_lt".to_string()), values));
        }
        if self.iter().any(|r| r.density_meas_temp.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_temp {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_meas_temp".to_string()), values));
        }
        if self.iter().any(|r| r.density_meas_temp_gt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_temp_gt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_meas_temp_gt".to_string()), values));
        }
        if self.iter().any(|r| r.density_meas_temp_lt.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_meas_temp_lt {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_meas_temp_lt".to_string()), values));
        }
        if self.iter().any(|r| r.density_method.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.density_method {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_density_method".to_string()), values));
        }
        if self.iter().any(|r| r.description.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.description {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_description".to_string()), values));
        }
        if self.iter().any(|r| r.f_000.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.f_000 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_F_000".to_string()), values));
        }
        if self.iter().any(|r| r.id.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.id {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_id".to_string()), values));
        }
        if self.iter().any(|r| r.preparation.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.preparation {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_preparation".to_string()), values));
        }
        if self.iter().any(|r| r.pressure_history.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.pressure_history {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_pressure_history".to_string()), values));
        }
        if self.iter().any(|r| r.recrystallization_method.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.recrystallization_method {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_recrystallization_method".to_string()), values));
        }
        if self.iter().any(|r| r.size_length.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_length {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_size_length".to_string()), values));
        }
        if self.iter().any(|r| r.size_max.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_max {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_size_max".to_string()), values));
        }
        if self.iter().any(|r| r.size_mid.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_mid {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_size_mid".to_string()), values));
        }
        if self.iter().any(|r| r.size_min.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_min {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_size_min".to_string()), values));
        }
        if self.iter().any(|r| r.size_rad.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.size_rad {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_size_rad".to_string()), values));
        }
        if self.iter().any(|r| r.thermal_history.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.thermal_history {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("exptl_crystal_thermal_history".to_string()), values));
        }
        if columns.is_empty() {
            return Vec::new();
        }
        vec![DataBlockMember::DataItems(DataItems::MultiValues(LoopColumns::new(columns)))]
    }

    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
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

fn exptlcrystal_rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ExptlCrystal> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ExptlCrystal::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_colour", v) {
                Ok(val) => rows[i].colour = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour_lustre") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("exptl_crystal_colour_lustre", v, |c| Lustre::try_from(c).ok(), "Lustre") {
                Ok(val) => rows[i].colour_lustre = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour_modifier") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("exptl_crystal_colour_modifier", v, |c| ColourModifier::try_from(c).ok(), "ColourModifier") {
                Ok(val) => rows[i].colour_modifier = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_colour_primary") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("exptl_crystal_colour_primary", v, |c| ColourPrimary::try_from(c).ok(), "ColourPrimary") {
                Ok(val) => rows[i].colour_primary = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_diffrn") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_diffrn", v) {
                Ok(val) => rows[i].density_diffrn = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas", v) {
                Ok(val) => rows[i].density_meas = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_gt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_gt", v) {
                Ok(val) => rows[i].density_meas_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_lt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_lt", v) {
                Ok(val) => rows[i].density_meas_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_temp") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_temp", v) {
                Ok(val) => rows[i].density_meas_temp = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_temp_gt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_temp_gt", v) {
                Ok(val) => rows[i].density_meas_temp_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_meas_temp_lt") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_density_meas_temp_lt", v) {
                Ok(val) => rows[i].density_meas_temp_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_density_method") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_density_method", v) {
                Ok(val) => rows[i].density_method = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_description") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_description", v) {
                Ok(val) => rows[i].description = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_f_000") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_f_000", v) {
                Ok(val) => rows[i].f_000 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_id") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_id", v) {
                Ok(val) => rows[i].id = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_preparation") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_preparation", v) {
                Ok(val) => rows[i].preparation = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_pressure_history") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_pressure_history", v) {
                Ok(val) => rows[i].pressure_history = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_recrystallization_method") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_recrystallization_method", v) {
                Ok(val) => rows[i].recrystallization_method = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_length") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_length", v) {
                Ok(val) => rows[i].size_length = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_max") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_max", v) {
                Ok(val) => rows[i].size_max = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_mid") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_mid", v) {
                Ok(val) => rows[i].size_mid = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_min") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_min", v) {
                Ok(val) => rows[i].size_min = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_size_rad") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("exptl_crystal_size_rad", v) {
                Ok(val) => rows[i].size_rad = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "exptl_crystal_thermal_history") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("exptl_crystal_thermal_history", v) {
                Ok(val) => rows[i].thermal_history = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
