//! The `geom_torsion` category of the typed document layer (roadmap D12).
//!
//! The `GeomTorsion` struct owns the single-value items (the T6 shell
//! items) and the loop rows (`rows`, the torsion special list).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::geom_torsion::PublFlag;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `geom_torsion` category struct. One field per registry item.
pub struct GeomTorsion {
    /// The `_geom_torsion_distances` item. The field holds the single value.
    pub distances: Option<Vec<Numeric>>,
    /// The `_geom_torsion_id` item. The field holds the single value.
    pub id: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<GeomTorsionRow>,
}

impl CifCategory for GeomTorsion {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.distances {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_torsion_distances".to_string()),
                    Value::List(v.iter().cloned().map(Value::from).collect()),
                )),
            )));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_torsion_id".to_string()),
                    Value::CharString(v.clone()),
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
                        "geom_torsion_distances" => {
                            match convert::read_numeric_list("geom_torsion_distances", value) {
                                Ok(v) => self.distances = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "geom_torsion_id" => match convert::read_char("geom_torsion_id", value) {
                            Ok(v) => self.id = v,
                            Err(e) => errors.push(e),
                        },
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
/// One loop row of the `geom_torsion` category: the per-observation values of the loop items.
pub struct GeomTorsionRow {
    /// The `_geom_torsion_atom_site_label_1` item. The field holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    pub atom_site_label_1: Option<CharString>,
    /// The `_geom_torsion_atom_site_label_2` item. The field holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    pub atom_site_label_2: Option<CharString>,
    /// The `_geom_torsion_atom_site_label_3` item. The field holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    pub atom_site_label_3: Option<CharString>,
    /// The `_geom_torsion_atom_site_label_4` item. The field holds the column values, one per loop row.
    ///
    /// The labels of the four atom sites which define the torsion angle
    /// specified by _geom_torsion. These must match codes specified as
    /// _atom_site_label in the atom list. The torsion-angle definition
    /// should be that of Klyne and Prelog. The vector direction *_label_2
    /// to *_label_3 is the viewing direction, and the torsion angle is the
    /// angle of twist required to superimpose the projection of the vector
    /// between site 2 and site 1 onto the projection of the vector between
    /// site 3 and site 4. Clockwise torsions are positive, anticlockwise
    /// torsions are negative. Ref: Klyne, W. & Prelog, V. (1960).
    /// Experientia, 16, 521-523.
    pub atom_site_label_4: Option<CharString>,
    /// The `_geom_torsion_publ_flag` item. The field holds the column values, one per loop row.
    ///
    /// This code signals whether the torsion angle is referred to in a
    /// publication or should be placed in a table of significant torsion
    /// angles.
    pub publ_flag: Option<PublFlag>,
    /// The `_geom_torsion_site_symmetry_1` item. The field holds the column values, one per loop row.
    ///
    /// The symmetry code of each atom site as the symmetry-equivalent
    /// position number 'n' and the cell translation number 'klm'. These
    /// numbers are combined to form the code 'n klm' or n_klm. The
    /// character string n_klm is composed as follows: n refers to the
    /// symmetry operation that is applied to the coordinates stored in
    /// _atom_site_fract_x, _atom_site_fract_y and _atom_site_fract_z. It
    /// must match a number given in _space_group_symop_id. k, l and m refer
    /// to the translations that are subsequently applied to the
    /// symmetry-transformed coordinates to generate the atom used in
    /// calculating the angle. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    pub site_symmetry_1: Option<CharString>,
    /// The `_geom_torsion_site_symmetry_2` item. The field holds the column values, one per loop row.
    ///
    /// The symmetry code of each atom site as the symmetry-equivalent
    /// position number 'n' and the cell translation number 'klm'. These
    /// numbers are combined to form the code 'n klm' or n_klm. The
    /// character string n_klm is composed as follows: n refers to the
    /// symmetry operation that is applied to the coordinates stored in
    /// _atom_site_fract_x, _atom_site_fract_y and _atom_site_fract_z. It
    /// must match a number given in _space_group_symop_id. k, l and m refer
    /// to the translations that are subsequently applied to the
    /// symmetry-transformed coordinates to generate the atom used in
    /// calculating the angle. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    pub site_symmetry_2: Option<CharString>,
    /// The `_geom_torsion_site_symmetry_3` item. The field holds the column values, one per loop row.
    ///
    /// The symmetry code of each atom site as the symmetry-equivalent
    /// position number 'n' and the cell translation number 'klm'. These
    /// numbers are combined to form the code 'n klm' or n_klm. The
    /// character string n_klm is composed as follows: n refers to the
    /// symmetry operation that is applied to the coordinates stored in
    /// _atom_site_fract_x, _atom_site_fract_y and _atom_site_fract_z. It
    /// must match a number given in _space_group_symop_id. k, l and m refer
    /// to the translations that are subsequently applied to the
    /// symmetry-transformed coordinates to generate the atom used in
    /// calculating the angle. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    pub site_symmetry_3: Option<CharString>,
    /// The `_geom_torsion_site_symmetry_4` item. The field holds the column values, one per loop row.
    ///
    /// The symmetry code of each atom site as the symmetry-equivalent
    /// position number 'n' and the cell translation number 'klm'. These
    /// numbers are combined to form the code 'n klm' or n_klm. The
    /// character string n_klm is composed as follows: n refers to the
    /// symmetry operation that is applied to the coordinates stored in
    /// _atom_site_fract_x, _atom_site_fract_y and _atom_site_fract_z. It
    /// must match a number given in _space_group_symop_id. k, l and m refer
    /// to the translations that are subsequently applied to the
    /// symmetry-transformed coordinates to generate the atom used in
    /// calculating the angle. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    pub site_symmetry_4: Option<CharString>,
    /// The `_geom_torsion` item. The field holds the column values, one per loop row.
    ///
    /// The torsion angle in degrees bounded by the four atom sites
    /// identified by the _geom_torsion_atom_site_label_ codes. These must
    /// match labels specified as _atom_site_label in the atom list. The
    /// torsion-angle definition should be that of Klyne and Prelog. Ref:
    /// Klyne, W. & Prelog, V. (1960). Experientia, 16, 521-523.
    /// Units: deg (degree of arc).
    pub torsion: Option<Numeric>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "geom_torsion",
    "geom_torsion_atom_site_label_1",
    "geom_torsion_atom_site_label_2",
    "geom_torsion_atom_site_label_3",
    "geom_torsion_atom_site_label_4",
    "geom_torsion_publ_flag",
    "geom_torsion_site_symmetry_1",
    "geom_torsion_site_symmetry_2",
    "geom_torsion_site_symmetry_3",
    "geom_torsion_site_symmetry_4",
];

fn rows_to_members(rows: &[GeomTorsionRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.atom_site_label_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_1 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_atom_site_label_1".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_2.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_2 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_atom_site_label_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_3.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_3 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_atom_site_label_3".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_4.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_4 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_atom_site_label_4".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.publ_flag.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.publ_flag {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_publ_flag".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_1 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_site_symmetry_1".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_2.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_2 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_site_symmetry_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_3.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_3 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_site_symmetry_3".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_4.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_4 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion_site_symmetry_4".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.torsion.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.torsion {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_torsion".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<GeomTorsionRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![GeomTorsionRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_1", v) {
                Ok(val) => rows[i].atom_site_label_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_2", v) {
                Ok(val) => rows[i].atom_site_label_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_3", v) {
                Ok(val) => rows[i].atom_site_label_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_atom_site_label_4")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_atom_site_label_4", v) {
                Ok(val) => rows[i].atom_site_label_4 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_publ_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("geom_torsion_publ_flag", v, PublFlag::parse, "PublFlag")
            {
                Ok(val) => rows[i].publ_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_1", v) {
                Ok(val) => rows[i].site_symmetry_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_2", v) {
                Ok(val) => rows[i].site_symmetry_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_3", v) {
                Ok(val) => rows[i].site_symmetry_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion_site_symmetry_4")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_torsion_site_symmetry_4", v) {
                Ok(val) => rows[i].site_symmetry_4 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_torsion")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("geom_torsion", v) {
                Ok(val) => rows[i].torsion = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
