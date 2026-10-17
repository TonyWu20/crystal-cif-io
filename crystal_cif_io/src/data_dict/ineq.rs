//! The inequality payload of the recorded numeric items (roadmap D12l).

/// A recorded value of a numeric item. DDL1 may record a
/// less-than assertion (`&lt;N`) under a numeric tag. The payload
/// keeps the assertion with the value of the plain field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ineq<T> {
    /// A recorded value.
    Value(T),
    /// A recorded less-than assertion (`&lt;N`).
    LessThan(T),
}

impl<T> Ineq<T> {
    /// The held value, an assertion or not.
    pub fn value(&self) -> &T {
        match self {
            Self::Value(v) => v,
            Self::LessThan(v) => v,
        }
    }
}
