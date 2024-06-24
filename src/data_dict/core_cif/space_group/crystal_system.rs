use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum CrystalSystem {
    #[default]
    Triclinic,
    Monoclinic,
    Orthorhombic,
    Tetragonal,
    Trigonal,
    Hexagonal,
    Cubic,
}

impl Display for CrystalSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        write!(f, "{}", name)
    }
}
