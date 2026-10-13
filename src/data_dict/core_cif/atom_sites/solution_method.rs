use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of the `_atom_sites_solution_*` items. The
/// codes identify how the initial atom sites, the remaining sites,
/// and the hydrogen sites were located.
///
/// The codes follow the dictionary source (`tests/dict/cif_core.dic`).
/// The T0 inventory records a parse artifact for these enumerations
/// (a stray `'dual-space` token, missing `mixed` and `other` codes);
/// the enum keeps the dictionary code set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolutionMethod {
    /// `difmap`: difference Fourier map.
    Difmap,
    /// `vecmap`: real-space vector search.
    Vecmap,
    /// `heavy`: heavy-atom method.
    Heavy,
    /// `direct`: structure-invariant direct methods.
    Direct,
    /// `geom`: inferred from neighbouring sites.
    Geom,
    /// `disper`: anomalous-dispersion techniques.
    Disper,
    /// `isomor`: isomorphous structure methods.
    Isomor,
    /// `mixed`: a mixture of `geom` and `difmap` (hydrogen sites).
    Mixed,
    /// `notdet`: coordinates were not determined.
    Notdet,
    /// `dual`: dual-space method.
    Dual,
    /// `iterative`: iterative algorithm, for example charge flipping.
    Iterative,
    /// `other`: a method not included elsewhere in the list.
    Other,
}

impl Display for SolutionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            SolutionMethod::Difmap => "difmap",
            SolutionMethod::Vecmap => "vecmap",
            SolutionMethod::Heavy => "heavy",
            SolutionMethod::Direct => "direct",
            SolutionMethod::Geom => "geom",
            SolutionMethod::Disper => "disper",
            SolutionMethod::Isomor => "isomor",
            SolutionMethod::Mixed => "mixed",
            SolutionMethod::Notdet => "notdet",
            SolutionMethod::Dual => "dual",
            SolutionMethod::Iterative => "iterative",
            SolutionMethod::Other => "other",
        };
        f.write_str(code)
    }
}

impl From<SolutionMethod> for UnquotedString {
    fn from(value: SolutionMethod) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<SolutionMethod> for CharString {
    fn from(value: SolutionMethod) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<SolutionMethod> for Value {
    fn from(value: SolutionMethod) -> Self {
        Value::CharString(CharString::from(value))
    }
}
