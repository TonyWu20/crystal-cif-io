//! The `atom_type` category (22 items, all loop columns).
//!
//! One loop lists the atom types of the structure. Each column is a
//! property of a type: the symbol, the oxidation number, the count,
//! the radii, and the scattering-factor parameters (Cromer-Mann
//! coefficients, dispersion terms, neutron length, source tags).

mod atom_type;
mod ranges;

pub use atom_type::{AtomType, AtomTypeRow};
pub use ranges::{AnalyticalMass, NumberInCell, OxidationNumber, Radius};

use std::fmt::Display;

use crate::data_dict::{CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Numeric, Tag, Value};

/// The loop-value terms of the `atom_type` category: one variant
/// per registry loop-value item, in item order. The category holds
/// 22 loop columns.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum AtomTypeLoopItem {
    /// The `_atom_type_analytical_mass_percent` item. The variant holds
    /// the column values, one per loop row.
    Analytical_mass_percent(Vec<AnalyticalMass>),
    /// The `_atom_type_description` item. The variant holds the column values, one per loop row.
    ///
    /// A description of the atom(s) designated by this atom type. In most
    /// cases, this will be the element name and oxidation state of a single
    /// atom species. For disordered or nonstoichiometric structures it will
    /// describe a combination of atom species.
    Description(Vec<CharString>),
    /// The `_atom_type_number_in_cell` item. The variant holds the column values, one per loop row.
    ///
    /// Total number of atoms of this atom type in the unit cell.
    Number_in_cell(Vec<NumberInCell>),
    /// The `_atom_type_oxidation_number` item. The variant holds the column values, one per loop row.
    ///
    /// Formal oxidation state of this atom type in the structure.
    Oxidation_number(Vec<OxidationNumber>),
    /// The `_atom_type_radius_bond` item. The variant holds the column values, one per loop row.
    ///
    /// The effective intra- and intermolecular bonding radii in angstroms
    /// of this atom type.
    /// Units: A (angstrom).
    Radius_bond(Vec<Radius>),
    /// The `_atom_type_radius_contact` item. The variant holds the column values, one per loop row.
    ///
    /// The effective intra- and intermolecular bonding radii in angstroms
    /// of this atom type.
    /// Units: A (angstrom).
    Radius_contact(Vec<Radius>),
    /// The `_atom_type_scat_Cromer_Mann_a1` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_a1(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_a2` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_a2(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_a3` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_a3(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_a4` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_a4(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_b1` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_b1(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_b2` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_b2(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_b3` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_b3(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_b4` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_b4(Vec<Numeric>),
    /// The `_atom_type_scat_Cromer_Mann_c` item. The variant holds the column values, one per loop row.
    ///
    /// The Cromer-Mann scattering-factor coefficients used to calculate the
    /// scattering factors for this atom type. Ref: International Tables for
    /// X-ray Crystallography (1974). Vol. IV, Table 2.2B or International
    /// Tables for Crystallography (2004). Vol. C, Tables 6.1.1.4 and
    /// 6.1.1.5
    Scat_Cromer_Mann_c(Vec<Numeric>),
    /// The `_atom_type_scat_dispersion_imag` item. The variant holds the column values, one per loop row.
    ///
    /// The imaginary and real components of the anomalous-dispersion
    /// scattering factor, f'' and f', in electrons for this atom type and
    /// the radiation given in _diffrn_radiation_wavelength.
    Scat_dispersion_imag(Vec<Numeric>),
    /// The `_atom_type_scat_dispersion_real` item. The variant holds the column values, one per loop row.
    ///
    /// The imaginary and real components of the anomalous-dispersion
    /// scattering factor, f'' and f', in electrons for this atom type and
    /// the radiation given in _diffrn_radiation_wavelength.
    Scat_dispersion_real(Vec<Numeric>),
    /// The `_atom_type_scat_dispersion_source` item. The variant holds the column values, one per loop row.
    ///
    /// Reference to source of real and imaginary dispersion corrections for
    /// scattering factors used for this atom type.
    Scat_dispersion_source(Vec<CharString>),
    /// The `_atom_type_scat_length_neutron` item. The variant holds the column values, one per loop row.
    ///
    /// The bound coherent scattering length in femtometres for the atom
    /// type at the isotopic composition used for the diffraction
    /// experiment.
    /// Units: fm (femtometre).
    Scat_length_neutron(Vec<Numeric>),
    /// The `_atom_type_scat_source` item. The variant holds the column values, one per loop row.
    ///
    /// Reference to source of scattering factors or scattering lengths used
    /// for this atom type.
    Scat_source(Vec<CharString>),
    /// The `_atom_type_scat_versus_stol_list` item. The variant holds the column values, one per loop row.
    ///
    /// A table of scattering factors as a function of sin theta over
    /// lambda. This table should be well commented to indicate the items
    /// present. Regularly formatted lists are strongly recommended.
    Scat_versus_stol_list(Vec<CharString>),
    /// The `_atom_type_symbol` item. The variant holds the column values, one per loop row.
    ///
    /// The code used to identify the atom species (singular or plural)
    /// representing this atom type. Normally this code is the element
    /// symbol. The code may be composed of any character except an
    /// underscore with the additional proviso that digits designate an
    /// oxidation state and must be followed by a + or - character.
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
/// The single-value terms of the `atom_type` category: one variant per

/// registry single-value item, in item order.
pub enum AtomTypeTerms {
    /// The `_atom_type_atomic_mass` item. The variant holds the column values, one per loop row.
    Atomic_mass(Numeric),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `1:`); the plain numeric form is kept.
    /// The `_atom_type_atomic_number` item. The variant holds the column values, one per loop row.
    Atomic_number(Numeric),
    /// The `_atom_type_display_colour` item. The variant holds the column values, one per loop row.
    Display_colour(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `1:`); the plain numeric form is kept.
    /// The `_atom_type_electron_count` item. The variant holds the column values, one per loop row.
    Electron_count(Numeric),
    /// The `_atom_type_element_symbol` item. The variant holds the column values, one per loop row.
    Element_symbol(CharString),
    /// The range constraint is recorded in the shell dictionary
    /// (`_enumeration` `1:`); the plain numeric form is kept.
    /// The `_atom_type_mass_number` item. The variant holds the column values, one per loop row.
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
