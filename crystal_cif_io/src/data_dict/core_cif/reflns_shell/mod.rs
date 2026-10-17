//! The `reflns_shell` category: the loop items that split the
//! measured reflection set into resolution shells (roadmap T2,
//! DDL1 `cif_core.dic`).

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{Numeric, Tag, Value};
mod reflns_shell;
pub use reflns_shell::ReflnsShell;

/// The loop items of the `reflns_shell` category. Each variant
/// holds one column of the loop. The dictionary records all 23
/// items as `numb` values.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The loop-value terms of the `reflns_shell` category: one variant per

/// registry loop-value item, in item order.
pub enum ReflnsShellLoopItem {
    /// The `_reflns_shell_Rmerge_F_all` item. The variant holds the column values, one per loop row.
    ///
    /// The value of Rmerge(F) for all reflections in a given shell. sum~i~
    /// ( sum~j~ | F~j~ - &lt;F&gt; | ) Rmerge(F) =
    /// -------------------------------- sum~i~ ( sum~j~ &lt;F&gt; ) F~j~ = the
    /// amplitude of the jth observation of reflection i &lt;F&gt; = the mean of
    /// the amplitudes of all observations of reflection i sum~i~ is taken
    /// over all reflections sum~j~ is taken over all observations of each
    /// reflection.
    Rmerge_F_all(Vec<Numeric>),
    /// The `_reflns_shell_Rmerge_F_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The value of Rmerge(F) for significantly intense reflections (see
    /// _reflns_threshold_expression) in a given shell. sum~i~ ( sum~j~ |
    /// F~j~ - &lt;F&gt; | ) Rmerge(F) = -------------------------------- sum~i~ (
    /// sum~j~ &lt;F&gt; ) F~j~ = the amplitude of the jth observation of
    /// reflection i &lt;F&gt; = the mean of the amplitudes of all observations of
    /// reflection i sum~i~ is taken over all reflections sum~j~ is taken
    /// over all observations of each reflection.
    Rmerge_F_gt(Vec<Numeric>),
    /// The `_reflns_shell_Rmerge_F_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The value of Rmerge(F) for reflections classified as 'observed' (see
    /// _reflns_observed_criterion) in a given shell. sum~i~ ( sum~j~ | F~j~
    /// - &lt;F&gt; | ) Rmerge(F) = -------------------------------- sum~i~ (
    /// sum~j~ &lt;F&gt; ) F~j~ = the amplitude of the jth observation of
    /// reflection i &lt;F&gt; = the mean of the amplitudes of all observations of
    /// reflection i sum~i~ is taken over all reflections sum~j~ is taken
    /// over all observations of each reflection.
    Rmerge_F_obs(Vec<Numeric>),
    /// The `_reflns_shell_Rmerge_I_all` item. The variant holds the column values, one per loop row.
    ///
    /// The value of Rmerge(I) for all reflections in a given shell. sum~i~
    /// ( sum~j~ | I~j~ - &lt;I&gt; | ) Rmerge(I) =
    /// -------------------------------- sum~i~ ( sum~j~ &lt;I&gt; ) I~j~ = the
    /// intensity of the jth observation of reflection i &lt;I&gt; = the mean of
    /// the intensities of all observations of reflection i sum~i~ is taken
    /// over all reflections sum~j~ is taken over all observations of each
    /// reflection.
    Rmerge_I_all(Vec<Numeric>),
    /// The `_reflns_shell_Rmerge_I_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The value of Rmerge(I) for significantly intense reflections (see
    /// _reflns_threshold_expression) in a given shell. sum~i~ ( sum~j~ |
    /// I~j~ - &lt;I&gt; | ) Rmerge(I) = -------------------------------- sum~i~ (
    /// sum~j~ &lt;I&gt; ) I~j~ = the intensity of the jth observation of
    /// reflection i &lt;I&gt; = the mean of the intensities of all observations
    /// of reflection i sum~i~ is taken over all reflections sum~j~ is taken
    /// over all observations of each reflection.
    Rmerge_I_gt(Vec<Numeric>),
    /// The `_reflns_shell_Rmerge_I_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The value of Rmerge(I) for reflections classified as 'observed' (see
    /// _reflns_observed_criterion) in a given shell. sum~i~ ( sum~j~ | I~j~
    /// - &lt;I&gt; | ) Rmerge(I) = -------------------------------- sum~i~ (
    /// sum~j~ &lt;I&gt; ) I~j~ = the intensity of the jth observation of
    /// reflection i &lt;I&gt; = the mean of the intensities of all observations
    /// of reflection i sum~i~ is taken over all reflections sum~j~ is taken
    /// over all observations of each reflection.
    Rmerge_I_obs(Vec<Numeric>),
    /// The `_reflns_shell_d_res_high` item. The variant holds the column values, one per loop row.
    ///
    /// The highest resolution in angstroms for the reflections in this
    /// shell. This is the smallest d value.
    /// Units: A (angstrom).
    D_res_high(Vec<Numeric>),
    /// The `_reflns_shell_d_res_low` item. The variant holds the column values, one per loop row.
    ///
    /// The lowest resolution in angstroms for the reflections in this
    /// shell. This is the largest d value.
    /// Units: A (angstrom).
    D_res_low(Vec<Numeric>),
    /// The `_reflns_shell_meanI_over_sigI_all` item. The variant holds the column values, one per loop row.
    ///
    /// The ratio of the mean of the intensities of all reflections in this
    /// shell to the mean of the standard uncertainties of the intensities
    /// of all reflections in the resolution shell.
    MeanI_over_sigI_all(Vec<Numeric>),
    /// The `_reflns_shell_meanI_over_sigI_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The ratio of the mean of the intensities of the significantly
    /// intense reflections (see _reflns_threshold_expression) in this shell
    /// to the mean of the standard uncertainties of the intensities of the
    /// significantly intense reflections in the resolution shell.
    MeanI_over_sigI_gt(Vec<Numeric>),
    /// The `_reflns_shell_meanI_over_sigI_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The ratio of the mean of the intensities of the reflections
    /// classified as 'observed' (see _reflns_observed_criterion) in this
    /// shell to the mean of the standard uncertainties of the intensities
    /// of the 'observed' reflections in the resolution shell.
    MeanI_over_sigI_obs(Vec<Numeric>),
    /// The `_reflns_shell_meanI_over_uI_all` item. The variant holds the column values, one per loop row.
    ///
    /// The ratio of the mean of the intensities of all reflections in this
    /// shell to the mean of the standard uncertainties of the intensities
    /// of all reflections in the resolution shell.
    MeanI_over_uI_all(Vec<Numeric>),
    /// The `_reflns_shell_meanI_over_uI_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The ratio of the mean of the intensities of the significantly
    /// intense reflections (see _reflns_threshold_expression) in this shell
    /// to the mean of the standard uncertainties of the intensities of the
    /// significantly intense reflections in the resolution shell.
    MeanI_over_uI_gt(Vec<Numeric>),
    /// The `_reflns_shell_number_measured_all` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of reflections measured for this resolution shell.
    Number_measured_all(Vec<Numeric>),
    /// The `_reflns_shell_number_measured_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The number of significantly intense reflections (see
    /// _reflns_threshold_expression) measured for this resolution shell.
    Number_measured_gt(Vec<Numeric>),
    /// The `_reflns_shell_number_measured_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The number of reflections classified as 'observed' (see
    /// _reflns_observed_criterion) measured for this resolution shell.
    Number_measured_obs(Vec<Numeric>),
    /// The `_reflns_shell_number_possible` item. The variant holds the column values, one per loop row.
    ///
    /// The number of unique reflections it is possible to measure in this
    /// reflection shell.
    Number_possible(Vec<Numeric>),
    /// The `_reflns_shell_number_unique_all` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of measured reflections resulting from merging
    /// measured symmetry-equivalent reflections for this resolution shell.
    Number_unique_all(Vec<Numeric>),
    /// The `_reflns_shell_number_unique_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of significantly intense reflections (see
    /// _reflns_threshold_expression) resulting from merging measured
    /// symmetry-equivalent reflections for this resolution shell.
    Number_unique_gt(Vec<Numeric>),
    /// The `_reflns_shell_number_unique_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The total number of reflections classified as 'observed' (see
    /// _reflns_observed_criterion) resulting from merging measured
    /// symmetry-equivalent reflections for this resolution shell.
    Number_unique_obs(Vec<Numeric>),
    /// The `_reflns_shell_percent_possible_all` item. The variant holds the column values, one per loop row.
    ///
    /// The percentage of geometrically possible reflections represented by
    /// all reflections measured for this resolution shell.
    Percent_possible_all(Vec<Numeric>),
    /// The `_reflns_shell_percent_possible_gt` item. The variant holds the column values, one per loop row.
    ///
    /// The percentage of geometrically possible reflections represented by
    /// significantly intense reflections (see _reflns_threshold_expression)
    /// measured for this resolution shell.
    Percent_possible_gt(Vec<Numeric>),
    /// The `_reflns_shell_percent_possible_obs` item. The variant holds the column values, one per loop row.
    ///
    /// The percentage of geometrically possible reflections represented by
    /// reflections classified as 'observed' (see
    /// _reflns_observed_criterion) measured for this resolution shell.
    Percent_possible_obs(Vec<Numeric>),
}

