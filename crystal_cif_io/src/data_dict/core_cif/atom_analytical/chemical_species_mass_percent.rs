use std::fmt::Display;

use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

/// The write-time range check for `_atom_analytical_chemical_species_mass_percent`. `new` clamps the value
/// to [0.0, 100.0], matching the dictionary `_enumeration`.
///
/// range constraint recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ChemicalSpeciesMassPercent(Float);

impl ChemicalSpeciesMassPercent {
    /// Build the guarded value: the input clamped to the dictionary range.
    pub fn new(float: Float) -> Self {
        Self(Float(float.clamp(0.0, 100.0)))
    }

    /// The recorded value, stored as written.
    pub fn value(&self) -> Float {
        self.0
    }
}

impl Display for ChemicalSpeciesMassPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ChemicalSpeciesMassPercent> for Value {
    fn from(value: ChemicalSpeciesMassPercent) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.value()), None))
    }
}

impl From<f64> for ChemicalSpeciesMassPercent {
    fn from(value: f64) -> Self {
        Self::new(Float(value))
    }
}

impl From<f32> for ChemicalSpeciesMassPercent {
    fn from(value: f32) -> Self {
        Self::new(Float(value as f64))
    }
}
