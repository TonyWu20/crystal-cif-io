//! The typed block view of one data block (roadmap D12).
//!
//! `CoreCif` is the typed document: the single-value category groups, the
//! two loop-row groups, and the `extra` residue. No typed category consumed
//! the residue (foreign tags, save frames); it keeps their order and bytes.
//! The guarantee is IR round-trip, not byte round-trip.

use std::fmt;

use crystal_cif_fmt::grammar::DataBlock;
use crystal_cif_fmt::{DataBlockHeading, DataBlockMember};

use crate::data_dict::core_cif::atom_site::AtomSite;
use crate::data_dict::core_cif::cell::Cell;
use crate::data_dict::core_cif::chemical_formula::ChemicalFormula;
use crate::data_dict::core_cif::space_group::SpaceGroup;
use crate::data_dict::core_cif::space_group_symop::Symop;
use crate::data_dict::core_cif::CifCategory;

/// A read-path conformance error of the typed layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CifError {
    /// A typed item carries a value class the dictionary denies.
    ValueClass {
        tag: String,
        expected: &'static str,
        found: &'static str,
    },
    /// A matrix item carries the wrong shape.
    Shape {
        tag: String,
        expected: (usize, usize),
        found: (usize, usize),
    },
    /// A required item is absent (from [`CoreCif::validate`]).
    Missing {
        category: &'static str,
        item: &'static str,
    },
}

impl fmt::Display for CifError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValueClass {
                tag,
                expected,
                found,
            } => write!(f, "{tag}: expected the {expected} value class, found {found}"),
            Self::Shape {
                tag,
                expected,
                found,
            } => write!(
                f,
                "{tag}: expected a {} x {} shape, found {} x {}",
                expected.0, expected.1, found.0, found.1
            ),
            Self::Missing { category, item } => {
                write!(f, "{category}: the required item {item} is missing")
            }
        }
    }
}

impl std::error::Error for CifError {}

/// The typed view of one data block (roadmap D12g).
#[derive(Debug, Clone, Default)]
pub struct CoreCif {
    pub cell: Cell,
    pub space_group: SpaceGroup,
    pub space_group_symop: Vec<Symop>,
    pub atom_site: Vec<AtomSite>,
    pub chemical_formula: ChemicalFormula,
    /// Members no typed category consumed, plus every save frame.
    pub extra: Vec<DataBlockMember>,
}

impl CoreCif {
    /// Read one data block into the typed view.
    ///
    /// Each typed category consumes its own tags in turn. Save-frame
    /// members and unknown tags land in `extra`, order kept. Every
    /// value-class and shape mismatch is collected; the call fails with
    /// the full list, never the first.
    pub fn from_block(block: &DataBlock) -> Result<CoreCif, Vec<CifError>> {
        let mut doc = CoreCif::default();
        let members: Vec<DataBlockMember> = block.members().to_vec();
        let mut errors: Vec<CifError> = Vec::new();
        // Each category consumes the residue of the last. The residue is
        // threaded, so the order of the `extra` members is kept.
        let (residue, cell_errors) = doc.cell.from_members(&members);
        errors.extend(cell_errors);
        let (residue, space_group_errors) = doc.space_group.from_members(&residue);
        errors.extend(space_group_errors);
        let (residue, symop_errors) = doc.space_group_symop.from_members(&residue);
        errors.extend(symop_errors);
        let (residue, atom_site_errors) = doc.atom_site.from_members(&residue);
        errors.extend(atom_site_errors);
        let (residue, formula_errors) = doc.chemical_formula.from_members(&residue);
        errors.extend(formula_errors);
        doc.extra = residue;
        if errors.is_empty() {
            Ok(doc)
        } else {
            Err(errors)
        }
    }

    /// Write the view back: the typed categories in a fixed order, then
    /// the `extra` residue.
    pub fn to_block(&self, heading: &str) -> DataBlock {
        let mut members = Vec::new();
        members.extend(self.cell.to_members());
        members.extend(self.space_group.to_members());
        members.extend(self.space_group_symop.to_members());
        members.extend(self.atom_site.to_members());
        members.extend(self.chemical_formula.to_members());
        members.extend(self.extra.iter().cloned());
        DataBlock::from_heading_members((
            DataBlockHeading::new(heading.to_string()),
            members,
        ))
    }

