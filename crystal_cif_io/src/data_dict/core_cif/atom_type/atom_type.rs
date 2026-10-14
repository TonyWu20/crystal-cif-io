//! The `atom_type` category of the typed document layer (roadmap D12).
//!
//! The `AtomType` single-value struct (the T6 element properties)
//! and the `AtomTypeRow` loop row (the type properties and the
//! scattering-factor columns).

use crystal_cif_fmt::grammar::{Tag, Numeric, CharString, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};
use {crate::data_dict::core_cif::atom_type::{AnalyticalMass, NumberInCell, OxidationNumber, Radius}};
use crate::data_dict::core_cif::{convert, CifCategory};
use crate::core_cif::CifError;

#[derive(Debug, Clone, Default)]
pub struct AtomType {
    pub atomic_mass: Option<Numeric>,
    pub atomic_number: Option<Numeric>,
    pub display_colour: Option<CharString>,
    pub electron_count: Option<Numeric>,
    pub element_symbol: Option<CharString>,
    pub mass_number: Option<Numeric>,
}

impl CifCategory for AtomType {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.atomic_mass {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("atom_type_atomic_mass".to_string()),
                Value::Numeric(*v),
            )))));
        }
        if let Some(v) = &self.atomic_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("atom_type_atomic_number".to_string()),
                Value::Numeric(*v),
            )))));
        }
        if let Some(v) = &self.display_colour {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("atom_type_display_colour".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.electron_count {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("atom_type_electron_count".to_string()),
                Value::Numeric(*v),
            )))));
        }
        if let Some(v) = &self.element_symbol {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("atom_type_element_symbol".to_string()),
                Value::CharString(v.clone()),
            )))));
        }
        if let Some(v) = &self.mass_number {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("atom_type_mass_number".to_string()),
                Value::Numeric(*v),
            )))));
        }
        out
    }

    fn from_members(&mut self, members: &[DataBlockMember]) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "atom_type_atomic_mass" => {
                        match convert::read_numeric("atom_type_atomic_mass", value) {
                            Ok(v) => self.atomic_mass = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "atom_type_atomic_number" => {
                        match convert::read_numeric("atom_type_atomic_number", value) {
                            Ok(v) => self.atomic_number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "atom_type_display_colour" => {
                        match convert::read_char("atom_type_display_colour", value) {
                            Ok(v) => self.display_colour = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "atom_type_electron_count" => {
                        match convert::read_numeric("atom_type_electron_count", value) {
                            Ok(v) => self.electron_count = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "atom_type_element_symbol" => {
                        match convert::read_char("atom_type_element_symbol", value) {
                            Ok(v) => self.element_symbol = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "atom_type_mass_number" => {
                        match convert::read_numeric("atom_type_mass_number", value) {
                            Ok(v) => self.mass_number = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    _ => residue.push(m.clone()),
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}

#[derive(Debug, Clone, Default)]
pub struct AtomTypeRow {
    pub analytical_mass_percent: Option<AnalyticalMass>,
    pub description: Option<CharString>,
    pub number_in_cell: Option<NumberInCell>,
    pub oxidation_number: Option<OxidationNumber>,
    pub radius_bond: Option<Radius>,
    pub radius_contact: Option<Radius>,
    pub scat_cromer_mann_a1: Option<Numeric>,
    pub scat_cromer_mann_a2: Option<Numeric>,
    pub scat_cromer_mann_a3: Option<Numeric>,
    pub scat_cromer_mann_a4: Option<Numeric>,
    pub scat_cromer_mann_b1: Option<Numeric>,
    pub scat_cromer_mann_b2: Option<Numeric>,
    pub scat_cromer_mann_b3: Option<Numeric>,
    pub scat_cromer_mann_b4: Option<Numeric>,
    pub scat_cromer_mann_c: Option<Numeric>,
    pub scat_dispersion_imag: Option<Numeric>,
    pub scat_dispersion_real: Option<Numeric>,
    pub scat_dispersion_source: Option<CharString>,
    pub scat_length_neutron: Option<Numeric>,
    pub scat_source: Option<CharString>,
    pub scat_versus_stol_list: Option<CharString>,
    pub symbol: Option<CharString>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "atom_type_analytical_mass_%",
    "atom_type_description",
    "atom_type_number_in_cell",
    "atom_type_oxidation_number",
    "atom_type_radius_bond",
    "atom_type_radius_contact",
    "atom_type_scat_cromer_mann_a1",
    "atom_type_scat_cromer_mann_a2",
    "atom_type_scat_cromer_mann_a3",
    "atom_type_scat_cromer_mann_a4",
    "atom_type_scat_cromer_mann_b1",
    "atom_type_scat_cromer_mann_b2",
    "atom_type_scat_cromer_mann_b3",
    "atom_type_scat_cromer_mann_b4",
    "atom_type_scat_cromer_mann_c",
    "atom_type_scat_dispersion_imag",
    "atom_type_scat_dispersion_real",
    "atom_type_scat_dispersion_source",
    "atom_type_scat_length_neutron",
    "atom_type_scat_source",
    "atom_type_scat_versus_stol_list",
    "atom_type_symbol",
];

impl CifCategory for Vec<AtomTypeRow> {
    fn to_members(&self) -> Vec<DataBlockMember> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut columns = Vec::new();
        if self.iter().any(|r| r.analytical_mass_percent.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.analytical_mass_percent {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_analytical_mass_%".to_string()), values));
        }
        if self.iter().any(|r| r.description.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.description {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_description".to_string()), values));
        }
        if self.iter().any(|r| r.number_in_cell.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.number_in_cell {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_number_in_cell".to_string()), values));
        }
        if self.iter().any(|r| r.oxidation_number.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.oxidation_number {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_oxidation_number".to_string()), values));
        }
        if self.iter().any(|r| r.radius_bond.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.radius_bond {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_radius_bond".to_string()), values));
        }
        if self.iter().any(|r| r.radius_contact.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.radius_contact {
                    Some(v) => Value::from(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_radius_contact".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_a1.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_a1 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_a1".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_a2.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_a2 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_a2".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_a3.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_a3 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_a3".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_a4.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_a4 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_a4".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_b1.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_b1 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_b1".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_b2.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_b2 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_b2".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_b3.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_b3 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_b3".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_b4.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_b4 {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_b4".to_string()), values));
        }
        if self.iter().any(|r| r.scat_cromer_mann_c.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_cromer_mann_c {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_Cromer_Mann_c".to_string()), values));
        }
        if self.iter().any(|r| r.scat_dispersion_imag.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_dispersion_imag {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_dispersion_imag".to_string()), values));
        }
        if self.iter().any(|r| r.scat_dispersion_real.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_dispersion_real {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_dispersion_real".to_string()), values));
        }
        if self.iter().any(|r| r.scat_dispersion_source.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_dispersion_source {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_dispersion_source".to_string()), values));
        }
        if self.iter().any(|r| r.scat_length_neutron.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_length_neutron {
                    Some(v) => Value::Numeric(*v),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_length_neutron".to_string()), values));
        }
        if self.iter().any(|r| r.scat_source.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_source {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_source".to_string()), values));
        }
        if self.iter().any(|r| r.scat_versus_stol_list.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.scat_versus_stol_list {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_scat_versus_stol_list".to_string()), values));
        }
        if self.iter().any(|r| r.symbol.is_some()) {
            let values: Vec<Value> = self
                .iter()
                .map(|r| match &r.symbol {
                    Some(v) => Value::CharString(v.clone()),
                    None => Value::Inapplicable,
                })
                .collect();
            columns.push(LoopColumn::new(Tag::new("atom_type_symbol".to_string()), values));
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
                    let is_ours = cols
                        .iter()
                        .any(|c| LOOP_TAGS.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if !is_ours {
                        residue.push(m.clone());
                    } else {
                        *self = rows_from_columns(cols, &mut errors);
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<AtomTypeRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![AtomTypeRow::default(); n];
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_analytical_mass_%") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_analytical_mass("atom_type_analytical_mass_%", v) {
                Ok(val) => rows[i].analytical_mass_percent = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_description") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_type_description", v) {
                Ok(val) => rows[i].description = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_number_in_cell") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_number_in_cell("atom_type_number_in_cell", v) {
                Ok(val) => rows[i].number_in_cell = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_oxidation_number") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_oxidation_number("atom_type_oxidation_number", v) {
                Ok(val) => rows[i].oxidation_number = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_radius_bond") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_radius("atom_type_radius_bond", v) {
                Ok(val) => rows[i].radius_bond = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_radius_contact") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_radius("atom_type_radius_contact", v) {
                Ok(val) => rows[i].radius_contact = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_a1") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_a1", v) {
                Ok(val) => rows[i].scat_cromer_mann_a1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_a2") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_a2", v) {
                Ok(val) => rows[i].scat_cromer_mann_a2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_a3") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_a3", v) {
                Ok(val) => rows[i].scat_cromer_mann_a3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_a4") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_a4", v) {
                Ok(val) => rows[i].scat_cromer_mann_a4 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_b1") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_b1", v) {
                Ok(val) => rows[i].scat_cromer_mann_b1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_b2") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_b2", v) {
                Ok(val) => rows[i].scat_cromer_mann_b2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_b3") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_b3", v) {
                Ok(val) => rows[i].scat_cromer_mann_b3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_b4") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_b4", v) {
                Ok(val) => rows[i].scat_cromer_mann_b4 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_cromer_mann_c") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_cromer_mann_c", v) {
                Ok(val) => rows[i].scat_cromer_mann_c = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_dispersion_imag") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_dispersion_imag", v) {
                Ok(val) => rows[i].scat_dispersion_imag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_dispersion_real") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_dispersion_real", v) {
                Ok(val) => rows[i].scat_dispersion_real = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_dispersion_source") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_type_scat_dispersion_source", v) {
                Ok(val) => rows[i].scat_dispersion_source = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_length_neutron") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_type_scat_length_neutron", v) {
                Ok(val) => rows[i].scat_length_neutron = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_source") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_type_scat_source", v) {
                Ok(val) => rows[i].scat_source = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_scat_versus_stol_list") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_type_scat_versus_stol_list", v) {
                Ok(val) => rows[i].scat_versus_stol_list = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols.iter().find(|c| c.tag().as_ref().to_lowercase() == "atom_type_symbol") {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("atom_type_symbol", v) {
                Ok(val) => rows[i].symbol = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
