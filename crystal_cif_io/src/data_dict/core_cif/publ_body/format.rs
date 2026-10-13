use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_publ_body_format`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Format {
    /// `ascii`
    Ascii,
    /// `cif`
    Cif,
    /// `latex`
    Latex,
    /// `rtf`
    Rtf,
    /// `sgml`
    Sgml,
    /// `tex`
    Tex,
    /// `troff`
    Troff,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Ascii => "ascii",
            Self::Cif => "cif",
            Self::Latex => "latex",
            Self::Rtf => "rtf",
            Self::Sgml => "sgml",
            Self::Tex => "tex",
            Self::Troff => "troff",
        };
        f.write_str(code)
    }
}

impl From<Format> for UnquotedString {
    fn from(value: Format) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Format> for CharString {
    fn from(value: Format) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Format> for Value {
    fn from(value: Format) -> Self {
        Value::CharString(CharString::from(value))
    }
}
