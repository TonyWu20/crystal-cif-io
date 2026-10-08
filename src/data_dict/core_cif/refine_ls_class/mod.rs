//! The `refine_ls_class` category: the loop items that split the
//! least-squares refinement set into classes (roadmap T2, DDL1
//! `cif_core.dic`).

use std::fmt::Display;

use crate::{
    data_dict::{CifTerm, LoopValueTerm},
    grammar::{CharString, Numeric, Tag, Value},
};

/// The loop items of the `refine_ls_class` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum RefineLsClassLoopItem {
    R_Fsqd_factor(Vec<Numeric>),
    R_I_factor(Vec<Numeric>),
    R_factor_all(Vec<Numeric>),
    R_factor_gt(Vec<Numeric>),
    Code(Vec<CharString>),
    D_res_high(Vec<Numeric>),
    D_res_low(Vec<Numeric>),
    W_r_factor_all(Vec<Numeric>),
}

impl CifTerm for RefineLsClassLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            RefineLsClassLoopItem::R_Fsqd_factor(_) => "R_Fsqd_factor",
            RefineLsClassLoopItem::R_I_factor(_) => "R_I_factor",
            RefineLsClassLoopItem::R_factor_all(_) => "R_factor_all",
            RefineLsClassLoopItem::R_factor_gt(_) => "R_factor_gt",
            RefineLsClassLoopItem::Code(_) => "code",
            RefineLsClassLoopItem::D_res_high(_) => "d_res_high",
            RefineLsClassLoopItem::D_res_low(_) => "d_res_low",
            RefineLsClassLoopItem::W_r_factor_all(_) => "wR_factor_all",
        };
        Tag::new(format!("refine_ls_class_{suffix}"))
    }
}

impl LoopValueTerm for RefineLsClassLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            RefineLsClassLoopItem::R_Fsqd_factor(v)
            | RefineLsClassLoopItem::R_I_factor(v)
            | RefineLsClassLoopItem::R_factor_all(v)
            | RefineLsClassLoopItem::R_factor_gt(v)
            | RefineLsClassLoopItem::D_res_high(v)
            | RefineLsClassLoopItem::D_res_low(v)
            | RefineLsClassLoopItem::W_r_factor_all(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            RefineLsClassLoopItem::Code(v) => v.iter().cloned().map(Value::from).collect(),
        }
    }
}

impl Display for RefineLsClassLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::UnquotedString;

    fn cs() -> CharString {
        CharString::Unquoted(UnquotedString::new("all".into()))
    }

    fn n() -> Numeric {
        Numeric::from(0.04)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<RefineLsClassLoopItem> = vec![
            RefineLsClassLoopItem::R_Fsqd_factor(vec![n()]),
            RefineLsClassLoopItem::R_I_factor(vec![n()]),
            RefineLsClassLoopItem::R_factor_all(vec![n()]),
            RefineLsClassLoopItem::R_factor_gt(vec![n()]),
            RefineLsClassLoopItem::Code(vec![cs()]),
            RefineLsClassLoopItem::D_res_high(vec![n()]),
            RefineLsClassLoopItem::D_res_low(vec![n()]),
            RefineLsClassLoopItem::W_r_factor_all(vec![n()]),
        ];
        let expected: Vec<&str> = vec![
            "refine_ls_class_R_Fsqd_factor",
            "refine_ls_class_R_I_factor",
            "refine_ls_class_R_factor_all",
            "refine_ls_class_R_factor_gt",
            "refine_ls_class_code",
            "refine_ls_class_d_res_high",
            "refine_ls_class_d_res_low",
            "refine_ls_class_wR_factor_all",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "refine_ls_class tag drift");
        }
    }
}
