//! The `space_group` category builder.
//!
//! One builder (`SpaceGroupBuilder`) over one unique identifier:
//! `IT_number`, `name_Hall`, or `name_H-M`. The
//! `crystallographic-group` crate (0.4) resolves the rest. The
//! computed items are builder outputs. A supplied value
//! cross-checks. A mismatch rejects with a `Result`, never a panic.
//!
//! `bon` gives one start function per builder type. The three entry
//! points share the type, so they are the variants of the needed
//! `identifier` field. The one-step entry points are `by_it_number`,
//! `by_name_hall`, and `by_name_h_m` over the same builder.
//!
//! The `name_H-M` entry point resolves against the table spelling:
//! the trimmed Hermann-Mauguin symbol of an all-settings row. A
//! spelling outside the table does not resolve.

use bon::bon;
use crystallographic_group::database::{
    PER_NUMBER_SPACE_GROUP_SYMBOLS, CrystalSystem, SpaceGroupTable,
};
use crystallographic_group::SpaceGroup as Resolved;
use crystallographic_group::SpaceGroupError as ResolveError;

use crate::DataItems;
use crate::data_dict::core_cif::space_group::{
    CrystalSystemCif, ITNumber, SpaceGroupItem,
};
use crate::data_dict::SingleValueTerm;
use crate::data_dict::core_cif::range_check::number_as_f64;
use crate::grammar::{CharString, Matrix, Number, Numeric};

use super::cell::DEFAULT_TOLERANCE;

/// The `SpaceGroupBuilder` input. Exactly one of the three carries
/// the group. The `bon` type state keeps the field needed at the
/// `build()` call.
#[derive(Debug, Clone, PartialEq)]
pub enum SpaceGroupIdentifier {
    /// Entry by International Tables number, 1 to 230.
    ItNumber(ITNumber),
    /// Entry by Hall symbol.
    NameHall(String),
    /// Entry by the full Hermann-Mauguin symbol of an all-settings
    /// table row.
    NameHm(String),
}

/// A build failure of the `space_group` builder.
#[derive(Debug, Clone, PartialEq)]
pub enum SpaceGroupError {
    /// The `IT_number` value is outside 1 to 230.
    InvalidItNumber { number: u64 },
    /// The Hall symbol resolves to no group.
    UnresolvedHall(String),
    /// The Hermann-Mauguin symbol matches no table row.
    UnresolvedHm(String),
    /// A supplied value misses the computed value.
    Mismatch {
        item: &'static str,
        supplied: String,
        computed: String,
    },
    /// A supplied transform matrix carries the wrong shape.
    BadShape {
        item: &'static str,
        actual: (usize, usize),
    },
    /// The resolved group carries no crystal system. The resolution
    /// paths guarantee a table row, so this does not occur.
    NoCrystalSystem(String),
}

impl std::fmt::Display for SpaceGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidItNumber { number } => {
                write!(f, "the IT number {number} is outside 1 to 230")
            }
            Self::UnresolvedHall(hall) => write!(f, "the Hall symbol {hall:?} resolves to no group"),
            Self::UnresolvedHm(hm) => write!(f, "the Hermann-Mauguin symbol {hm:?} matches no table row"),
            Self::Mismatch {
                item,
                supplied,
                computed,
            } => write!(f, "the supplied {item} {supplied:?} misses the computed {computed:?}"),
            Self::BadShape { item, actual } => {
                write!(f, "the supplied {item} has the shape {actual:?}, not (3, 3)")
            }
            Self::NoCrystalSystem(detail) => {
                write!(f, "the resolved group carries no crystal system: {detail}")
            }
        }
    }
}

impl std::error::Error for SpaceGroupError {}

/// The resolved `space_group` category. The computed items are
/// outputs. The optional supplied items emit as given.
#[derive(Debug, Clone)]
pub struct SpaceGroup {
    inner: Resolved,
    it_number: ITNumber,
    crystal_system: CrystalSystemCif,
    bravais_type: String,
    centring_type: String,
    multiplicity: Numeric,
    name_hall: String,
    name_h_m: String,
    terms: Vec<SpaceGroupItem>,
}