impl CifTerm for ReflnsShellLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsShellLoopItem::Rmerge_F_all(_) => "Rmerge_F_all",
            ReflnsShellLoopItem::Rmerge_F_gt(_) => "Rmerge_F_gt",
            ReflnsShellLoopItem::Rmerge_F_obs(_) => "Rmerge_F_obs",
            ReflnsShellLoopItem::Rmerge_I_all(_) => "Rmerge_I_all",
            ReflnsShellLoopItem::Rmerge_I_gt(_) => "Rmerge_I_gt",
            ReflnsShellLoopItem::Rmerge_I_obs(_) => "Rmerge_I_obs",
            ReflnsShellLoopItem::D_res_high(_) => "d_res_high",
            ReflnsShellLoopItem::D_res_low(_) => "d_res_low",
            ReflnsShellLoopItem::MeanI_over_sigI_all(_) => "meanI_over_sigI_all",
            ReflnsShellLoopItem::MeanI_over_sigI_gt(_) => "meanI_over_sigI_gt",
            ReflnsShellLoopItem::MeanI_over_sigI_obs(_) => "meanI_over_sigI_obs",
            ReflnsShellLoopItem::MeanI_over_uI_all(_) => "meanI_over_uI_all",
            ReflnsShellLoopItem::MeanI_over_uI_gt(_) => "meanI_over_uI_gt",
            ReflnsShellLoopItem::Number_measured_all(_) => "number_measured_all",
            ReflnsShellLoopItem::Number_measured_gt(_) => "number_measured_gt",
            ReflnsShellLoopItem::Number_measured_obs(_) => "number_measured_obs",
            ReflnsShellLoopItem::Number_possible(_) => "number_possible",
            ReflnsShellLoopItem::Number_unique_all(_) => "number_unique_all",
            ReflnsShellLoopItem::Number_unique_gt(_) => "number_unique_gt",
            ReflnsShellLoopItem::Number_unique_obs(_) => "number_unique_obs",
            ReflnsShellLoopItem::Percent_possible_all(_) => "percent_possible_all",
            ReflnsShellLoopItem::Percent_possible_gt(_) => "percent_possible_gt",
            ReflnsShellLoopItem::Percent_possible_obs(_) => "percent_possible_obs",
        };
        Tag::new(format!("reflns_shell_{suffix}"))
    }
}

