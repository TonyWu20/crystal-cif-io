use std::fmt::Display;

use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_journal_paper_pages`. `new` clamps the value
/// to [1.0, +infinity), matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PaperPages(Float);

impl PaperPages {
    /// Build the guarded value: the input clamped to the dictionary range.
    pub fn new(float: Float) -> Self {
        Self(Float(float.max(1.0)))
    }

    /// The recorded value, stored as written.
    pub fn value(&self) -> Float {
        self.0
    }

    /// The recorded value, stored as written. The range guard applies at
    /// the write.
    pub fn from_numeric(n: Numeric) -> Self {
        let raw = match n.number() {
            Number::Integer(i) => i.0 as f64,
            Number::Float(f) => f.0,
        };
        Self(Float(raw))
    }
}

impl Display for PaperPages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PaperPages> for Value {
    fn from(value: PaperPages) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for PaperPages {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for PaperPages {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
