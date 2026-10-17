//! The `chemical` category of the typed document layer (roadmap D12).
//!
//! The `Chemical` struct owns the single-value items (the compound
//! names, the temperatures, and the enantiomeric-excess items) and
//! the loop rows (`rows`, the temperature range companions).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::chemical::{
    AbsoluteConfiguration, Enantioexcess, EnantioexcessBulkTechnique,
    EnantioexcessCrystalTechnique, Temperature,
};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `chemical` category struct. One field per registry item.
pub struct Chemical {
    /// The `_chemical_absolute_configuration` item. The field holds the single value.
    ///
    /// Necessary conditions for the assignment of
    /// _chemical_absolute_configuration are given by H. D. Flack and G.
    /// Bernardinelli (1999, 2000). Ref: Flack, H. D. & Bernardinelli, G.
    /// (1999). Acta Cryst. A55, 908-915. (`http://www.iucr.org/paper?sh0129`)
    /// Flack, H. D. & Bernardinelli, G. (2000). J. Appl. Cryst. 33,
    /// 1143-1148. (`http://www.iucr.org/paper?ks0021`)
    pub absolute_configuration: Option<AbsoluteConfiguration>,
    /// The `_chemical_compound_source` item. The field holds the single value.
    ///
    /// Description of the source of the compound under study, or of the
    /// parent molecule if a simple derivative is studied. This includes the
    /// place of discovery for minerals or the actual source of a natural
    /// product.
    pub compound_source: Option<CharString>,
    /// The `_chemical_enantioexcess_bulk` item. The field holds the single value.
    ///
    /// The enantioexcess of the bulk material from which the crystals were
    /// grown. A value of 0.0 indicates the racemate. A value of 1.0
    /// indicates that the compound is enantiomerically pure. Enantioexcess
    /// is defined in the IUPAC Recommendations (Moss et al., 1996). The
    /// composition of the crystal and bulk must be the same. Ref: Moss G.
    /// P. et al. (1996). Basic Terminology of Stereochemistry. Pure Appl.
    /// Chem., 68, 2193-2222.
    /// `http://www.chem.qmul.ac.uk/iupac/stereo/index.html`
    /// Units: . (dimensionless).
    pub enantioexcess_bulk: Option<Enantioexcess>,
    /// The `_chemical_enantioexcess_bulk_technique` item. The field holds the single value.
    ///
    /// The experimental technique used to determine the enantioexcess of
    /// the bulk compound.
    pub enantioexcess_bulk_technique: Option<EnantioexcessBulkTechnique>,
    /// The `_chemical_enantioexcess_crystal` item. The field holds the single value.
    ///
    /// The enantioexcess of the crystal used for the diffraction study. A
    /// value of 0.0 indicates the racemate. A value of 1.0 indicates that
    /// the crystal is enantiomerically pure. Enantioexcess is defined in
    /// the IUPAC Recommendations (Moss et al., 1996). Ref: Moss G. P. et
    /// al. (1996). Basic Terminology of Stereochemistry. Pure Appl. Chem.,
    /// 68, 2193-2222. `http://www.chem.qmul.ac.uk/iupac/stereo/index.html`
    /// Units: . (dimensionless).
    pub enantioexcess_crystal: Option<Enantioexcess>,
    /// The `_chemical_enantioexcess_crystal_technique` item. The field holds the single value.
    ///
    /// The experimental technique used to determine the enantioexcess of
    /// the crystal.
    pub enantioexcess_crystal_technique: Option<EnantioexcessCrystalTechnique>,
    /// The `_chemical_identifier_inchi` item. The field holds the single value.
    ///
    /// The IUPAC International Chemical Identifier (InChI) is a textual
    /// identifier for chemical substances, designed to provide a standard
    /// and human-readable way to encode molecular information and to
    /// facilitate the search for such information in databases and on the
    /// web. Ref: McNaught, A. (2006). Chem. Int. (IUPAC), 28 (6), 12-14.
    /// `http://www.iupac.org/inchi/`
    pub identifier_inchi: Option<CharString>,
    /// The `_chemical_identifier_inchi_key` item. The field holds the single value.
    ///
    /// The InChIKey is a compact hashed version of the full InChI (IUPAC
    /// International Chemical Identifier), designed to allow for easy web
    /// searches of chemical compounds. See `http://www.iupac.org/inchi/`
    pub identifier_inchi_key: Option<CharString>,
    /// The `_chemical_identifier_inchi_version` item. The field holds the single value.
    ///
    /// The version number of the InChI standard to which the associated
    /// chemical identifier string applies.
    pub identifier_inchi_version: Option<CharString>,
    /// The `_chemical_melting_point` item. The field holds the single value.
    ///
    /// The temperature in kelvins at which the crystalline solid changes to
    /// a liquid.
    /// Units: K (kelvin).
    pub melting_point: Option<Temperature>,
    /// The `_chemical_name_common` item. The field holds the single value.
    ///
    /// Trivial name by which the compound is commonly known.
    pub name_common: Option<CharString>,
    /// The `_chemical_name_mineral` item. The field holds the single value.
    ///
    /// Mineral name accepted by the International Mineralogical
    /// Association. Use only for natural minerals. See also
    /// _chemical_compound_source.
    pub name_mineral: Option<CharString>,
    /// The `_chemical_name_structure_type` item. The field holds the single value.
    ///
    /// Commonly used structure-type name. Usually only applied to minerals
    /// or inorganic compounds.
    pub name_structure_type: Option<CharString>,
    /// The `_chemical_name_systematic` item. The field holds the single value.
    ///
    /// IUPAC or Chemical Abstracts full name of the compound.
    pub name_systematic: Option<CharString>,
    /// The `_chemical_optical_rotation` item. The field holds the single value.
    ///
    /// The optical rotation in solution of the compound is specified in the
    /// following format: '[\a]^TEMP^~WAVE~ = SORT (c = CONC, SOLV)' where:
    /// TEMP is the temperature of the measurement in degrees Celsius, WAVE
    /// is an indication of the wavelength of the light used for the
    /// measurement, CONC is the concentration of the solution given as the
    /// mass of the substance in g per 100 ml of solution, SORT is the
    /// signed value (preceded by a + or a - sign) of 100.\a/(l.c), where \a
    /// is the signed optical rotation in degrees measured in a cell of
    /// length l in dm and c is the value of CONC as defined above, and SOLV
    /// is the chemical formula of the solvent.
    pub optical_rotation: Option<CharString>,
    /// The `_chemical_properties_biological` item. The field holds the single value.
    ///
    /// A free-text description of the biological properties of the
    /// material.
    pub properties_biological: Option<CharString>,
    /// The `_chemical_properties_physical` item. The field holds the single value.
    ///
    /// A free-text description of the physical properties of the material.
    pub properties_physical: Option<CharString>,
    /// The `_chemical_temperature_decomposition` item. The field holds the single value.
    ///
    /// The temperature in kelvins at which the solid decomposes.
    /// Units: K (kelvin).
    pub temperature_decomposition: Option<Temperature>,
    /// The `_chemical_temperature_sublimation` item. The field holds the single value.
    ///
    /// The temperature in kelvins at which the solid sublimes.
    /// Units: K (kelvin).
    pub temperature_sublimation: Option<Temperature>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<ChemicalRow>,
}

