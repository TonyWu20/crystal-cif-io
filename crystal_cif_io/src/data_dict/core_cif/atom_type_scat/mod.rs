use std::fmt::Display;

use crate::data_dict::{CifTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// The single-value terms of the `atom_type_scat` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomTypeScatTerms {
    /// The `_atom_type_scat_cromer_mann_as` item. The variant holds the single value.
    Cromer_mann_as(Numeric),
    /// The `_atom_type_scat_cromer_mann_bs` item. The variant holds the single value.
    Cromer_mann_bs(Numeric),
    /// The `_atom_type_scat_dispersion` item. The variant holds the single value.
    Dispersion(Numeric),
    /// The `_atom_type_scat_dispersion_imag_cu` item. The variant holds the single value.
    Dispersion_imag_cu(Numeric),
    /// The `_atom_type_scat_dispersion_imag_mo` item. The variant holds the single value.
    Dispersion_imag_mo(Numeric),
    /// The `_atom_type_scat_dispersion_real_cu` item. The variant holds the single value.
    Dispersion_real_cu(Numeric),
    /// The `_atom_type_scat_dispersion_real_mo` item. The variant holds the single value.
    Dispersion_real_mo(Numeric),
    /// The `_atom_type_scat_exponential_polynomial_coefs` item. The variant holds the single value.
    Exponential_polynomial_coefs(Numeric),
    /// The `_atom_type_scat_exponential_polynomial_lower_limit` item. The variant holds the single value.
    Exponential_polynomial_lower_limit(Numeric),
    /// The `_atom_type_scat_exponential_polynomial_upper_limit` item. The variant holds the single value.
    Exponential_polynomial_upper_limit(Numeric),
    /// The `_atom_type_scat_gaussian_as` item. The variant holds the single value.
    Gaussian_as(Numeric),
    /// The `_atom_type_scat_gaussian_bs` item. The variant holds the single value.
    Gaussian_bs(Numeric),
    /// The `_atom_type_scat_gaussian_c` item. The variant holds the single value.
    Gaussian_c(Numeric),
    /// The `_atom_type_scat_gaussian_lower_limit` item. The variant holds the single value.
    Gaussian_lower_limit(Numeric),
    /// The `_atom_type_scat_gaussian_upper_limit` item. The variant holds the single value.
    Gaussian_upper_limit(Numeric),
    /// The `_atom_type_scat_hi_ang_fox_c0` item. The variant holds the single value.
    Hi_ang_fox_c0(Numeric),
    /// The `_atom_type_scat_hi_ang_fox_c1` item. The variant holds the single value.
    Hi_ang_fox_c1(Numeric),
    /// The `_atom_type_scat_hi_ang_fox_c2` item. The variant holds the single value.
    Hi_ang_fox_c2(Numeric),
    /// The `_atom_type_scat_hi_ang_fox_c3` item. The variant holds the single value.
    Hi_ang_fox_c3(Numeric),
    /// The `_atom_type_scat_hi_ang_fox_coefs` item. The variant holds the single value.
    Hi_ang_fox_coefs(Numeric),
    /// The `_atom_type_scat_inv_mott_bethe_as` item. The variant holds the single value.
    Inv_mott_bethe_as(Numeric),
    /// The `_atom_type_scat_inv_mott_bethe_bs` item. The variant holds the single value.
    Inv_mott_bethe_bs(Numeric),
    /// The `_atom_type_scat_inv_mott_bethe_c` item. The variant holds the single value.
    Inv_mott_bethe_c(Numeric),
    /// The `_atom_type_scat_inv_mott_bethe_lower_limit` item. The variant holds the single value.
    Inv_mott_bethe_lower_limit(Numeric),
    /// The `_atom_type_scat_inv_mott_bethe_upper_limit` item. The variant holds the single value.
    Inv_mott_bethe_upper_limit(Numeric),
    /// The `_atom_type_scat_symbol` item. The variant holds the single value.
    Symbol(CharString),
}

