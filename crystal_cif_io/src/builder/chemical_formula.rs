//! The `chemical_formula` category builder.
//!
//! The builder takes the recorded formula spellings: `structural`
//! is needed, the other spellings optional. The `weight` item is a
//! helper, not a builder field: the molar mass computes from the
//! formula's element composition, and the optional density
//! argument cross-checks it against the structure-level density
//! when the structure Z and the cell volume are in hand.

use bon::bon;
use elements_rs::Element;
use std::str::FromStr;

use crate::data_dict::core_cif::chemical_formula::{ChemicalFormula, FormulaWeight};
use crate::data_dict::core_cif::range_check::number_as_f64;
use crystal_cif_fmt::grammar::{CharString, Numeric};

use super::cell::DEFAULT_TOLERANCE;

/// Avogadro's number, g/mol per mol of formula units.
const AVOGADRO: f64 = 6.022_140_76e23;

/// A build or helper failure of the `chemical_formula` category.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum FormulaError {
    /// The formula text parses to no element composition.
    #[error("the formula {0:?} parses to no element composition")]
    UnparsedFormula(String),
    /// The composition names a symbol that is no element.
    #[error("the symbol {0} is no element")]
    UnknownElement(String),
    /// The computed molar mass misses the `1.0:` range.
    #[error("the {item} value misses its range: {detail}")]
    Range { item: &'static str, detail: String },
    /// The supplied density misses the structure-level expectation
    /// beyond the relative tolerance.
    #[error("the supplied density {supplied} misses the computed {computed} beyond the relative {tolerance}")]
    DensityMismatch {
        supplied: f64,
        computed: f64,
        tolerance: f64,
    },
    /// The density is supplied but the check needs the Z and the
    /// cell volume alongside it.
    #[error("the density {density} needs the structure Z and the cell volume for the check")]
    MissingDensityInputs { density: f64 },
}

#[cfg(test)]
fn unquoted(text: &str) -> CharString {
    CharString::Unquoted(crystal_cif_fmt::grammar::UnquotedString::new(
        text.to_string(),
    ))
}

/// The element composition of a formula text. The element counts
/// merge across the text. Parenthesized groups multiply. The
/// hydration dot and the charge signs are separators.
pub fn composition(formula: &str) -> Result<Vec<(Element, u32)>, FormulaError> {
    let chars: Vec<char> = formula.chars().filter(|c| !c.is_whitespace()).collect();
    let mut out: Vec<(Element, u32)> = Vec::new();
    let end = parse_group(&chars, 0, 1u8, &mut out)?;
    if end != chars.len() {
        return Err(FormulaError::UnparsedFormula(formula.to_string()));
    }
    let mut merged: Vec<(Element, u32)> = Vec::new();
    for (element, count) in out {
        if let Some(existing) = merged
            .iter_mut()
            .find(|(e, _)| e.symbol() == element.symbol())
        {
            existing.1 += count;
        } else {
            merged.push((element, count));
        }
    }
    Ok(merged)
}

