use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
#[derive(Debug, Clone, Copy)]
pub struct NumValue<T: Display + Copy>(T, Option<u32>);

impl<
        T: Display
            + Copy
            + Add
            + AddAssign
            + Sum
            + Sub
            + SubAssign
            + Mul
            + MulAssign
            + Div
            + DivAssign,
    > NumValue<T>
{
    pub fn number(&self) -> T {
        self.0
    }
}

impl NumValue<f64> {
    pub fn new(real: f64, standard_uncertainty: Option<u32>) -> Self {
        Self(real, standard_uncertainty)
    }
}

impl NumValue<f32> {
    pub fn new(real: f32, standard_uncertainty: Option<u32>) -> Self {
        Self(real, standard_uncertainty)
    }
}

impl<T: Display + Copy> Display for NumValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uncertainty = if let Some(u) = self.1 {
            format!("({u})")
        } else {
            String::new()
        };
        write!(f, "{:.4}{}", self.0, uncertainty)
    }
}
