//! The closed code sets of the `exptl_crystal` colour items.

use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

use super::ExptlCrystalCifError;

/// The closed value set of `_exptl_crystal_colour_lustre`.
///
/// The set includes the inapplicable value `.`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lustre {
    Metallic,
    Lustrous,
    Transparent,
    Translucent,
    Opaque,
    Dull,
    Clear,
    Inapplicable,
}

impl Lustre {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Metallic => "metallic",
            Self::Lustrous => "lustrous",
            Self::Transparent => "transparent",
            Self::Translucent => "translucent",
            Self::Opaque => "opaque",
            Self::Dull => "dull",
            Self::Clear => "clear",
            Self::Inapplicable => ".",
        }
    }
}

impl Display for Lustre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<Lustre> for UnquotedString {
    fn from(value: Lustre) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<Lustre> for CharString {
    fn from(value: Lustre) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<Lustre> for Value {
    fn from(value: Lustre) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for Lustre {
    type Error = ExptlCrystalCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "metallic" => Ok(Self::Metallic),
            "lustrous" => Ok(Self::Lustrous),
            "transparent" => Ok(Self::Transparent),
            "translucent" => Ok(Self::Translucent),
            "opaque" => Ok(Self::Opaque),
            "dull" => Ok(Self::Dull),
            "clear" => Ok(Self::Clear),
            "." => Ok(Self::Inapplicable),
            other => Err(ExptlCrystalCifError::new(format!(
                "'{other}' is not a value of the _exptl_crystal_colour_lustre closed set"
            ))),
        }
    }
}

impl TryFrom<String> for Lustre {
    type Error = ExptlCrystalCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// The closed value set of `_exptl_crystal_colour_modifier`.
///
/// The set includes the inapplicable value `.`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColourModifier {
    Light,
    Dark,
    Pale,
    Intense,
    Whitish,
    Blackish,
    Grayish,
    Brownish,
    Reddish,
    Pinkish,
    Orangish,
    Yellowish,
    Greenish,
    Bluish,
    Inapplicable,
}

impl ColourModifier {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Pale => "pale",
            Self::Intense => "intense",
            Self::Whitish => "whitish",
            Self::Blackish => "blackish",
            Self::Grayish => "grayish",
            Self::Brownish => "brownish",
            Self::Reddish => "reddish",
            Self::Pinkish => "pinkish",
            Self::Orangish => "orangish",
            Self::Yellowish => "yellowish",
            Self::Greenish => "greenish",
            Self::Bluish => "bluish",
            Self::Inapplicable => ".",
        }
    }
}

impl Display for ColourModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<ColourModifier> for UnquotedString {
    fn from(value: ColourModifier) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<ColourModifier> for CharString {
    fn from(value: ColourModifier) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<ColourModifier> for Value {
    fn from(value: ColourModifier) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for ColourModifier {
    type Error = ExptlCrystalCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "pale" => Ok(Self::Pale),
            "intense" => Ok(Self::Intense),
            "whitish" => Ok(Self::Whitish),
            "blackish" => Ok(Self::Blackish),
            "grayish" => Ok(Self::Grayish),
            "brownish" => Ok(Self::Brownish),
            "reddish" => Ok(Self::Reddish),
            "pinkish" => Ok(Self::Pinkish),
            "orangish" => Ok(Self::Orangish),
            "yellowish" => Ok(Self::Yellowish),
            "greenish" => Ok(Self::Greenish),
            "bluish" => Ok(Self::Bluish),
            "." => Ok(Self::Inapplicable),
            other => Err(ExptlCrystalCifError::new(format!(
                "'{other}' is not a value of the _exptl_crystal_colour_modifier closed set"
            ))),
        }
    }
}

impl TryFrom<String> for ColourModifier {
    type Error = ExptlCrystalCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// The closed value set of `_exptl_crystal_colour_primary`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColourPrimary {
    Colourless,
    White,
    Black,
    Gray,
    Brown,
    Red,
    Pink,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
}

impl ColourPrimary {
    /// The dictionary spelling of the value.
    pub fn dictionary_value(&self) -> &str {
        match self {
            Self::Colourless => "colourless",
            Self::White => "white",
            Self::Black => "black",
            Self::Gray => "gray",
            Self::Brown => "brown",
            Self::Red => "red",
            Self::Pink => "pink",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Blue => "blue",
            Self::Violet => "violet",
        }
    }
}

impl Display for ColourPrimary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.dictionary_value())
    }
}

impl From<ColourPrimary> for UnquotedString {
    fn from(value: ColourPrimary) -> Self {
        UnquotedString::new(value.dictionary_value().to_string())
    }
}

impl From<ColourPrimary> for CharString {
    fn from(value: ColourPrimary) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<ColourPrimary> for Value {
    fn from(value: ColourPrimary) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl TryFrom<&str> for ColourPrimary {
    type Error = ExptlCrystalCifError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "colourless" => Ok(Self::Colourless),
            "white" => Ok(Self::White),
            "black" => Ok(Self::Black),
            "gray" => Ok(Self::Gray),
            "brown" => Ok(Self::Brown),
            "red" => Ok(Self::Red),
            "pink" => Ok(Self::Pink),
            "orange" => Ok(Self::Orange),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            "violet" => Ok(Self::Violet),
            other => Err(ExptlCrystalCifError::new(format!(
                "'{other}' is not a value of the _exptl_crystal_colour_primary closed set"
            ))),
        }
    }
}

impl TryFrom<String> for ColourPrimary {
    type Error = ExptlCrystalCifError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lustre_round_trip() {
        for value in [
            Lustre::Metallic,
            Lustre::Lustrous,
            Lustre::Transparent,
            Lustre::Translucent,
            Lustre::Opaque,
            Lustre::Dull,
            Lustre::Clear,
            Lustre::Inapplicable,
        ] {
            let parsed = Lustre::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
    }

    #[test]
    fn modifier_round_trip() {
        for value in [
            ColourModifier::Light,
            ColourModifier::Dark,
            ColourModifier::Pale,
            ColourModifier::Intense,
            ColourModifier::Whitish,
            ColourModifier::Blackish,
            ColourModifier::Grayish,
            ColourModifier::Brownish,
            ColourModifier::Reddish,
            ColourModifier::Pinkish,
            ColourModifier::Orangish,
            ColourModifier::Yellowish,
            ColourModifier::Greenish,
            ColourModifier::Bluish,
            ColourModifier::Inapplicable,
        ] {
            let parsed = ColourModifier::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
    }

    #[test]
    fn primary_round_trip() {
        for value in [
            ColourPrimary::Colourless,
            ColourPrimary::White,
            ColourPrimary::Black,
            ColourPrimary::Gray,
            ColourPrimary::Brown,
            ColourPrimary::Red,
            ColourPrimary::Pink,
            ColourPrimary::Orange,
            ColourPrimary::Yellow,
            ColourPrimary::Green,
            ColourPrimary::Blue,
            ColourPrimary::Violet,
        ] {
            let parsed = ColourPrimary::try_from(value.dictionary_value()).unwrap();
            assert_eq!(value, parsed);
        }
    }

    #[test]
    fn closed_sets_reject_unknown_values() {
        assert!(Lustre::try_from("shiny").is_err());
        assert!(ColourModifier::try_from("shiny").is_err());
        assert!(ColourPrimary::try_from("shiny").is_err());
    }
}
