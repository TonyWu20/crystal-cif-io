use std::fmt::Display;

use crystallographic_group::database::CrystalSystem as CS;
// use serde::{Deserialize, Serialize};

// /// This crate
// #[cfg(feature = "serde")]
// #[derive(Serialize, Deserialize)]
// #[serde(
//     rename_all(serialize = "lowercase"),
//     try_from = "String",
//     into = "String"
// )]
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

impl From<CrystalSystemCif> for String {
    fn from(value: CrystalSystemCif) -> Self {
        value.to_string()
    }
}

#[derive(Debug)]
pub struct CrystalSystemCifError;

impl Display for CrystalSystemCifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Not a valid crystal system option")
    }
}

impl std::error::Error for CrystalSystemCifError {}

impl TryFrom<&str> for CrystalSystemCif {
    type Error = CrystalSystemCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Triclinic" | "triclinic" => Ok(Self::Triclinic),
            "Monoclinic" | "monoclinic" => Ok(Self::Monoclinic),
            "Orthorhombic" | "orthorhombic" => Ok(Self::Orthorhombic),
            "Tetragonal" | "tetragonal" => Ok(Self::Tetragonal),
            "Trigonal" | "trigonal" => Ok(Self::Trigonal),
            "Hexagonal" | "hexagonal" => Ok(Self::Hexagonal),
            "Cubic" | "cubic" => Ok(Self::Cubic),
            _ => Err(CrystalSystemCifError),
        }
    }
}

impl TryFrom<String> for CrystalSystemCif {
    type Error = CrystalSystemCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
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

// #[cfg(test)]
// #[test]
// fn test_cs_from_str() {
//     use crate::{CharString, UnquotedString, Value};

//     let input = "cubic".to_string();
//     let value = Value::CharString(CharString::Unquoted(UnquotedString::new(input)));
//     let system = CrystalSystemCif::deserialize(&value).unwrap();
//     dbg!(system);
// }
