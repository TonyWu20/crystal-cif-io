//! The `atom_type` category (22 items, all loop columns).
//!
//! One loop lists the atom types of the structure. Each column is a
//! property of a type: the symbol, the oxidation number, the count,
//! the radii, and the scattering-factor parameters (Cromer-Mann
//! coefficients, dispersion terms, neutron length, source tags).

mod ranges;

pub use ranges::{AnalyticalMass, NumberInCell, OxidationNumber, Radius};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// Loop columns of the `atom_type` category (22 items).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomTypeLoopItem {
    Analytical_mass_percent(Vec<AnalyticalMass>),
    Description(Vec<CharString>),
    Number_in_cell(Vec<NumberInCell>),
    Oxidation_number(Vec<OxidationNumber>),
    Radius_bond(Vec<Radius>),
    Radius_contact(Vec<Radius>),
    Scat_Cromer_Mann_a1(Vec<Numeric>),
    Scat_Cromer_Mann_a2(Vec<Numeric>),
    Scat_Cromer_Mann_a3(Vec<Numeric>),
    Scat_Cromer_Mann_a4(Vec<Numeric>),
    Scat_Cromer_Mann_b1(Vec<Numeric>),
    Scat_Cromer_Mann_b2(Vec<Numeric>),
    Scat_Cromer_Mann_b3(Vec<Numeric>),
    Scat_Cromer_Mann_b4(Vec<Numeric>),
    Scat_Cromer_Mann_c(Vec<Numeric>),
    Scat_dispersion_imag(Vec<Numeric>),
    Scat_dispersion_real(Vec<Numeric>),
    Scat_dispersion_source(Vec<CharString>),
    Scat_length_neutron(Vec<Numeric>),
    Scat_source(Vec<CharString>),
    Scat_versus_stol_list(Vec<CharString>),
    Symbol(Vec<CharString>),
}

impl CifTerm for AtomTypeLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomTypeLoopItem::Analytical_mass_percent(_) => "analytical_mass_%",
            AtomTypeLoopItem::Description(_) => "description",
            AtomTypeLoopItem::Number_in_cell(_) => "number_in_cell",
            AtomTypeLoopItem::Oxidation_number(_) => "oxidation_number",
            AtomTypeLoopItem::Radius_bond(_) => "radius_bond",
            AtomTypeLoopItem::Radius_contact(_) => "radius_contact",
            AtomTypeLoopItem::Scat_Cromer_Mann_a1(_) => "scat_Cromer_Mann_a1",
            AtomTypeLoopItem::Scat_Cromer_Mann_a2(_) => "scat_Cromer_Mann_a2",
            AtomTypeLoopItem::Scat_Cromer_Mann_a3(_) => "scat_Cromer_Mann_a3",
            AtomTypeLoopItem::Scat_Cromer_Mann_a4(_) => "scat_Cromer_Mann_a4",
            AtomTypeLoopItem::Scat_Cromer_Mann_b1(_) => "scat_Cromer_Mann_b1",
            AtomTypeLoopItem::Scat_Cromer_Mann_b2(_) => "scat_Cromer_Mann_b2",
            AtomTypeLoopItem::Scat_Cromer_Mann_b3(_) => "scat_Cromer_Mann_b3",
            AtomTypeLoopItem::Scat_Cromer_Mann_b4(_) => "scat_Cromer_Mann_b4",
            AtomTypeLoopItem::Scat_Cromer_Mann_c(_) => "scat_Cromer_Mann_c",
            AtomTypeLoopItem::Scat_dispersion_imag(_) => "scat_dispersion_imag",
            AtomTypeLoopItem::Scat_dispersion_real(_) => "scat_dispersion_real",
            AtomTypeLoopItem::Scat_dispersion_source(_) => "scat_dispersion_source",
            AtomTypeLoopItem::Scat_length_neutron(_) => "scat_length_neutron",
            AtomTypeLoopItem::Scat_source(_) => "scat_source",
            AtomTypeLoopItem::Scat_versus_stol_list(_) => "scat_versus_stol_list",
            AtomTypeLoopItem::Symbol(_) => "symbol",
        };
        Tag::new(format!("atom_type_{suffix}"))
    }
}

impl LoopValueTerm for AtomTypeLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            AtomTypeLoopItem::Analytical_mass_percent(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            AtomTypeLoopItem::Description(v)
            | AtomTypeLoopItem::Scat_dispersion_source(v)
            | AtomTypeLoopItem::Scat_source(v)
            | AtomTypeLoopItem::Scat_versus_stol_list(v)
            | AtomTypeLoopItem::Symbol(v) => v.iter().cloned().map(Value::from).collect(),
            AtomTypeLoopItem::Number_in_cell(v) => v.iter().cloned().map(Value::from).collect(),
            AtomTypeLoopItem::Oxidation_number(v) => v.iter().cloned().map(Value::from).collect(),
            AtomTypeLoopItem::Radius_bond(v) | AtomTypeLoopItem::Radius_contact(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
            AtomTypeLoopItem::Scat_Cromer_Mann_a1(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_a2(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_a3(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_a4(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_b1(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_b2(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_b3(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_b4(v)
            | AtomTypeLoopItem::Scat_Cromer_Mann_c(v)
            | AtomTypeLoopItem::Scat_dispersion_imag(v)
            | AtomTypeLoopItem::Scat_dispersion_real(v)
            | AtomTypeLoopItem::Scat_length_neutron(v) => {
                v.iter().cloned().map(Value::from).collect()
            }
        }
    }
}

// The T6 single-value items of the `atom_type` category.

/// The single-value items of the `atom_type` category (T6).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomTypeTerms {
    Atomic_mass(Numeric),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `1:`); the plain numeric form is kept.
    Atomic_number(Numeric),
    Display_colour(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `1:`); the plain numeric form is kept.
    Electron_count(Numeric),
    Element_symbol(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `1:`); the plain numeric form is kept.
    Mass_number(Numeric),
}

impl CifTerm for AtomTypeTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            AtomTypeTerms::Atomic_mass(_) => "atomic_mass",
            AtomTypeTerms::Atomic_number(_) => "atomic_number",
            AtomTypeTerms::Display_colour(_) => "display_colour",
            AtomTypeTerms::Electron_count(_) => "electron_count",
            AtomTypeTerms::Element_symbol(_) => "element_symbol",
            AtomTypeTerms::Mass_number(_) => "mass_number",
        };
        Tag::new(format!("atom_type_{suffix}"))
    }
}

impl SingleValueTerm for AtomTypeTerms {
    fn value(&self) -> Value {
        match self {
            AtomTypeTerms::Atomic_mass(n)
            | AtomTypeTerms::Atomic_number(n)
            | AtomTypeTerms::Electron_count(n)
            | AtomTypeTerms::Mass_number(n) => Value::Numeric(*n),
            AtomTypeTerms::Display_colour(s) | AtomTypeTerms::Element_symbol(s) => {
                Value::CharString(s.clone())
            }
        }
    }
}

impl Display for AtomTypeTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

impl Display for AtomTypeLoopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = self.to_loop_column();
        write!(f, "{}", column.tag())?;
        for value in column.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}
