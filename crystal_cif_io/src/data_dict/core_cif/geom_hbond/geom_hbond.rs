//! The `geom_hbond` category of the typed document layer (roadmap D12).
//!
//! The `GeomHbond` struct owns the single-value item (the T6 shell
//! item) and the loop rows (`rows`, the hydrogen-bond special list).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::geom_hbond::{HbondDistance, PublFlag};
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `geom_hbond` category struct. One field per registry item.
pub struct GeomHbond {
    /// The `_geom_hbond_id` item. The field holds the single value.
    pub id: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<GeomHbondRow>,
}

impl CifCategory for GeomHbond {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.id {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("geom_hbond_id".to_string()),
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
                        "geom_hbond_id" => match convert::read_char("geom_hbond_id", value) {
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
/// One loop row of the `geom_hbond` category: the per-observation values of the loop items.
pub struct GeomHbondRow {
    /// The `_geom_hbond_angle_DHA` item. The field holds the column values, one per loop row.
    ///
    /// Angle in degrees defined by the three sites
    /// _geom_hbond_atom_site_label_D, *_H and *_A. The site at *_H (the
    /// hydrogen atom participating in the interaction) is at the apex of
    /// the angle.
    /// Units: deg (degree of arc).
    pub angle_dha: Option<Numeric>,
    /// The `_geom_hbond_atom_site_label_A` item. The field holds the column values, one per loop row.
    ///
    /// The labels of three atom sites (respectively, the donor atom,
    /// hydrogen atom and acceptor atom) participating in a hydrogen bond.
    /// These must match labels specified as _atom_site_label in the atom
    /// list.
    pub atom_site_label_a: Option<CharString>,
    /// The `_geom_hbond_atom_site_label_D` item. The field holds the column values, one per loop row.
    ///
    /// The labels of three atom sites (respectively, the donor atom,
    /// hydrogen atom and acceptor atom) participating in a hydrogen bond.
    /// These must match labels specified as _atom_site_label in the atom
    /// list.
    pub atom_site_label_d: Option<CharString>,
    /// The `_geom_hbond_atom_site_label_H` item. The field holds the column values, one per loop row.
    ///
    /// The labels of three atom sites (respectively, the donor atom,
    /// hydrogen atom and acceptor atom) participating in a hydrogen bond.
    /// These must match labels specified as _atom_site_label in the atom
    /// list.
    pub atom_site_label_h: Option<CharString>,
    /// The `_geom_hbond_distance_DA` item. The field holds the column values, one per loop row.
    ///
    /// Distances in angstroms between the donor and hydrogen (*_DH),
    /// hydrogen and acceptor (*_HA) and donor and acceptor (*_DA) sites in
    /// a hydrogen bond.
    /// Units: A (angstrom).
    pub distance_da: Option<HbondDistance>,
    /// The `_geom_hbond_distance_DH` item. The field holds the column values, one per loop row.
    ///
    /// Distances in angstroms between the donor and hydrogen (*_DH),
    /// hydrogen and acceptor (*_HA) and donor and acceptor (*_DA) sites in
    /// a hydrogen bond.
    /// Units: A (angstrom).
    pub distance_dh: Option<HbondDistance>,
    /// The `_geom_hbond_distance_HA` item. The field holds the column values, one per loop row.
    ///
    /// Distances in angstroms between the donor and hydrogen (*_DH),
    /// hydrogen and acceptor (*_HA) and donor and acceptor (*_DA) sites in
    /// a hydrogen bond.
    /// Units: A (angstrom).
    pub distance_ha: Option<HbondDistance>,
    /// The `_geom_hbond_publ_flag` item. The field holds the column values, one per loop row.
    ///
    /// This code signals whether the hydrogen-bond information is referred
    /// to in a publication or should be placed in a table of significant
    /// hydrogen-bond geometry.
    pub publ_flag: Option<PublFlag>,
    /// The `_geom_hbond_site_symmetry_A` item. The field holds the column values, one per loop row.
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
    /// calculating the hydrogen bond. These translations (x,y,z) are
    /// related to (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By
    /// adding 5 to the translations, the use of negative numbers is
    /// avoided.
    pub site_symmetry_a: Option<CharString>,
    /// The `_geom_hbond_site_symmetry_D` item. The field holds the column values, one per loop row.
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
    /// calculating the hydrogen bond. These translations (x,y,z) are
    /// related to (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By
    /// adding 5 to the translations, the use of negative numbers is
    /// avoided.
    pub site_symmetry_d: Option<CharString>,
    /// The `_geom_hbond_site_symmetry_H` item. The field holds the column values, one per loop row.
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
    /// calculating the hydrogen bond. These translations (x,y,z) are
    /// related to (k,l,m) by the relations k = 5 + x l = 5 + y m = 5 + z By
    /// adding 5 to the translations, the use of negative numbers is
    /// avoided.
    pub site_symmetry_h: Option<CharString>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "geom_hbond_angle_dha",
    "geom_hbond_atom_site_label_a",
    "geom_hbond_atom_site_label_d",
    "geom_hbond_atom_site_label_h",
    "geom_hbond_distance_da",
    "geom_hbond_distance_dh",
    "geom_hbond_distance_ha",
    "geom_hbond_publ_flag",
    "geom_hbond_site_symmetry_a",
    "geom_hbond_site_symmetry_d",
    "geom_hbond_site_symmetry_h",
];

fn rows_to_members(rows: &[GeomHbondRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.angle_dha.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.angle_dha {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_angle_DHA".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_a.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_a {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_atom_site_label_A".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_d.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_d {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_atom_site_label_D".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.atom_site_label_h.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.atom_site_label_h {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_atom_site_label_H".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance_da.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance_da {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_distance_DA".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance_dh.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance_dh {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_distance_DH".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.distance_ha.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.distance_ha {
                Some(v) => Value::from(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_distance_HA".to_string()),
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
            Tag::new("geom_hbond_publ_flag".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_a.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_a {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_site_symmetry_A".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_d.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_d {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_site_symmetry_D".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.site_symmetry_h.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.site_symmetry_h {
                Some(v) => Value::CharString(v.clone()),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("geom_hbond_site_symmetry_H".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<GeomHbondRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![GeomHbondRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_angle_dha")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("geom_hbond_angle_dha", v) {
                Ok(val) => rows[i].angle_dha = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_atom_site_label_a")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_atom_site_label_a", v) {
                Ok(val) => rows[i].atom_site_label_a = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_atom_site_label_d")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_atom_site_label_d", v) {
                Ok(val) => rows[i].atom_site_label_d = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_atom_site_label_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_atom_site_label_h", v) {
                Ok(val) => rows[i].atom_site_label_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_distance_da")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_hbond_distance("geom_hbond_distance_da", v) {
                Ok(val) => rows[i].distance_da = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_distance_dh")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_hbond_distance("geom_hbond_distance_dh", v) {
                Ok(val) => rows[i].distance_dh = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_distance_ha")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_hbond_distance("geom_hbond_distance_ha", v) {
                Ok(val) => rows[i].distance_ha = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_publ_flag")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_closed_set("geom_hbond_publ_flag", v, PublFlag::parse, "PublFlag") {
                Ok(val) => rows[i].publ_flag = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_site_symmetry_a")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_site_symmetry_a", v) {
                Ok(val) => rows[i].site_symmetry_a = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_site_symmetry_d")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_site_symmetry_d", v) {
                Ok(val) => rows[i].site_symmetry_d = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "geom_hbond_site_symmetry_h")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_char("geom_hbond_site_symmetry_h", v) {
                Ok(val) => rows[i].site_symmetry_h = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
