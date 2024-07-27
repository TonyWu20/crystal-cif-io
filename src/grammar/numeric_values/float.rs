use std::fmt::Display;

use winnow::{
    ascii::{digit0, digit1},
    combinator::{alt, repeat, separated_pair, terminated},
    error::StrContext,
    PResult, Parser,
};

use crate::grammar::{SyntacticUnit, Value};

use super::{exponent::Exponent, integer::Integer, Number, Numeric};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float(pub f32);

impl Display for Float {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <f32 as Display>::fmt(&self.0, fmt)
    }
}

impl std::ops::Deref for Float {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn sign_zero_or_one(input: &mut &str) -> PResult<String> {
    repeat(0..=1, alt(('+', '-')))
        .context(StrContext::Label("{'+'|'-'}?"))
        .parse_next(input)
}

fn exponent_zero_or_one(input: &mut &str) -> PResult<String> {
    repeat(0..=1, Exponent::parser.map(|e| e.formatted_output()))
        .context(StrContext::Label("{<Exponent>}?"))
        .map(|exps: Vec<String>| exps.concat())
        .parse_next(input)
}

fn float_1(input: &mut &str) -> PResult<f32> {
    (Integer::parser, Exponent::parser)
        .map(|(i, e)| *i as f32 * 10_f32.powi(*e))
        .context(StrContext::Label("<Integer><Exponent>"))
        .parse_next(input)
}

fn float_2(input: &mut &str) -> PResult<f32> {
    (
        sign_zero_or_one,
        alt((
            separated_pair(digit0, '.', digit1).map(|(i, decimal)| format!("{i}.{decimal}")),
            terminated(digit1, '.').map(|d| format!("{d}.")),
        )),
        exponent_zero_or_one,
    )
        .map(|(sign, float, exp)| {
            format!("{sign}{float}{exp}")
                .parse::<f32>()
                .expect("Failed to parse as f32")
        })
        .parse_next(input)
}

impl SyntacticUnit for Float {
    type ParseResult = Self;

    type FormatOutput = f32;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((float_1, float_2)).map(Float).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.0
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float(value as f32)
    }
}

impl From<Float> for Number {
    fn from(value: Float) -> Self {
        Number::Float(value)
    }
}

impl From<Float> for Numeric {
    fn from(value: Float) -> Self {
        Self::new(Number::from(value), None)
    }
}

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Value::Numeric(Numeric::from(value))
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::{Float, SyntacticUnit};

    #[test]
    fn float_parsing() {
        let mut input = "0.0033";
        dbg!(Float::parser(&mut input).unwrap());
        dbg!(Float::parser(&mut ".4254").unwrap());
    }
}
