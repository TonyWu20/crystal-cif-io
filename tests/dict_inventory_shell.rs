//! Shell inventory test (roadmap T6, section 7).
//!
//! The recorded shell inventory (`tests/dict_inventory_shell.json`,
//! from `tests/dict/cif_core_shell.dic`) is the T6 source. The test
//! pins its counts, checks the full T6 tag coverage in the central
//! registry (`src/data_dict/tags.rs`), and verifies that the
//! registry adds nothing beyond the standard inventory plus the T6
//! items. The `_su` items are out of scope per decision D2: their
//! tags must stay out of the registry, and the skipped set is
//! printed.

use std::collections::HashSet;
use std::path::PathBuf;

use crystal_cif_io::data_dict::tags;
use serde_json::Value;

/// Implemented tags whose items the recorded dictionaries no longer
/// define (the DDL1 archive changelog records both removals). They
/// stay registered for legacy output.
const KNOWN_DICTIONARY_DRIFT: &[&str] = &["space_group_id", "space_group_symop_sg_id"];

fn load(path: &std::path::Path) -> Value {
    let raw =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

fn shell_items() -> Vec<Value> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/dict_inventory_shell.json");
    load(&path)["items"]
        .as_array()
        .cloned()
        .expect("items array")
}

fn standard_items() -> Vec<Value> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/dict_inventory.json");
    load(&path)["items"]
        .as_array()
        .cloned()
        .expect("items array")
}

fn lowercase_names(items: &[Value]) -> HashSet<String> {
    items
        .iter()
        .map(|i| i["name"].as_str().expect("item name").to_ascii_lowercase())
        .collect()
}

/// T6 items: shell items the standard inventory lacks.
fn t6_items() -> Vec<Value> {
    let std_names = lowercase_names(&standard_items());
    shell_items()
        .into_iter()
        .filter(|i| {
            let key = i["name"].as_str().expect("item name").to_ascii_lowercase();
            !std_names.contains(&key)
        })
        .collect()
}

#[test]
fn recorded_counts_hold() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/dict_inventory_shell.json");
    let inv = load(&path);
    let counts = &inv["counts"];
    assert_eq!(
        counts["total_items"].as_u64().expect("total_items"),
        1139,
        "the recorded item count moved"
    );
    assert_eq!(
        counts["categories"].as_u64().expect("categories"),
        91,
        "the recorded category count moved"
    );
    assert_eq!(
        counts["t6_items"].as_u64().expect("t6_items"),
        453,
        "the recorded T6 split moved"
    );
    assert_eq!(
        counts["t6_su_items"].as_u64().expect("t6_su_items"),
        191,
        "the recorded T6 _su split moved"
    );
    assert_eq!(
        counts["t6_implemented"].as_u64().expect("t6_implemented"),
        262,
        "the recorded T6 implementation count moved"
    );
    // The split must be internally consistent.
    assert_eq!(
        counts["t6_items"].as_u64().unwrap(),
        counts["t6_su_items"].as_u64().unwrap() + counts["t6_implemented"].as_u64().unwrap()
    );
}

/// The registry tag set, lowercased: the dictionary writes
/// conventional case, the registry stores the crate tag spelling.
fn registry_tags() -> HashSet<String> {
    tags::ALL.iter().map(|t| t.to_ascii_lowercase()).collect()
}

#[test]
fn every_t6_item_has_a_registry_tag() {
    let all = registry_tags();
    let t6 = t6_items();
    let su: Vec<&Value> = t6
        .iter()
        .filter(|i| {
            i["name"]
                .as_str()
                .unwrap()
                .to_ascii_lowercase()
                .ends_with("_su")
        })
        .collect();
    let impl_items: Vec<&Value> = t6
        .iter()
        .filter(|i| {
            !i["name"]
                .as_str()
                .unwrap()
                .to_ascii_lowercase()
                .ends_with("_su")
        })
        .collect();
    assert_eq!(su.len(), 191, "the _su split moved");
    assert_eq!(impl_items.len(), 262, "the T6 implementation split moved");

    let mut missing: Vec<String> = Vec::new();
    for item in &impl_items {
        let tag = item["name"]
            .as_str()
            .expect("item name")
            .to_ascii_lowercase();
        if !all.contains(&tag) {
            missing.push(tag);
        }
    }
    assert!(
        missing.is_empty(),
        "T6 items without a registry tag: {missing:?}"
    );
    println!(
        "T6 items with a registry tag: {}; _su items skipped per D2: {}",
        impl_items.len(),
        su.len()
    );
}

#[test]
fn the_su_items_stay_out_of_the_registry() {
    let all = registry_tags();
    let mut skipped: Vec<String> = Vec::new();
    for item in t6_items().iter().filter(|i| {
        i["name"]
            .as_str()
            .unwrap()
            .to_ascii_lowercase()
            .ends_with("_su")
    }) {
        let tag = item["name"]
            .as_str()
            .expect("item name")
            .to_ascii_lowercase();
        assert!(
            !all.contains(&tag),
            "{tag} is a T6 _su tag; D2 keeps the _su items out of the registry"
        );
        skipped.push(tag);
    }
    println!("skipped _su tags: {}", skipped.len());
}

#[test]
fn the_registry_adds_nothing_beyond_standard_plus_t6() {
    let std_names = lowercase_names(&standard_items());
    let t6_names: HashSet<String> = t6_items()
        .iter()
        .map(|i| i["name"].as_str().expect("item name").to_ascii_lowercase())
        .collect();
    let mut extras: Vec<String> = Vec::new();
    for tag in tags::ALL {
        let key = tag.to_ascii_lowercase();
        if !std_names.contains(&key)
            && !t6_names.contains(&key)
            && !KNOWN_DICTIONARY_DRIFT.contains(&key.as_str())
        {
            extras.push(tag.to_string());
        }
    }
    assert!(
        extras.is_empty(),
        "registry tags in neither inventory: {extras:?}"
    );
}
