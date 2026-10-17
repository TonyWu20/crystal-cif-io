//! The T2 wave round-trip (roadmap D12j).
//!
//! Real files that carry tier tags type the twelve T2 category groups.
//! Read, rewrite, re-read, and the typed fields and the `extra` residue
//! stay stable. The residue rule and the closed-set rule hold too.

use crystal_cif_fmt::grammar::{SingleLineData, Tag, Value};
use crystal_cif_fmt::{
    parse_cif, CharString, DataBlock, DataBlockHeading, DataBlockMember, DataItems, Numeric,
    SaveFrame, SaveFrameHeading, UnquotedString,
};

use crystal_cif_io::core_cif::CoreCif;
use crystal_cif_io::data_dict::core_cif::CifCategory;

fn file(name: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the worktree root");
    let path = root.join("crystal_cif_fmt/tests/").join(name);
    std::fs::read_to_string(path).expect("the file reads")
}

fn block_at(text: &str, index: usize) -> DataBlock {
    let doc = parse_cif(text).expect("the file parses");
    let blocks = doc.data_blocks().expect("the blocks are present");
    blocks.get(index).expect("a block at the index").clone()
}

/// A canonical signature of the twelve T2 typed groups and the `extra`
/// residue. Stable across an IR round-trip when no typed value is lost.
fn fingerprint(doc: &CoreCif) -> String {
    let groups: Vec<Vec<DataBlockMember>> = vec![
        doc.exptl.to_members(),
        doc.exptl_crystal.to_members(),
        doc.exptl_crystal_face.to_members(),
        doc.refln.to_members(),
        doc.reflns.to_members(),
        doc.reflns_class.to_members(),
        doc.reflns_scale.to_members(),
        doc.reflns_shell.to_members(),
        doc.refine.to_members(),
        doc.refine_ls_class.to_members(),
        doc.symmetry.to_members(),
        doc.symmetry_equiv.to_members(),
    ];
    let mut s = String::new();
    for group in &groups {
        for m in group {
            s += &format!("{m}\n");
        }
    }
    for m in &doc.extra {
        s += &format!("{m}\n");
    }
    s
}

/// An extra single-value member holds the given tag.
fn has_single_in_extra(doc: &CoreCif, tag: &str) -> bool {
    doc.extra.iter().any(|m| {
        if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
            sv.tag().as_ref() == tag
        } else {
            false
        }
    })
}

#[test]
fn the_reflns_fixture_types_the_whole_block() {
    let block = block_at(&file("fixtures/14_reflns.cif"), 0);
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    assert!(
        doc.exptl.crystals_number.is_some(),
        "the crystal count types"
    );
    assert!(
        doc.exptl.absorpt_correction_type.is_some(),
        "the correction type types"
    );
    assert!(
        doc.reflns.number_total.is_some(),
        "the reflection count types"
    );
    assert!(
        doc.refine.ls_hydrogen_treatment.is_some(),
        "the hydrogen treatment types"
    );
    // The file spells the tag with an upper R. The read lowercases the
    // tag and the value still types.
    assert!(
        doc.refine.ls_r_factor_gt.is_some(),
        "the case-variant tag types"
    );
    assert!(!doc.refln.is_empty(), "the refln rows type");
    assert!(
        doc.symmetry.cell_setting.is_some(),
        "the cell setting types"
    );
    assert!(doc.extra.is_empty(), "the whole block types");
    let rewritten = doc.to_block();
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        fingerprint(&doc),
        fingerprint(&again),
        "the IR round-trip is stable"
    );
}

#[test]
fn the_cod_sg_file_round_trips_the_tier_groups() {
    let block = block_at(&file("real_data/cod_sg/sg019_1000001.cif"), 0);
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    assert!(
        doc.exptl.absorpt_coefficient_mu.is_some(),
        "the absorption coefficient types"
    );
    assert!(
        doc.exptl.absorpt_correction_type.is_some(),
        "the correction type types"
    );
    assert!(
        doc.refine.ls_hydrogen_treatment.is_some(),
        "the hydrogen treatment types"
    );
    assert!(
        doc.reflns.number_total.is_some(),
        "the reflection count types"
    );
    assert!(!doc.symmetry_equiv.is_empty(), "the equiv rows type");
    assert!(
        doc.symmetry_equiv[0].pos_as_xyz.is_some(),
        "the equivalence position types"
    );
    assert!(
        doc.symmetry.cell_setting.is_some(),
        "the cell setting types"
    );
    // The file records the loop categories as single items. Those tags
    // stay in the residue, order kept (the D12j deviation record).
    assert!(
        has_single_in_extra(&doc, "exptl_crystal_colour"),
        "the single crystal items stay in extra"
    );
    assert!(
        has_single_in_extra(&doc, "exptl_absorpt_correction_T_min"),
        "the single transmission items stay in extra"
    );
    assert!(
        has_single_in_extra(&doc, "reflns_limit_h_max"),
        "the single limit items stay in extra"
    );
    assert!(
        has_single_in_extra(&doc, "refine_diff_density_max"),
        "the single diff density items stay in extra"
    );
    let rewritten = doc.to_block();
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        fingerprint(&doc),
        fingerprint(&again),
        "the IR round-trip is stable"
    );
}

