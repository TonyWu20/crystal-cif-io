//! The typed document layer round-trip of the T5 wave (roadmap D12,
//! tier T5: the raw-diffraction categories).
//!
//! `CoreCif` reads a data block into the T5 typed category structs
//! and writes them back. The guarantee is IR round-trip: read,
//! write, read again, and the typed fields and the `extra` residue
//! are unchanged. Save frames and foreign tags land in `extra`,
//! order kept.

use crystal_cif_fmt::grammar::{Numeric, UnquotedString};
use crystal_cif_fmt::{
    parse_cif, CharString, DataBlock, DataBlockHeading, DataBlockMember, DataItems, SaveFrame,
    SaveFrameHeading,
};

use crystal_cif_io::core_cif::{CifError, CoreCif};
use crystal_cif_io::data_dict::core_cif::diffrn_refln::ScanMode;
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

fn real_data(corpus: &str, name: &str) -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crystal_cif_fmt/tests/real_data/"
    )
    .to_string()
        + corpus
        + "/"
        + name;
    std::fs::read_to_string(&path).expect("the real-data file reads")
}

fn first_block(cif: &str) -> DataBlock {
    let doc = parse_cif(cif).expect("the file parses");
    doc.data_blocks()
        .expect("a data block is present")
        .first()
        .expect("a first block")
        .clone()
}

/// A signature of the T5 typed fields and the `extra` residue.
/// Stable across an IR round-trip when no typed value is lost.
fn fingerprint(doc: &CoreCif) -> String {
    let mut s = String::new();
    let num = |o: &Option<Numeric>| {
        o.as_ref()
            .map(|n| format!("{:.6}", number_as_f64(&n.number())))
            .unwrap_or_else(|| "absent".to_string())
    };
    let nt = |o: Option<f64>| {
        o.map(|v| format!("{v:.6}"))
            .unwrap_or_else(|| "absent".to_string())
    };
    let chr = |o: &Option<CharString>| {
        o.as_ref()
            .map(|c| c.content().to_string())
            .unwrap_or_else(|| "absent".to_string())
    };
    let amb = doc.diffrn.ambient_temperature.as_ref().map(|v| v.value().0);
    let area = doc
        .diffrn_detector
        .area_resol_mean
        .as_ref()
        .map(|v| v.value().0);
    let dtime = doc.diffrn_detector.dtime.as_ref().map(|v| v.value().0);
    let rad_dtime = doc
        .diffrn_detector
        .diffrn_radiation_detector_dtime
        .as_ref()
        .map(|v| v.value().0);
    let volt = doc.diffrn_source.voltage.as_ref().map(|v| v.value().0);
    let reflns_n = doc.diffrn_reflns.number.as_ref().map(|v| v.value().0);
    let theta_max = doc.diffrn_reflns.theta_max.as_ref().map(|v| v.value().0);
    let theta_min = doc.diffrn_reflns.theta_min.as_ref().map(|v| v.value().0);
    let res_max = doc
        .diffrn_reflns
        .resolution_max
        .as_ref()
        .map(|v| v.value().0);
    s += &format!(
        "diffrn[ambient_temperature={} crystal_treatment={} special_details={}]\n",
        nt(amb),
        chr(&doc.diffrn.crystal_treatment),
        chr(&doc.diffrn.special_details),
    );
    s += &format!(
        "diffrn_measurement[device={} method={} specimen_support={}]\n",
        chr(&doc.diffrn_measurement.device),
        chr(&doc.diffrn_measurement.method),
        chr(&doc.diffrn_measurement.specimen_support),
    );
    s += &format!(
        "diffrn_detector[detector={} area={} dtime={} rad_dtime={}]\n",
        chr(&doc.diffrn_detector.diffrn_detector),
        nt(area),
        nt(dtime),
        nt(rad_dtime),
    );
    s += &format!(
        "diffrn_radiation[monochromator={} type={} probe={} xray_symbol={}]\n",
        chr(&doc.diffrn_radiation.monochromator),
        chr(&doc.diffrn_radiation.r#type),
        doc.diffrn_radiation
            .probe
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_else(|| "absent".to_string()),
        doc.diffrn_radiation
            .xray_symbol
            .as_ref()
            .map(|x| x.to_string())
            .unwrap_or_else(|| "absent".to_string()),
    );
    s += &format!(
        "diffrn_source[radiation_source={} target={} voltage={}]\n",
        chr(&doc.diffrn_source.diffrn_radiation_source),
        doc.diffrn_source
            .target
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or_else(|| "absent".to_string()),
        nt(volt),
    );
    s += &format!(
        "diffrn_reflns[number={} theta_max={} theta_min={} resolution_max={}]\n",
        nt(reflns_n),
        nt(theta_max),
        nt(theta_min),
        nt(res_max),
    );
    for row in &doc.diffrn_refln {
        s += &format!(
            "refln[{}/{}/{}|omega={}|i={} s={} m={}]\n",
            num(&row.index_h),
            num(&row.index_k),
            num(&row.index_l),
            num(&row.angle_omega),
            row.intensity_net
                .as_ref()
                .map(|v| v.value().0)
                .map(|v| format!("{v:.6}"))
                .unwrap_or_else(|| "absent".to_string()),
            row.intensity_sigma
                .as_ref()
                .map(|v| v.value().0)
                .map(|v| format!("{v:.6}"))
                .unwrap_or_else(|| "absent".to_string()),
            row.scan_mode
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "absent".to_string()),
        );
    }
    s += &format!("refln[rows={}]\n", doc.diffrn_refln.len());
    for row in &doc.diffrn_orient_refln {
        s += &format!(
            "orient[{}/{}/{}|omega={}]\n",
            num(&row.index_h),
            num(&row.index_k),
            num(&row.index_l),
            num(&row.angle_omega),
        );
    }
    s += &format!("orient[rows={}]\n", doc.diffrn_orient_refln.len());
    for m in &doc.extra {
        s += &format!("{m}\n");
    }
    s
}