impl LoopValueTerm for ReflnsShellLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            ReflnsShellLoopItem::Rmerge_F_all(v)
            | ReflnsShellLoopItem::Rmerge_F_gt(v)
            | ReflnsShellLoopItem::Rmerge_F_obs(v)
            | ReflnsShellLoopItem::Rmerge_I_all(v)
            | ReflnsShellLoopItem::Rmerge_I_gt(v)
            | ReflnsShellLoopItem::Rmerge_I_obs(v)
            | ReflnsShellLoopItem::D_res_high(v)
            | ReflnsShellLoopItem::D_res_low(v)
            | ReflnsShellLoopItem::MeanI_over_sigI_all(v)
            | ReflnsShellLoopItem::MeanI_over_sigI_gt(v)
            | ReflnsShellLoopItem::MeanI_over_sigI_obs(v)
            | ReflnsShellLoopItem::MeanI_over_uI_all(v)
            | ReflnsShellLoopItem::MeanI_over_uI_gt(v)
            | ReflnsShellLoopItem::Number_measured_all(v)
            | ReflnsShellLoopItem::Number_measured_gt(v)
            | ReflnsShellLoopItem::Number_measured_obs(v)
            | ReflnsShellLoopItem::Number_possible(v)
            | ReflnsShellLoopItem::Number_unique_all(v)
            | ReflnsShellLoopItem::Number_unique_gt(v)
            | ReflnsShellLoopItem::Number_unique_obs(v)
            | ReflnsShellLoopItem::Percent_possible_all(v)
            | ReflnsShellLoopItem::Percent_possible_gt(v)
            | ReflnsShellLoopItem::Percent_possible_obs(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

impl Display for ReflnsShellLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}

