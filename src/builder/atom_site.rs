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

use crate::DataItems;
use crate::data_dict::core_cif::atom_site::{
    AdpType, AtomSiteLoopItem, CalcFlag, OccupancyRefineFlag, PosnRefineFlag, AdpRefineFlag,
    TypeSymbol, TypeSymbolCode,
};
use crate::data_dict::LoopValueTerm;
use crate::grammar::{CharString, Float, LoopColumns, Numeric};

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
#[derive(Debug, Clone)]
pub enum RowError {
    /// The row carries no `label` and no `label_component` items.
    NoLabel,
    /// The displacement type does not pair with the `adp_type`.
    DisplacementMismatch {
        adp_type: AdpType,
        displacement: &'static str,
    },
    /// The `occupancy` value misses the `0:1` range.
    OccupancyOutOfRange(Float),
    /// A displacement value misses the `0:` range.
    DisplacementOutOfRange { item: &'static str, value: f64 },
    /// The `type_symbol` text parses to no element code.
    UnparsableTypeSymbol(String),
}

impl std::fmt::Display for RowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoLabel => {
                write!(f, "the row carries no label and no label components")
            }
            Self::DisplacementMismatch {
                adp_type,
                displacement,
            } => write!(
                f,
                "the displacement type {displacement} does not pair with the adp_type {adp_type}"
            ),
            Self::OccupancyOutOfRange(value) => {
                write!(f, "the occupancy {value} misses the 0 to 1 range")
            }
            Self::DisplacementOutOfRange { item, value } => {
                write!(f, "the {item} value {value} misses the 0: range")
            }
            Self::UnparsableTypeSymbol(text) => {
                write!(f, "the type symbol {text:?} parses to no element code")
            }
        }
    }
}

impl std::error::Error for RowError {}

/// One materialized `atom_site` loop row.
#[derive(Debug, Clone)]
pub struct AtomSiteRow {
    type_symbol: TypeSymbol,
    coordinates: Coordinates,
    adp_type: AdpType,
    displacement: Displacement,
    label: Option<CharString>,
    label_components: Vec<Option<CharString>>,
    occupancy: Float,
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
}