impl CifCategory for Chemical {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.absolute_configuration {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_absolute_configuration".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.compound_source {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_compound_source".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_bulk {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_bulk".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_bulk_technique {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_bulk_technique".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_crystal {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_crystal".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.enantioexcess_crystal_technique {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_enantioexcess_crystal_technique".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.identifier_inchi {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_identifier_inchi".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.identifier_inchi_key {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_identifier_inchi_key".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.identifier_inchi_version {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_identifier_inchi_version".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.melting_point {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_melting_point".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.name_common {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_common".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_mineral {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_mineral".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_structure_type {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_structure_type".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.name_systematic {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_name_systematic".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.optical_rotation {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_optical_rotation".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.properties_biological {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_properties_biological".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.properties_physical {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_properties_physical".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.temperature_decomposition {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_temperature_decomposition".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.temperature_sublimation {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_temperature_sublimation".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        out.extend(rows_to_members(&self.rows));
        out
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            match m {
                DataBlockMember::DataItems(DataItems::SingleValue(sv)) => {
                    let tag = sv.tag().as_ref().to_lowercase();
                    let value = sv.value();
                    match tag.as_str() {
                        "chemical_absolute_configuration" => {
                            match convert::read_closed_set(
                                "chemical_absolute_configuration",
                                value,
                                AbsoluteConfiguration::parse,
                                "AbsoluteConfiguration",
                            ) {
                                Ok(v) => self.absolute_configuration = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_compound_source" => {
                            match convert::read_char("chemical_compound_source", value) {
                                Ok(v) => self.compound_source = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_enantioexcess_bulk" => {
                            match convert::read_enantioexcess("chemical_enantioexcess_bulk", value)
                            {
                                Ok(v) => self.enantioexcess_bulk = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_enantioexcess_bulk_technique" => {
                            match convert::read_closed_set(
                                "chemical_enantioexcess_bulk_technique",
                                value,
                                EnantioexcessBulkTechnique::parse,
                                "EnantioexcessBulkTechnique",
                            ) {
                                Ok(v) => self.enantioexcess_bulk_technique = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_enantioexcess_crystal" => {
                            match convert::read_enantioexcess(
                                "chemical_enantioexcess_crystal",
                                value,
                            ) {
                                Ok(v) => self.enantioexcess_crystal = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_enantioexcess_crystal_technique" => {
                            match convert::read_closed_set(
                                "chemical_enantioexcess_crystal_technique",
                                value,
                                EnantioexcessCrystalTechnique::parse,
                                "EnantioexcessCrystalTechnique",
                            ) {
                                Ok(v) => self.enantioexcess_crystal_technique = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_identifier_inchi" => {
                            match convert::read_char("chemical_identifier_inchi", value) {
                                Ok(v) => self.identifier_inchi = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_identifier_inchi_key" => {
                            match convert::read_char("chemical_identifier_inchi_key", value) {
                                Ok(v) => self.identifier_inchi_key = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_identifier_inchi_version" => {
                            match convert::read_char("chemical_identifier_inchi_version", value) {
                                Ok(v) => self.identifier_inchi_version = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_melting_point" => {
                            match convert::read_temperature("chemical_melting_point", value) {
                                Ok(v) => self.melting_point = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_name_common" => {
                            match convert::read_char("chemical_name_common", value) {
                                Ok(v) => self.name_common = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_name_mineral" => {
                            match convert::read_char("chemical_name_mineral", value) {
                                Ok(v) => self.name_mineral = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_name_structure_type" => {
                            match convert::read_char("chemical_name_structure_type", value) {
                                Ok(v) => self.name_structure_type = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_name_systematic" => {
                            match convert::read_char_text("chemical_name_systematic", value) {
                                Ok(v) => self.name_systematic = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_optical_rotation" => {
                            match convert::read_char("chemical_optical_rotation", value) {
                                Ok(v) => self.optical_rotation = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_properties_biological" => {
                            match convert::read_char("chemical_properties_biological", value) {
                                Ok(v) => self.properties_biological = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_properties_physical" => {
                            match convert::read_char("chemical_properties_physical", value) {
                                Ok(v) => self.properties_physical = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_temperature_decomposition" => {
                            match convert::read_temperature(
                                "chemical_temperature_decomposition",
                                value,
                            ) {
                                Ok(v) => self.temperature_decomposition = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "chemical_temperature_sublimation" => {
                            match convert::read_temperature(
                                "chemical_temperature_sublimation",
                                value,
                            ) {
                                Ok(v) => self.temperature_sublimation = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        _ => residue.push(m.clone()),
                    }
                }
                DataBlockMember::DataItems(DataItems::MultiValues(cols)) => {
                    let cols = cols.columns();
                    let is_ours = cols
                        .iter()
                        .any(|c| LOOP_TAGS.contains(&c.tag().as_ref().to_lowercase().as_str()));
                    if is_ours {
                        self.rows = rows_from_columns(cols, &mut errors);
                    } else {
                        residue.push(m.clone());
                    }
                }
                other => residue.push(other.clone()),
            }
        }
        (residue, errors)
    }
}

#[derive(Debug, Clone, Default)]
/// One loop row of the `chemical` category: the per-observation values of the loop items.
pub struct ChemicalRow {
    /// The `_chemical_melting_point_gt` item. The field holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the melting point (the temperature at which the crystalline solid
    /// changes to a liquid) lies. These items allow a range of temperatures
    /// to be given. _chemical_melting_point should always be used in
    /// preference to these items whenever possible.
    /// Units: K (kelvin).
    pub melting_point_gt: Option<Temperature>,
    /// The `_chemical_melting_point_lt` item. The field holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the melting point (the temperature at which the crystalline solid
    /// changes to a liquid) lies. These items allow a range of temperatures
    /// to be given. _chemical_melting_point should always be used in
    /// preference to these items whenever possible.
    /// Units: K (kelvin).
    pub melting_point_lt: Option<Temperature>,
    /// The `_chemical_temperature_decomposition_gt` item. The field holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to decompose. These items allow a range of
    /// temperatures to be given. _chemical_temperature_decomposition should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    pub temperature_decomposition_gt: Option<Temperature>,
    /// The `_chemical_temperature_decomposition_lt` item. The field holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to decompose. These items allow a range of
    /// temperatures to be given. _chemical_temperature_decomposition should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    pub temperature_decomposition_lt: Option<Temperature>,
    /// The `_chemical_temperature_sublimation_gt` item. The field holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to sublime. These items allow a range of
    /// temperatures to be given. _chemical_temperature_sublimation should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    pub temperature_sublimation_gt: Option<Temperature>,
    /// The `_chemical_temperature_sublimation_lt` item. The field holds the column values, one per loop row.
    ///
    /// A temperature in kelvins below which (*_lt) or above which (*_gt)
    /// the solid is known to sublime. These items allow a range of
    /// temperatures to be given. _chemical_temperature_sublimation should
    /// always be used in preference to these items whenever possible.
    /// Units: K (kelvin).
    pub temperature_sublimation_lt: Option<Temperature>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "chemical_melting_point_gt",
    "chemical_melting_point_lt",
    "chemical_temperature_decomposition_gt",
    "chemical_temperature_decomposition_lt",
    "chemical_temperature_sublimation_gt",
    "chemical_temperature_sublimation_lt",
];

fn rows_to_members(rows: &[ChemicalRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.melting_point_gt.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.melting_point_gt {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_melting_point_gt".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.melting_point_lt.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.melting_point_lt {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_melting_point_lt".to_string()),
            values,
        ));
    }
    if rows
        .iter()
        .any(|r| r.temperature_decomposition_gt.is_some())
    {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.temperature_decomposition_gt {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_temperature_decomposition_gt".to_string()),
            values,
        ));
    }
    if rows
        .iter()
        .any(|r| r.temperature_decomposition_lt.is_some())
    {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.temperature_decomposition_lt {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_temperature_decomposition_lt".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.temperature_sublimation_gt.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.temperature_sublimation_gt {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_temperature_sublimation_gt".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.temperature_sublimation_lt.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.temperature_sublimation_lt {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("chemical_temperature_sublimation_lt".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<ChemicalRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![ChemicalRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_melting_point_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_melting_point_gt", v) {
                Ok(val) => rows[i].melting_point_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_melting_point_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_melting_point_lt", v) {
                Ok(val) => rows[i].melting_point_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_decomposition_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_decomposition_gt", v) {
                Ok(val) => rows[i].temperature_decomposition_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_decomposition_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_decomposition_lt", v) {
                Ok(val) => rows[i].temperature_decomposition_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_sublimation_gt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_sublimation_gt", v) {
                Ok(val) => rows[i].temperature_sublimation_gt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "chemical_temperature_sublimation_lt")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_temperature("chemical_temperature_sublimation_lt", v) {
                Ok(val) => rows[i].temperature_sublimation_lt = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
