//! The `atom_site` row builder.
//!
//! The builder materializes one row of the `atom_site` loop through
//! the `AtomSiteLoopItem` loop terms. The coordinate mode is
//! exclusive: a row carries fractional or Cartesian coordinates,
//! not both. The aniso or iso displacement choice is a needed
//! enum field. A row without a displacement type rejects. The
//! supplied values cross-check against the row's own items. A
//! mismatch rejects with a `Result`, never a panic.
//!
//! The displacement type cross-checks against the closed
//! `adp_type` set: `Uiso` and `Biso` pair with the iso arms, the
//! aniso `U` arms with `Uani`, `Uovl`, and `Umpe`, the aniso `B`
//! arms with `Bani` and `Bovl`.

use std::str::FromStr;

use bon::bon;
use elements_rs::Element;

use crate::data_dict::core_cif::atom_site::{
    AdpRefineFlag, AdpType, AtomSite, CalcFlag, OccupancyRefineFlag, PosnRefineFlag, TypeSymbol,
    TypeSymbolCode,
};
use crystal_cif_fmt::grammar::{CharString, Float, Numeric};

/// The exclusive coordinate mode of one `atom_site` row. A row
/// carries fractional or Cartesian coordinates, not both.
#[derive(Debug, Clone, PartialEq)]
pub enum Coordinates {
    /// The `fract_x`, `fract_y`, `fract_z` items.
    Fract { x: Numeric, y: Numeric, z: Numeric },
    /// The `Cartn_x`, `Cartn_y`, `Cartn_z` items.
    Cartn { x: Numeric, y: Numeric, z: Numeric },
}

/// The displacement type of one `atom_site` row. The aniso arms
/// carry the six displacement values; the iso arms carry the one
/// value.
#[derive(Debug, Clone, PartialEq)]
pub enum Displacement {
    /// The six `aniso_U_11` to `aniso_U_33` items.
    Uani {
        u_11: Numeric,
        u_12: Numeric,
        u_13: Numeric,
        u_22: Numeric,
        u_23: Numeric,
        u_33: Numeric,
    },
    /// The six `aniso_B_11` to `aniso_B_33` items.
    Bani {
        b_11: Numeric,
        b_12: Numeric,
        b_13: Numeric,
        b_22: Numeric,
        b_23: Numeric,
        b_33: Numeric,
    },
    /// The `U_iso_or_equiv` item.
    Uiso { u_iso: Float },
    /// The `B_iso_or_equiv` item.
    Biso { b_iso: Numeric },
}

/// A build failure of the `atom_site` row builder.
#[derive(Debug, Clone, thiserror::Error)]
pub enum RowError {
    /// The row carries no `label` and no `label_component` items.
    #[error("the row carries no label and no label components")]
    NoLabel,
    /// The displacement type does not pair with the `adp_type`.
    #[error("the displacement type {displacement} does not pair with the adp_type {adp_type}")]
    DisplacementMismatch {
        adp_type: AdpType,
        displacement: &'static str,
    },
    /// The `occupancy` value misses the `0:1` range.
    #[error("the occupancy {0} misses the 0 to 1 range")]
    OccupancyOutOfRange(Float),
    /// A displacement value misses the `0:` range.
    #[error("the {item} value {value} misses the 0: range")]
    DisplacementOutOfRange { item: &'static str, value: f64 },
    /// The `type_symbol` text parses to no element code.
    #[error("the type symbol {0:?} parses to no element code")]
    UnparsableTypeSymbol(String),
}

fn displacement_name(displacement: &Displacement) -> &'static str {
    match displacement {
        Displacement::Uani { .. } => "Uani",
        Displacement::Bani { .. } => "Bani",
        Displacement::Uiso { .. } => "Uiso",
        Displacement::Biso { .. } => "Biso",
    }
}

fn displacement_pairs(adp_type: AdpType, displacement: &Displacement) -> bool {
    match (adp_type, displacement) {
        (AdpType::Uiso, Displacement::Uiso { .. }) => true,
        (AdpType::Biso, Displacement::Biso { .. }) => true,
        (AdpType::Uani | AdpType::Uovl | AdpType::Umpe, Displacement::Uani { .. }) => true,
        (AdpType::Bani | AdpType::Bovl, Displacement::Bani { .. }) => true,
        _ => false,
    }
}