impl AtomSiteRow {
    pub fn type_symbol(&self) -> &TypeSymbol {
        &self.type_symbol
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    pub fn adp_type(&self) -> AdpType {
        self.adp_type
    }

    pub fn displacement(&self) -> &Displacement {
        &self.displacement
    }

    /// The `occupancy` value. The row records `1.0` when the
    /// author supplied none.
    pub fn occupancy(&self) -> Float {
        self.occupancy
    }

    /// The row materialized as one `MultiValues` item per loop
    /// item the row carries, one column value each.
    pub fn to_data_items(&self) -> Vec<DataItems> {
        let mut columns: Vec<AtomSiteLoopItem> = Vec::new();
        if let Some(label) = &self.label {
            columns.push(AtomSiteLoopItem::Label(vec![label.clone()]));
        }
        for (index, component) in self.label_components.iter().enumerate() {
            if let Some(component) = component {
                let item = match index {
                    0 => AtomSiteLoopItem::Label_component_0(vec![component.clone()]),
                    1 => AtomSiteLoopItem::Label_component_1(vec![component.clone()]),
                    2 => AtomSiteLoopItem::Label_component_2(vec![component.clone()]),
                    3 => AtomSiteLoopItem::Label_component_3(vec![component.clone()]),
                    4 => AtomSiteLoopItem::Label_component_4(vec![component.clone()]),
                    5 => AtomSiteLoopItem::Label_component_5(vec![component.clone()]),
                    6 => AtomSiteLoopItem::Label_component_6(vec![component.clone()]),
                    _ => unreachable!("the label components stop at 6"),
                };
                columns.push(item);
            }
        }
        columns.push(AtomSiteLoopItem::Type_symbol(vec![
            self.type_symbol.clone(),
        ]));
        match &self.coordinates {
            Coordinates::Fract { x, y, z } => {
                columns.push(AtomSiteLoopItem::Fract_x(vec![*x]));
                columns.push(AtomSiteLoopItem::Fract_y(vec![*y]));
                columns.push(AtomSiteLoopItem::Fract_z(vec![*z]));
            }
            Coordinates::Cartn { x, y, z } => {
                columns.push(AtomSiteLoopItem::Cartn_x(vec![*x]));
                columns.push(AtomSiteLoopItem::Cartn_y(vec![*y]));
                columns.push(AtomSiteLoopItem::Cartn_z(vec![*z]));
            }
        }
        columns.push(AtomSiteLoopItem::Adp_type(vec![self.adp_type]));
        match &self.displacement {
            Displacement::Uani {
                u_11,
                u_12,
                u_13,
                u_22,
                u_23,
                u_33,
            } => {
                columns.push(AtomSiteLoopItem::Aniso_U_11(vec![*u_11]));
                columns.push(AtomSiteLoopItem::Aniso_U_12(vec![*u_12]));
                columns.push(AtomSiteLoopItem::Aniso_U_13(vec![*u_13]));
                columns.push(AtomSiteLoopItem::Aniso_U_22(vec![*u_22]));
                columns.push(AtomSiteLoopItem::Aniso_U_23(vec![*u_23]));
                columns.push(AtomSiteLoopItem::Aniso_U_33(vec![*u_33]));
            }
            Displacement::Bani {
                b_11,
                b_12,
                b_13,
                b_22,
                b_23,
                b_33,
            } => {
                columns.push(AtomSiteLoopItem::Aniso_B_11(vec![*b_11]));
                columns.push(AtomSiteLoopItem::Aniso_B_12(vec![*b_12]));
                columns.push(AtomSiteLoopItem::Aniso_B_13(vec![*b_13]));
                columns.push(AtomSiteLoopItem::Aniso_B_22(vec![*b_22]));
                columns.push(AtomSiteLoopItem::Aniso_B_23(vec![*b_23]));
                columns.push(AtomSiteLoopItem::Aniso_B_33(vec![*b_33]));
            }
            Displacement::Uiso { u_iso } => {
                columns.push(AtomSiteLoopItem::U_iso_or_equiv(vec![*u_iso]));
            }
            Displacement::Biso { b_iso } => {
                columns.push(AtomSiteLoopItem::B_iso_or_equiv(vec![*b_iso]));
            }
        }
        columns.push(AtomSiteLoopItem::Occupancy(vec![self.occupancy]));
        if let Some(v) = &self.aniso_label {
            columns.push(AtomSiteLoopItem::Aniso_label(vec![v.clone()]));
        }
        if let Some(v) = &self.aniso_ratio {
            columns.push(AtomSiteLoopItem::Aniso_ratio(vec![*v]));
        }
        if let Some(v) = &self.aniso_type_symbol {
            columns.push(AtomSiteLoopItem::Aniso_type_symbol(vec![v.clone()]));
        }
        if let Some(v) = self.calc_flag {
            columns.push(AtomSiteLoopItem::Calc_flag(vec![v]));
        }
        if let Some(v) = self.refinement_flags_posn {
            columns.push(AtomSiteLoopItem::Refinement_flags_posn(vec![v]));
        }
        if let Some(v) = self.refinement_flags_adp {
            columns.push(AtomSiteLoopItem::Refinement_flags_adp(vec![v]));
        }
        if let Some(v) = self.refinement_flags_occupancy {
            columns.push(AtomSiteLoopItem::Refinement_flags_occupancy(vec![v]));
        }
        if let Some(v) = &self.description {
            columns.push(AtomSiteLoopItem::Description(vec![v.clone()]));
        }
        if let Some(v) = &self.constraints {
            columns.push(AtomSiteLoopItem::Constraints(vec![v.clone()]));
        }
        if let Some(v) = &self.restraints {
            columns.push(AtomSiteLoopItem::Restraints(vec![v.clone()]));
        }
        if let Some(v) = &self.disorder_group {
            columns.push(AtomSiteLoopItem::Disorder_group(vec![v.clone()]));
        }
        if let Some(v) = &self.disorder_assembly {
            columns.push(AtomSiteLoopItem::Disorder_assembly(vec![v.clone()]));
        }
        if let Some(v) = &self.attached_hydrogens {
            columns.push(AtomSiteLoopItem::Attached_hydrogens(vec![*v]));
        }
        if let Some(v) = &self.wyckoff_symbol {
            columns.push(AtomSiteLoopItem::Wyckoff_symbol(vec![v.clone()]));
        }
        columns
            .into_iter()
            .map(|item| DataItems::MultiValues(LoopColumns::new(vec![item.to_loop_column()])))
            .collect()
    }
}

#[cfg(test)]
fn unquoted(text: &str) -> CharString {
    CharString::Unquoted(crate::grammar::UnquotedString::new(text.to_string()))
}

/// Parse a `type_symbol` text into the `TypeSymbol` value. The
/// element code is the leading symbol; the optional charge is the
/// trailing sign and digit, e.g. `Fe2+` and `Cl-`.
pub fn type_symbol_from_str(text: &str) -> Result<TypeSymbol, RowError> {
    let charge: Vec<char> = text.chars().filter(|c| c.is_ascii_digit() || matches!(c, '+' | '-')).collect();
    let code: String = text.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if code.is_empty() {
        return Err(RowError::UnparsableTypeSymbol(text.to_string()));
    }
    let element = Element::from_str(&code)
        .map_err(|_| RowError::UnparsableTypeSymbol(text.to_string()))?;
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

fn non_negative(name: &'static str, value: &Numeric) -> Result<(), RowError> {
    let f = crate::data_dict::core_cif::range_check::number_as_f64(&value.number());
    if f < 0.0 {
        return Err(RowError::DisplacementOutOfRange { item: name, value: f });
    }
    Ok(())
}

#[bon]
impl AtomSiteRow {
    /// The `type_symbol`, the coordinate mode, the `adp_type`, and
    /// the displacement are needed at the `build()` call. The
    /// coordinate mode is exclusive by type. The displacement type
    /// cross-checks against the `adp_type`. The `occupancy`
    /// records `1.0` when absent.
    #[builder(start_fn = builder, builder_type = AtomSiteRowBuilder, finish_fn = build)]
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
    ) -> Result<AtomSiteRow, RowError> {
        let label_components = vec![
            label_component_0,
            label_component_1,
            label_component_2,
            label_component_3,
            label_component_4,
            label_component_5,
            label_component_6,
        ];
        if label.is_none() && label_components.iter().all(Option::is_none) {
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

        Ok(AtomSiteRow {
            type_symbol,
            coordinates,
            adp_type,
            displacement,
            label,
            label_components,
            occupancy,
            aniso_label,
            aniso_ratio,
            aniso_type_symbol,
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol(text: &str) -> TypeSymbol {
        type_symbol_from_str(text).expect("the symbol parses")
    }

    fn uiso_row() -> Result<AtomSiteRow, RowError> {
        AtomSiteRow::builder()
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
    fn the_uiso_row_materializes_the_columns() {
        let row = uiso_row().expect("the row builds");
        let items = row.to_data_items();
        let tags: Vec<String> = items
            .iter()
            .map(|item| {
                item.as_multi_values()
                    .expect("a loop item")
                    .columns()[0]
                    .tag()
                    .as_ref()
                    .to_string()
            })
            .collect();
        assert!(tags.iter().any(|t| t == "atom_site_label"));
        assert!(tags.iter().any(|t| t == "atom_site_fract_x"));
        assert!(tags.iter().any(|t| t == "atom_site_adp_type"));
        assert!(tags.iter().any(|t| t == "atom_site_u_iso_or_equiv"));
        assert!(tags.iter().any(|t| t == "atom_site_occupancy"));
        assert_eq!(row.occupancy().0, 1.0);
    }

    #[test]
    fn a_missing_label_rejects() {
        let err = AtomSiteRow::builder()
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
        let err = AtomSiteRow::builder()
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
        let err = AtomSiteRow::builder()
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
        let err = AtomSiteRow::builder()
            .type_symbol(symbol("Ba"))
            .coordinates(Coordinates::Fract {
                x: Numeric::from(0.0),
                y: Numeric::from(0.0),
                z: Numeric::from(0.0),
            })
            .adp_type(AdpType::Biso)
            .displacement(Displacement::Biso { b_iso: Numeric::from(-1.0) })
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
    fn the_cartn_mode_materializes_the_cartn_columns() {
        let row = AtomSiteRow::builder()
            .type_symbol(symbol("O"))
            .coordinates(Coordinates::Cartn {
                x: Numeric::from(1.0),
                y: Numeric::from(2.0),
                z: Numeric::from(3.0),
            })
            .adp_type(AdpType::Biso)
            .displacement(Displacement::Biso { b_iso: Numeric::from(0.5) })
            .label(unquoted("O1"))
            .build()
            .expect("the row builds");
        let items = row.to_data_items();
        let tags: Vec<String> = items
            .iter()
            .map(|item| {
                item.as_multi_values()
                    .expect("a loop item")
                    .columns()[0]
                    .tag()
                    .as_ref()
                    .to_string()
            })
            .collect();
        assert!(tags.iter().any(|t| t == "atom_site_cartn_x"));
        assert!(tags.iter().any(|t| t == "atom_site_b_iso_or_equiv"));
    }
}