fn parse_group(
    chars: &[char],
    start: usize,
    depth: u8,
    out: &mut Vec<(Element, u32)>,
) -> Result<usize, FormulaError> {
    if depth > 4 {
        return Err(FormulaError::UnparsedFormula(String::new()));
    }
    let mut i = start;
    loop {
        if i >= chars.len() {
            if depth == 1 {
                return Ok(i);
            }
            return Err(FormulaError::UnparsedFormula(String::new()));
        }
        let c = chars[i];
        match c {
            ')' => return Ok(i + 1),
            '(' => {
                i += 1;
                let snapshot = out.len();
                let (close, _) = parse_group(chars, i, depth + 1, out)
                    .map(|end| (end, ()))
                    .map_err(|_| FormulaError::UnparsedFormula(String::new()))?;
                i = close;
                let factor = parse_count(chars, &mut i)?;
                if factor > 1 {
                    for entry in &mut out[snapshot..] {
                        entry.1 = entry
                            .1
                            .checked_mul(factor)
                            .ok_or(FormulaError::UnparsedFormula(String::new()))?;
                    }
                }
            }
            d if d.is_ascii_digit() => {
                let factor = parse_count(chars, &mut i)?;
                if i >= chars.len() {
                    return Err(FormulaError::UnparsedFormula(String::new()));
                }
                if chars[i] == '(' {
                    i += 1;
                    let snapshot = out.len();
                    let close = parse_group(chars, i, depth + 1, out)?;
                    for entry in &mut out[snapshot..] {
                        entry.1 = entry
                            .1
                            .checked_mul(factor)
                            .ok_or(FormulaError::UnparsedFormula(String::new()))?;
                    }
                    i = close;
                } else {
                    let (element, consumed) = read_symbol(chars, i)?;
                    i += consumed;
                    let count = parse_count(chars, &mut i)?;
                    out.push((
                        element,
                        count
                            .checked_mul(factor)
                            .ok_or(FormulaError::UnparsedFormula(String::new()))?,
                    ));
                }
            }
            c if c.is_ascii_uppercase() => {
                let (element, consumed) = read_symbol(chars, i)?;
                i += consumed;
                let count = parse_count(chars, &mut i)?;
                out.push((element, count));
            }
            '.' | '+' | '-' => {
                i += 1;
            }
            _ => return Err(FormulaError::UnparsedFormula(String::new())),
        }
    }
}

fn parse_count(chars: &[char], i: &mut usize) -> Result<u32, FormulaError> {
    let start = *i;
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        *i += 1;
    }
    if *i == start {
        Ok(1)
    } else {
        chars[start..*i]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .map_err(|_| FormulaError::UnparsedFormula(String::new()))
    }
}

fn read_symbol(chars: &[char], i: usize) -> Result<(Element, usize), FormulaError> {
    let first = chars[i].to_string();
    let two = if i + 1 < chars.len() && chars[i + 1].is_ascii_lowercase() {
        format!("{}{}", first, chars[i + 1])
    } else {
        String::new()
    };
    if !two.is_empty() {
        if let Ok(element) = Element::from_str(&two) {
            return Ok((element, 2));
        }
    }
    if let Ok(element) = Element::from_str(&first) {
        return Ok((element, 1));
    }
    Err(FormulaError::UnknownElement(if two.is_empty() {
        first
    } else {
        two
    }))
}

/// The derived `_chemical_formula_weight` helper. The molar mass
/// computes from the formula's element composition (the standard
/// atomic weights of `elements_rs`). The optional density
/// cross-checks the molar mass against the structure-level
/// expectation when the structure Z and the cell volume are in
/// hand: the expectation is Z times the molar mass over the cell
/// volume and Avogadro's number. A mismatch beyond the relative
/// tolerance rejects.
pub fn formula_weight(
    formula: &str,
    density: Option<Numeric>,
    z: Option<Numeric>,
    volume: Option<Numeric>,
) -> Result<FormulaWeight, FormulaError> {
    let composition = composition(formula)?;
    let mass: f64 = composition
        .iter()
        .map(|(element, count)| element.standard_atomic_weight() * *count as f64)
        .sum();
    let weight =
        FormulaWeight::try_new("weight", Numeric::from(mass)).map_err(|e| FormulaError::Range {
            item: "weight",
            detail: e.to_string(),
        })?;
    if let Some(density) = density {
        let (Some(z), Some(volume)) = (z, volume) else {
            return Err(FormulaError::MissingDensityInputs {
                density: number_as_f64(&density.number()),
            });
        };
        let expected =
            number_as_f64(&z.number()) * mass / (number_as_f64(&volume.number()) * AVOGADRO);
        let supplied = number_as_f64(&density.number());
        if !within_tolerance(supplied, expected, DEFAULT_TOLERANCE) {
            return Err(FormulaError::DensityMismatch {
                supplied,
                computed: expected,
                tolerance: DEFAULT_TOLERANCE,
            });
        }
    }
    Ok(weight)
}

