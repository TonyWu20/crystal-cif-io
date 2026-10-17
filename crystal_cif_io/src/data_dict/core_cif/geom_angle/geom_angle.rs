//! The `geom_angle` category of the typed document layer (roadmap D12).
//!
//! The `GeomAngle` struct owns the single-value items (the T6 shell
//! items) and the loop rows (`rows`, the angle special list).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::geom_angle::PublFlag;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `geom_angle` category struct. One field per registry item.
pub struct GeomAngle {
    /// The `_geom_angle_distances` item. The field holds the single value.
    pub distances: Option<Vec<Numeric>>,
    /// The `_geom_angle_id` item. The field holds the single value.
    pub id: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<GeomAngleRow>,
}

impl CifCategory for GeomAngle {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.distances {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_angle_distances".to_string()),
                    Value::List(v.iter().cloned().map(Value::from).collect()),
                )),
            )));
        }
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_angle_id".to_string()),
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
                        "geom_angle_distances" => {
                            match convert::read_numeric_list("geom_angle_distances", value) {
                                Ok(v) => self.distances = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "geom_angle_id" => match convert::read_char("geom_angle_id", value) {
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
/// One loop row of the `geom_angle` category: the per-observation values of the loop items.
pub struct GeomAngleRow {
    /// The `_geom_angle_angles` item. The angle between the three
    /// atom sites of the row, in degrees.
    pub angle: Option<Numeric>,
    /// The `_geom_angle_atom_site_label_1` item. The field holds the column values, one per loop row.
    ///
    /// The labels of the three atom sites which define the angle given by
    /// _geom_angle. These must match labels specified as _atom_site_label
    /// in the atom list. Label 2 identifies the site at the apex of the
    /// angle.
    pub atom_site_label_1: Option<CharString>,
    /// The `_geom_angle_atom_site_label_2` item. The field holds the column values, one per loop row.
    ///
    /// The labels of the three atom sites which define the angle given by
    /// _geom_angle. These must match labels specified as _atom_site_label
    /// in the atom list. Label 2 identifies the site at the apex of the
    /// angle.
    pub atom_site_label_2: Option<CharString>,
    /// The `_geom_angle_atom_site_label_3` item. The field holds the column values, one per loop row.
    ///
    /// The labels of the three atom sites which define the angle given by
    /// _geom_angle. These must match labels specified as _atom_site_label
    /// in the atom list. Label 2 identifies the site at the apex of the
    /// angle.
    pub atom_site_label_3: Option<CharString>,
    /// The `_geom_angle_publ_flag` item. The field holds the column values, one per loop row.
    ///
    /// This code signals whether the angle is referred to in a publication
    /// or should be placed in a table of significant angles.
    pub publ_flag: Option<PublFlag>,
    /// The `_geom_angle_site_symmetry_1` item. The field holds the column values, one per loop row.
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
    /// The `_geom_angle_site_symmetry_2` item. The field holds the column values, one per loop row.
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
    /// The `_geom_angle_site_symmetry_3` item. The field holds the column values, one per loop row.
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
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "geom_angle",
    "geom_angle_atom_site_label_1",
    "geom_angle_atom_site_label_2",
    "geom_angle_atom_site_label_3",
    "geom_angle_publ_flag",
    "geom_angle_site_symmetry_1",
    "geom_angle_site_symmetry_2",
    "geom_angle_site_symmetry_3",
];

fn rows_to_members(rows: &[GeomAngleRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.angle.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.angle {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(Tag::new("geom_angle".to_string()), values));
    }
    if rows.iter().any(|r| r.atom_site_label_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_1 {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_angle_atom_site_label_1".to_string()),
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
            Tag::new("geom_angle_atom_site_label_2".to_string()),
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
            Tag::new("geom_angle_atom_site_label_3".to_string()),
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
            Tag::new("geom_angle_publ_flag".to_string()),
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
            Tag::new("geom_angle_site_symmetry_1".to_string()),
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
            Tag::new("geom_angle_site_symmetry_2".to_string()),
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
            Tag::new("geom_angle_site_symmetry_3".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<GeomAngleRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![GeomAngleRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("geom_angle", v) {
                Ok(val) => rows[i].angle = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle_atom_site_label_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_angle_atom_site_label_1", v) {
                Ok(val) => rows[i].atom_site_label_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle_atom_site_label_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_angle_atom_site_label_2", v) {
                Ok(val) => rows[i].atom_site_label_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle_atom_site_label_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_angle_atom_site_label_3", v) {
                Ok(val) => rows[i].atom_site_label_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle_publ_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("geom_angle_publ_flag", v, PublFlag::parse, "PublFlag") {
                Ok(val) => rows[i].publ_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle_site_symmetry_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_angle_site_symmetry_1", v) {
                Ok(val) => rows[i].site_symmetry_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle_site_symmetry_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_angle_site_symmetry_2", v) {
                Ok(val) => rows[i].site_symmetry_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_angle_site_symmetry_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_angle_site_symmetry_3", v) {
                Ok(val) => rows[i].site_symmetry_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
