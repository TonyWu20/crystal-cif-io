use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// One `cell` angle item: the value clamped into the 0 to 180 range.
pub struct CellAngle(Float);

impl CellAngle {
    /// Clamps the value into the 0 to 180 range.
    pub fn new(float: Float) -> Self {
        Self(Float(float.clamp(0.0, 180.0)))
    }
    /// The clamped angle value.
    pub fn angle(&self) -> Float {
        self.0
    }
}

impl From<CellAngle> for Value {
    fn from(value: CellAngle) -> Self {
        Value::Numeric(Numeric::new(Number::Float(value.angle()), None))
    }
}

impl From<f32> for CellAngle {
    fn from(value: f32) -> Self {
        Self::new(Float(value.into()))
    }
}

impl From<f64> for CellAngle {
    fn from(value: f64) -> Self {
        (value as f32).into()
    }
}