fn within_tolerance(supplied: f64, computed: f64, tolerance: f64) -> bool {
    (supplied - computed).abs() <= tolerance * supplied.abs().max(computed.abs())
}

#[bon]
impl ChemicalFormula {
    /// The `structural` spelling is needed at the `build()` call.
    /// The other spellings and the measured weight emit as given.
    /// The `weight` item is the helper, not a builder field.
    #[builder(start_fn = builder, builder_type = FormulaBuilder, finish_fn = build)]
    pub fn build_formula(
        structural: CharString,
        moiety: Option<CharString>,
        sum: Option<CharString>,
        analytical: Option<CharString>,
        iupac: Option<CharString>,
        weight_meas: Option<FormulaWeight>,
    ) -> Result<ChemicalFormula, FormulaError> {
        Ok(ChemicalFormula {
            structural: Some(structural),
            moiety,
            sum,
            analytical,
            iupac,
            weight_meas,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_dict::core_cif::CifCategory;

    fn cs(text: &str) -> CharString {
        unquoted(text)
    }

    #[test]
    fn the_builder_returns_the_document_struct() {
        let formula = ChemicalFormula::builder()
            .structural(cs("H2O"))
            .build()
            .expect("the formula builds");
        assert!(
            formula.structural.is_some(),
            "the structural spelling is set"
        );
        assert!(formula.moiety.is_none(), "the unset spellings stay absent");
        assert_eq!(formula.to_members().len(), 1);
    }

    #[test]
    fn the_optional_spellings_emit() {
        let weight = FormulaWeight::try_new("weight_meas", Numeric::from(17.0)).expect("in range");
        let formula = ChemicalFormula::builder()
            .structural(cs("H2O"))
            .moiety(cs("(H2O)"))
            .sum(cs("H2O"))
            .analytical(cs("H2O"))
            .iupac(cs("water"))
            .weight_meas(weight)
            .build()
            .expect("the formula builds");
        assert_eq!(formula.to_members().len(), 6);
    }

    #[test]
    fn the_composition_merges_the_counts() {
        let parsed = composition("BaTiO3").expect("the formula parses");
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[2].1, 3);
        let hydrated = composition("H2O.5H2O").expect("the hydrated formula parses");
        let h: u32 = hydrated
            .iter()
            .filter(|(e, _)| e.symbol() == "H")
            .map(|(_, c)| *c)
            .sum();
        assert_eq!(h, 12);
    }

    #[test]
    fn an_unknown_symbol_rejects() {
        assert!(matches!(
            composition("Qq"),
            Err(FormulaError::UnknownElement(_))
        ));
    }

    #[test]
    fn the_weight_computes_the_molar_mass() {
        let weight = formula_weight("H2O", None, None, None).expect("the mass computes");
        assert_eq!(
            number_as_f64(&weight.value().number()),
            18.015,
            "2 x 1.008 + 15.999"
        );
    }

    #[test]
    fn a_density_cross_check_passes() {
        let volume = 8.0 * 18.015 / AVOGADRO;
        let ok = formula_weight(
            "H2O",
            Some(Numeric::from(1.0)),
            Some(Numeric::from(8.0)),
            Some(Numeric::from(volume)),
        )
        .expect("the density matches the expectation");
        assert_eq!(number_as_f64(&ok.value().number()), 18.015,);
    }

    #[test]
    fn a_mismatching_density_rejects() {
        let volume = 8.0 * 18.015 / AVOGADRO;
        let err = formula_weight(
            "H2O",
            Some(Numeric::from(2.0)),
            Some(Numeric::from(8.0)),
            Some(Numeric::from(volume)),
        )
        .unwrap_err();
        assert!(matches!(err, FormulaError::DensityMismatch { .. }));
    }

    #[test]
    fn a_density_without_the_inputs_rejects() {
        let err = formula_weight("H2O", Some(Numeric::from(1.0)), None, None).unwrap_err();
        assert!(matches!(err, FormulaError::MissingDensityInputs { .. }));
    }
}
