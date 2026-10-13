use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_diffrn_measurement_sample_tracking`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum SampleTracking {
    /// `yes`
    Yes,
    /// `y`
    Y,
    /// `no`
    No,
    /// `n`
    N,
}

impl Display for SampleTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Yes => "yes",
            Self::Y => "y",
            Self::No => "no",
            Self::N => "n",
        };
        f.write_str(code)
    }
}

impl From<SampleTracking> for UnquotedString {
    fn from(value: SampleTracking) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<SampleTracking> for CharString {
    fn from(value: SampleTracking) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<SampleTracking> for Value {
    fn from(value: SampleTracking) -> Self {
        Value::CharString(CharString::from(value))
    }
}