impl SpaceGroup {
    /// The resolved group of the `crystallographic-group` crate.
    pub fn inner(&self) -> &Resolved {
        &self.inner
    }

    pub fn it_number(&self) -> ITNumber {
        self.it_number
    }

    pub fn crystal_system(&self) -> CrystalSystemCif {
        self.crystal_system
    }

    pub fn bravais_type(&self) -> &str {
        &self.bravais_type
    }

    pub fn centring_type(&self) -> &str {
        &self.centring_type
    }

    pub fn multiplicity(&self) -> Numeric {
        self.multiplicity
    }

    pub fn name_hall(&self) -> &str {
        &self.name_hall
    }

    pub fn name_h_m(&self) -> &str {
        &self.name_h_m
    }

    /// The materialized single-value items, one `DataItems` each.
    pub fn to_data_items(&self) -> Vec<DataItems> {
        self.terms
            .iter()
            .map(|term| term.to_single_value_data())
            .collect()
    }

    /// The materialized single-value terms.
    pub fn terms(&self) -> &[SpaceGroupItem] {
        &self.terms
    }
}

impl SpaceGroup {
    /// One-step entry by International Tables number.
    pub fn by_it_number(number: ITNumber) -> Result<Self, SpaceGroupError> {
        Self::builder()
            .identifier(SpaceGroupIdentifier::ItNumber(number))
            .build()
    }

    /// One-step entry by Hall symbol.
    pub fn by_name_hall(hall: &str) -> Result<Self, SpaceGroupError> {
        Self::builder()
            .identifier(SpaceGroupIdentifier::NameHall(hall.to_string()))
            .build()
    }

    /// One-step entry by the full Hermann-Mauguin symbol of an
    /// all-settings table row.
    pub fn by_name_h_m(hm: &str) -> Result<Self, SpaceGroupError> {
        Self::builder()
            .identifier(SpaceGroupIdentifier::NameHm(hm.to_string()))
            .build()
    }
}

struct ResolvedParts {
    group: Resolved,
    it_number: ITNumber,
    hall: String,
    hm: String,
}

fn collapse_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn number_of_code(code: &str) -> Option<u64> {
    let digits: String = code.chars().take_while(char::is_ascii_digit).collect();
    let value = digits.parse::<u64>().ok()?;
    (1..=230).contains(&value).then_some(value)
}

fn resolve(identifier: &SpaceGroupIdentifier) -> Result<ResolvedParts, SpaceGroupError> {
    let table = SpaceGroupTable::all();
    match identifier {
        SpaceGroupIdentifier::ItNumber(it) => {
            let number = u64::from(*it);
            if !(1..=230).contains(&number) {
                return Err(SpaceGroupError::InvalidItNumber { number });
            }
            let row = (number - 1) as usize;
            let hall = PER_NUMBER_SPACE_GROUP_SYMBOLS[2][row].to_string();
            let group = Resolved::try_from_str(&hall)
                .map_err(|_| SpaceGroupError::UnresolvedHall(hall.clone()))?;
            Ok(ResolvedParts {
                group,
                it_number: ITNumber::from(number as i32),
                hall,
                hm: collapse_ws(&PER_NUMBER_SPACE_GROUP_SYMBOLS[1][row]),
            })
        }
        SpaceGroupIdentifier::NameHall(hall) => {
            let group = Resolved::try_from_str(hall).map_err(|error| {
                SpaceGroupError::UnresolvedHall(error_display(&error, hall.clone()))
            })?;
            let row = group
                .number()
                .ok_or_else(|| SpaceGroupError::UnresolvedHall(hall.clone()))?;
            let hm = collapse_ws(&table.hm_full_notation(row.row()).unwrap_or(""));
            Ok(ResolvedParts {
                group,
                it_number: number_of_code(table.number(row.row()).unwrap_or("0"))
                    .map(|n| ITNumber::from(n as i32))
                    .ok_or_else(|| SpaceGroupError::UnresolvedHall(hall.clone()))?,
                hall: hall.clone(),
                hm,
            })
        }
        SpaceGroupIdentifier::NameHm(hm) => {
            let wanted = collapse_ws(hm);
            let row = (0..table.len() as u16)
                .find(|&i| collapse_ws(table.hm_full_notation(i).unwrap_or("")) == wanted)
                .ok_or_else(|| SpaceGroupError::UnresolvedHm(hm.clone()))?;
            let hall = table.hall_symbol(row).unwrap_or("").to_string();
            let group = Resolved::try_from_str(&hall)
                .map_err(|_| SpaceGroupError::UnresolvedHm(hm.clone()))?;
            Ok(ResolvedParts {
                group,
                it_number: number_of_code(table.number(row).unwrap_or("0"))
                    .map(|n| ITNumber::from(n as i32))
                    .ok_or_else(|| SpaceGroupError::UnresolvedHm(hm.clone()))?,
                hall,
                hm: wanted,
            })
        }
    }
}

