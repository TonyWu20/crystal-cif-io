//! Closed-set value gate (roadmap D12).
//!
//! Every typed closed-set enum must agree with the recorded
//! dictionary closed values (`tests/dict_inventory.json`) plus the
//! marked real-data extensions
//! (`codegen/closed_value_extensions.json`). The `.` code is the
//! dictionary absent code. An enum that models it as a variant
//! keeps it (the row `absent` flag). A divergence on either side
//! fails the gate.

use std::collections::BTreeSet;
use std::path::PathBuf;

use crystal_cif_io::data_dict::core_cif::{
    atom_site::{
        AdpRefineFlag, AdpType, CalcFlag, OccupancyRefineFlag, PosnRefineFlag, RefinementFlag,
    },
    atom_sites::SolutionMethod,
    chemical::{AbsoluteConfiguration, EnantioexcessBulkTechnique, EnantioexcessCrystalTechnique},
    chemical_conn_bond::BondType,
    citation::CoordinateLinkage,
    diffrn_radiation::{Probe, XraySymbol},
    diffrn_radiation_wavelength::Determination,
    diffrn_refln::{ScanMode, ScanModeBackgd},
    diffrn_source::Target,
    exptl::AbsorptCorrectionType,
    exptl_crystal::{ColourModifier, ColourPrimary, Lustre},
    geom::PublFlag,
    publ::RequestedCategory,
    publ_body::{Element, Format},
    publ_manuscript_incl::ExtraDefn,
    refine::{HydrogenTreatment, MatrixType, StructureFactorCoef, WeightingScheme},
    refln::{IncludeStatus, ObservedStatus, RefinementStatus},
    space_group::CrystalSystemCif,
    symmetry::CellSetting,
};

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn pin_values(item: &str) -> BTreeSet<String> {
    let raw = std::fs::read_to_string(manifest().join("tests/dict_inventory.json"))
        .unwrap_or_else(|e| panic!("read the inventory pin: {e}"));
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    v["items"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|it| it["name"].as_str() == Some(item))
        .find_map(|it| it.get("enumeration")?.get("values")?.as_array())
        .map(|vals| {
            vals.iter()
                .map(|c| c.as_str().unwrap().to_string())
                .collect()
        })
        .unwrap_or_default()
}

fn extension_codes(item: &str) -> BTreeSet<String> {
    let raw = std::fs::read_to_string(
        manifest()
            .parent()
            .unwrap()
            .join("codegen/closed_value_extensions.json"),
    )
    .unwrap_or_else(|e| panic!("read the closed-value extensions: {e}"));
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    v.get(item)
        .and_then(|e| e.get("codes"))
        .map(|codes| {
            codes
                .as_array()
                .unwrap()
                .iter()
                .map(|c| c.as_str().unwrap().to_string())
                .collect()
        })
        .unwrap_or_default()
}

struct Row {
    /// The item of the recorded dictionary.
    item: &'static str,
    /// The variant codes of the enum. A `(variant, code)` pair.
    variants: &'static [(&'static str, &'static str)],
    /// The absent `.` code is modeled as a variant.
    absent: bool,
    /// Accepts a recorded code and returns its dictionary spelling.
    accept: fn(&str) -> Option<String>,
}

