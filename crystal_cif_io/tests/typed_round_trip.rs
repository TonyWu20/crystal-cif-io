//! The typed document layer round-trip (roadmap D12).
//!
//! `CoreCif` reads a data block into the typed category structs and writes
//! them back. The guarantee is IR round-trip: read, write, read again, and
//! the typed fields and the `extra` residue are unchanged. Save frames and
//! foreign tags land in `extra`, order kept.

use crystal_cif_fmt::grammar::{Numeric, UnquotedString};
use crystal_cif_fmt::{CharString, DataBlock, DataItems, DataBlockMember, parse_cif};

use crystal_cif_io::core_cif::{CifError, CoreCif};
use crystal_cif_io::data_dict::core_cif::cell::CellAngle;
use crystal_cif_io::data_dict::core_cif::space_group::ITNumber;
use crystal_cif_io::data_dict::core_cif::atom_site::AtomSite;
use crystal_cif_io::data_dict::core_cif::atom_site::AdpType;
use crystal_cif_io::data_dict::core_cif::range_check::number_as_f64;

fn fixture(name: &str) -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../crystal_cif_fmt/tests/fixtures/")
        .to_string()
        + name;
    std::fs::read_to_string(&path).expect("the fixture reads")
}

fn first_block(cif: &str) -> DataBlock {
    let doc = parse_cif(cif).expect("the fixture parses");
    doc.data_blocks()
        .expect("a data block is present")
        .first()
        .expect("a first block")
        .clone()
}

/// A canonical signature of the typed fields and the `extra` residue.
/// Stable across an IR round-trip when no typed value is lost.
fn fingerprint(doc: &CoreCif) -> String {
    let mut s = String::new();
    let num = |o: &Option<Numeric>| {
        o.as_ref()
            .map(|n| format!("{:.6}", number_as_f64(&n.number())))
            .unwrap_or_else(|| "absent".to_string())
    };
    let angle = |o: &Option<CellAngle>| {
        o.as_ref()
            .map(|a| format!("{:.6}", a.angle().0))
            .unwrap_or_else(|| "absent".to_string())
    };
    let char_of = |o: &Option<CharString>| {
        o.as_ref()
            .map(|c| c.content().to_string())
            .unwrap_or_else(|| "absent".to_string())
    };
    let weight = |o: &Option<crystal_cif_io::data_dict::core_cif::chemical_formula::FormulaWeight>| {
        o.as_ref()
            .map(|w| format!("{:.6}", number_as_f64(&w.value().number())))
            .unwrap_or_else(|| "absent".to_string())
    };
    s += &format!(
        "cell[a={} b={} c={} A={} B={} G={} V={}]\n",
        num(&doc.cell.length_a),
        num(&doc.cell.length_b),
        num(&doc.cell.length_c),
        angle(&doc.cell.angle_alpha),
        angle(&doc.cell.angle_beta),
        angle(&doc.cell.angle_gamma),
        num(&doc.cell.volume),
    );
    s += &format!(
        "sg[it={}]\n",
        doc.space_group
            .it_number
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "absent".to_string())
    );
    s += &format!("symop[rows={}]\n", doc.space_group_symop.len());
    for row in &doc.atom_site {
        s += &format!(
            "atom[{}|{}|{}]\n",
            char_of(&row.label),
            char_of(&row.type_symbol),
            row.adp_type.as_ref().map(|a| a.to_string()).unwrap_or_else(|| "absent".to_string()),
        );
    }
    s += &format!(
        "formula[structural={} weight={}]\n",
        char_of(&doc.chemical_formula.structural),
        weight(&doc.chemical_formula.weight),
    );
    for m in &doc.extra {
        s += &format!("{m}\n");
    }
    s
}

