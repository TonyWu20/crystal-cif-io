use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct SymMultiplicity {
    value: u8,
}

impl SymMultiplicity {
    pub fn new(value: u8) -> Self {
        Self {
            value: value.clamp(1, 192),
        }
    }
}

impl Display for SymMultiplicity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
