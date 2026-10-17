//! The `geom_contact` category of the typed document layer (roadmap D12).
//!
//! The `GeomContact` struct owns the single-value item (the T6 shell
//! item) and the loop rows (`rows`, the contact special list).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::geom_contact::{ContactDistance, PublFlag};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `geom_contact` category struct. One field per registry item.
pub struct GeomContact {
    /// The `_geom_contact_id` item. The field holds the single value.
    pub id: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<GeomContactRow>,
}

impl CifCategory for GeomContact {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_contact_id".to_string()),
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
                        "geom_contact_id" => match convert::read_char("geom_contact_id", value) {
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
/// One loop row of the `geom_contact` category: the per-observation values of the loop items.
pub struct GeomContactRow {
    /// The `_geom_contact_atom_site_label_1` item. The field holds the column values, one per loop row.
    ///
    /// The labels of two atom sites that are within contact distance. The
    /// labels must match _atom_site_label codes in the atom list.
    pub atom_site_label_1: Option<CharString>,
    /// The `_geom_contact_atom_site_label_2` item. The field holds the column values, one per loop row.
    ///
    /// The labels of two atom sites that are within contact distance. The
    /// labels must match _atom_site_label codes in the atom list.
    pub atom_site_label_2: Option<CharString>,
    /// The `_geom_contact_distance` item. The field holds the column values, one per loop row.
    ///
    /// The interatomic contact distance in angstroms.
    /// Units: A (angstrom).
    pub distance: Option<ContactDistance>,
    /// The `_geom_contact_publ_flag` item. The field holds the column values, one per loop row.
    ///
    /// This code signals whether the contact distance is referred to in a
    /// publication or should be placed in a list of significant contact
    /// distances.
    pub publ_flag: Option<PublFlag>,
    /// The `_geom_contact_site_symmetry_1` item. The field holds the column values, one per loop row.
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
    /// calculating the contact. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    pub site_symmetry_1: Option<CharString>,
    /// The `_geom_contact_site_symmetry_2` item. The field holds the column values, one per loop row.
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
    /// calculating the contact. These translations (x,y,z) are related to
    /// (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By adding 5
    /// to the translations, the use of negative numbers is avoided.
    pub site_symmetry_2: Option<CharString>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "geom_contact_atom_site_label_1",
    "geom_contact_atom_site_label_2",
    "geom_contact_distance",
    "geom_contact_publ_flag",
    "geom_contact_site_symmetry_1",
    "geom_contact_site_symmetry_2",
];

fn rows_to_members(rows: &[GeomContactRow]) -> Vec<DataBlockMember> {
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
            Tag::new("geom_contact_atom_site_label_1".to_string()),
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
            Tag::new("geom_contact_atom_site_label_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_contact_distance".to_string()),
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
            Tag::new("geom_contact_publ_flag".to_string()),
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
            Tag::new("geom_contact_site_symmetry_1".to_string()),
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
            Tag::new("geom_contact_site_symmetry_2".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<GeomContactRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![GeomContactRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_contact_atom_site_label_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_contact_atom_site_label_1", v) {
                Ok(val) => rows[i].atom_site_label_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_contact_atom_site_label_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_contact_atom_site_label_2", v) {
                Ok(val) => rows[i].atom_site_label_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_contact_distance")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_contact_distance("geom_contact_distance", v) {
                Ok(val) => rows[i].distance = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_contact_publ_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("geom_contact_publ_flag", v, PublFlag::parse, "PublFlag")
            {
                Ok(val) => rows[i].publ_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_contact_site_symmetry_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_contact_site_symmetry_1", v) {
                Ok(val) => rows[i].site_symmetry_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_contact_site_symmetry_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_contact_site_symmetry_2", v) {
                Ok(val) => rows[i].site_symmetry_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