impl CifTerm for AtomTypeScatTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomTypeScatTerms::Cromer_mann_as(_) => "cromer_mann_as",
            AtomTypeScatTerms::Cromer_mann_bs(_) => "cromer_mann_bs",
            AtomTypeScatTerms::Dispersion(_) => "dispersion",
            AtomTypeScatTerms::Dispersion_imag_cu(_) => "dispersion_imag_cu",
            AtomTypeScatTerms::Dispersion_imag_mo(_) => "dispersion_imag_mo",
            AtomTypeScatTerms::Dispersion_real_cu(_) => "dispersion_real_cu",
            AtomTypeScatTerms::Dispersion_real_mo(_) => "dispersion_real_mo",
            AtomTypeScatTerms::Exponential_polynomial_coefs(_) => "exponential_polynomial_coefs",
            AtomTypeScatTerms::Exponential_polynomial_lower_limit(_) => {
                "exponential_polynomial_lower_limit"
            }
            AtomTypeScatTerms::Exponential_polynomial_upper_limit(_) => {
                "exponential_polynomial_upper_limit"
            }
            AtomTypeScatTerms::Gaussian_as(_) => "gaussian_as",
            AtomTypeScatTerms::Gaussian_bs(_) => "gaussian_bs",
            AtomTypeScatTerms::Gaussian_c(_) => "gaussian_c",
            AtomTypeScatTerms::Gaussian_lower_limit(_) => "gaussian_lower_limit",
            AtomTypeScatTerms::Gaussian_upper_limit(_) => "gaussian_upper_limit",
            AtomTypeScatTerms::Hi_ang_fox_c0(_) => "hi_ang_fox_c0",
            AtomTypeScatTerms::Hi_ang_fox_c1(_) => "hi_ang_fox_c1",
            AtomTypeScatTerms::Hi_ang_fox_c2(_) => "hi_ang_fox_c2",
            AtomTypeScatTerms::Hi_ang_fox_c3(_) => "hi_ang_fox_c3",
            AtomTypeScatTerms::Hi_ang_fox_coefs(_) => "hi_ang_fox_coefs",
            AtomTypeScatTerms::Inv_mott_bethe_as(_) => "inv_mott_bethe_as",
            AtomTypeScatTerms::Inv_mott_bethe_bs(_) => "inv_mott_bethe_bs",
            AtomTypeScatTerms::Inv_mott_bethe_c(_) => "inv_mott_bethe_c",
            AtomTypeScatTerms::Inv_mott_bethe_lower_limit(_) => "inv_mott_bethe_lower_limit",
            AtomTypeScatTerms::Inv_mott_bethe_upper_limit(_) => "inv_mott_bethe_upper_limit",
            AtomTypeScatTerms::Symbol(_) => "symbol",
        };
        Tag::new(format!("atom_type_scat_{suffix}"))
    }
}

impl SingleValueTerm for AtomTypeScatTerms {
    fn value(&self) -> Value {
        match self {
            AtomTypeScatTerms::Cromer_mann_as(n)
            | AtomTypeScatTerms::Cromer_mann_bs(n)
            | AtomTypeScatTerms::Dispersion(n)
            | AtomTypeScatTerms::Dispersion_imag_cu(n)
            | AtomTypeScatTerms::Dispersion_imag_mo(n)
            | AtomTypeScatTerms::Dispersion_real_cu(n)
            | AtomTypeScatTerms::Dispersion_real_mo(n)
            | AtomTypeScatTerms::Exponential_polynomial_coefs(n)
            | AtomTypeScatTerms::Exponential_polynomial_lower_limit(n)
            | AtomTypeScatTerms::Exponential_polynomial_upper_limit(n)
            | AtomTypeScatTerms::Gaussian_as(n)
            | AtomTypeScatTerms::Gaussian_bs(n)
            | AtomTypeScatTerms::Gaussian_c(n)
            | AtomTypeScatTerms::Gaussian_lower_limit(n)
            | AtomTypeScatTerms::Gaussian_upper_limit(n)
            | AtomTypeScatTerms::Hi_ang_fox_c0(n)
            | AtomTypeScatTerms::Hi_ang_fox_c1(n)
            | AtomTypeScatTerms::Hi_ang_fox_c2(n)
            | AtomTypeScatTerms::Hi_ang_fox_c3(n)
            | AtomTypeScatTerms::Hi_ang_fox_coefs(n)
            | AtomTypeScatTerms::Inv_mott_bethe_as(n)
            | AtomTypeScatTerms::Inv_mott_bethe_bs(n)
            | AtomTypeScatTerms::Inv_mott_bethe_c(n)
            | AtomTypeScatTerms::Inv_mott_bethe_lower_limit(n)
            | AtomTypeScatTerms::Inv_mott_bethe_upper_limit(n) => Value::Numeric(*n),
            AtomTypeScatTerms::Symbol(s) => Value::CharString(s.clone()),
        }
    }
}

impl Display for AtomTypeScatTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}