fn error_display(error: &ResolveError, hall: String) -> String {
    if matches!(error, ResolveError::NotInTable(_)) {
        format!("{hall:?} is not an all-settings table row")
    } else {
        format!("{hall:?} does not parse: {error}")
    }
}

fn system_cif(system: CrystalSystem) -> CrystalSystemCif {
    match system {
        CrystalSystem::Triclinic => CrystalSystemCif::Triclinic,
        CrystalSystem::Monoclinic => CrystalSystemCif::Monoclinic,
        CrystalSystem::Orthorhombic => CrystalSystemCif::Orthorhombic,
        CrystalSystem::Tetragonal => CrystalSystemCif::Tetragonal,
        CrystalSystem::Trigonal => CrystalSystemCif::Trigonal,
        CrystalSystem::Hexagonal => CrystalSystemCif::Hexagonal,
        CrystalSystem::Cubic => CrystalSystemCif::Cubic,
    }
}

fn lattice_letter(hm: &str) -> Option<char> {
    hm.chars().find(|c| c.is_ascii_alphabetic())
}

fn bravais_code(system: CrystalSystem, letter: char) -> Option<&'static str> {
    Some(match (system, letter) {
        (CrystalSystem::Triclinic, 'P') => "aP",
        (CrystalSystem::Monoclinic, 'P') => "mP",
        (CrystalSystem::Monoclinic, 'A' | 'B' | 'C') => "mS",
        (CrystalSystem::Orthorhombic, 'P') => "oP",
        (CrystalSystem::Orthorhombic, 'A' | 'B' | 'C') => "oS",
        (CrystalSystem::Orthorhombic, 'I') => "oI",
        (CrystalSystem::Orthorhombic, 'F') => "oF",
        (CrystalSystem::Tetragonal, 'P') => "tP",
        (CrystalSystem::Tetragonal, 'I') => "tI",
        (CrystalSystem::Trigonal, 'P') => "hP",
        (CrystalSystem::Trigonal, 'R') => "hR",
        (CrystalSystem::Hexagonal, 'P') => "hP",
        (CrystalSystem::Hexagonal, 'R') => "hR",
        (CrystalSystem::Cubic, 'P') => "cP",
        (CrystalSystem::Cubic, 'I') => "cI",
        (CrystalSystem::Cubic, 'F') => "cF",
        _ => return None,
    })
}

fn unquoted(text: &str) -> CharString {
    CharString::Unquoted(
        crate::grammar::UnquotedString::new(text.to_string()),
    )
}

fn within_tolerance(supplied: f64, computed: f64, tolerance: f64) -> bool {
    (supplied - computed).abs() <= tolerance * supplied.abs().max(computed.abs())
}

