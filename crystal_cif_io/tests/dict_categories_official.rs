//! Scope gate for the typed struct layer (roadmap D12i).
//!
//! The IUCr category view of the CIF 1.1 standard dictionary is
//! the scope authority for the struct layer. This test pins it
//! (`tests/dict_categories_official.json`) and checks that the
//! recorded DDL1 inventory carries exactly that category set.
//! A changed official list or a drifted inventory fails here.

use std::collections::BTreeSet;
use std::path::PathBuf;

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn load(path: &std::path::Path) -> Value {
    let raw =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

#[test]
fn the_official_category_set_matches_the_inventory() {
    let official = load(&root().join("tests/dict_categories_official.json"));
    let official_cats: BTreeSet<String> = official["categories"]
        .as_array()
        .expect("categories array")
        .iter()
        .map(|c| c.as_str().expect("category name").to_string())
        .collect();

    let inventory = load(&root().join("tests/dict_inventory.json"));
    let recorded: BTreeSet<String> = inventory["items"]
        .as_array()
        .expect("items array")
        .iter()
        .map(|i| i["category"].as_str().expect("category name").to_string())
        .collect();

    assert_eq!(
        official_cats.len(),
        62,
        "the official list holds 62 categories"
    );
    let missing: Vec<&String> = official_cats
        .iter()
        .filter(|c| !recorded.contains(*c))
        .collect();
    assert!(
        missing.is_empty(),
        "official categories absent from the recorded inventory: {missing:?}"
    );
    let extra: Vec<&String> = recorded
        .iter()
        .filter(|c| !official_cats.contains(*c))
        .collect();
    assert!(
        extra.is_empty(),
        "recorded inventory categories absent from the official list: {extra:?}"
    );
}