    /// Check the wave-1 required sets. Reports every missing required
    /// item, not just the first.
    pub fn validate(&self) -> Result<(), Vec<CifError>> {
        let mut errors: Vec<CifError> = Vec::new();
        // `cell`: the direct six.
        for (item, present) in [
            ("length_a", self.cell.length_a.is_some()),
            ("length_b", self.cell.length_b.is_some()),
            ("length_c", self.cell.length_c.is_some()),
            ("angle_alpha", self.cell.angle_alpha.is_some()),
            ("angle_beta", self.cell.angle_beta.is_some()),
            ("angle_gamma", self.cell.angle_gamma.is_some()),
        ] {
            if !present {
                errors.push(CifError::Missing {
                    category: "cell",
                    item,
                });
            }
        }
        // `space_group`: one identifier (any of the three names).
        let identified = self.space_group.it_number.is_some()
            || self.space_group.name_hall.is_some()
            || self.space_group.symmetry_name_h_m.is_some();
        if !identified {
            errors.push(CifError::Missing {
                category: "space_group",
                item: "it_number, name_hall, or symmetry_name_h_m",
            });
        }
        // `chemical_formula`: the required spelling.
        if self.chemical_formula.structural.is_none() {
            errors.push(CifError::Missing {
                category: "chemical_formula",
                item: "structural",
            });
        }
        // `space_group_symop`: the required column per row.
        for row in &self.space_group_symop {
            if row.operation_xyz.is_none() {
                errors.push(CifError::Missing {
                    category: "space_group_symop",
                    item: "operation_xyz",
                });
            }
        }
        // `atom_site`: the required columns per row.
        for row in &self.atom_site {
            for (item, present) in [
                ("label", row.label.is_some()),
                ("type_symbol", row.type_symbol.is_some()),
                ("adp_type", row.adp_type.is_some()),
            ] {
                if !present {
                    errors.push(CifError::Missing {
                        category: "atom_site",
                        item,
                    });
                }
            }
            // Exactly one coordinate mode per row.
            let fract = row.fract_x.is_some()
                && row.fract_y.is_some()
                && row.fract_z.is_some();
            let cartn = row.cartn_x.is_some()
                && row.cartn_y.is_some()
                && row.cartn_z.is_some();
            if !fract && !cartn {
                errors.push(CifError::Missing {
                    category: "atom_site",
                    item: "fract_x/y/z or cartn_x/y/z",
                });
            }
            // The aniso columns, when the displacement type is aniso.
            let need_u = matches!(
                row.adp_type,
                Some(crate::data_dict::core_cif::atom_site::AdpType::Uani)
                    | Some(crate::data_dict::core_cif::atom_site::AdpType::Uovl)
            );
            let need_b = matches!(
                row.adp_type,
                Some(crate::data_dict::core_cif::atom_site::AdpType::Bani)
                    | Some(crate::data_dict::core_cif::atom_site::AdpType::Bovl)
            );
            if need_u
                && !(row.aniso_u_11.is_some()
                    && row.aniso_u_12.is_some()
                    && row.aniso_u_13.is_some()
                    && row.aniso_u_22.is_some()
                    && row.aniso_u_23.is_some()
                    && row.aniso_u_33.is_some())
            {
                errors.push(CifError::Missing {
                    category: "atom_site",
                    item: "aniso_U_11..33",
                });
            }
            if need_b
                && !(row.aniso_b_11.is_some()
                    && row.aniso_b_12.is_some()
                    && row.aniso_b_13.is_some()
                    && row.aniso_b_22.is_some()
                    && row.aniso_b_23.is_some()
                    && row.aniso_b_33.is_some())
            {
                errors.push(CifError::Missing {
                    category: "atom_site",
                    item: "aniso_B_11..33",
                });
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

