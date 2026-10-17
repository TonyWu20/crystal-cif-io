//! The `atom_sites` category of the typed document layer (roadmap D12).
//!
//! The `AtomSites` struct owns the single-value items (the solution
//! codes and the transform axes) and the loop rows (`rows`, the
//! numbered transform-matrix and transform-vector families).

use crate::core_cif::CifError;
use crate::data_dict::core_cif::atom_sites::SolutionMethod;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, LoopColumn, LoopColumns, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `atom_sites` category struct. One field per registry item.
pub struct AtomSites {
    /// The `_atom_sites_Cartn_transform_axes` item. The field holds the single value.
    ///
    /// A description of the relative alignment of the crystal cell axes to
    /// the Cartesian orthogonal axes as applied in the transformation
    /// matrix _atom_sites_Cartn_tran_matrix_.
    pub cartn_transform_axes: Option<CharString>,
    /// The `_atom_sites_solution_hydrogens` item. The field holds the single value.
    ///
    /// Codes which identify the methods used to locate the initial atom
    /// sites. The *_primary code identifies how the first atom sites were
    /// determined; the *_secondary code identifies how the remaining
    /// non-hydrogen sites were located; and the *_hydrogens code identifies
    /// how the hydrogen sites were located. Ref: Sheldrick, G. M.,
    /// Hauptman, H. A., Weeks, C. M., Miller, R. and Us\'on, I. (2001). Ab
    /// initio phasing. In International Tables for Crystallography, Vol. F.
    /// Crystallography of biological macromolecules, edited by M. G.
    /// Rossmann and E. Arnold, ch. 16.1. Dordrecht: Kluwer Academic
    /// Publishers.
    pub solution_hydrogens: Option<SolutionMethod>,
    /// The `_atom_sites_solution_primary` item. The field holds the single value.
    ///
    /// Codes which identify the methods used to locate the initial atom
    /// sites. The *_primary code identifies how the first atom sites were
    /// determined; the *_secondary code identifies how the remaining
    /// non-hydrogen sites were located; and the *_hydrogens code identifies
    /// how the hydrogen sites were located. Ref: Sheldrick, G. M.,
    /// Hauptman, H. A., Weeks, C. M., Miller, R. and Us\'on, I. (2001). Ab
    /// initio phasing. In International Tables for Crystallography, Vol. F.
    /// Crystallography of biological macromolecules, edited by M. G.
    /// Rossmann and E. Arnold, ch. 16.1. Dordrecht: Kluwer Academic
    /// Publishers.
    pub solution_primary: Option<SolutionMethod>,
    /// The `_atom_sites_solution_secondary` item. The field holds the single value.
    ///
    /// Codes which identify the methods used to locate the initial atom
    /// sites. The *_primary code identifies how the first atom sites were
    /// determined; the *_secondary code identifies how the remaining
    /// non-hydrogen sites were located; and the *_hydrogens code identifies
    /// how the hydrogen sites were located. Ref: Sheldrick, G. M.,
    /// Hauptman, H. A., Weeks, C. M., Miller, R. and Us\'on, I. (2001). Ab
    /// initio phasing. In International Tables for Crystallography, Vol. F.
    /// Crystallography of biological macromolecules, edited by M. G.
    /// Rossmann and E. Arnold, ch. 16.1. Dordrecht: Kluwer Academic
    /// Publishers.
    pub solution_secondary: Option<SolutionMethod>,
    /// The `_atom_sites_special_details` item. The field holds the single value.
    ///
    /// Additional information about the atomic coordinates not coded
    /// elsewhere in the CIF.
    pub special_details: Option<CharString>,
    /// The loop rows of the category: one row per observation.
    pub rows: Vec<AtomSitesRow>,
}

