//! The `refine_ls_class` category: the loop items that split the
//! least-squares refinement set into classes (roadmap T2, DDL1
//! `cif_core.dic`).

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};
mod refine_ls_class;
pub use refine_ls_class::RefineLsClass;

/// The loop items of the `refine_ls_class` category. Each variant
/// holds one column of the loop.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `refine_ls_class` category: one variant per

/// registry loop-value item, in item order.
pub enum RefineLsClassLoopItem {
    /// The `_refine_ls_class_R_Fsqd_factor` item. The variant holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factor R(F^2^) calculated on
    /// the squared amplitudes of the observed and calculated structure
    /// factors for the reflections judged significantly intense (i.e.
    /// satisfying the threshold specified by _reflns_threshold_expression)
    /// and included in the refinement. The reflections also satisfy the
    /// resolution limits established by _refine_ls_class_d_res_high and
    /// _refine_ls_class_d_res_low. sum | F(obs)^2^ - F(calc)^2^ | R(Fsqd) =
    /// ------------------------------- sum F(obs)^2^ F(obs)^2^ = squares of
    /// the observed structure-factor amplitudes F(calc)^2^ = squares of the
    /// calculated structure-factor amplitudes and the sum is taken over the
    /// reflections of this class.
    R_Fsqd_factor(Vec<Numeric>),
    /// The `_refine_ls_class_R_I_factor` item. The variant holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factor R(I) for the
    /// reflections judged significantly intense (i.e. satisfying the
    /// threshold specified by _reflns_threshold_expression) and included in
    /// the refinement. This is most often calculated in Rietveld
    /// refinements against powder data, where it is referred to as R~B~ or
    /// R~Bragg~. sum | I(obs) - I(calc) | R(I) = ------------------------
    /// sum | I(obs) | I(obs) = the net observed intensities I(calc) = the
    /// net calculated intensities and the sum is taken over the reflections
    /// of this class.
    R_I_factor(Vec<Numeric>),
    /// The `_refine_ls_class_R_factor_all` item. The variant holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factors for all reflections,
    /// and for significantly intense reflections (see
    /// _reflns_threshold_expression), included in the refinement. The
    /// reflections also satisfy the resolution limits established by
    /// _refine_ls_class_d_res_high and _refine_ls_class_d_res_low. This is
    /// the conventional R factor. sum | F(obs) - F(calc) | R =
    /// ------------------------ sum | F(obs) | F(obs) = the observed
    /// structure-factor amplitudes F(calc) = the calculated
    /// structure-factor amplitudes and the sum is taken over the
    /// reflections of this class. See also _refine_ls_class_wR_factor_all
    /// definitions.
    R_factor_all(Vec<Numeric>),
    /// The `_refine_ls_class_R_factor_gt` item. The variant holds the column values, one per loop row.
    ///
    /// For each reflection class, the residual factors for all reflections,
    /// and for significantly intense reflections (see
    /// _reflns_threshold_expression), included in the refinement. The
    /// reflections also satisfy the resolution limits established by
    /// _refine_ls_class_d_res_high and _refine_ls_class_d_res_low. This is
    /// the conventional R factor. sum | F(obs) - F(calc) | R =
    /// ------------------------ sum | F(obs) | F(obs) = the observed
    /// structure-factor amplitudes F(calc) = the calculated
    /// structure-factor amplitudes and the sum is taken over the
    /// reflections of this class. See also _refine_ls_class_wR_factor_all
    /// definitions.
    R_factor_gt(Vec<Numeric>),
    /// The `_refine_ls_class_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying a certain reflection class. This code must
    /// match a _reflns_class_code.
    Code(Vec<CharString>),
    /// The `_refine_ls_class_d_res_high` item. The variant holds the column values, one per loop row.
    ///
    /// For each reflection class, the highest resolution in angstroms for
    /// the reflections used in the refinement. This is the lowest d value
    /// in a reflection class.
    /// Units: A (angstrom).
    D_res_high(Vec<Numeric>),
    /// The `_refine_ls_class_d_res_low` item. The variant holds the column values, one per loop row.
    ///
    /// For each reflection class, the lowest resolution in angstroms for
    /// the reflections used in the refinement. This is the highest d value
    /// in a reflection class.
    /// Units: A (angstrom).
    D_res_low(Vec<Numeric>),
    /// The `refine_ls_class_W_R_factor_all` item. The variant holds the
    /// column values, one per loop row.
    ///
    /// The weighted R factor of the refinement, over all reflections.
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
    use crystal_cif_fmt::grammar::UnquotedString;

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