// The T6 single-value item of the `reflns_shell` category.

/// The single-value item of the `reflns_shell` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// The single-value terms of the `reflns_shell` category: one variant per

/// registry single-value item, in item order.
pub enum ReflnsShellTerms {
    /// The shell records a two-value list (min and max d). The
    /// shape is noted from the shell `_type.dimension`; the read
    /// path does not bound it.
    /// The `_reflns_shell_d_res_limits` item. The variant holds the column values, one per loop row.
    D_res_limits(Vec<Numeric>),
}

impl CifTerm for ReflnsShellTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            ReflnsShellTerms::D_res_limits(_) => "d_res_limits",
        };
        Tag::new(format!("reflns_shell_{suffix}"))
    }
}

impl SingleValueTerm for ReflnsShellTerms {
    fn value(&self) -> Value {
        match self {
            ReflnsShellTerms::D_res_limits(v) => {
                Value::List(v.iter().cloned().map(Value::from).collect())
            }
        }
    }
}

impl Display for ReflnsShellTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n() -> Numeric {
        Numeric::from(0.5)
    }

    #[test]
    fn tag_spellings_match_the_inventory() {
        let items: Vec<ReflnsShellLoopItem> = vec![
            ReflnsShellLoopItem::Rmerge_F_all(vec![n()]),
            ReflnsShellLoopItem::Rmerge_F_gt(vec![n()]),
            ReflnsShellLoopItem::Rmerge_F_obs(vec![n()]),
            ReflnsShellLoopItem::Rmerge_I_all(vec![n()]),
            ReflnsShellLoopItem::Rmerge_I_gt(vec![n()]),
            ReflnsShellLoopItem::Rmerge_I_obs(vec![n()]),
            ReflnsShellLoopItem::D_res_high(vec![n()]),
            ReflnsShellLoopItem::D_res_low(vec![n()]),
            ReflnsShellLoopItem::MeanI_over_sigI_all(vec![n()]),
            ReflnsShellLoopItem::MeanI_over_sigI_gt(vec![n()]),
            ReflnsShellLoopItem::MeanI_over_sigI_obs(vec![n()]),
            ReflnsShellLoopItem::MeanI_over_uI_all(vec![n()]),
            ReflnsShellLoopItem::MeanI_over_uI_gt(vec![n()]),
            ReflnsShellLoopItem::Number_measured_all(vec![n()]),
            ReflnsShellLoopItem::Number_measured_gt(vec![n()]),
            ReflnsShellLoopItem::Number_measured_obs(vec![n()]),
            ReflnsShellLoopItem::Number_possible(vec![n()]),
            ReflnsShellLoopItem::Number_unique_all(vec![n()]),
            ReflnsShellLoopItem::Number_unique_gt(vec![n()]),
            ReflnsShellLoopItem::Number_unique_obs(vec![n()]),
            ReflnsShellLoopItem::Percent_possible_all(vec![n()]),
            ReflnsShellLoopItem::Percent_possible_gt(vec![n()]),
            ReflnsShellLoopItem::Percent_possible_obs(vec![n()]),
        ];
        let expected: Vec<&str> = vec![
            "reflns_shell_Rmerge_F_all",
            "reflns_shell_Rmerge_F_gt",
            "reflns_shell_Rmerge_F_obs",
            "reflns_shell_Rmerge_I_all",
            "reflns_shell_Rmerge_I_gt",
            "reflns_shell_Rmerge_I_obs",
            "reflns_shell_d_res_high",
            "reflns_shell_d_res_low",
            "reflns_shell_meanI_over_sigI_all",
            "reflns_shell_meanI_over_sigI_gt",
            "reflns_shell_meanI_over_sigI_obs",
            "reflns_shell_meanI_over_uI_all",
            "reflns_shell_meanI_over_uI_gt",
            "reflns_shell_number_measured_all",
            "reflns_shell_number_measured_gt",
            "reflns_shell_number_measured_obs",
            "reflns_shell_number_possible",
            "reflns_shell_number_unique_all",
            "reflns_shell_number_unique_gt",
            "reflns_shell_number_unique_obs",
            "reflns_shell_percent_possible_all",
            "reflns_shell_percent_possible_gt",
            "reflns_shell_percent_possible_obs",
        ];
        for (item, tag) in items.iter().zip(expected) {
            assert_eq!(item.tag().as_ref(), tag, "reflns_shell tag drift");
        }
    }
}
