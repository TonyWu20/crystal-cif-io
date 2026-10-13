use std::fmt::Display;

use crate::grammar::{CharString, UnquotedString, Value};

/// The closed code set of the legacy `_atom_site_refinement_flags`.
/// The dictionary marks the item as a legacy compatibility item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefinementFlag {
    /// `.`: no refinement constraints.
    NoConstraint,
    /// `S`: special-position constraint on the site.
    S,
    /// `G`: rigid-group refinement of the site.
    G,
    /// `R`: riding-atom site attached to a non-riding atom.
    R,
    /// `D`: distance or angle restraint on the site.
    D,
    /// `T`: thermal displacement constraints.
    T,
    /// `U`: Uiso or Uij restraint (rigid bond).
    U,
    /// `P`: partial occupancy constraint.
    P,
}

impl Display for RefinementFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::NoConstraint => ".",
            Self::S => "S",
            Self::G => "G",
            Self::R => "R",
            Self::D => "D",
            Self::T => "T",
            Self::U => "U",
            Self::P => "P",
        };
        f.write_str(code)
    }
}

impl From<RefinementFlag> for UnquotedString {
    fn from(value: RefinementFlag) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<RefinementFlag> for CharString {
    fn from(value: RefinementFlag) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<RefinementFlag> for Value {
    fn from(value: RefinementFlag) -> Self {
        Value::CharString(CharString::from(value))
    }
}

/// The closed code set of `_atom_site_refinement_flags_adp`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdpRefineFlag {
    /// `.`: no constraints on the atomic displacement parameters.
    NoConstraint,
    /// `T`: special-position constraints.
    T,
    /// `U`: Uiso or Uij restraint (rigid bond).
    U,
    /// `TU`: both constraints applied.
    TU,
}

impl Display for AdpRefineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::NoConstraint => ".",
            Self::T => "T",
            Self::U => "U",
            Self::TU => "TU",
        };
        f.write_str(code)
    }
}

impl From<AdpRefineFlag> for UnquotedString {
    fn from(value: AdpRefineFlag) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<AdpRefineFlag> for CharString {
    fn from(value: AdpRefineFlag) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<AdpRefineFlag> for Value {
    fn from(value: AdpRefineFlag) -> Self {
        Value::CharString(CharString::from(value))
    }
}

/// The closed code set of `_atom_site_refinement_flags_occupancy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OccupancyRefineFlag {
    /// `.`: no constraints on the site-occupancy parameters.
    NoConstraint,
    /// `P`: site-occupancy constraint.
    P,
}

impl Display for OccupancyRefineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::NoConstraint => ".",
            Self::P => "P",
        };
        f.write_str(code)
    }
}

impl From<OccupancyRefineFlag> for UnquotedString {
    fn from(value: OccupancyRefineFlag) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<OccupancyRefineFlag> for CharString {
    fn from(value: OccupancyRefineFlag) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<OccupancyRefineFlag> for Value {
    fn from(value: OccupancyRefineFlag) -> Self {
        Value::CharString(CharString::from(value))
    }
}

/// The closed code set of `_atom_site_refinement_flags_posn`. The
/// dictionary records the single-letter codes and their
/// combinations.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosnRefineFlag {
    /// `.`: no constraints on the positional coordinates.
    NoConstraint,
    /// `D`: distance or angle restraint.
    D,
    /// `G`: rigid-group refinement.
    G,
    /// `R`: riding-atom site.
    R,
    /// `S`: special-position constraint.
    S,
    /// `DG`: combination of the distance and rigid-group codes.
    DG,
    /// `DR`: combination of the distance and riding-atom codes.
    DR,
    /// `DS`: combination of the distance and special-position codes.
    DS,
    /// `GR`: combination of the rigid-group and riding-atom codes.
    GR,
    /// `GS`: combination of the rigid-group and special-position codes.
    GS,
    /// `RS`: combination of the riding-atom and special-position codes.
    RS,
    /// `DGR`: combination of the distance, rigid-group, and riding-atom codes.
    DGR,
    /// `DGS`: combination of the distance, rigid-group, and special-position codes.
    DGS,
    /// `DRS`: combination of the distance, riding-atom, and special-position codes.
    DRS,
    /// `GRS`: combination of the rigid-group, riding-atom, and special-position codes.
    GRS,
    /// `DGRS`: combination of all four single-letter codes.
    DGRS,
}

impl Display for PosnRefineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::NoConstraint => ".",
            Self::D => "D",
            Self::G => "G",
            Self::R => "R",
            Self::S => "S",
            Self::DG => "DG",
            Self::DR => "DR",
            Self::DS => "DS",
            Self::GR => "GR",
            Self::GS => "GS",
            Self::RS => "RS",
            Self::DGR => "DGR",
            Self::DGS => "DGS",
            Self::DRS => "DRS",
            Self::GRS => "GRS",
            Self::DGRS => "DGRS",
        };
        f.write_str(code)
    }
}

impl From<PosnRefineFlag> for UnquotedString {
    fn from(value: PosnRefineFlag) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<PosnRefineFlag> for CharString {
    fn from(value: PosnRefineFlag) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<PosnRefineFlag> for Value {
    fn from(value: PosnRefineFlag) -> Self {
        Value::CharString(CharString::from(value))
    }
}
