use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed code set of `_publ_body_element`. The codes are recorded in
/// the dictionary `_enumeration` (see `tests/dict_inventory.json`).
///
/// closed code set recorded in the dictionary _enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Element {
    /// `section`
    Section,
    /// `subsection`
    Subsection,
    /// `subsubsection`
    Subsubsection,
    /// `appendix`
    Appendix,
    /// `footnote`
    Footnote,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Section => "section",
            Self::Subsection => "subsection",
            Self::Subsubsection => "subsubsection",
            Self::Appendix => "appendix",
            Self::Footnote => "footnote",
        };
        f.write_str(code)
    }
}

impl From<Element> for UnquotedString {
    fn from(value: Element) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<Element> for CharString {
    fn from(value: Element) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Element> for Value {
    fn from(value: Element) -> Self {
        Value::CharString(CharString::from(value))
    }
}
