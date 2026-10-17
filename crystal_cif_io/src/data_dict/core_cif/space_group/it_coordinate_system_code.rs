use std::fmt::Display;

use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};

/// The closed set of IT coordinate system codes (the shell dictionary records forty).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItCoordinateSystemCode {
    /// `b1`
    B1,
    /// `b2`
    B2,
    /// `b3`
    B3,
    /// `-b1`
    MinusB1,
    /// `-b2`
    MinusB2,
    /// `-b3`
    MinusB3,
    /// `c1`
    C1,
    /// `c2`
    C2,
    /// `c3`
    C3,
    /// `-c1`
    MinusC1,
    /// `-c2`
    MinusC2,
    /// `-c3`
    MinusC3,
    /// `a1`
    A1,
    /// `a2`
    A2,
    /// `a3`
    A3,
    /// `-a1`
    MinusA1,
    /// `-a2`
    MinusA2,
    /// `-a3`
    MinusA3,
    /// `abc`
    Abc,
    /// `ba-c`
    BaMinusC,
    /// `cab`
    Cab,
    /// `-cba`
    MinusCba,
    /// `bca`
    Bca,
    /// `a-cb`
    AMinusCb,
    /// `1abc`
    OneAbc,
    /// `1ba-c`
    OneBaMinusC,
    /// `1cab`
    OneCab,
    /// `1-cba`
    OneMinusCba,
    /// `1bca`
    OneBca,
    /// `1a-cb`
    OneAMinusCb,
    /// `2abc`
    TwoAbc,
    /// `2ba-c`
    TwoBaMinusC,
    /// `2cab`
    TwoCab,
    /// `2-cba`
    TwoMinusCba,
    /// `2bca`
    TwoBca,
    /// `2a-cb`
    TwoAMinusCb,
    /// `1`
    One,
    /// `2`
    Two,
    /// `h`
    H,
    /// `r`
    R,
}

impl Display for ItCoordinateSystemCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::B1 => "b1",
            Self::B2 => "b2",
            Self::B3 => "b3",
            Self::MinusB1 => "-b1",
            Self::MinusB2 => "-b2",
            Self::MinusB3 => "-b3",
            Self::C1 => "c1",
            Self::C2 => "c2",
            Self::C3 => "c3",
            Self::MinusC1 => "-c1",
            Self::MinusC2 => "-c2",
            Self::MinusC3 => "-c3",
            Self::A1 => "a1",
            Self::A2 => "a2",
            Self::A3 => "a3",
            Self::MinusA1 => "-a1",
            Self::MinusA2 => "-a2",
            Self::MinusA3 => "-a3",
            Self::Abc => "abc",
            Self::BaMinusC => "ba-c",
            Self::Cab => "cab",
            Self::MinusCba => "-cba",
            Self::Bca => "bca",
            Self::AMinusCb => "a-cb",
            Self::OneAbc => "1abc",
            Self::OneBaMinusC => "1ba-c",
            Self::OneCab => "1cab",
            Self::OneMinusCba => "1-cba",
            Self::OneBca => "1bca",
            Self::OneAMinusCb => "1a-cb",
            Self::TwoAbc => "2abc",
            Self::TwoBaMinusC => "2ba-c",
            Self::TwoCab => "2cab",
            Self::TwoMinusCba => "2-cba",
            Self::TwoBca => "2bca",
            Self::TwoAMinusCb => "2a-cb",
            Self::One => "1",
            Self::Two => "2",
            Self::H => "h",
            Self::R => "r",
        };
        write!(f, "{}", code)
    }
}

impl From<ItCoordinateSystemCode> for UnquotedString {
    fn from(value: ItCoordinateSystemCode) -> Self {
        UnquotedString::new(format!("{value}"))
    }
}

impl From<ItCoordinateSystemCode> for CharString {
    fn from(value: ItCoordinateSystemCode) -> Self {
        CharString::Unquoted(UnquotedString::from(value))
    }
}

impl From<ItCoordinateSystemCode> for Value {
    fn from(value: ItCoordinateSystemCode) -> Self {
        Value::CharString(CharString::from(value))
    }
}

impl ItCoordinateSystemCode {
    /// Parse a closed-set code into a variant. None when the code is not in the set.
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "b1" => Some(Self::B1),
            "b2" => Some(Self::B2),
            "b3" => Some(Self::B3),
            "-b1" => Some(Self::MinusB1),
            "-b2" => Some(Self::MinusB2),
            "-b3" => Some(Self::MinusB3),
            "c1" => Some(Self::C1),
            "c2" => Some(Self::C2),
            "c3" => Some(Self::C3),
            "-c1" => Some(Self::MinusC1),
            "-c2" => Some(Self::MinusC2),
            "-c3" => Some(Self::MinusC3),
            "a1" => Some(Self::A1),
            "a2" => Some(Self::A2),
            "a3" => Some(Self::A3),
            "-a1" => Some(Self::MinusA1),
            "-a2" => Some(Self::MinusA2),
            "-a3" => Some(Self::MinusA3),
            "abc" => Some(Self::Abc),
            "ba-c" => Some(Self::BaMinusC),
            "cab" => Some(Self::Cab),
            "-cba" => Some(Self::MinusCba),
            "bca" => Some(Self::Bca),
            "a-cb" => Some(Self::AMinusCb),
            "1abc" => Some(Self::OneAbc),
            "1ba-c" => Some(Self::OneBaMinusC),
            "1cab" => Some(Self::OneCab),
            "1-cba" => Some(Self::OneMinusCba),
            "1bca" => Some(Self::OneBca),
            "1a-cb" => Some(Self::OneAMinusCb),
            "2abc" => Some(Self::TwoAbc),
            "2ba-c" => Some(Self::TwoBaMinusC),
            "2cab" => Some(Self::TwoCab),
            "2-cba" => Some(Self::TwoMinusCba),
            "2bca" => Some(Self::TwoBca),
            "2a-cb" => Some(Self::TwoAMinusCb),
            "1" => Some(Self::One),
            "2" => Some(Self::Two),
            "h" => Some(Self::H),
            "r" => Some(Self::R),
            _ => None,
        }
    }
}
