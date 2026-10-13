use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_atom_site_calc_flag`. The dictionary
/// records the codes `d`, `calc`, `c`, and `dum`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcFlag {
    /// `d`: determined from diffraction measurements.
    D,
    /// `calc`: calculated from molecular geometry.
    Calc,
    /// `c`: abbreviation for `calc`.
    C,
    /// `dum`: dummy site with meaningless coordinates.
    Dum,
}

impl Display for CalcFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            CalcFlag::D => "d",
            CalcFlag::Calc => "calc",
            CalcFlag::C => "c",
            CalcFlag::Dum => "dum",
        };
        f.write_str(code)
    }
}

impl From<CalcFlag> for UnquotedString {
    fn from(value: CalcFlag) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<CalcFlag> for CharString {
    fn from(value: CalcFlag) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<CalcFlag> for Value {
    fn from(value: CalcFlag) -> Self {
        Value::CharString(CharString::from(value))
    }
}


impl CalcFlag {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "d" => Some(Self::D),
            "calc" => Some(Self::Calc),
            "c" => Some(Self::C),
            "dum" => Some(Self::Dum),
            _ => None,
        }
    }
}
