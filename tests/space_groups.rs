//! Space-group coverage harness (test axis 1).
//!
//! The corpus in `tests/real_data/cod_sg/` holds one COD entry per
//! three-dimensional space group (1..230), fetched via the COD REST
//! API. `manifest.json` records the provenance (entry id, group
//! number, Hermann-Mauguin symbol, candidate list). See
//! `tests/real_data/cod_sg/SOURCE.md` and `docs/test-axes.md`.
//!
//! This test is deliberately lighter than the canonical-pin
//! harness: it pins parseability, validator cleanliness, and
//! space-group tag consistency for every group. It does not pin
//! the full value content of the 230 files.

use std::fs;
use std::path::PathBuf;

use crystal_cif_io::{canonical, parse_cif, CifDocument, DataBlock, DataBlockMember, Value};

use serde_json::Value as Json;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/real_data/cod_sg")
}

struct Group {
    sg: u32,
    file: String,
}

fn load_manifest(dir: &PathBuf) -> (Vec<Group>, Vec<u32>) {
    let raw = fs::read_to_string(dir.join("manifest.json"))
        .expect("read cod_sg manifest");
    let manifest: Json = serde_json::from_str(&raw).expect("manifest must be valid JSON");
    let mut groups: Vec<Group> = Vec::new();
    for entry in manifest["groups"].as_array().expect("manifest groups array") {
        groups.push(Group {
            sg: entry["sg"].as_u64().expect("manifest sg") as u32,
            file: entry["file"].as_str().expect("manifest file").to_string(),
        });
    }
    let gaps: Vec<u32> = manifest["gaps"]
        .as_array()
        .map(|a| a.iter().map(|g| g.as_u64().expect("gap sg") as u32).collect())
        .unwrap_or_default();
    (groups, gaps)
}

/// The space-group item names the corpus files may carry. CIF tag
/// names compare case-insensitively; the list holds the conventional
/// spellings lowercased.
// Tag names compare without the leading underscore; the parser
// stores the data name without it.
const IT_NUMBER_TAGS: &[&str] =
    &["space_group_it_number", "symmetry_int_tables_number"];
const HM_NAME_TAGS: &[&str] = &[
    "symmetry_space_group_name_h-m",
    "space_group_name_h-m",
    "space_group_name_h-m_alt",
];

/// Walk a block (and its save frames) for a lowercased tag name.
fn find_value<'a>(
    block: &'a DataBlock,
    target: &str,
) -> Option<&'a Value> {
    let target = target.to_lowercase();
    for member in block.members() {
        match member {
            DataBlockMember::DataItems(items) => {
                if let Some(data) = items.as_single_value() {
                    if data.tag().as_str().to_lowercase() == target {
                        return Some(data.value());
                    }
                }
                if let Some(cols) = items.as_multi_values() {
                    if let Some(col) = cols
                        .columns()
                        .iter()
                        .find(|c| c.tag().as_str().to_lowercase() == target)
                    {
                        return col.values().first();
                    }
                }
            }
            DataBlockMember::SaveFrame(frame) => {
                for items in frame.data_items() {
                    if let Some(data) = items.as_single_value() {
                        if data.tag().as_str().to_lowercase() == target {
                            return Some(data.value());
                        }
                    }
                }
            }
        }
    }
    None
}

fn parse_value_text(value: &Value) -> String {
    if let Some(numeric) = value.as_numeric() {
        return numeric.to_string();
    }
    if let Some(text) = value.as_char_string() {
        return text.to_string();
    }
    value.to_string()
}

#[test]
fn space_group_coverage() {
    let dir = corpus_dir();
    let (groups, gaps) = load_manifest(&dir);

    // Axis invariant: every one of the 230 groups occurs exactly
    // once.
    let sgs: Vec<u32> = groups.iter().map(|g| g.sg).collect();
    let mut expected: Vec<u32> = (1..=230).collect();
    expected.sort();
    let mut sorted_sgs = sgs.clone();
    sorted_sgs.sort();
    assert_eq!(
        sorted_sgs, expected,
        "manifest must cover space groups 1..230 exactly once"
    );

    let mut failures: Vec<String> = Vec::new();
    let mut parsed = 0u32;

    for group in &groups {
        let name = group.file.clone();
        let path = dir.join(&name);
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                failures.push(
                    format!("sg {sg:3}: missing file {name}: {e}", sg = group.sg),
                );
                continue;
            }
        };

        let doc = match parse_cif(&content) {
            Ok(doc) => doc,
            Err(errors) => {
                failures.push(format!(
                    "sg {sg:3}: parser rejected {name}: {errors:?}",
                    sg = group.sg
                ));
                continue;
            }
        };
        let doc: &CifDocument = &doc;

        let violations = canonical::validate(doc);
        if !violations.is_empty() {
            failures.push(format!(
                "sg {sg:3}: validator rejected {name}: {violations:?}",
                sg = group.sg
            ));
            continue;
        }
        parsed += 1;

        let blocks = match doc.data_blocks() {
            Some(blocks) if !blocks.is_empty() => blocks,
            _ => {
                failures.push(
                    format!("sg {sg:3}: no data block in {name}", sg = group.sg),
                );
                continue;
            }
        };

        // The IT-number tag must agree with the group of the
        // manifest entry.
        let mut it_found: Option<i32> = None;
        let mut hm_present = false;
        for block in blocks {
            for tag in IT_NUMBER_TAGS {
                if let Some(value) = find_value(block, tag) {
                    it_found = parse_value_text(value).trim().parse::<i32>().ok();
                    break;
                }
            }
            if it_found.is_some() {
                break;
            }
        }
        for block in blocks {
            for tag in HM_NAME_TAGS {
                if find_value(block, tag).is_some() {
                    hm_present = true;
                    break;
                }
            }
            if hm_present {
                break;
            }
        }

        if it_found != Some(group.sg as i32) {
            failures.push(format!(
                "sg {:3}: IT-number tag in {name} is {it_found:?}, expected {}",
                group.sg, group.sg
            ));
        }
        if !hm_present {
            failures.push(format!(
                "sg {:3}: no Hermann-Mauguin name tag in {name}",
                group.sg
            ));
        }
    }

    if !gaps.is_empty() {
        failures.push(format!(
            "manifest records unfetchable space groups: {gaps:?}"
        ));
    }

    println!(
        "space groups: {parsed}/230 parsed clean; {}",
        if failures.is_empty() {
            "all tag checks passed".to_string()
        } else {
            "see failures below".to_string()
        }
    );
    assert!(
        failures.is_empty(),
        "space-group corpus failures:\n{failures}",
        failures = failures.join("\n")
    );
}