#[bon]
impl SpaceGroup {
    /// The unique identifier is needed at the `build()` call. The
    /// computed items resolve through the `crystallographic-group`
    /// crate. A supplied `crystal_system`, `multiplicity`, or
    /// `name_h_m_ref` cross-checks against the resolved value.
    /// `tolerance` relaxes the numeric cross-checks (default
    /// [`DEFAULT_TOLERANCE`]).
    #[builder(start_fn = builder, builder_type = SpaceGroupBuilder, finish_fn = build)]
    pub fn build_group(
        identifier: SpaceGroupIdentifier,
        crystal_system: Option<CrystalSystemCif>,
        multiplicity: Option<Numeric>,
        name_h_m_ref: Option<CharString>,
        name_h_m_alt: Option<CharString>,
        name_schoenflies: Option<CharString>,
        patterson_name_h_m: Option<CharString>,
        point_group_h_m: Option<CharString>,
        laue_class: Option<CharString>,
        reference_setting: Option<CharString>,
        it_coordinate_system_code: Option<CharString>,
        transform_pp_abc: Option<Matrix>,
        transform_qq_xyz: Option<Matrix>,
        tolerance: Option<f64>,
    ) -> Result<SpaceGroup, SpaceGroupError> {
        let parts = resolve(&identifier)?;
        let tolerance = tolerance.unwrap_or(DEFAULT_TOLERANCE);
        let system = parts
            .group
            .crystal_system()
            .ok_or_else(|| SpaceGroupError::NoCrystalSystem(parts.hall.clone()))?;
        let multiplicity_count = parts.group.general_positions().len();
        let multiplicity_value = Numeric::new(Number::from(multiplicity_count as i32), None);

        if let Some(supplied) = crystal_system {
            if supplied != system_cif(system) {
                return Err(SpaceGroupError::Mismatch {
                    item: "crystal_system",
                    supplied: supplied.to_string(),
                    computed: system_cif(system).to_string(),
                });
            }
        }
        if let Some(supplied) = &multiplicity {
            if !within_tolerance(
                number_as_f64(&supplied.number()),
                multiplicity_count as f64,
                tolerance,
            ) {
                return Err(SpaceGroupError::Mismatch {
                    item: "multiplicity",
                    supplied: supplied.to_string(),
                    computed: multiplicity_value.to_string(),
                });
            }
        }
        if let Some(supplied) = &name_h_m_ref {
            if supplied.content() != parts.hall {
                return Err(SpaceGroupError::Mismatch {
                    item: "name_h_m_ref",
                    supplied: supplied.content().to_string(),
                    computed: parts.hall.clone(),
                });
            }
        }
        if let Some(m) = &transform_pp_abc {
            if m.shape() != (3, 3) {
                return Err(SpaceGroupError::BadShape {
                    item: "transform_pp_abc",
                    actual: m.shape(),
                });
            }
        }
        if let Some(m) = &transform_qq_xyz {
            if m.shape() != (3, 3) {
                return Err(SpaceGroupError::BadShape {
                    item: "transform_qq_xyz",
                    actual: m.shape(),
                });
            }
        }

        let letter = lattice_letter(&parts.hm).ok_or_else(|| {
            SpaceGroupError::UnresolvedHm(parts.hm.clone())
        })?;
        let bravais = bravais_code(system, letter)
            .ok_or_else(|| SpaceGroupError::UnresolvedHm(parts.hm.clone()))?;

        let mut terms = vec![
            SpaceGroupItem::Crystal_system(system_cif(system)),
            SpaceGroupItem::IT_number(parts.it_number),
            SpaceGroupItem::Name_Hall(parts.hall.clone()),
            SpaceGroupItem::Bravais_type(unquoted(bravais)),
            SpaceGroupItem::Centring_type(unquoted(&letter.to_string())),
            SpaceGroupItem::Multiplicity(multiplicity_value),
            SpaceGroupItem::Name_h_m_ref(unquoted(&parts.hall)),
        ];
        if let Some(v) = name_h_m_alt {
            terms.push(SpaceGroupItem::Name_H_M_alt(v.content().to_string()));
        }
        if let Some(v) = name_schoenflies {
            terms.push(SpaceGroupItem::Name_schoenflies(v.clone()));
        }
        if let Some(v) = patterson_name_h_m {
            terms.push(SpaceGroupItem::Patterson_name_h_m(v.clone()));
        }
        if let Some(v) = point_group_h_m {
            terms.push(SpaceGroupItem::Point_group_h_m(v.clone()));
        }
        if let Some(v) = laue_class {
            terms.push(SpaceGroupItem::Laue_class(v.clone()));
        }
        if let Some(v) = reference_setting {
            terms.push(SpaceGroupItem::Reference_setting(v.clone()));
        }
        if let Some(v) = it_coordinate_system_code {
            terms.push(SpaceGroupItem::It_coordinate_system_code(v.clone()));
        }
        if let Some(v) = &transform_pp_abc {
            terms.push(SpaceGroupItem::Transform_pp_abc(unquoted(&v.to_string())));
        }
        if let Some(v) = &transform_qq_xyz {
            terms.push(SpaceGroupItem::Transform_qq_xyz(unquoted(&v.to_string())));
        }

        Ok(SpaceGroup {
            inner: parts.group,
            it_number: parts.it_number,
            crystal_system: system_cif(system),
            bravais_type: bravais.to_string(),
            centring_type: letter.to_string(),
            multiplicity: multiplicity_value,
            name_hall: parts.hall,
            name_h_m: parts.hm,
            terms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_dict::CifTerm;
    use crate::grammar::{Float, UnquotedString};

    fn cs(text: &str) -> CharString {
        CharString::Unquoted(UnquotedString::new(text.to_string()))
    }

    #[test]
    fn the_it_number_entry_resolves_the_category() {
        let group = SpaceGroup::by_it_number(ITNumber::from(14i32)).expect("P21/c resolves");
        assert_eq!(group.it_number(), ITNumber::from(14i32));
        assert_eq!(group.name_hall(), "-P 2ybc");
        assert_eq!(group.name_h_m(), "P 1 21");
        assert_eq!(group.crystal_system(), CrystalSystemCif::Monoclinic);
        assert_eq!(group.bravais_type(), "mP");
        assert_eq!(group.centring_type(), "P");
        assert_eq!(number_as_f64(&group.multiplicity().number()), 4.0);
        assert_eq!(group.to_data_items().len(), 7);
    }

    #[test]
    fn the_hall_entry_resolves_the_category() {
        let group = SpaceGroup::by_name_hall("P 2 2").expect("P222 resolves");
        assert_eq!(group.it_number(), ITNumber::from(16i32));
        assert_eq!(group.name_h_m(), "P 2 2 2");
        assert_eq!(group.crystal_system(), CrystalSystemCif::Orthorhombic);
        assert_eq!(group.bravais_type(), "oP");
        assert_eq!(number_as_f64(&group.multiplicity().number()), 4.0);
    }

    #[test]
    fn the_h_m_entry_resolves_the_category() {
        let group = SpaceGroup::by_name_h_m("P 4 2 2").expect("P422 resolves");
        assert_eq!(group.it_number(), ITNumber::from(89i32));
        assert_eq!(group.name_hall(), "P 4 2");
        assert_eq!(group.crystal_system(), CrystalSystemCif::Tetragonal);
        assert_eq!(group.bravais_type(), "tP");
    }

    #[test]
    fn a_supplied_crystal_system_cross_checks() {
        let ok = SpaceGroup::by_it_number(ITNumber::from(14i32))
            .and_then(|_| {
                SpaceGroup::builder()
                    .identifier(SpaceGroupIdentifier::ItNumber(ITNumber::from(14i32)))
                    .crystal_system(CrystalSystemCif::Monoclinic)
                    .build()
            })
            .expect("the supplied system matches");
        assert_eq!(ok.crystal_system(), CrystalSystemCif::Monoclinic);
    }

    #[test]
    fn a_mismatching_crystal_system_rejects() {
        let err = SpaceGroup::builder()
            .identifier(SpaceGroupIdentifier::ItNumber(ITNumber::from(14i32)))
            .crystal_system(CrystalSystemCif::Cubic)
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            SpaceGroupError::Mismatch {
                item: "crystal_system",
                ..
            }
        ));
    }

    #[test]
    fn a_mismatching_multiplicity_rejects() {
        let err = SpaceGroup::builder()
            .identifier(SpaceGroupIdentifier::ItNumber(ITNumber::from(14i32)))
            .multiplicity(Numeric::from(8.0))
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            SpaceGroupError::Mismatch {
                item: "multiplicity",
                ..
            }
        ));
    }

    #[test]
    fn a_mismatching_name_h_m_ref_rejects() {
        let err = SpaceGroup::builder()
            .identifier(SpaceGroupIdentifier::ItNumber(ITNumber::from(14i32)))
            .name_h_m_ref(cs("P 2y"))
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            SpaceGroupError::Mismatch {
                item: "name_h_m_ref",
                ..
            }
        ));
        let ok = SpaceGroup::builder()
            .identifier(SpaceGroupIdentifier::ItNumber(ITNumber::from(14i32)))
            .name_h_m_ref(cs("-P 2ybc"))
            .build()
            .expect("the hall symbol is the reference");
        assert!(ok.terms().iter().any(|t| t.tag().as_ref() == "space_group_name_h_m_ref"));
    }


    #[test]
    fn an_it_number_outside_the_range_rejects() {
        let err = SpaceGroup::by_it_number(ITNumber::from(0i32)).unwrap_err();
        assert!(matches!(
            err,
            SpaceGroupError::InvalidItNumber { number: 0 }
        ));
        let err = SpaceGroup::by_it_number(ITNumber::from(231i32)).unwrap_err();
        assert!(matches!(
            err,
            SpaceGroupError::InvalidItNumber { number: 231 }
        ));
    }

    #[test]
    fn a_missing_hall_symbol_rejects() {
        let err = SpaceGroup::by_name_hall("P 41").unwrap_err();
        assert!(matches!(err, SpaceGroupError::UnresolvedHall(_)));
        let err = SpaceGroup::by_name_h_m("P 21/c").unwrap_err();
        assert!(matches!(err, SpaceGroupError::UnresolvedHm(_)));
    }

    #[test]
    fn a_bad_transform_shape_rejects() {
        let square = Matrix::try_new(vec![
            vec![
                Number::Float(Float(1.0)),
                Number::Float(Float(1.0)),
            ],
            vec![
                Number::Float(Float(1.0)),
                Number::Float(Float(1.0)),
            ],
        ])
        .expect("a 2 x 2 matrix");
        let err = SpaceGroup::builder()
            .identifier(SpaceGroupIdentifier::ItNumber(ITNumber::from(14i32)))
            .transform_pp_abc(square)
            .build()
            .unwrap_err();
        assert!(matches!(err, SpaceGroupError::BadShape { .. }));
    }

    #[test]
    fn the_optional_pass_through_items_emit() {
        let group = SpaceGroup::builder()
            .identifier(SpaceGroupIdentifier::ItNumber(ITNumber::from(14i32)))
            .name_h_m_alt(cs("P 21/c"))
            .name_schoenflies(cs("C2h"))
            .build()
            .expect("builds");
        let tags: Vec<String> =
            group.terms().iter().map(|t| t.tag().as_ref().to_string()).collect();
        assert!(tags.iter().any(|t| t == "space_group_name_H-M_alt"));
        assert!(tags.iter().any(|t| t == "space_group_name_schoenflies"));
        assert_eq!(group.to_data_items().len(), 9);
    }
}
