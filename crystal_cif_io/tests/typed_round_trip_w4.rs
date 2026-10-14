//! The tier T4 typed document layer round-trip (roadmap D12).
//!
//! `CoreCif` reads a data block into the sixteen T4 category structs
//! and writes them back. The guarantee is IR round-trip: read, write,
//! re-read, and the typed fields and the `extra` residue are
//! unchanged. Save frames and foreign tags land in `extra`, order
//! kept.

use crystal_cif_fmt::grammar::{CharString, Numeric, SingleLineData, Tag, UnquotedString, Value};
use crystal_cif_fmt::{
    parse_cif, DataBlock, DataBlockHeading, DataBlockMember, DataItems, SaveFrame, SaveFrameHeading,
};

use crystal_cif_io::core_cif::{CifError, CoreCif};
use crystal_cif_io::data_dict::core_cif::journal_index::Type;
use crystal_cif_io::data_dict::core_cif::publ::RequestedCategory;

fn corpus_file(name: &str) -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crystal_cif_fmt/tests/real_data/"
    )
    .to_string()
        + name;
    std::fs::read_to_string(&path).expect("the corpus file reads")
}

fn first_block(cif: &str) -> DataBlock {
    let doc = parse_cif(cif).expect("the corpus file parses");
    doc.data_blocks()
        .expect("a data block is present")
        .first()
        .expect("a first block")
        .clone()
}

/// Read, rewrite, re-read. The emitted block text is the signature of
/// every typed field and the `extra` residue. A lost or moved value
/// changes the text.
fn round_trip_is_stable(block: &DataBlock) -> CoreCif {
    let doc = CoreCif::from_block(block).expect("the typed read is clean");
    let rewritten = doc.to_block(block.heading());
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        rewritten.to_string(),
        again.to_block(block.heading()).to_string(),
        "the IR round-trip is stable"
    );
    doc
}

#[test]
fn the_publcif_file_round_trips_the_t4_tier() {
    let block = first_block(&corpus_file("iucr/example_publcif.cif"));
    let doc = round_trip_is_stable(&block);
    // The audit items type.
    assert!(
        doc.audit.creation_method.is_some(),
        "the audit creation method is typed"
    );
    // The closed set types to the enum.
    assert_eq!(
        doc.publ.requested_category,
        Some(RequestedCategory::FA),
        "the requested category is a closed-set code"
    );
    // The loop rows type.
    assert!(!doc.publ_body.is_empty(), "the publ_body rows type");
    assert!(
        matches!(
            doc.publ_body[0].element,
            Some(crystal_cif_io::data_dict::core_cif::publ_body::Element::Section)
        ),
        "the body element parses the closed set"
    );
    assert!(
        !doc.publ_author_rows.is_empty(),
        "the publ_author rows type"
    );
    // The single-value journal items of the file stay in the residue.
    assert!(!doc.extra.is_empty(), "the untyped members land in extra");
}

#[test]
fn the_cod_file_round_trips_the_t4_tier() {
    let block = first_block(&corpus_file("cod/1000000.cif"));
    let doc = round_trip_is_stable(&block);
    assert!(
        !doc.publ_author_rows.is_empty(),
        "the publ_author rows type"
    );
    // The journal and computing items of the file are single-value
    // lines. They are loop items of the tier and stay in the residue.
    assert!(!doc.extra.is_empty(), "the untyped members land in extra");
}

#[test]
fn the_squeeze_file_round_trips_the_t4_tier() {
    let block = first_block(&corpus_file("iucr/example_squeeze.cif"));
    let doc = round_trip_is_stable(&block);
    assert!(
        doc.audit.creation_method.is_some(),
        "the audit creation method is typed"
    );
    // A text-field value reads into the shared string type.
    assert!(
        doc.publ.contact_author_address.is_some(),
        "the text-field address types"
    );
    assert!(
        !doc.publ_author_rows.is_empty(),
        "the publ_author rows type"
    );
}

#[test]
fn save_frames_and_foreign_tags_are_the_extra_residue() {
    // A typed tier item, a foreign tag, and a save frame that carries
    // a tier item. The typed item types. The other two are the
    // residue, order kept.
    let members = vec![
        DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
            Tag::new("audit_creation_date".to_string()),
            Value::CharString(CharString::Unquoted(UnquotedString::new(
                "recorded now".to_string(),
            ))),
        )))),
        DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
            Tag::new("cod_database_code".to_string()),
            Value::Numeric(Numeric::from(1000000i32)),
        )))),
        DataBlockMember::SaveFrame(SaveFrame::from_heading_items((
            SaveFrameHeading::new("publ_body".to_string()),
            vec![DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("publ_body_element".to_string()),
                Value::CharString(CharString::Unquoted(UnquotedString::new(
                    "section".to_string(),
                ))),
            )))],
        ))),
    ];
    let block =
        DataBlock::from_heading_members((DataBlockHeading::new("probe".to_string()), members));
    let doc = CoreCif::from_block(&block).expect("the read is clean");
    assert!(doc.audit.creation_date.is_some(), "the typed item types");
    assert_eq!(
        doc.extra.len(),
        2,
        "the foreign tag and the save frame are the residue"
    );
    match (&doc.extra[0], &doc.extra[1]) {
        (DataBlockMember::DataItems(_), DataBlockMember::SaveFrame(frame)) => {
            assert_eq!(
                frame.heading().as_ref(),
                "publ_body",
                "the frame survives with its tier item"
            );
        }
        _ => panic!("the residue keeps the foreign tag before the frame"),
    }
    if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = &doc.extra[0] {
        assert_eq!(sv.tag().as_ref(), "cod_database_code");
    } else {
        panic!("the residue is the foreign single-value item");
    }
}

#[test]
fn the_journal_index_type_enum_parses_the_dictionary_codes() {
    for code in ["O", "I", "M", "S"] {
        assert_eq!(
            Type::parse(code).unwrap().to_string(),
            code,
            "the code {code} parses"
        );
    }
    assert!(Type::parse("X").is_none(), "the outside code is rejected");
    // The read path records a value-class error for the outside code.
    let rows = vec![DataBlockMember::DataItems(DataItems::MultiValues(
        crystal_cif_fmt::grammar::LoopColumns::new(vec![
            crystal_cif_fmt::grammar::LoopColumn::new(
                Tag::new("journal_index_subterm".to_string()),
                vec![Value::CharString(CharString::Unquoted(
                    UnquotedString::new("term".to_string()),
                ))],
            ),
            crystal_cif_fmt::grammar::LoopColumn::new(
                Tag::new("journal_index_type".to_string()),
                vec![Value::CharString(CharString::Unquoted(
                    UnquotedString::new("X".to_string()),
                ))],
            ),
        ]),
    ))];
    let block = DataBlock::from_heading_members((DataBlockHeading::new("probe".to_string()), rows));
    let errors = CoreCif::from_block(&block).expect_err("the outside code fails");
    assert!(
        errors.iter().any(|e| matches!(
            e,
            CifError::ValueClass {
                tag,
                ..
            } if tag == "journal_index_type"
        )),
        "the closed-set mismatch reports"
    );
}