fn rows() -> Vec<Row> {
    let r = |item, variants, absent, accept| Row {
        item,
        variants,
        absent,
        accept,
    };
    vec![
        r(
            "exptl_absorpt_correction_type",
            &[
                ("analytical", "analytical"),
                ("cylinder", "cylinder"),
                ("empirical", "empirical"),
                ("gaussian", "gaussian"),
                ("integration", "integration"),
                ("multi-scan", "multi-scan"),
                ("none", "none"),
                ("numerical", "numerical"),
                ("psi-scan", "psi-scan"),
                ("refdelf", "refdelf"),
                ("semi-empirical", "semi-empirical"),
                ("sphere", "sphere"),
            ],
            false,
            |c| {
                AbsorptCorrectionType::try_from(c)
                    .ok()
                    .map(|v| v.to_string())
            },
        ),
        r(
            "refine_ls_hydrogen_treatment",
            &[
                ("refall", "refall"),
                ("refxyz", "refxyz"),
                ("refU", "refU"),
                ("noref", "noref"),
                ("constr", "constr"),
                ("hetero", "hetero"),
                ("heteroxyz", "heteroxyz"),
                ("heteroU", "heteroU"),
                ("heteronoref", "heteronoref"),
                ("hetero-mixed", "hetero-mixed"),
                ("heteroxyz-mixed", "heteroxyz-mixed"),
                ("heteroU-mixed", "heteroU-mixed"),
                ("heteronoref-mixed", "heteronoref-mixed"),
                ("mixed", "mixed"),
                ("undef", "undef"),
            ],
            false,
            |c| HydrogenTreatment::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "refine_ls_matrix_type",
            &[
                ("full", "full"),
                ("fullcycle", "fullcycle"),
                ("atomblock", "atomblock"),
                ("userblock", "userblock"),
                ("diagonal", "diagonal"),
                ("sparse", "sparse"),
            ],
            false,
            |c| MatrixType::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "refine_ls_structure_factor_coef",
            &[("F", "F"), ("Fsqd", "Fsqd"), ("Inet", "Inet")],
            false,
            |c| StructureFactorCoef::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "refine_ls_weighting_scheme",
            &[("sigma", "sigma"), ("unit", "unit"), ("calc", "calc")],
            false,
            |c| WeightingScheme::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "chemical_absolute_configuration",
            &[
                ("rm", "rm"),
                ("ad", "ad"),
                ("dyn", "dyn"),
                ("rmad", "rmad"),
                ("syn", "syn"),
                ("unk", "unk"),
            ],
            false,
            |c| AbsoluteConfiguration::parse(c).map(|v| v.to_string()),
        ),
        r(
            "chemical_enantioexcess_bulk_technique",
            &[("OA", "OA"), ("CD", "CD"), ("EC", "EC"), ("other", "other")],
            false,
            |c| EnantioexcessBulkTechnique::parse(c).map(|v| v.to_string()),
        ),
        r(
            "chemical_enantioexcess_crystal_technique",
            &[("CD", "CD"), ("EC", "EC"), ("other", "other")],
            false,
            |c| EnantioexcessCrystalTechnique::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_sites_solution_primary",
            &[
                ("difmap", "difmap"),
                ("vecmap", "vecmap"),
                ("heavy", "heavy"),
                ("direct", "direct"),
                ("geom", "geom"),
                ("disper", "disper"),
                ("isomor", "isomor"),
                ("mixed", "mixed"),
                ("notdet", "notdet"),
                ("dual", "dual"),
                ("iterative", "iterative"),
                ("other", "other"),
            ],
            false,
            |c| SolutionMethod::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_sites_solution_secondary",
            &[
                ("difmap", "difmap"),
                ("vecmap", "vecmap"),
                ("heavy", "heavy"),
                ("direct", "direct"),
                ("geom", "geom"),
                ("disper", "disper"),
                ("isomor", "isomor"),
                ("mixed", "mixed"),
                ("notdet", "notdet"),
                ("dual", "dual"),
                ("iterative", "iterative"),
                ("other", "other"),
            ],
            false,
            |c| SolutionMethod::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_sites_solution_hydrogens",
            &[
                ("difmap", "difmap"),
                ("vecmap", "vecmap"),
                ("heavy", "heavy"),
                ("direct", "direct"),
                ("geom", "geom"),
                ("disper", "disper"),
                ("isomor", "isomor"),
                ("mixed", "mixed"),
                ("notdet", "notdet"),
                ("dual", "dual"),
                ("iterative", "iterative"),
                ("other", "other"),
            ],
            false,
            |c| SolutionMethod::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_site_adp_type",
            &[
                ("Uani", "Uani"),
                ("Uiso", "Uiso"),
                ("Uovl", "Uovl"),
                ("Umpe", "Umpe"),
                ("Bani", "Bani"),
                ("Biso", "Biso"),
                ("Bovl", "Bovl"),
                ("betaani", "betaani"),
            ],
            false,
            |c| AdpType::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_site_thermal_displace_type",
            &[
                ("Uani", "Uani"),
                ("Uiso", "Uiso"),
                ("Uovl", "Uovl"),
                ("Umpe", "Umpe"),
                ("Bani", "Bani"),
                ("Biso", "Biso"),
                ("Bovl", "Bovl"),
                ("betaani", "betaani"),
            ],
            false,
            |c| AdpType::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_site_calc_flag",
            &[("d", "d"), ("calc", "calc"), ("c", "c"), ("dum", "dum")],
            false,
            |c| CalcFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_site_refinement_flags",
            &[
                (".", "."),
                ("S", "S"),
                ("G", "G"),
                ("R", "R"),
                ("D", "D"),
                ("T", "T"),
                ("U", "U"),
                ("P", "P"),
            ],
            true,
            |c| RefinementFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_site_refinement_flags_adp",
            &[(".", "."), ("T", "T"), ("U", "U"), ("TU", "TU")],
            true,
            |c| AdpRefineFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_site_refinement_flags_occupancy",
            &[(".", "."), ("P", "P")],
            true,
            |c| OccupancyRefineFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "atom_site_refinement_flags_posn",
            &[
                (".", "."),
                ("D", "D"),
                ("G", "G"),
                ("R", "R"),
                ("S", "S"),
                ("DG", "DG"),
                ("DR", "DR"),
                ("DS", "DS"),
                ("GR", "GR"),
                ("GS", "GS"),
                ("RS", "RS"),
                ("DGR", "DGR"),
                ("DGS", "DGS"),
                ("DRS", "DRS"),
                ("GRS", "GRS"),
                ("DGRS", "DGRS"),
            ],
            true,
            |c| PosnRefineFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "exptl_crystal_colour_lustre",
            &[
                ("metallic", "metallic"),
                ("lustrous", "lustrous"),
                ("transparent", "transparent"),
                ("translucent", "translucent"),
                ("opaque", "opaque"),
                ("dull", "dull"),
                ("clear", "clear"),
                (".", "."),
            ],
            true,
            |c| Lustre::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "exptl_crystal_colour_modifier",
            &[
                ("dark", "dark"),
                ("light", "light"),
                ("intense", "intense"),
                ("pale", "pale"),
                ("whitish", "whitish"),
                ("blackish", "blackish"),
                ("greyish", "greyish"),
                ("grayish", "grayish"),
                ("brownish", "brownish"),
                ("reddish", "reddish"),
                ("pinkish", "pinkish"),
                ("orangish", "orangish"),
                ("yellowish", "yellowish"),
                ("greenish", "greenish"),
                ("bluish", "bluish"),
                (".", "."),
            ],
            true,
            |c| ColourModifier::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "exptl_crystal_colour_primary",
            &[
                ("colourless", "colourless"),
                ("white", "white"),
                ("black", "black"),
                ("gray", "gray"),
                ("brown", "brown"),
                ("red", "red"),
                ("pink", "pink"),
                ("orange", "orange"),
                ("yellow", "yellow"),
                ("green", "green"),
                ("blue", "blue"),
                ("violet", "violet"),
            ],
            false,
            |c| ColourPrimary::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "refln_refinement_status",
            &[("incl", "incl"), ("excl", "excl"), ("extn", "extn")],
            false,
            |c| RefinementStatus::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "refln_include_status",
            &[
                ("o", "o"),
                ("<", "<"),
                ("-", "-"),
                ("x", "x"),
                ("h", "h"),
                ("l", "l"),
            ],
            false,
            |c| IncludeStatus::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "refln_observed_status",
            &[
                ("o", "o"),
                ("<", "<"),
                ("-", "-"),
                ("x", "x"),
                ("h", "h"),
                ("l", "l"),
            ],
            false,
            |c| ObservedStatus::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "diffrn_refln_scan_mode",
            &[("om", "om"), ("ot", "ot"), ("q", "q")],
            false,
            |c| ScanMode::parse(c).map(|v| v.to_string()),
        ),
        r(
            "diffrn_refln_scan_mode_backgd",
            &[("st", "st"), ("mo", "mo")],
            false,
            |c| ScanModeBackgd::parse(c).map(|v| v.to_string()),
        ),
        r(
            "diffrn_source_target",
            &[
                ("H", "H"),
                ("Ar", "Ar"),
                ("As", "As"),
                ("Cd", "Cd"),
                ("Eu", "Eu"),
                ("Pt", "Pt"),
                ("Np", "Np"),
                ("Hs", "Hs"),
            ],
            false,
            |c| Target::parse(c).map(|v| v.to_string()),
        ),
        r(
            "diffrn_radiation_probe",
            &[
                ("x-ray", "x-ray"),
                ("neutron", "neutron"),
                ("electron", "electron"),
                ("gamma", "gamma"),
            ],
            false,
            |c| Probe::parse(c).map(|v| v.to_string()),
        ),
        r(
            "diffrn_radiation_xray_symbol",
            &[
                ("K-L~3~", "K-L~3~"),
                ("K-L~2~", "K-L~2~"),
                ("K-M~3~", "K-M~3~"),
                ("K-L~2,3~", "K-L~2,3~"),
            ],
            false,
            |c| XraySymbol::parse(c).map(|v| v.to_string()),
        ),
        r(
            "diffrn_radiation_wavelength_determination",
            &[
                ("fundamental", "fundamental"),
                ("estimated", "estimated"),
                ("refined", "refined"),
            ],
            false,
            |c| Determination::parse(c).map(|v| v.to_string()),
        ),
        r(
            "publ_requested_category",
            &[
                ("AD", "AD"),
                ("CI", "CI"),
                ("CM", "CM"),
                ("CO", "CO"),
                ("EI", "EI"),
                ("EM", "EM"),
                ("EO", "EO"),
                ("FA", "FA"),
                ("FI", "FI"),
                ("FM", "FM"),
                ("FO", "FO"),
                ("GI", "GI"),
                ("GM", "GM"),
                ("GO", "GO"),
                ("HI", "HI"),
                ("HM", "HM"),
                ("HO", "HO"),
                ("QI", "QI"),
                ("QM", "QM"),
                ("QO", "QO"),
                ("SC", "SC"),
                ("TE", "TE"),
            ],
            false,
            |c| RequestedCategory::parse(c).map(|v| v.to_string()),
        ),
        r(
            "citation_coordinate_linkage",
            &[("no", "no"), ("n", "n"), ("yes", "yes"), ("y", "y")],
            false,
            |c| CoordinateLinkage::parse(c).map(|v| v.to_string()),
        ),
        r(
            "publ_body_element",
            &[
                ("section", "section"),
                ("subsection", "subsection"),
                ("subsubsection", "subsubsection"),
                ("appendix", "appendix"),
                ("footnote", "footnote"),
            ],
            false,
            |c| Element::parse(c).map(|v| v.to_string()),
        ),
        r(
            "publ_body_format",
            &[
                ("ascii", "ascii"),
                ("cif", "cif"),
                ("latex", "latex"),
                ("rtf", "rtf"),
                ("sgml", "sgml"),
                ("tex", "tex"),
                ("troff", "troff"),
            ],
            false,
            |c| Format::parse(c).map(|v| v.to_string()),
        ),
        r(
            "publ_manuscript_incl_extra_defn",
            &[("no", "no"), ("n", "n"), ("yes", "yes"), ("y", "y")],
            false,
            |c| ExtraDefn::parse(c).map(|v| v.to_string()),
        ),
        r(
            "chemical_conn_bond_type",
            &[
                ("sing", "sing"),
                ("doub", "doub"),
                ("trip", "trip"),
                ("quad", "quad"),
                ("arom", "arom"),
                ("poly", "poly"),
                ("delo", "delo"),
                ("pi", "pi"),
            ],
            false,
            |c| BondType::parse(c).map(|v| v.to_string()),
        ),
        r(
            "symmetry_cell_setting",
            &[
                ("triclinic", "triclinic"),
                ("monoclinic", "monoclinic"),
                ("orthorhombic", "orthorhombic"),
                ("tetragonal", "tetragonal"),
                ("rhombohedral", "rhombohedral"),
                ("trigonal", "trigonal"),
                ("hexagonal", "hexagonal"),
                ("cubic", "cubic"),
            ],
            false,
            |c| CellSetting::try_from(c).ok().map(|v| v.to_string()),
        ),
        r(
            "geom_angle_publ_flag",
            &[("no", "no"), ("n", "n"), ("yes", "yes"), ("y", "y")],
            false,
            |c| PublFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "geom_bond_publ_flag",
            &[("no", "no"), ("n", "n"), ("yes", "yes"), ("y", "y")],
            false,
            |c| PublFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "geom_contact_publ_flag",
            &[("no", "no"), ("n", "n"), ("yes", "yes"), ("y", "y")],
            false,
            |c| PublFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "geom_hbond_publ_flag",
            &[("no", "no"), ("n", "n"), ("yes", "yes"), ("y", "y")],
            false,
            |c| PublFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "geom_torsion_publ_flag",
            &[("no", "no"), ("n", "n"), ("yes", "yes"), ("y", "y")],
            false,
            |c| PublFlag::parse(c).map(|v| v.to_string()),
        ),
        r(
            "space_group_crystal_system",
            &[
                ("triclinic", "triclinic"),
                ("monoclinic", "monoclinic"),
                ("orthorhombic", "orthorhombic"),
                ("tetragonal", "tetragonal"),
                ("trigonal", "trigonal"),
                ("hexagonal", "hexagonal"),
                ("cubic", "cubic"),
            ],
            false,
            |c| CrystalSystemCif::try_from(c).ok().map(|v| v.to_string()),
        ),
    ]
}