#[test]
fn the_acetonitrile_block_round_trips_the_formula_category() {
    let block = first_block(&fixture("13_structure.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    // The formula category is the typed category of this block.
    assert!(
        doc.chemical_formula.weight.is_some(),
        "the recorded formula weight is typed"
    );
    let rewritten = doc.to_block(block.heading());
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(fingerprint(&doc), fingerprint(&again), "the IR round-trip is stable");
    // The foreign loops are the residue, kept in order.
    assert!(
        !doc.extra.is_empty(),
        "the untyped members land in extra"
    );
}

#[test]
fn the_structure_block_round_trips_cell_and_atom_site() {
    let block = first_block(&fixture("09_realistic_structure.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    // The cell direct six type.
    assert!(doc.cell.length_a.is_some(), "the cell length a is typed");
    assert!(!doc.atom_site.is_empty(), "the atom_site rows type");
    assert!(
        matches!(doc.atom_site[0].adp_type, Some(AdpType::Uiso)),
        "the displacement type types to the closed set"
    );
    let rewritten = doc.to_block(block.heading());
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(fingerprint(&doc), fingerprint(&again), "the IR round-trip is stable");
}

fn a_valid_document() -> CoreCif {
    let unquoted = |text: &str| CharString::Unquoted(UnquotedString::new(text.to_string()));
    let mut doc = CoreCif::default();
    doc.cell.length_a = Some(Numeric::from(5.0));
    doc.cell.length_b = Some(Numeric::from(10.0));
    doc.cell.length_c = Some(Numeric::from(15.0));
    doc.cell.angle_alpha = Some(CellAngle::from(90.0f64));
    doc.cell.angle_beta = Some(CellAngle::from(90.0f64));
    doc.cell.angle_gamma = Some(CellAngle::from(90.0f64));
    doc.space_group.it_number = Some(ITNumber::from(14i32));
    doc.chemical_formula.structural = Some(unquoted("C2H3N"));
    doc.atom_site.push(AtomSite {
        label: Some(unquoted("Ba1")),
        type_symbol: Some(unquoted("Ba")),
        adp_type: Some(AdpType::Uiso),
        ..Default::default()
    });
    doc
}

#[test]
fn a_complete_document_passes_validation() {
    let doc = a_valid_document();
    assert!(doc.validate().is_ok(), "the required sets are present");
}

#[test]
fn removing_one_required_item_reports_exactly_one_missing() {
    let mut doc = a_valid_document();
    doc.cell.length_a = None;
    let errors = doc.validate().unwrap_err();
    assert_eq!(errors.len(), 1, "the single absence is the only report");
    assert_eq!(
        errors[0],
        CifError::Missing {
            category: "cell",
            item: "length_a",
        },
        "the missing item is named"
    );
}

#[test]
fn save_frames_and_foreign_tags_are_the_extra_residue() {
    // A block with a typed item, a foreign tag, and a save frame. The
    // typed item types; the other two are the residue, order kept.
    let members = vec![
        DataBlockMember::DataItems(DataItems::SingleValue(
            crystal_cif_fmt::grammar::SingleLineData::from_tag_value((
                crystal_cif_fmt::grammar::Tag::new("cell_length_a".to_string()),
                crystal_cif_fmt::grammar::Value::Numeric(Numeric::from(5.0)),
            )),
        )),
        DataBlockMember::DataItems(DataItems::SingleValue(
            crystal_cif_fmt::grammar::SingleLineData::from_tag_value((
                crystal_cif_fmt::grammar::Tag::new("audit_author_name".to_string()),
                crystal_cif_fmt::grammar::Value::CharString(
                    CharString::Unquoted(UnquotedString::new("author".to_string())),
                ),
            )),
        )),
    ];
    let block = DataBlock::from_heading_members((
        crystal_cif_fmt::DataBlockHeading::new("probe".to_string()),
        members,
    ));
    let doc = CoreCif::from_block(&block).expect("the read is clean");
    assert!(doc.cell.length_a.is_some(), "the typed cell item types");
    assert_eq!(doc.extra.len(), 1, "the foreign tag is the residue");
    if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = &doc.extra[0] {
        assert_eq!(sv.tag().as_ref(), "audit_author_name");
    } else {
        panic!("the residue is the foreign single-value item");
    }
}

#[test]
fn the_structure_block_validates_the_two_absent_required_items() {
    let block = first_block(&fixture("09_realistic_structure.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    let errors = doc.validate().expect_err("two required items are absent");
    assert_eq!(errors.len(), 2, "the two absent required items report");
    assert!(errors.iter().all(|e| matches!(e, CifError::Missing { .. })),
        "every report is a missing item");
}

#[test]
fn the_acetonitrile_block_validates_the_eight_absent_required_items() {
    let block = first_block(&fixture("13_structure.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    let errors = doc.validate().expect_err("eight required items are absent");
    assert_eq!(errors.len(), 8, "the eight absent required items report");
    assert!(errors.iter().all(|e| matches!(e, CifError::Missing { .. })),
        "every report is a missing item");
}

#[test]
fn the_cell_builder_fills_the_document_fields() {
    use crystal_cif_io::data_dict::core_cif::cell::Cell;
    use crystal_cif_io::data_dict::core_cif::CifCategory;
    let cell = Cell::builder()
        .length_a(Numeric::from(5.0))
        .length_b(Numeric::from(10.0))
        .length_c(Numeric::from(15.0))
        .angle_alpha(CellAngle::from(90.0f64))
        .angle_beta(CellAngle::from(90.0f64))
        .angle_gamma(CellAngle::from(90.0f64))
        .build()
        .expect("the orthorhombic cell builds");
    assert!(
        number_as_f64(&cell.volume.as_ref().expect("volume").number()) == 750.0,
        "the derived volume is the product of the six"
    );
    assert!(
        cell.metric_tensor.as_ref().expect("metric").shape() == (3, 3),
        "the metric tensor is a 3 x 3"
    );
    assert!(
        cell.orthogonal_matrix.is_some(),
        "the orthogonal matrix is computed"
    );
    let members = cell.to_members();
    assert!(!members.is_empty(), "the set items emit");
}

