//! The typed document layer round-trip of tier T3 (roadmap D12).
//!
//! The T3 categories of the `CoreCif` view read a data block and
//! write it back. The guarantee is IR round-trip: read, write,
//! read again, and the typed fields and the `extra` residue are
//! unchanged. Save frames and foreign tags land in `extra`, order
//! kept.

use crystal_cif_fmt::grammar::{DataBlock, Numeric, UnquotedString};
use crystal_cif_fmt::{
    parse_cif, CharString, DataBlockHeading, DataBlockMember, DataItems, LoopColumn, LoopColumns,
    SaveFrame, SaveFrameHeading, SingleLineData, Tag, Value,
};
use crystal_cif_io::core_cif::CoreCif;
use crystal_cif_io::data_dict::core_cif::atom_sites::SolutionMethod;
use crystal_cif_io::data_dict::core_cif::chemical_conn_bond::BondType;
use crystal_cif_io::data_dict::core_cif::range_check::number_as_f64;

fn fixture(name: &str) -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crystal_cif_fmt/tests/fixtures/"
    )
    .to_string()
        + name;
    std::fs::read_to_string(&path).expect("the fixture reads")
}

fn corpus(dir: &str, name: &str) -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crystal_cif_fmt/tests/real_data/"
    )
    .to_string()
        + dir
        + "/"
        + name;
    std::fs::read_to_string(&path).expect("the corpus file reads")
}

fn first_block(cif: &str) -> DataBlock {
    let doc = parse_cif(cif).expect("the file parses");
    doc.data_blocks()
        .expect("a data block is present")
        .first()
        .expect("a first block")
        .clone()
}

fn block_by_name(cif: &str, name: &str) -> DataBlock {
    let doc = parse_cif(cif).expect("the file parses");
    doc.data_blocks()
        .expect("a data block is present")
        .iter()
        .find(|b| b.heading() == name)
        .expect("the named block is present")
        .clone()
}

fn unq(text: &str) -> CharString {
    CharString::Unquoted(UnquotedString::new(text.to_string()))
}

