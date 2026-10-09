use std::fmt::Display;

use crate::{
    data_dict::{matrix, CifTerm, SingleValueTerm},
    grammar::{CharString, Matrix, Numeric, Tag, Value},
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DisplayColourTerms {
    Blue(Numeric),
    Green(Numeric),
    Hue(CharString),
    Red(Numeric),
    /// list or array payload; the shape is recorded in the shell
    /// _type.dimension
    Rgb(Matrix),
}

impl CifTerm for DisplayColourTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DisplayColourTerms::Blue(_) => "blue",
            DisplayColourTerms::Green(_) => "green",
            DisplayColourTerms::Hue(_) => "hue",
            DisplayColourTerms::Red(_) => "red",
            DisplayColourTerms::Rgb(_) => "rgb",
        };
        Tag::new(format!("display_colour_{suffix}"))
    }
}

impl SingleValueTerm for DisplayColourTerms {
    fn value(&self) -> Value {
        match self {
            DisplayColourTerms::Blue(n)
            | DisplayColourTerms::Green(n)
            | DisplayColourTerms::Red(n) => Value::Numeric(*n),
            DisplayColourTerms::Hue(s) => Value::CharString(s.clone()),
            DisplayColourTerms::Rgb(m) => matrix::to_value(m),
        }
    }
}

impl Display for DisplayColourTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
