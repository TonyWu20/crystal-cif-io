use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CellAngle {
    A(f32),
    B(f32),
}

impl CellAngle {
    pub fn new(degree: f64) -> Self {
        Self(degree.clamp(0.0, 180.0))
    }
}

impl std::ops::Deref for CellAngle {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for CellAngle {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <f64 as Display>::fmt(&self.0, fmt)
    }
}

impl Default for CellAngle {
    fn default() -> Self {
        Self(90.0)
    }
}
