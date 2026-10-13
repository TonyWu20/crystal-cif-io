use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of the `_geom_*_publ_flag` items of the
/// `geom_angle`, `geom_bond`, `geom_contact`, `geom_hbond`, and
/// `geom_torsion` categories. The flag decides whether the value
/// enters the published special list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublFlag {
    /// `no`: do not include the value in the special list.
    No,
    /// `n`: abbreviation for `no`.
    N,
    /// `yes`: include the value in the special list.
    Yes,
    /// `y`: abbreviation for `yes`.
    Y,
}

impl Display for PublFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            PublFlag::No => "no",
            PublFlag::N => "n",
            PublFlag::Yes => "yes",
            PublFlag::Y => "y",
        };
        f.write_str(code)
    }
}

impl From<PublFlag> for UnquotedString {
    fn from(value: PublFlag) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<PublFlag> for CharString {
    fn from(value: PublFlag) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<PublFlag> for Value {
    fn from(value: PublFlag) -> Self {
        Value::CharString(CharString::from(value))
    }
}
