mod av_r_eq;
mod av_sg_i_over_i;
mod av_u_i_over_i;
mod d_res_high;
mod d_res_low;
mod reflns_class_number;

use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Tag, Value},
};

pub use self::av_r_eq::AvREq;
pub use self::av_sg_i_over_i::AvSgIOverI;
pub use self::av_u_i_over_i::AvUIOverI;
pub use self::d_res_high::DResHigh;
pub use self::d_res_low::DResLow;
pub use self::reflns_class_number::ReflnsClassNumber;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnReflnsClassLoopItem {
    /// range constraint recorded in the dictionary _enumeration
    Av_R_eq(Vec<AvREq>),
    /// range constraint recorded in the dictionary _enumeration
    AvSgIOverI(Vec<AvSgIOverI>),
    /// range constraint recorded in the dictionary _enumeration
    AvUIOverI(Vec<AvUIOverI>),
    Code(Vec<CharString>),
    /// range constraint recorded in the dictionary _enumeration
    D_res_high(Vec<DResHigh>),
    /// range constraint recorded in the dictionary _enumeration
    D_res_low(Vec<DResLow>),
    Description(Vec<CharString>),
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