#[test]
fn the_iucr_example_round_trips_the_tier_groups() {
    // The file carries two blocks. The compound block holds the tier
    // single items. The global block holds only audit and journal tags.
    let block = block_at(&file("real_data/iucr/example_single.cif"), 1);
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    assert!(
        doc.exptl.absorpt_coefficient_mu.is_some(),
        "the absorption coefficient types"
    );
    assert!(
        doc.exptl.absorpt_correction_type.is_some(),
        "the correction type types"
    );
    assert!(
        doc.reflns.number_total.is_some(),
        "the reflection count types"
    );
    assert!(
        doc.refine.ls_weighting_scheme.is_some(),
        "the weighting scheme types"
    );
    assert!(!doc.symmetry_equiv.is_empty(), "the equiv rows type");
    assert!(
        doc.symmetry.cell_setting.is_some(),
        "the cell setting types"
    );
    // The file records the loop categories as single items. Those tags
    // stay in the residue, order kept (the D12j deviation record).
    assert!(
        has_single_in_extra(&doc, "exptl_absorpt_correction_T_min"),
        "the single transmission items stay in extra"
    );
    assert!(
        has_single_in_extra(&doc, "exptl_crystal_colour"),
        "the single crystal items stay in extra"
    );
    assert!(
        has_single_in_extra(&doc, "refine_diff_density_max"),
        "the single diff density items stay in extra"
    );
    let rewritten = doc.to_block();
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        fingerprint(&doc),
        fingerprint(&again),
        "the IR round-trip is stable"
    );
}

#[test]
fn the_tier_save_frame_and_foreign_tag_are_the_residue() {
    // A typed tier item, a tier save frame, and a foreign tag. The
    // typed item types. The other two are the residue, order kept.
    let unquoted = |text: &str| CharString::Unquoted(UnquotedString::new(text.to_string()));
    let members = vec![
        DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
            Tag::new("symmetry_cell_setting".to_string()),
            Value::CharString(unquoted("monoclinic")),
        )))),
        DataBlockMember::SaveFrame(SaveFrame::from_heading_items((
            SaveFrameHeading::new("reflns_scale_I_obsd".to_string()),
            vec![DataItems::SingleValue(SingleLineData::from_tag_value((
                Tag::new("reflns_scale_FH".to_string()),
                Value::Numeric(Numeric::from(10.0)),
            )))],
        ))),
        DataBlockMember::DataItems(DataItems::SingleValue(SingleLineData::from_tag_value((
            Tag::new("audit_author_name".to_string()),
            Value::CharString(unquoted("author")),
        )))),
    ];
    let block =
        DataBlock::from_heading_members((DataBlockHeading::new("probe".to_string()), members));
    let doc = CoreCif::from_block(&block).expect("the read is clean");
    assert!(
        doc.symmetry.cell_setting.is_some(),
        "the typed tier item types"
    );
    assert_eq!(
        doc.extra.len(),
        2,
        "the save frame and the foreign tag are the residue"
    );
    match (&doc.extra[0], &doc.extra[1]) {
        (DataBlockMember::SaveFrame(frame), DataBlockMember::DataItems(_)) => {
            assert_eq!(frame.heading().as_ref(), "reflns_scale_I_obsd");
        }
        _ => panic!("the residue keeps the save frame before the foreign tag"),
    }
    if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = &doc.extra[1] {
        assert_eq!(sv.tag().as_ref(), "audit_author_name");
    } else {
        panic!("the residue is the foreign single-value item");
    }
}

#[test]
fn the_cell_setting_closed_set_rejects_an_outside_code() {
    use crystal_cif_io::data_dict::core_cif::symmetry::CellSetting;
    for code in [
        "triclinic",
        "monoclinic",
        "orthorhombic",
        "tetragonal",
        "rhombohedral",
        "trigonal",
        "hexagonal",
        "cubic",
    ] {
        let setting = CellSetting::try_from(code).expect("the dictionary code parses");
        assert_eq!(setting.dictionary_value(), code, "the spelling keeps");
    }
    assert!(
        CellSetting::try_from("cubic_system").is_err(),
        "the outside code rejects"
    );
    assert!(
        CellSetting::try_from("MONOCLINIC").is_ok(),
        "the read is case-insensitive"
    );
}