pub fn type_symbol_from_str(text: &str) -> Result<TypeSymbol, RowError> {
    let charge: Vec<char> = text
        .chars()
        .filter(|c| c.is_ascii_digit() || matches!(c, '+' | '-'))
        .collect();
    let code: String = text.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if code.is_empty() {
        return Err(RowError::UnparsableTypeSymbol(text.to_string()));
    }
    let element =
        Element::from_str(&code).map_err(|_| RowError::UnparsableTypeSymbol(text.to_string()))?;
    let digits: String = charge.iter().filter(|c| c.is_ascii_digit()).collect();
    let oxidation_state = if digits.is_empty() {
        None
    } else {
        let sign = if charge.contains(&'-') { -1 } else { 1 };
        Some(sign * digits.parse::<i32>().unwrap_or(0))
    };
    Ok(TypeSymbol::new(
        TypeSymbolCode::Element(element),
        oxidation_state,
    ))
}

fn non_negative(name: &'static str, value: &Numeric) -> Result<(), RowError> {
    let f = crate::data_dict::core_cif::range_check::number_as_f64(&value.number());
    if f < 0.0 {
        return Err(RowError::DisplacementOutOfRange {
            item: name,
            value: f,
        });
    }
    Ok(())
}

#[bon]
impl AtomSite {
    /// The `type_symbol`, the coordinate mode, the `adp_type`, and
    /// the displacement are needed at the `build()` call. The
    /// coordinate mode is exclusive by type. The displacement type
    /// cross-checks against the `adp_type`. The `occupancy`
    /// records `1.0` when absent.
    #[builder(start_fn = builder, builder_type = AtomSiteBuilder, finish_fn = build)]
    pub fn build_row(
        type_symbol: TypeSymbol,
        coordinates: Coordinates,
        adp_type: AdpType,
        displacement: Displacement,
        label: Option<CharString>,
        label_component_0: Option<CharString>,
        label_component_1: Option<CharString>,
        label_component_2: Option<CharString>,
        label_component_3: Option<CharString>,
        label_component_4: Option<CharString>,
        label_component_5: Option<CharString>,
        label_component_6: Option<CharString>,
        occupancy: Option<Float>,
        aniso_label: Option<CharString>,
        aniso_ratio: Option<Numeric>,
        aniso_type_symbol: Option<TypeSymbol>,
        calc_flag: Option<CalcFlag>,
        refinement_flags_posn: Option<PosnRefineFlag>,
        refinement_flags_adp: Option<AdpRefineFlag>,
        refinement_flags_occupancy: Option<OccupancyRefineFlag>,
        description: Option<CharString>,
        constraints: Option<CharString>,
        restraints: Option<CharString>,
        disorder_group: Option<CharString>,
        disorder_assembly: Option<CharString>,
        attached_hydrogens: Option<Numeric>,
        wyckoff_symbol: Option<CharString>,
    ) -> Result<AtomSite, RowError> {
        let no_label = label.is_none()
            && [
                &label_component_0,
                &label_component_1,
                &label_component_2,
                &label_component_3,
                &label_component_4,
                &label_component_5,
                &label_component_6,
            ]
            .iter()
            .all(|c| c.is_none());
        if no_label {
            return Err(RowError::NoLabel);
        }
        if !displacement_pairs(adp_type, &displacement) {
            return Err(RowError::DisplacementMismatch {
                adp_type,
                displacement: displacement_name(&displacement),
            });
        }
        let occupancy = occupancy.unwrap_or(Float(1.0));
        if occupancy.0 < 0.0 || occupancy.0 > 1.0 {
            return Err(RowError::OccupancyOutOfRange(occupancy));
        }
        match &displacement {
            Displacement::Uani {
                u_11,
                u_12,
                u_13,
                u_22,
                u_23,
                u_33,
            } => {
                non_negative("aniso_u_11", u_11)?;
                non_negative("aniso_u_12", u_12)?;
                non_negative("aniso_u_13", u_13)?;
                non_negative("aniso_u_22", u_22)?;
                non_negative("aniso_u_23", u_23)?;
                non_negative("aniso_u_33", u_33)?;
            }
            Displacement::Bani {
                b_11,
                b_12,
                b_13,
                b_22,
                b_23,
                b_33,
            } => {
                non_negative("aniso_b_11", b_11)?;
                non_negative("aniso_b_12", b_12)?;
                non_negative("aniso_b_13", b_13)?;
                non_negative("aniso_b_22", b_22)?;
                non_negative("aniso_b_23", b_23)?;
                non_negative("aniso_b_33", b_33)?;
            }
            Displacement::Uiso { u_iso } => {
                if u_iso.0 < 0.0 {
                    return Err(RowError::DisplacementOutOfRange {
                        item: "u_iso_or_equiv",
                        value: u_iso.0,
                    });
                }
            }
            Displacement::Biso { b_iso } => {
                non_negative("b_iso_or_equiv", b_iso)?;
            }
        }

        let (fract_x, fract_y, fract_z, cartn_x, cartn_y, cartn_z) = match &coordinates {
            Coordinates::Fract { x, y, z } => (Some(*x), Some(*y), Some(*z), None, None, None),
            Coordinates::Cartn { x, y, z } => (None, None, None, Some(*x), Some(*y), Some(*z)),
        };
        let (
            aniso_u_11,
            aniso_u_12,
            aniso_u_13,
            aniso_u_22,
            aniso_u_23,
            aniso_u_33,
            aniso_b_11,
            aniso_b_12,
            aniso_b_13,
            aniso_b_22,
            aniso_b_23,
            aniso_b_33,
            u_iso_or_equiv,
            b_iso_or_equiv,
        ) = match &displacement {
            Displacement::Uani {
                u_11,
                u_12,
                u_13,
                u_22,
                u_23,
                u_33,
            } => (
                Some(*u_11),
                Some(*u_12),
                Some(*u_13),
                Some(*u_22),
                Some(*u_23),
                Some(*u_33),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Displacement::Bani {
                b_11,
                b_12,
                b_13,
                b_22,
                b_23,
                b_33,
            } => (
                None,
                None,
                None,
                None,
                None,
                None,
                Some(*b_11),
                Some(*b_12),
                Some(*b_13),
                Some(*b_22),
                Some(*b_23),
                Some(*b_33),
                None,
                None,
            ),
            Displacement::Uiso { u_iso } => (
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Numeric::from(u_iso.0)),
                None,
            ),
            Displacement::Biso { b_iso } => (
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(*b_iso),
            ),
        };

        Ok(AtomSite {
            type_symbol: Some(CharString::from(type_symbol)),
            fract_x,
            fract_y,
            fract_z,
            cartn_x,
            cartn_y,
            cartn_z,
            adp_type: Some(adp_type),
            aniso_u_11,
            aniso_u_12,
            aniso_u_13,
            aniso_u_22,
            aniso_u_23,
            aniso_u_33,
            aniso_b_11,
            aniso_b_12,
            aniso_b_13,
            aniso_b_22,
            aniso_b_23,
            aniso_b_33,
            u_iso_or_equiv,
            b_iso_or_equiv,
            label,
            label_component_0,
            label_component_1,
            label_component_2,
            label_component_3,
            label_component_4,
            label_component_5,
            label_component_6,
            occupancy: Some(Numeric::from(occupancy.0)),
            aniso_label,
            aniso_ratio,
            aniso_type_symbol: aniso_type_symbol.map(CharString::from),
            calc_flag,
            refinement_flags_posn,
            refinement_flags_adp,
            refinement_flags_occupancy,
            description,
            constraints,
            restraints,
            disorder_group,
            disorder_assembly,
            attached_hydrogens,
            wyckoff_symbol,
            ..Default::default()
        })
    }
}

