//! The `cell_formula_units_Z` category builder.
//!
//! `formula_units` is a derived quantity. The structure Z counts
//! the formula units in the structure: the atom-site count times
//! the symmetry multiplicity, over the atom count of one formula
//! unit. The Z-prime scales the Z by the multiplicity. The
//! computed values cross-check against supplied values within the
//! relative tolerance.

use bon::bon;

use crate::data_dict::core_cif::range_check::number_as_f64;
use crystal_cif_fmt::grammar::{Numeric, UnsignedInteger};

use super::chemical_formula::composition;

/// A failure of the `formula_units` derivation.
#[derive(Debug, Clone, PartialEq)]
pub enum FormulaUnitsError {
    /// The formula carries no atoms.
    NoFormulaAtoms,
    /// The multiplicity is no integer of at least one.
    BadMultiplicity(f64),
    /// The atom count over the formula atom count is not integral.
    NonIntegralZ { total: u64, per_formula: u64 },
    /// A supplied value misses the computed one beyond the
    /// relative tolerance.
    Mismatch {
        item: &'static str,
        supplied: String,
        computed: String,
        tolerance: f64,
    },
    /// The formula composition did not parse.
    Composition(super::chemical_formula::FormulaError),
}

impl std::fmt::Display for FormulaUnitsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFormulaAtoms => {
                write!(f, "the formula carries no atoms")
            }
            Self::BadMultiplicity(m) => {
                write!(f, "the multiplicity {m} is no integer of at least one")
            }
            Self::NonIntegralZ {
                total,
                per_formula,
            } => write!(
                f,
                "the atom count {total} over the formula atom count {per_formula} is not integral"
            ),
            Self::Mismatch {
                item,
                supplied,
                computed,
                tolerance,
            } => write!(
                f,
                "the supplied {item} {supplied} misses the computed {computed} beyond the relative {tolerance}"
            ),
            Self::Composition(e) => write!(f, "the formula composition fails: {e}"),
        }
    }
}

impl std::error::Error for FormulaUnitsError {}

/// The derived `formula_units` category. The values fill the
/// `cell` document fields `formula_units_z` and
/// `formula_units_z_prime`.
#[derive(Debug, Clone)]
pub struct FormulaUnits {
    pub z: UnsignedInteger,
    pub z_prime: Numeric,
    pub multiplicity: Numeric,
}

/// The structure Z from the structure-level quantities. The Z
/// counts the formula units in the structure: the atom-site count
/// times the symmetry multiplicity, over the atom count of one
/// formula unit. The Z-prime divides the Z by the multiplicity.
/// A supplied Z or Z-prime cross-checks against the computed
/// values within the relative tolerance.
pub fn structure_z(
    atom_sites: u32,
    structural_formula: &str,
    multiplicity: Numeric,
    supplied_z: Option<UnsignedInteger>,
    supplied_z_prime: Option<Numeric>,
    tolerance: f64,
) -> Result<FormulaUnits, FormulaUnitsError> {
    let per_formula: u64 = composition(structural_formula)
        .map_err(FormulaUnitsError::Composition)?
        .iter()
        .map(|(_, count)| *count as u64)
        .sum();
    if per_formula == 0 {
        return Err(FormulaUnitsError::NoFormulaAtoms);
    }
    let m = number_as_f64(&multiplicity.number());
    if m < 1.0 || m.fract() != 0.0 {
        return Err(FormulaUnitsError::BadMultiplicity(m));
    }
    let m = m as u64;
    let total = atom_sites as u64 * m;
    let (z, remainder) = (total / per_formula, total % per_formula);
    if remainder != 0 {
        return Err(FormulaUnitsError::NonIntegralZ { total, per_formula });
    }
    let z_u32 = z as u32;
    let z = UnsignedInteger(z_u32);
    let z_prime = Numeric::from(z_u32 as f64 / m as f64);
    if let Some(supplied) = supplied_z {
        if *supplied as f64 != z_u32 as f64 {
            return Err(FormulaUnitsError::Mismatch {
                item: "z",
                supplied: format!("{supplied}"),
                computed: format!("{z}"),
                tolerance,
            });
        }
    }
    if let Some(supplied) = supplied_z_prime {
        let computed = number_as_f64(&z_prime.number());
        if !within_tolerance(number_as_f64(&supplied.number()), computed, tolerance) {
            return Err(FormulaUnitsError::Mismatch {
                item: "z_prime",
                supplied: format!("{supplied}"),
                computed: format!("{z_prime}"),
                tolerance,
            });
        }
    }
    Ok(FormulaUnits {
        z,
        z_prime,
        multiplicity,
    })
}

