//! The `reflns_class` category: the loop items that split the
//! measured reflection set into classes (roadmap T2, DDL1
//! `cif_core.dic`).

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// The loop items of the `reflns_class` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ReflnsClassLoopItem {
    R_Fsqd_factor(Vec<Numeric>),
    R_I_factor(Vec<Numeric>),
    R_factor_all(Vec<Numeric>),
    R_factor_gt(Vec<Numeric>),
    Code(Vec<CharString>),
    D_res_high(Vec<Numeric>),
    D_res_low(Vec<Numeric>),
    Description(Vec<CharString>),
    Number_gt(Vec<Numeric>),
    Number_total(Vec<Numeric>),
    W_r_factor_all(Vec<Numeric>),
}

impl CifTerm for ReflnsClassLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsClassLoopItem::R_Fsqd_factor(_) => "R_Fsqd_factor",
            ReflnsClassLoopItem::R_I_factor(_) => "R_I_factor",
            ReflnsClassLoopItem::R_factor_all(_) => "R_factor_all",
            ReflnsClassLoopItem::R_factor_gt(_) => "R_factor_gt",
            ReflnsClassLoopItem::Code(_) => "code",
            ReflnsClassLoopItem::D_res_high(_) => "d_res_high",
            ReflnsClassLoopItem::D_res_low(_) => "d_res_low",
            ReflnsClassLoopItem::Description(_) => "description",
            ReflnsClassLoopItem::Number_gt(_) => "number_gt",
            ReflnsClassLoopItem::Number_total(_) => "number_total",
            ReflnsClassLoopItem::W_r_factor_all(_) => "wR_factor_all",
        };
        Tag::new(format!("reflns_class_{suffix}"))
    }
}

impl LoopValueTerm for ReflnsClassLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ReflnsClassLoopItem::R_Fsqd_factor(v)
            | ReflnsClassLoopItem::R_I_factor(v)
            | ReflnsClassLoopItem::R_factor_all(v)
            | ReflnsClassLoopItem::R_factor_gt(v)
            | ReflnsClassLoopItem::D_res_high(v)
            | ReflnsClassLoopItem::D_res_low(v)
            | ReflnsClassLoopItem::Number_gt(v)
            | ReflnsClassLoopItem::Number_total(v)
            | ReflnsClassLoopItem::W_r_factor_all(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            ReflnsClassLoopItem::Code(v) | ReflnsClassLoopItem::Description(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for ReflnsClassLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}

// The T6 single-value item of the `reflns_class` category.

/// The single-value item of the `reflns_class` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ReflnsClassTerms {
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `0:`); the plain numeric form is kept.
    Number_observed(Numeric),
}

impl CifTerm for ReflnsClassTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsClassTerms::Number_observed(_) => "number_observed",
        };
        Tag::new(format!("reflns_class_{suffix}"))
    }
}

impl SingleValueTerm for ReflnsClassTerms {
    fn value(&self) -> Value {
        match self {
            ReflnsClassTerms::Number_observed(n) => Value::Numeric(*n),
        }
    }
}

impl Display for ReflnsClassTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::grammar::UnquotedString;

    fn cs() -> CharString {
        CharString::Unquoted(UnquotedString::new("all".into()))
    }

    fn n() -> Numeric {
        Numeric::from(0.04)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<ReflnsClassLoopItem> = vec![
            ReflnsClassLoopItem::R_Fsqd_factor(vec![n()]),
            ReflnsClassLoopItem::R_I_factor(vec![n()]),
            ReflnsClassLoopItem::R_factor_all(vec![n()]),
            ReflnsClassLoopItem::R_factor_gt(vec![n()]),
            ReflnsClassLoopItem::Code(vec![cs()]),
            ReflnsClassLoopItem::D_res_high(vec![n()]),
            ReflnsClassLoopItem::D_res_low(vec![n()]),
            ReflnsClassLoopItem::Description(vec![cs()]),
            ReflnsClassLoopItem::Number_gt(vec![n()]),
            ReflnsClassLoopItem::Number_total(vec![n()]),
            ReflnsClassLoopItem::W_r_factor_all(vec![n()]),
        ];
        let expected: Vec<&str> = vec![
            "reflns_class_R_Fsqd_factor",
            "reflns_class_R_I_factor",
            "reflns_class_R_factor_all",
            "reflns_class_R_factor_gt",
            "reflns_class_code",
            "reflns_class_d_res_high",
            "reflns_class_d_res_low",
            "reflns_class_description",
            "reflns_class_number_gt",
            "reflns_class_number_total",
            "reflns_class_wR_factor_all",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "reflns_class tag drift");
        }
    }
}