#[cfg(test)]
fn unquoted(text: &str) -> CharString {
    CharString::Unquoted(crystal_cif_fmt::grammar::UnquotedString::new(
        text.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_dict::core_cif::range_check::number_as_f64;
    use crate::data_dict::core_cif::CifCategory;

    fn symbol(text: &str) -> TypeSymbol {
        type_symbol_from_str(text).expect("the symbol parses")
    }

    fn uiso_row() -> Result<AtomSite, RowError> {
        AtomSite::builder()
            .type_symbol(symbol("Ba"))
            .coordinates(Coordinates::Fract {
                x: Numeric::from(0.0),
                y: Numeric::from(0.0),
                z: Numeric::from(0.0),
            })
            .adp_type(AdpType::Uiso)
            .displacement(Displacement::Uiso { u_iso: Float(0.0) })
            .label(unquoted("Ba1"))
            .build()
    }

    #[test]
    fn the_uiso_row_carries_the_document_fields() {
        let row = uiso_row().expect("the row builds");
        assert!(row.label.is_some(), "the label is set");
        assert!(row.fract_x.is_some(), "the fractional x is set");
        assert!(row.adp_type.is_some(), "the adp_type is set");
        assert!(row.u_iso_or_equiv.is_some(), "the iso displacement is set");
        assert!(row.occupancy.is_some(), "the occupancy is set");
        assert_eq!(
            number_as_f64(&row.occupancy.as_ref().expect("occupancy").number()),
            1.0,
            "the default occupancy is 1.0"
        );
        let members = vec![row].to_members();
        assert_eq!(members.len(), 1, "one row is one loop member");
    }

    #[test]
    fn a_missing_label_rejects() {
        let err = AtomSite::builder()
            .type_symbol(symbol("Ba"))
            .coordinates(Coordinates::Fract {
                x: Numeric::from(0.0),
                y: Numeric::from(0.0),
                z: Numeric::from(0.0),
            })
            .adp_type(AdpType::Uiso)
            .displacement(Displacement::Uiso { u_iso: Float(0.0) })
            .build()
            .unwrap_err();
        assert!(matches!(err, RowError::NoLabel));
    }

    #[test]
    fn a_mismatching_displacement_type_rejects() {
        let err = AtomSite::builder()
            .type_symbol(symbol("Ba"))
            .coordinates(Coordinates::Fract {
                x: Numeric::from(0.0),
                y: Numeric::from(0.0),
                z: Numeric::from(0.0),
            })
            .adp_type(AdpType::Uiso)
            .displacement(Displacement::Uani {
                u_11: Numeric::from(0.0),
                u_12: Numeric::from(0.0),
                u_13: Numeric::from(0.0),
                u_22: Numeric::from(0.0),
                u_23: Numeric::from(0.0),
                u_33: Numeric::from(0.0),
            })
            .label(unquoted("Ba1"))
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            RowError::DisplacementMismatch {
                adp_type: AdpType::Uiso,
                displacement: "Uani",
            }
        ));
    }

    #[test]
    fn an_out_of_range_occupancy_rejects() {
        let err = AtomSite::builder()
            .type_symbol(symbol("Ba"))
            .coordinates(Coordinates::Fract {
                x: Numeric::from(0.0),
                y: Numeric::from(0.0),
                z: Numeric::from(0.0),
            })
            .adp_type(AdpType::Uiso)
            .displacement(Displacement::Uiso { u_iso: Float(0.0) })
            .label(unquoted("Ba1"))
            .occupancy(Float(2.0))
            .build()
            .unwrap_err();
        assert!(matches!(err, RowError::OccupancyOutOfRange(_)));
    }

    #[test]
    fn a_negative_displacement_value_rejects() {
        let err = AtomSite::builder()
            .type_symbol(symbol("Ba"))
            .coordinates(Coordinates::Fract {
                x: Numeric::from(0.0),
                y: Numeric::from(0.0),
                z: Numeric::from(0.0),
            })
            .adp_type(AdpType::Biso)
            .displacement(Displacement::Biso {
                b_iso: Numeric::from(-1.0),
            })
            .label(unquoted("Ba1"))
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            RowError::DisplacementOutOfRange {
                item: "b_iso_or_equiv",
                ..
            }
        ));
    }

    #[test]
    fn the_type_symbol_parser_carries_the_charge() {
        let ba = symbol("Ba");
        assert_eq!(ba.to_string(), "Ba");
        let fe = type_symbol_from_str("Fe2+").expect("the charged symbol parses");
        assert_eq!(fe.to_string(), "Fe2+");
        let cl = type_symbol_from_str("Cl-").expect("the bare sign parses");
        assert_eq!(cl.to_string(), "Cl");
        let fe3 = type_symbol_from_str("Fe3+").expect("the charged symbol parses");
        assert_eq!(fe3.to_string(), "Fe3+");
        assert!(type_symbol_from_str("Xy9+").is_err());
    }

    #[test]
    fn the_cartn_mode_carries_the_cartn_fields() {
        let row = AtomSite::builder()
            .type_symbol(symbol("O"))
            .coordinates(Coordinates::Cartn {
                x: Numeric::from(1.0),
                y: Numeric::from(2.0),
                z: Numeric::from(3.0),
            })
            .adp_type(AdpType::Biso)
            .displacement(Displacement::Biso {
                b_iso: Numeric::from(0.5),
            })
            .label(unquoted("O1"))
            .build()
            .expect("the row builds");
        assert!(row.cartn_x.is_some(), "the Cartesian x is set");
        assert!(row.fract_x.is_none(), "the fractional x stays absent");
        assert!(row.b_iso_or_equiv.is_some(), "the iso B is set");
    }
}