impl CifCategory for AtomSites {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.cartn_transform_axes {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("atom_sites_Cartn_transform_axes".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.solution_hydrogens {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("atom_sites_solution_hydrogens".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.solution_primary {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("atom_sites_solution_primary".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.solution_secondary {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("atom_sites_solution_secondary".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.special_details {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("atom_sites_special_details".to_string()),
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
                        "atom_sites_cartn_transform_axes" => {
                            match convert::read_char("atom_sites_cartn_transform_axes", value) {
                                Ok(v) => self.cartn_transform_axes = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "atom_sites_solution_hydrogens" => {
                            match convert::read_closed_set(
                                "atom_sites_solution_hydrogens",
                                value,
                                SolutionMethod::parse,
                                "SolutionMethod",
                            ) {
                                Ok(v) => self.solution_hydrogens = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "atom_sites_solution_primary" => {
                            match convert::read_closed_set(
                                "atom_sites_solution_primary",
                                value,
                                SolutionMethod::parse,
                                "SolutionMethod",
                            ) {
                                Ok(v) => self.solution_primary = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "atom_sites_solution_secondary" => {
                            match convert::read_closed_set(
                                "atom_sites_solution_secondary",
                                value,
                                SolutionMethod::parse,
                                "SolutionMethod",
                            ) {
                                Ok(v) => self.solution_secondary = v,
                                Err(e) => errors.push(e),
                            }
                        }
                        "atom_sites_special_details" => {
                            match convert::read_char("atom_sites_special_details", value) {
                                Ok(v) => self.special_details = v,
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
/// One loop row of the `atom_sites` category: the per-observation values of the loop items.
pub struct AtomSitesRow {
    /// The `_atom_sites_Cartn_tran_matrix_11` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_11: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_12` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_12: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_13` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_13: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_21` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_21: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_22` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_22: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_23` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_23: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_31` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_31: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_32` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_32: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_matrix_33` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform fractional coordinates in the
    /// ATOM_SITE category to Cartesian coordinates. The axial alignments of
    /// this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_Cartn_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33| z | 3
    /// |
    pub cartn_tran_matrix_33: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_vector_1` item. The field holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// fractional coordinates in the ATOM_SITE category to Cartesian
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33|
    /// z | 3 |
    pub cartn_tran_vector_1: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_vector_2` item. The field holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// fractional coordinates in the ATOM_SITE category to Cartesian
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33|
    /// z | 3 |
    pub cartn_tran_vector_2: Option<Numeric>,
    /// The `_atom_sites_Cartn_tran_vector_3` item. The field holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// fractional coordinates in the ATOM_SITE category to Cartesian
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) Cartesian = |21 22 23| ( y ) fractional + | 2 | z' |31 32 33|
    /// z | 3 |
    pub cartn_tran_vector_3: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_11` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_11: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_12` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_12: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_13` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_13: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_21` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_21: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_22` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_22: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_23` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_23: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_31` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_31: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_32` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_32: Option<Numeric>,
    /// The `_atom_sites_fract_tran_matrix_33` item. The field holds the column values, one per loop row.
    ///
    /// Matrix elements used to transform Cartesian coordinates in the
    /// ATOM_SITE category to fractional coordinates. The axial alignments
    /// of this transformation are described in
    /// _atom_sites_Cartn_transform_axes. The 3 x 1 translation is defined
    /// in _atom_sites_fract_tran_vector_. x' |11 12 13| x | 1 | ( y' )
    /// fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33| z | 3
    /// |
    pub fract_tran_matrix_33: Option<Numeric>,
    /// The `_atom_sites_fract_tran_vector_1` item. The field holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// Cartesian coordinates in the ATOM_SITE category to fractional
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33|
    /// z | 3 |
    pub fract_tran_vector_1: Option<Numeric>,
    /// The `_atom_sites_fract_tran_vector_2` item. The field holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// Cartesian coordinates in the ATOM_SITE category to fractional
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33|
    /// z | 3 |
    pub fract_tran_vector_2: Option<Numeric>,
    /// The `_atom_sites_fract_tran_vector_3` item. The field holds the column values, one per loop row.
    ///
    /// Elements of a 3 x 1 translation vector used in the transformation of
    /// Cartesian coordinates in the ATOM_SITE category to fractional
    /// coordinates. The axial alignments of this transformation are
    /// described in _atom_sites_Cartn_transform_axes. x' |11 12 13| x | 1 |
    /// ( y' ) fractional = |21 22 23| ( y ) Cartesian + | 2 | z' |31 32 33|
    /// z | 3 |
    pub fract_tran_vector_3: Option<Numeric>,
}

/// The loop tags of the category. A loop is the own loop of
/// the category when it carries one of these tags.
const LOOP_TAGS: &[&str] = &[
    "atom_sites_cartn_tran_matrix_11",
    "atom_sites_cartn_tran_matrix_12",
    "atom_sites_cartn_tran_matrix_13",
    "atom_sites_cartn_tran_matrix_21",
    "atom_sites_cartn_tran_matrix_22",
    "atom_sites_cartn_tran_matrix_23",
    "atom_sites_cartn_tran_matrix_31",
    "atom_sites_cartn_tran_matrix_32",
    "atom_sites_cartn_tran_matrix_33",
    "atom_sites_cartn_tran_vector_1",
    "atom_sites_cartn_tran_vector_2",
    "atom_sites_cartn_tran_vector_3",
    "atom_sites_fract_tran_matrix_11",
    "atom_sites_fract_tran_matrix_12",
    "atom_sites_fract_tran_matrix_13",
    "atom_sites_fract_tran_matrix_21",
    "atom_sites_fract_tran_matrix_22",
    "atom_sites_fract_tran_matrix_23",
    "atom_sites_fract_tran_matrix_31",
    "atom_sites_fract_tran_matrix_32",
    "atom_sites_fract_tran_matrix_33",
    "atom_sites_fract_tran_vector_1",
    "atom_sites_fract_tran_vector_2",
    "atom_sites_fract_tran_vector_3",
];

fn rows_to_members(rows: &[AtomSitesRow]) -> Vec<DataBlockMember> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut columns = Vec::new();
    if rows.iter().any(|r| r.cartn_tran_matrix_11.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_11 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_11".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_12.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_12 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_12".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_13.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_13 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_13".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_21.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_21 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_21".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_22.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_22 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_22".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_23.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_23 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_23".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_31.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_31 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_31".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_32.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_32 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_32".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_matrix_33.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_matrix_33 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_matrix_33".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_vector_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_vector_1 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_vector_1".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_vector_2.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_vector_2 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_vector_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.cartn_tran_vector_3.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.cartn_tran_vector_3 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_Cartn_tran_vector_3".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_11.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_11 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_11".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_12.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_12 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_12".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_13.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_13 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_13".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_21.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_21 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_21".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_22.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_22 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_22".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_23.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_23 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_23".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_31.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_31 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_31".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_32.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_32 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_32".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_matrix_33.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_matrix_33 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_matrix_33".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_vector_1.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_vector_1 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_vector_1".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_vector_2.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_vector_2 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_vector_2".to_string()),
            values,
        ));
    }
    if rows.iter().any(|r| r.fract_tran_vector_3.is_some()) {
        let values: Vec<Value> = rows
            .iter()
            .map(|r| match &r.fract_tran_vector_3 {
                Some(v) => Value::Numeric(*v),
                None => Value::Inapplicable,
            })
            .collect();
        columns.push(LoopColumn::new(
            Tag::new("atom_sites_fract_tran_vector_3".to_string()),
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

fn rows_from_columns(cols: &[LoopColumn], errors: &mut Vec<CifError>) -> Vec<AtomSitesRow> {
    let n = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let mut rows = vec![AtomSitesRow::default(); n];
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_11")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_11", v) {
                Ok(val) => rows[i].cartn_tran_matrix_11 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_12")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_12", v) {
                Ok(val) => rows[i].cartn_tran_matrix_12 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_13")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_13", v) {
                Ok(val) => rows[i].cartn_tran_matrix_13 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_21")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_21", v) {
                Ok(val) => rows[i].cartn_tran_matrix_21 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_22")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_22", v) {
                Ok(val) => rows[i].cartn_tran_matrix_22 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_23")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_23", v) {
                Ok(val) => rows[i].cartn_tran_matrix_23 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_31")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_31", v) {
                Ok(val) => rows[i].cartn_tran_matrix_31 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_32")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_32", v) {
                Ok(val) => rows[i].cartn_tran_matrix_32 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_matrix_33")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_matrix_33", v) {
                Ok(val) => rows[i].cartn_tran_matrix_33 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_vector_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_vector_1", v) {
                Ok(val) => rows[i].cartn_tran_vector_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_vector_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_vector_2", v) {
                Ok(val) => rows[i].cartn_tran_vector_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_cartn_tran_vector_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_cartn_tran_vector_3", v) {
                Ok(val) => rows[i].cartn_tran_vector_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_11")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_11", v) {
                Ok(val) => rows[i].fract_tran_matrix_11 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_12")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_12", v) {
                Ok(val) => rows[i].fract_tran_matrix_12 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_13")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_13", v) {
                Ok(val) => rows[i].fract_tran_matrix_13 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_21")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_21", v) {
                Ok(val) => rows[i].fract_tran_matrix_21 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_22")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_22", v) {
                Ok(val) => rows[i].fract_tran_matrix_22 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_23")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_23", v) {
                Ok(val) => rows[i].fract_tran_matrix_23 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_31")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_31", v) {
                Ok(val) => rows[i].fract_tran_matrix_31 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_32")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_32", v) {
                Ok(val) => rows[i].fract_tran_matrix_32 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_matrix_33")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_matrix_33", v) {
                Ok(val) => rows[i].fract_tran_matrix_33 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_vector_1")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_vector_1", v) {
                Ok(val) => rows[i].fract_tran_vector_1 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_vector_2")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_vector_2", v) {
                Ok(val) => rows[i].fract_tran_vector_2 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    if let Some(col) = cols
        .iter()
        .find(|c| c.tag().as_ref().to_lowercase() == "atom_sites_fract_tran_vector_3")
    {
        for (i, v) in col.values().iter().enumerate() {
            match convert::read_numeric("atom_sites_fract_tran_vector_3", v) {
                Ok(val) => rows[i].fract_tran_vector_3 = val,
                Err(e) => errors.push(e),
            }
        }
    }
    rows
}
