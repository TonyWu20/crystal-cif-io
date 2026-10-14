//! The closed code sets of the `chemical` category.

use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_chemical_absolute_configuration`.
/// The codes record how the absolute or relative configuration of the
/// structure was established.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsoluteConfiguration {
    /// `rm`: relative configuration of a chiral reference compound.
    Rm,
    /// `ad`: the absolute configuration of the structure.
    Ad,
    /// `dyn`: a dynamic chiral system.
    Dyn,
    /// `rmad`: the relative configuration with the absolute
    /// configuration of the chiral reference compound.
    Rmad,
    /// `syn`: the configuration of a synthetic chiral reference
    /// compound.
    Syn,
    /// `unk`: the configuration was not established.
    Unk,
}

impl Display for AbsoluteConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            AbsoluteConfiguration::Rm => "rm",
            AbsoluteConfiguration::Ad => "ad",
            AbsoluteConfiguration::Dyn => "dyn",
            AbsoluteConfiguration::Rmad => "rmad",
            AbsoluteConfiguration::Syn => "syn",
            AbsoluteConfiguration::Unk => "unk",
        };
        f.write_str(code)
    }
}

impl From<AbsoluteConfiguration> for UnquotedString {
    fn from(value: AbsoluteConfiguration) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<AbsoluteConfiguration> for CharString {
    fn from(value: AbsoluteConfiguration) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<AbsoluteConfiguration> for Value {
    fn from(value: AbsoluteConfiguration) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl AbsoluteConfiguration {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "rm" => Some(Self::Rm),
            "ad" => Some(Self::Ad),
            "dyn" => Some(Self::Dyn),
            "rmad" => Some(Self::Rmad),
            "syn" => Some(Self::Syn),
            "unk" => Some(Self::Unk),
            _ => None,
        }
    }
}

/// The closed code set of `_chemical_enantioexcess_bulk_technique`.
/// The codes record the technique of the bulk measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnantioexcessBulkTechnique {
    /// `OA`: optical activity of the bulk compound in solution.
    Oa,
    /// `CD`: circular dichroism.
    Cd,
    /// `EC`: the electronic circular dichroism.
    Ec,
    /// `other`: a technique not in the listed set.
    Other,
}

impl Display for EnantioexcessBulkTechnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            EnantioexcessBulkTechnique::Oa => "OA",
            EnantioexcessBulkTechnique::Cd => "CD",
            EnantioexcessBulkTechnique::Ec => "EC",
            EnantioexcessBulkTechnique::Other => "other",
        };
        f.write_str(code)
    }
}

impl From<EnantioexcessBulkTechnique> for UnquotedString {
    fn from(value: EnantioexcessBulkTechnique) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<EnantioexcessBulkTechnique> for CharString {
    fn from(value: EnantioexcessBulkTechnique) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<EnantioexcessBulkTechnique> for Value {
    fn from(value: EnantioexcessBulkTechnique) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl EnantioexcessBulkTechnique {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "OA" => Some(Self::Oa),
            "CD" => Some(Self::Cd),
            "EC" => Some(Self::Ec),
            "other" => Some(Self::Other),
            _ => None,
        }
    }
}

/// The closed code set of `_chemical_enantioexcess_crystal_technique`.
/// The codes record the technique of the crystal measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnantioexcessCrystalTechnique {
    /// `CD`: circular dichroism of the crystal.
    Cd,
    /// `EC`: the electronic circular dichroism.
    Ec,
    /// `other`: a technique not in the listed set.
    Other,
}

impl Display for EnantioexcessCrystalTechnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            EnantioexcessCrystalTechnique::Cd => "CD",
            EnantioexcessCrystalTechnique::Ec => "EC",
            EnantioexcessCrystalTechnique::Other => "other",
        };
        f.write_str(code)
    }
}

impl From<EnantioexcessCrystalTechnique> for UnquotedString {
    fn from(value: EnantioexcessCrystalTechnique) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<EnantioexcessCrystalTechnique> for CharString {
    fn from(value: EnantioexcessCrystalTechnique) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<EnantioexcessCrystalTechnique> for Value {
    fn from(value: EnantioexcessCrystalTechnique) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl EnantioexcessCrystalTechnique {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "CD" => Some(Self::Cd),
            "EC" => Some(Self::Ec),
            "other" => Some(Self::Other),
            _ => None,
        }
    }
}
