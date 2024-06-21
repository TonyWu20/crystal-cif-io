use std::fmt::Display;

use crate::data_dict::DataLabel;

use super::SpaceGroup;

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
        let value = match self {
            CrystalSystem::Triclinic => "triclinic".to_string(),
            CrystalSystem::Monoclinic => "monoclinic".to_string(),
            CrystalSystem::Orthorhombic => "orthorhombic".to_string(),
            CrystalSystem::Tetragonal => "tetragonal".to_string(),
            CrystalSystem::Trigonal => "trigonal".to_string(),
            CrystalSystem::Hexagonal => "hexagonal".to_string(),
            CrystalSystem::Cubic => "cubic".to_string(),
        };
        write!(f, "{:<34}{}", self.full_label(), value)
    }
}

impl DataLabel for CrystalSystem {
    type C = SpaceGroup;

    fn tag(&self) -> String {
        "crystal_system".to_string()
    }
}

#[cfg(test)]
mod test_space_group_crystal_system {
    use crate::data_dict::symmetry_cif::space_group::crystal_system::CrystalSystem;

    #[test]
    fn test_crystal_system_format() {
        println!("{}", CrystalSystem::Triclinic)
    }
}
