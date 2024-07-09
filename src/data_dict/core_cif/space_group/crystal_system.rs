use std::fmt::Display;

use crystallographic_group::database::CrystalSystem as CS;

/// This crate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum CrystalSystemCif {
    #[default]
    Triclinic,
    Monoclinic,
    Orthorhombic,
    Tetragonal,
    Trigonal,
    Hexagonal,
    Cubic,
}

impl Display for CrystalSystemCif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        write!(f, "{}", name)
    }
}

#[cfg(feature = "chemrust-core")]
impl From<crystallographic_group::database::CrystalSystem> for CrystalSystemCif {
    fn from(value: crystallographic_group::database::CrystalSystem) -> Self {
        match value {
            CS::Triclinic => Self::Triclinic,
            CS::Monoclinic => Self::Monoclinic,
            CS::Orthorhombic => Self::Orthorhombic,
            CS::Tetragonal => Self::Tetragonal,
            CS::Trigonal => Self::Trigonal,
            CS::Hexagonal => Self::Hexagonal,
            CS::Cubic => Self::Cubic,
        }
    }
}