#[test]
fn the_diffrn_fixture_round_trips_the_tier_items() {
    let block = first_block(&fixture("15_diffrn.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    // The typed single-value items of the tier.
    assert!(
        doc.diffrn_measurement.device.is_some(),
        "the measurement device is typed"
    );
    assert!(
        doc.diffrn_radiation.monochromator.is_some(),
        "the monochromator is typed"
    );
    assert!(
        doc.diffrn.ambient_temperature.is_some(),
        "the ambient temperature is typed"
    );
    // The two tier loops type to the row groups.
    assert_eq!(doc.diffrn_refln.len(), 4, "the refln rows type");
    assert_eq!(
        doc.diffrn_orient_refln.len(),
        3,
        "the orient refln rows type"
    );
    assert!(
        matches!(doc.diffrn_refln[0].scan_mode, Some(ScanMode::Om)),
        "the scan mode codes to the closed set"
    );
    let rewritten = doc.to_block(block.heading());
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        fingerprint(&doc),
        fingerprint(&again),
        "the IR round-trip is stable"
    );
    assert!(
        doc.extra.is_empty(),
        "the fixture is fully typed by the tier"
    );
}

#[test]
fn the_shelxl_block_round_trips_the_single_value_items() {
    let block = first_block(&real_data("iucr", "example_shelxl2013.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    assert!(
        doc.diffrn.ambient_temperature.is_some(),
        "the ambient temperature is typed"
    );
    assert!(
        doc.diffrn.measured_fraction_theta_full.is_some(),
        "the measured fraction types to the clamped newtype"
    );
    assert!(
        doc.diffrn_reflns.number.is_some(),
        "the reflection count is typed"
    );
    let rewritten = doc.to_block(block.heading());
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        fingerprint(&doc),
        fingerprint(&again),
        "the IR round-trip is stable"
    );
    assert!(!doc.extra.is_empty(), "the untyped members land in extra");
}

#[test]
fn the_cod_block_round_trips_the_radiation_and_source_items() {
    let block = first_block(&real_data("cod", "1000000.cif"));
    let doc = CoreCif::from_block(&block).expect("the typed read is clean");
    assert!(
        doc.diffrn_source.diffrn_radiation_source.is_some(),
        "the radiation source is typed"
    );
    assert!(
        doc.diffrn_radiation.r#type.is_some(),
        "the radiation type is typed"
    );
    assert!(
        doc.diffrn_measurement.device.is_some(),
        "the measurement device is typed"
    );
    assert!(
        doc.diffrn_reflns.number.is_some(),
        "the reflection count is typed"
    );
    let rewritten = doc.to_block(block.heading());
    let again = CoreCif::from_block(&rewritten).expect("the rewrite re-reads");
    assert_eq!(
        fingerprint(&doc),
        fingerprint(&again),
        "the IR round-trip is stable"
    );
}

#[test]
fn the_foreign_tag_and_tier_save_frame_are_the_extra_residue() {
    // A block with a typed T5 item, a save frame of a tier category,
    // and a foreign tag. The typed item types; the frame and the
    // foreign tag are the residue, order kept.
    let members = vec![
        DataBlockMember::DataItems(DataItems::SingleValue(
            crystal_cif_fmt::grammar::SingleLineData::from_tag_value((
                crystal_cif_fmt::grammar::Tag::new("diffrn_standards_interval_count".to_string()),
                crystal_cif_fmt::grammar::Value::Numeric(Numeric::from(5.0)),
            )),
        )),
        DataBlockMember::SaveFrame(SaveFrame::from_heading_items((
            SaveFrameHeading::new("diffrn_reflns_class".to_string()),
            vec![DataItems::SingleValue(
                crystal_cif_fmt::grammar::SingleLineData::from_tag_value((
                    crystal_cif_fmt::grammar::Tag::new("diffrn_reflns_class_code".to_string()),
                    crystal_cif_fmt::grammar::Value::CharString(CharString::Unquoted(
                        UnquotedString::new("1".to_string()),
                    )),
                )),
            )],
        ))),
        DataBlockMember::DataItems(DataItems::SingleValue(
            crystal_cif_fmt::grammar::SingleLineData::from_tag_value((
                crystal_cif_fmt::grammar::Tag::new("audit_author_name".to_string()),
                crystal_cif_fmt::grammar::Value::CharString(CharString::Unquoted(
                    UnquotedString::new("author".to_string()),
                )),
            )),
        )),
    ];
    let block =
        DataBlock::from_heading_members((DataBlockHeading::new("probe".to_string()), members));
    let doc = CoreCif::from_block(&block).expect("the read is clean");
    assert!(
        doc.diffrn_standards.interval_count.is_some(),
        "the typed standards item types"
    );
    assert_eq!(
        doc.extra.len(),
        2,
        "the frame and the foreign tag are the residue"
    );
    match (&doc.extra[0], &doc.extra[1]) {
        (DataBlockMember::SaveFrame(frame), DataBlockMember::DataItems(_)) => {
            assert_eq!(
                frame.heading().as_ref(),
                "diffrn_reflns_class",
                "the frame survives"
            );
        }
        _ => panic!("the residue keeps the frame before the foreign tag"),
    }
    if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = &doc.extra[1] {
        assert_eq!(sv.tag().as_ref(), "audit_author_name");
    } else {
        panic!("the residue is the foreign single-value item");
    }
}

#[test]
fn the_scan_mode_enum_parses_the_dictionary_codes() {
    for code in ["om", "ot", "q"] {
        assert_eq!(
            ScanMode::parse(code).unwrap().to_string(),
            code,
            "the code {code} parses"
        );
    }
    assert!(
        ScanMode::parse("xx").is_none(),
        "the outside code is rejected"
    );
    assert!(
        ScanMode::parse("OM").is_none(),
        "the codes are case-sensitive"
    );
}

#[test]
fn the_outside_scan_mode_code_reports_a_value_class_error() {
    // A tier loop with one outside scan-mode code. The read reports
    // the value-class mismatch and never stops at the first one.
    let members = vec![DataBlockMember::DataItems(DataItems::MultiValues(
        crystal_cif_fmt::grammar::LoopColumns::new(vec![
            crystal_cif_fmt::grammar::LoopColumn::new(
                crystal_cif_fmt::grammar::Tag::new("diffrn_refln_index_h".to_string()),
                vec![crystal_cif_fmt::grammar::Value::Numeric(Numeric::from(1.0))],
            ),
            crystal_cif_fmt::grammar::LoopColumn::new(
                crystal_cif_fmt::grammar::Tag::new("diffrn_refln_scan_mode".to_string()),
                vec![crystal_cif_fmt::grammar::Value::CharString(
                    CharString::Unquoted(UnquotedString::new("no-scan".to_string())),
                )],
            ),
        ]),
    ))];
    let block =
        DataBlock::from_heading_members((DataBlockHeading::new("probe".to_string()), members));
    let errors = CoreCif::from_block(&block).expect_err("the outside code fails the read");
    assert_eq!(errors.len(), 1, "the one mismatch reports");
    assert!(
        matches!(
            errors[0],
            CifError::ValueClass {
                ref tag,
                expected: "ScanMode",
                found: "CharString",
            } if tag == "diffrn_refln_scan_mode"
        ),
        "the report names the tag and the class"
    );
}