/// A compact signature of the T3 typed fields and the `extra`
/// residue. Stable across an IR round-trip when no typed value is
/// lost.
fn t3_fingerprint(doc: &CoreCif) -> String {
    let char_of = |o: Option<CharString>| {
        o.map(|c| c.content().to_string())
            .unwrap_or_else(|| "absent".to_string())
    };
    let num_of = |o: Option<Numeric>| {
        o.map(|n| format!("{:.6}", number_as_f64(&n.number())))
            .unwrap_or_else(|| "absent".to_string())
    };
    let code_of = |o: Option<SolutionMethod>| {
        o.map(|c| c.to_string())
            .unwrap_or_else(|| "absent".to_string())
    };
    let mut s = String::new();
    s += &format!(
        "atom_sites[primary={} hydrogens={} axes={} rows={}]\n",
        code_of(doc.atom_sites.solution_primary),
        code_of(doc.atom_sites.solution_hydrogens),
        char_of(doc.atom_sites.cartn_transform_axes.clone()),
        doc.atom_sites.rows.len(),
    );
    s += &format!(
        "atom_type[rows={} symbol={} scat={}]\n",
        doc.atom_type.rows.len(),
        char_of(doc.atom_type.rows.first().and_then(|r| r.symbol.clone())),
        char_of(
            doc.atom_type.rows
                .first()
                .and_then(|r| r.scat_source.clone())
        ),
    );
    s += &format!(
        "chemical[name={} melting={} inchi={}]\n",
        char_of(doc.chemical.name_common.clone()),
        num_of(doc.chemical.melting_point.map(|t| t.value())),
        char_of(doc.chemical.identifier_inchi.clone()),
    );
    s += &format!(
        "chemical[rows={} gt={}]\n",
        doc.chemical.rows.len(),
        num_of(
            doc.chemical.rows
                .first()
                .and_then(|r| r.melting_point_gt)
                .map(|t| t.value())
        ),
    );
    s += &format!(
        "conn_atom[rows={} number={} charge={}]\n",
        doc.chemical_conn_atom.len(),
        num_of(
            doc.chemical_conn_atom
                .first()
                .and_then(|r| r.number)
                .map(|n| n.value())
        ),
        num_of(
            doc.chemical_conn_atom
                .first()
                .and_then(|r| r.charge)
                .map(|n| n.value())
        ),
    );
    s += &format!(
        "conn_bond[rows={} type={} dist={}]\n",
        doc.chemical_conn_bond.rows.len(),
        doc.chemical_conn_bond.rows
            .first()
            .and_then(|r| r.r#type)
            .map(|t| t.to_string())
            .unwrap_or_else(|| "absent".to_string()),
        num_of(doc.chemical_conn_bond.chem_comp_bond_value_dist),
    );
    s += &format!(
        "geom[details={} incr={}]\n",
        char_of(doc.geom.special_details.clone()),
        num_of(doc.geom.bond_distance_incr),
    );
    s += &format!(
        "geom_angle[rows={} angle={} flag={}]\n",
        doc.geom_angle.rows.len(),
        num_of(doc.geom_angle.rows.first().and_then(|r| r.angle)),
        doc.geom_angle.rows
            .first()
            .and_then(|r| r.publ_flag)
            .map(|f| f.to_string())
            .unwrap_or_else(|| "absent".to_string()),
    );
    s += &format!(
        "geom_bond[rows={} distance={} id={}]\n",
        doc.geom_bond_rows.len(),
        num_of(
            doc.geom_bond_rows
                .first()
                .and_then(|r| r.distance)
                .map(|d| d.value())
        ),
        char_of(doc.geom_bond.id.clone()),
    );
    s += &format!(
        "geom_contact[rows={} distance={}]\n",
        doc.geom_contact_rows.len(),
        num_of(
            doc.geom_contact_rows
                .first()
                .and_then(|r| r.distance)
                .map(|d| d.value())
        ),
    );
    s += &format!(
        "geom_hbond[rows={} dha={} flag={}]\n",
        doc.geom_hbond_rows.len(),
        num_of(doc.geom_hbond_rows.first().and_then(|r| r.angle_dha)),
        doc.geom_hbond_rows
            .first()
            .and_then(|r| r.publ_flag)
            .map(|f| f.to_string())
            .unwrap_or_else(|| "absent".to_string()),
    );
    s += &format!(
        "geom_torsion[rows={} value={}]\n",
        doc.geom_torsion_rows.len(),
        num_of(doc.geom_torsion_rows.first().and_then(|r| r.torsion)),
    );
    s += &format!(
        "valence_param[rows={} B={} citation={}]\n",
        doc.valence_param_rows.len(),
        num_of(doc.valence_param_rows.first().and_then(|r| r.b)),
        char_of(doc.valence_param.citation_id.clone()),
    );
    s += &format!(
        "valence_ref[rows={} reference={}]\n",
        doc.valence_ref.len(),
        char_of(doc.valence_ref.first().and_then(|r| r.reference.clone())),
    );
    for m in &doc.extra {
        s += &format!("{m}\n");
    }
    s
}

#[test]
fn the_structure_fixture_round_trips_the_t3_categories() {
    let block = first_block(&fixture("13_structure.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    // Every T3 category of the fixture types.
    assert!(!doc.atom_type.rows.is_empty(), "the atom_type loop types");
    assert!(
        !doc.chemical_conn_atom.is_empty(),
        "the chemical_conn_atom loop types"
    );
    assert!(
        !doc.chemical_conn_bond.rows.is_empty(),
        "the chemical_conn_bond loop types"
    );
    assert!(
        !doc.geom_angle.rows.is_empty()
            && !doc.geom_bond_rows.is_empty()
            && !doc.geom_contact_rows.is_empty()
            && !doc.geom_hbond_rows.is_empty()
            && !doc.geom_torsion_rows.is_empty(),
        "the geom special-list loops type"
    );
    assert!(
        !doc.valence_param_rows.is_empty() && !doc.valence_ref.is_empty(),
        "the valence loops type"
    );
    assert!(
        doc.chemical.name_common.is_some(),
        "the chemical single items type"
    );
    assert!(
        doc.atom_sites.solution_primary.is_some(),
        "the atom_sites solution codes type"
    );
    let rewritten = doc.to_block();
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        t3_fingerprint(&doc),
        t3_fingerprint(&again),
        "the IR round-trip is stable"
    );
}

#[test]
fn the_iucr_example_round_trips_the_t3_categories() {
    let block = block_by_name(&corpus("iucr", "example_VRF.cif"), "I");
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    assert!(
        doc.chemical.name_common.is_some(),
        "the chemical single items type"
    );
    let rewritten = doc.to_block();
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        t3_fingerprint(&doc),
        t3_fingerprint(&again),
        "the IR round-trip is stable"
    );
}

#[test]
fn the_cod_entry_round_trips_the_t3_categories() {
    let block = first_block(&corpus("cod", "1000000.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    assert!(
        !doc.geom_bond_rows.is_empty()
            && !doc.geom_angle.rows.is_empty()
            && !doc.geom_hbond_rows.is_empty(),
        "the geom special-list loops type"
    );
    assert!(
        doc.geom_bond_rows.iter().any(|r| r.distance.is_some()),
        "the bond distance types through the range newtype"
    );
    let rewritten = doc.to_block();
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        t3_fingerprint(&doc),
        t3_fingerprint(&again),
        "the IR round-trip is stable"
    );
}

#[test]
fn save_frames_and_foreign_tags_are_the_t3_extra_residue() {
    // A block with a T3 single item, a T3 loop, a save frame, and a
    // foreign tag. The T3 items type. The frame and the foreign tag
    // are the residue, order kept.
    let members = vec![
        DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
            Tag::new("geom_bond_id".to_string()),
            Value::CharString(unq("B1")),
        )))),
        DataBlockMember::DataItems(DataItems::MultiValues(LoopColumns::new(vec![
            LoopColumn::new(
                Tag::new("geom_bond_atom_site_label_1".to_string()),
                vec![Value::CharString(unq("C1"))],
            ),
            LoopColumn::new(
                Tag::new("geom_bond_distance".to_string()),
                vec![Value::Numeric(Numeric::from(1.4))],
            ),
        ]))),
        DataBlockMember::SaveFrame(SaveFrame::from_heading_items((
            SaveFrameHeading::new("geom_bond".to_string()),
            vec![DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("geom_bond_valence".to_string()),
                Value::Numeric(Numeric::from(3.0)),
            )))],
        ))),
        DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
            Tag::new("my_foreign_tag".to_string()),
            Value::CharString(unq("test")),
        )))),
    ];
    let block =
        DataBlock::from_heading_members((DataBlockHeading::new("probe".to_string()), members));
    let doc = CoreCif::from_block(&block).expect("the read is clean");
    assert_eq!(
        doc.geom_bond.id.as_ref().map(|c| c.content()),
        Some("B1"),
        "the T3 single item types"
    );
    assert!(
        doc.geom_bond_rows.iter().any(|r| r.distance.is_some()),
        "the T3 loop types"
    );
    assert_eq!(
        doc.extra.len(),
        2,
        "the save frame and the foreign tag are the residue"
    );
    match (&doc.extra[0], &doc.extra[1]) {
        (DataBlockMember::SaveFrame(frame), DataBlockMember::DataItems(_)) => {
            assert_eq!(frame.heading().as_ref(), "geom_bond", "the frame survives");
        }
        _ => panic!("the residue keeps the save frame before the foreign tag"),
    }
    let rewritten = doc.to_block();
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        t3_fingerprint(&doc),
        t3_fingerprint(&again),
        "the residue round-trips in order"
    );
}

#[test]
fn the_wave_closed_sets_parse_the_dictionary_codes() {
    for code in ["sing", "doub", "trip", "quad", "arom", "poly", "delo", "pi"] {
        assert_eq!(
            BondType::parse(code)
                .expect("the code is in the set")
                .to_string(),
            code,
            "the bond type code {code}"
        );
    }
    assert!(
        BondType::parse("quud").is_none(),
        "the code is not in the set"
    );
    for code in [
        "difmap",
        "vecmap",
        "heavy",
        "direct",
        "geom",
        "disper",
        "isomor",
        "mixed",
        "notdet",
        "dual",
        "iterative",
        "other",
    ] {
        assert_eq!(
            SolutionMethod::parse(code)
                .expect("the code is in the set")
                .to_string(),
            code,
            "the solution method code {code}"
        );
    }
    assert!(
        SolutionMethod::parse("neutron").is_none(),
        "the code is not in the set"
    );
}
