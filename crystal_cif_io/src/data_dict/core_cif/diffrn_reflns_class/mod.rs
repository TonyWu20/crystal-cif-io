mod av_r_eq;
mod av_sg_i_over_i;
mod av_u_i_over_i;
mod av_ui_over_i;
mod d_res_high;
mod d_res_low;
mod diffrn_reflns_class;
mod reflns_class_number;

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};

pub use self::av_r_eq::AvREq;
pub use self::av_sg_i_over_i::AvSgIOverI;
pub use self::av_u_i_over_i::AvUIOverI;
pub use self::av_ui_over_i::AvUiOverI;
pub use self::d_res_high::DResHigh;
pub use self::d_res_low::DResLow;
pub use self::diffrn_reflns_class::DiffrnReflnsClass;
pub use self::reflns_class_number::ReflnsClassNumber;

/// The single-value terms of the `diffrn_reflns_class` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsClassTerms {
    /// The `_diffrn_reflns_class_av_ui_over_i` item. The variant holds the single value.
    /// range constraint recorded in the dictionary _enumeration
    Av_ui_over_i(AvUiOverI),
}

impl CifTerm for DiffrnReflnsClassTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnReflnsClassTerms::Av_ui_over_i(_) => "av_ui_over_i",
        };
        Tag::new(format!("diffrn_reflns_class_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnReflnsClassTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnReflnsClassTerms::Av_ui_over_i(v) => Value::from(*v),
        }
    }
}

impl Display for DiffrnReflnsClassTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `diffrn_reflns_class` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsClassLoopItem {
    /// The `_diffrn_reflns_class_av_R_eq` item. The variant holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual [sum av|del(I)|/sum|av(I)|]
    /// for symmetry-equivalent reflections used to calculate the average
    /// intensity av(I). The av|del(I)| term is the average absolute
    /// difference between av(I) and the individual intensities.
    /// range constraint recorded in the dictionary _enumeration
    Av_R_eq(Vec<AvREq>),
    /// The `_diffrn_reflns_class_av_sgI/I` item. The variant holds the column values, one per loop row.
    ///
    /// Measure [sum|u(net I)|/sum|net I|] for all measured intensities in a
    /// reflection class.
    /// range constraint recorded in the dictionary _enumeration
    AvSgIOverI(Vec<AvSgIOverI>),
    /// The `_diffrn_reflns_class_av_uI/I` item. The variant holds the column values, one per loop row.
    ///
    /// Measure [sum|u(net I)|/sum|net I|] for all measured intensities in a
    /// reflection class.
    /// range constraint recorded in the dictionary _enumeration
    AvUIOverI(Vec<AvUIOverI>),
    /// The `_diffrn_reflns_class_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying a certain reflection class.
    Code(Vec<CharString>),
    /// The `_diffrn_reflns_class_d_res_high` item. The variant holds the column values, one per loop row.
    ///
    /// The smallest value in angstroms of the interplanar spacings of the
    /// reflections in each reflection class. This is called the highest
    /// resolution for this reflection class.
    /// Units: A (angstrom).
    /// range constraint recorded in the dictionary _enumeration
    D_res_high(Vec<DResHigh>),
    /// The `_diffrn_reflns_class_d_res_low` item. The variant holds the column values, one per loop row.
    ///
    /// The highest value in angstroms of the interplanar spacings of the
    /// reflections in each reflection class. This is called the lowest
    /// resolution for this reflection class.
    /// Units: A (angstrom).
    /// range constraint recorded in the dictionary _enumeration
    D_res_low(Vec<DResLow>),
    /// The `_diffrn_reflns_class_description` item. The variant holds the column values, one per loop row.
    ///
    /// Description of each reflection class.
    Description(Vec<CharString>),
    /// The `_diffrn_reflns_class_number` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of measured intensities for each reflection class,
    /// excluding the systematic absences arising from centring
    /// translations.
    /// range constraint recorded in the dictionary _enumeration
    ReflnsClassNumber(Vec<ReflnsClassNumber>),
}

impl CifTerm for DiffrnReflnsClassLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnReflnsClassLoopItem::Av_R_eq(_) => "av_r_eq",
            DiffrnReflnsClassLoopItem::AvSgIOverI(_) => "av_sgi/i",
            DiffrnReflnsClassLoopItem::AvUIOverI(_) => "av_ui/i",
            DiffrnReflnsClassLoopItem::Code(_) => "code",
            DiffrnReflnsClassLoopItem::D_res_high(_) => "d_res_high",
            DiffrnReflnsClassLoopItem::D_res_low(_) => "d_res_low",
            DiffrnReflnsClassLoopItem::Description(_) => "description",
            DiffrnReflnsClassLoopItem::ReflnsClassNumber(_) => "number",
        };
        Tag::new(format!("diffrn_reflns_class_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnReflnsClassLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnReflnsClassLoopItem::Av_R_eq(v) => v.iter().cloned().map(Value::from).collect(),
            DiffrnReflnsClassLoopItem::AvSgIOverI(m) => {
                m.iter().cloned().map(Value::from).collect()
            }
            DiffrnReflnsClassLoopItem::AvUIOverI(c) => c.iter().cloned().map(Value::from).collect(),
            DiffrnReflnsClassLoopItem::Code(s) | DiffrnReflnsClassLoopItem::Description(s) => {
                s.iter().cloned().map(Value::from).collect()
            }
            DiffrnReflnsClassLoopItem::D_res_high(b) => {
                b.iter().cloned().map(Value::from).collect()
            }
            DiffrnReflnsClassLoopItem::D_res_low(t) => t.iter().cloned().map(Value::from).collect(),
            DiffrnReflnsClassLoopItem::ReflnsClassNumber(u) => {
                u.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