fn within_tolerance(supplied: f64, computed: f64, tolerance: f64) -> bool {
    (supplied - computed).abs() <= tolerance * supplied.abs().max(computed.abs())
}

#[bon]
impl FormulaUnits {
    /// The derived category is built from the computed Z and
    /// Z-prime. The `builder()` start function exists for the
    /// uniform surface; the derivation is the `structure_z`
    /// helper.
    #[builder(start_fn = builder, builder_type = FormulaUnitsBuilder, finish_fn = build)]
    pub fn build_formula_units(
        z: UnsignedInteger,
        z_prime: Numeric,
        multiplicity: Numeric,
    ) -> Result<FormulaUnits, FormulaUnitsError> {
        Ok(FormulaUnits {
            z,
            z_prime,
            multiplicity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_structure_z_computes() {
        let units = structure_z(
            5,
            "BaTiO3",
            Numeric::from(1.0),
            Some(UnsignedInteger(1)),
            Some(Numeric::from(1.0)),
            1e-3,
        )
        .expect("the Z computes");
        assert_eq!(units.z.0, 1);
        assert_eq!(number_as_f64(&units.z_prime.number()), 1.0);
    }

    #[test]
    fn a_supplied_z_cross_checks() {
        let err = structure_z(
            5,
            "BaTiO3",
            Numeric::from(1.0),
            Some(UnsignedInteger(8)),
            None,
            1e-3,
        )
        .unwrap_err();
        assert!(matches!(err, FormulaUnitsError::Mismatch { item: "z", .. }));
    }

    #[test]
    fn a_supplied_z_prime_cross_checks() {
        let err = structure_z(
            5,
            "BaTiO3",
            Numeric::from(1.0),
            None,
            Some(Numeric::from(0.0)),
            1e-3,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            FormulaUnitsError::Mismatch {
                item: "z_prime",
                ..
            }
        ));
    }

    #[test]
    fn a_non_integral_z_rejects() {
        let err = structure_z(1, "H2O", Numeric::from(4.0), None, None, 1e-3).unwrap_err();
        assert!(matches!(err, FormulaUnitsError::NonIntegralZ { .. }));
    }

    #[test]
    fn a_fractional_multiplicity_rejects() {
        let err = structure_z(1, "H2O", Numeric::from(2.5), None, None, 1e-3).unwrap_err();
        assert!(matches!(err, FormulaUnitsError::BadMultiplicity(2.5)));
    }

    #[test]
    fn an_atomless_formula_rejects() {
        let err = structure_z(1, "", Numeric::from(1.0), None, None, 1e-3).unwrap_err();
        assert!(matches!(err, FormulaUnitsError::NoFormulaAtoms));
    }

    #[test]
    fn the_multiplicity_scales_the_site_count() {
        let units = structure_z(
            2,
            "H2O",
            Numeric::from(6.0),
            Some(UnsignedInteger(4)),
            None,
            1e-3,
        )
        .expect("the Z computes");
        assert_eq!(units.z.0, 4);
        assert!(
            (number_as_f64(&units.z_prime.number()) - 2.0 / 3.0).abs() < 1e-6,
            "the Z-prime is two thirds"
        );
    }
}