#[test]
fn the_enum_variant_sets_match_the_recorded_dictionary() {
    for row in rows() {
        let mut expected = pin_values(row.item);
        expected.extend(extension_codes(row.item));
        if !row.absent {
            expected.remove(".");
        }
        let enum_codes: BTreeSet<String> =
            row.variants.iter().map(|(_, c)| c.to_string()).collect();
        assert_eq!(
            enum_codes, expected,
            "{}: the enum variant set matches the recorded codes",
            row.item
        );
        for code in &expected {
            let got = (row.accept)(code).unwrap_or_else(|| {
                panic!("{}: the enum parses the recorded code {code}", row.item)
            });
            assert_eq!(
                got, *code,
                "{}: the code round-trips through the enum",
                row.item
            );
        }
    }
}

#[test]
fn every_closed_item_of_the_pin_carries_a_row() {
    let raw = std::fs::read_to_string(manifest().join("tests/dict_inventory.json"))
        .unwrap_or_else(|e| panic!("read the inventory pin: {e}"));
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let pin_items: BTreeSet<String> = v["items"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|it| {
            it.get("enumeration")
                .and_then(|e| e.get("kind"))
                .map(|k| k == "closed")
                .unwrap_or(false)
        })
        .map(|it| it["name"].as_str().unwrap().to_string())
        .collect();
    let watched: BTreeSet<String> = rows().into_iter().map(|r| r.item.to_string()).collect();
    let missing: Vec<&String> = pin_items.difference(&watched).collect();
    assert!(
        missing.is_empty(),
        "closed pin items without a row: {missing:?}"
    );
}
